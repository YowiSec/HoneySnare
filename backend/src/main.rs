use alloy_sol_types::sol;
use alloy_primitives::Address;
use alloy_sol_types::SolEvent;
use dotenv::dotenv;
use reqwest::Client;
use std::env;
use std::fs::{self, File, OpenOptions};
use std::path::Path;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::io::{Write, Read};
use hex;
use serde_json::Value;
use log::{info, warn, error, debug};
use chrono::Utc;

const LOG_DIR: &str = "logs";
const CURRENT_LOG_FILE: &str = "logs/current.json";
const ARCHIVE_DIR: &str = "logs/archive";
const LOG_THRESHOLD: usize = 3; // Set a low threshold for demonstration

sol! {
    #[derive(Debug)]
    event ExploitAttempt(address indexed bot, string action, uint256 amount, bool success);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    dotenv().ok();

    let eth_node = env::var("ARB_RPC_URL").expect("ARB_RPC_URL not set");
    println!("Connecting to Arbitrum node: {}", eth_node);

    let honeypot_address = match hex::decode("0x6d22de3D3C70F67C323F15975De19b53b7a73Dac") {
        Ok(bytes) => {
            if bytes.len() != 20 {
                println!("Invalid Arbitrum address length: {}", bytes.len());
                return Err("Invalid Arbitrum address length".into());
            }
            Address::from_slice(&bytes)
        },
        Err(e) => {
            println!("Failed to decode honeypot address: {}", e);
            return Err("Failed to decode honeypot address".into());
        }
    };

    println!("Monitoring Honeypot Address: 0x{}", hex::encode(honeypot_address.as_slice()));

    // Create necessary directories
    fs::create_dir_all(LOG_DIR)?;
    fs::create_dir_all(ARCHIVE_DIR)?;

    let mut log_count = 0;

    println!("Fetching logs...");
    match get_logs(&eth_node, &honeypot_address).await {
        Ok(logs) => {
            if logs.is_empty() {
                println!("No new logs found");
            } else {
                println!("Found {} new log(s)", logs.len());
                for log in logs {
                    println!("Processing log: {:?}", log);
                    let topics: Vec<[u8; 32]> = log.get("topics")
                        .and_then(|t| t.as_array())
                        .map(|arr| arr.iter()
                            .filter_map(|v| v.as_str())
                            .filter_map(|s| hex::decode(s.trim_start_matches("0x")).ok())
                            .filter_map(|bytes| bytes.try_into().ok())
                            .collect())
                        .unwrap_or_default();

                    let data = log.get("data")
                        .and_then(|d| d.as_str())
                        .map(|s| hex::decode(s.trim_start_matches("0x")))
                        .transpose()
                        .map_err(|e| format!("Failed to decode log data: {}", e))?
                        .unwrap_or_default();

                    match ExploitAttempt::decode_raw_log(topics, &data, true) {
                        Ok(event) => {
                            println!("Decoded event: {:?}", event);
                            let log_data = serde_json::json!({
                                "bot_address": format!("0x{}", hex::encode(event.bot.as_slice())),
                                "action": event.action,
                                "timestamp": Utc::now().timestamp(),
                                "amount": event.amount.to_string(),
                                "success": event.success
                            });

                            if let Err(e) = append_log(&log_data.to_string(), CURRENT_LOG_FILE) {
                                println!("Failed to append log: {}", e);
                            } else {
                                println!("Log data appended successfully");
                                log_count += 1;

                                if log_count >= LOG_THRESHOLD {
                                    if let Err(e) = archive_logs() {
                                        println!("Failed to archive logs: {}", e);
                                    } else {
                                        log_count = 0;
                                    }
                                }
                            }
                        },
                        Err(e) => {
                            println!("Failed to decode log data: {}", e);
                            continue;
                        }
                    }
                }
            }
        },
        Err(e) => {
            println!("Failed to fetch logs: {}", e);
        }
    }

    Ok(())
}

async fn get_logs(eth_node: &str, honeypot_address: &Address) -> Result<Vec<Value>, Box<dyn std::error::Error>> {
    let client = Client::new();
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "eth_getLogs",
        "params": [{
            "address": format!("0x{}", hex::encode(honeypot_address.as_slice())),
            "fromBlock": "latest",
            "topics": []
        }],
        "id": 1
    });

    println!("Sending request to Ethereum node: {:?}", payload);

    let response = client
        .post(eth_node)
        .json(&payload)
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(format!("Non-success status code: {}", response.status()).into());
    }

    let response_body: serde_json::Value = response.json().await?;
    println!("Received response: {:?}", response_body);
    
    if let Some(result) = response_body.get("result") {
        if result.is_array() {
            return Ok(result.as_array().unwrap().to_vec());
        } else if result.is_null() {
            return Ok(Vec::new());
        }
    }

    Err("Unexpected response format from Ethereum node".into())
}

fn append_log(log_data: &str, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(file_path);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)?;
    
    writeln!(file, "{}", log_data)?;
    Ok(())
}

fn archive_logs() -> Result<(), Box<dyn std::error::Error>> {
    println!("Archiving logs...");
    let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
    let archive_filename = format!("{}/logs_{}.gz", ARCHIVE_DIR, timestamp);

    // Read current logs
    let mut current_logs = String::new();
    File::open(CURRENT_LOG_FILE)?.read_to_string(&mut current_logs)?;

    // Compress current logs
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(current_logs.as_bytes())?;
    let compressed_logs = encoder.finish()?;

    // Write compressed logs to archive file
    fs::write(&archive_filename, compressed_logs)?;

    println!("Logs archived to: {}", archive_filename);

    // Clear current log file
    fs::write(CURRENT_LOG_FILE, "")?;

    println!("Current log file cleared");
    Ok(())
}