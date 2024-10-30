use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use chrono::Utc;
use reqwest::Client;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::io::Write;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChainConfig {
    pub chain: String,
    pub rpc_url_env: String,
    pub honeypot_address: Option<String>,
    pub enabled: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogEntry {
    pub chain: String,
    pub attacker: String,
    pub action: String,
    pub amount: String,
    pub timestamp: i64,
    pub tx_hash: String,
}

// Get RPC URL from environment with better error message
fn get_rpc_url(chain: &str, env_var: &str) -> Result<String, Box<dyn Error>> {
    std::env::var(env_var).map_err(|_| {
        format!(
            "Missing RPC URL for {}. Ensure {} is set in GitHub Actions secrets",
            chain, env_var
        )
        .into()
    })
}

pub async fn verify_contract_deployed(config: &ChainConfig) -> Result<bool, Box<dyn Error>> {
    let address = match &config.honeypot_address {
        Some(addr) => addr,
        None => return Ok(false),
    };

    let client = Client::new();
    let rpc_url = get_rpc_url(&config.chain, &config.rpc_url_env)?;

    let response = client
        .post(&rpc_url)
        .json(&serde_json::json!({
            "jsonrpc": "2.0",
            "method": "eth_getCode",
            "params": [address, "latest"],
            "id": 1
        }))
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    let code = response.get("result")
        .and_then(|r| r.as_str())
        .unwrap_or("0x");
    
    Ok(code != "0x" && code != "0x0")
}

pub fn write_log(entry: &LogEntry) -> Result<(), Box<dyn Error>> {
    let log_file = "logs/current.json";
    fs::create_dir_all("logs/archive")?;

    let log_line = serde_json::to_string(&entry)?;
    fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_file)?
        .write_all(format!("{}\n", log_line).as_bytes())?;

    // Check if we should archive (e.g., file size > 1MB)
    if fs::metadata(log_file)?.len() > 1_000_000 {
        archive_logs()?;
    }

    Ok(())
}

fn archive_logs() -> Result<(), Box<dyn Error>> {
    let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
    let archive_path = format!("logs/archive/logs_{}.gz", timestamp);
    
    // Read current logs
    let logs = fs::read_to_string("logs/current.json")?;
    if !logs.trim().is_empty() {
        // Create gzip archive
        let file = fs::File::create(&archive_path)?;
        let mut encoder = GzEncoder::new(file, Compression::default());
        encoder.write_all(logs.as_bytes())?;
        encoder.finish()?;

        // Clear current log file
        fs::write("logs/current.json", "")?;
    }

    Ok(())
}

pub async fn fetch_chain_logs(config: &ChainConfig) -> Result<Vec<LogEntry>, Box<dyn Error>> {
    if !config.enabled || config.honeypot_address.is_none() {
        return Ok(Vec::new());
    }

    let client = Client::new();
    let rpc_url = get_rpc_url(&config.chain, &config.rpc_url_env)?;

    // Verify contract is deployed
    if !verify_contract_deployed(config).await? {
        println!("No contract deployed on {} yet, skipping...", config.chain);
        return Ok(Vec::new());
    }

    let logs_response = client
        .post(&rpc_url)
        .json(&serde_json::json!({
            "jsonrpc": "2.0",
            "method": "eth_getLogs",
            "params": [{
                "address": config.honeypot_address.as_ref().unwrap(),
                "fromBlock": "0x0",
                "toBlock": "latest"
            }],
            "id": 1
        }))
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    let mut entries = Vec::new();
    if let Some(logs) = logs_response.get("result").and_then(|r| r.as_array()) {
        for log in logs {
            if let Some(entry) = parse_log(log, &config.chain) {
                entries.push(entry);
            }
        }
    }

    Ok(entries)
}

fn parse_log(log: &serde_json::Value, chain: &str) -> Option<LogEntry> {
    let tx_hash = log.get("transactionHash")?.as_str()?.to_string();
    let data = log.get("data")?.as_str()?.to_string();
    let topics = log.get("topics")?.as_array()?;

    Some(LogEntry {
        chain: chain.to_string(),
        attacker: topics.get(1)
            .and_then(|t| t.as_str())
            .unwrap_or("unknown")
            .to_string(),
        action: "interaction".to_string(),
        amount: data,
        timestamp: Utc::now().timestamp(),
        tx_hash,
    })
}

pub fn load_chain_configs() -> Result<Vec<ChainConfig>, Box<dyn Error>> {
    let configs = vec![
        ChainConfig {
            chain: "arbitrum".to_string(),
            rpc_url_env: "ARB_RPC_URL".to_string(),
            honeypot_address: Some("0xf693DAC3dF95a731FA169C3aFAE6e0C3c416AF47".to_string()),
            enabled: true,
        },
        ChainConfig {
            chain: "optimism".to_string(),
            rpc_url_env: "OP_RPC_URL".to_string(),
            honeypot_address: None,
            enabled: false,
        },
        ChainConfig {
            chain: "base".to_string(),
            rpc_url_env: "BASE_RPC_URL".to_string(),
            honeypot_address: None,
            enabled: false,
        },
        ChainConfig {
            chain: "blast".to_string(),
            rpc_url_env: "BLAST_RPC_URL".to_string(),
            honeypot_address: None,
            enabled: false,
        },
        ChainConfig {
            chain: "solana".to_string(),
            rpc_url_env: "SOL_RPC_URL".to_string(),
            honeypot_address: None,
            enabled: false,  // Will be enabled when Solana honeypot is ready
        },
    ];

    Ok(configs)
}

pub fn verify_environment() -> Result<(), Box<dyn Error>> {
    // First get active chains
    let configs = load_chain_configs()?;
    
    // Only check RPC URLs for enabled chains
    let mut missing = Vec::new();
    for config in configs {
        if config.enabled {
            if std::env::var(&config.rpc_url_env).is_err() {
                missing.push((&config.rpc_url_env, &config.chain));
            }
        }
    }

    if !missing.is_empty() {
        let error_msg = missing
            .iter()
            .map(|(var, chain)| format!("{} ({} RPC URL)", var, chain))
            .collect::<Vec<_>>()
            .join(", ");
        
        return Err(format!(
            "Missing required environment variables: {}. Please set these in GitHub Actions secrets.",
            error_msg
        ).into());
    }

    Ok(())
}
