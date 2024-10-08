use alloy_sol_types::sol;
use alloy_sol_types::sol_data::{Address, Uint};
use alloy_dyn_abi::Decoder;
use dotenv::dotenv;
use reqwest::Client;
use std::env;
use std::fs::File;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::io::Write;
use tokio::time::{sleep, Duration};
use hex;
use serde_json::Value;
use log::{info, warn, error};

// Define the Solidity type using the `sol!` macro
sol! {
    type LogData = (address, bytes, uint256, bool);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the logger
    env_logger::init();
    dotenv().ok();

    // Step 1: Load RPC URL from .env file
    let eth_node = env::var("ETH_RPC_URL").expect("ETH_RPC_URL not set");

    // Honeypot contract address
    let honeypot_address = match hex::decode("5FbDB2315678afecb367f032d93F642f64180aa3") {
        Ok(bytes) => {
            if bytes.len() != 20 {
                error!("Invalid Ethereum address length: {}", bytes.len());
                return Err("Invalid Ethereum address length".into());
            }
            Address::from_slice(&bytes)
        },
        Err(e) => {
            error!("Failed to decode honeypot address: {}", e);
            return Err("Failed to decode honeypot address".into());
        }
    };

    info!("Honeypot Address: {:?}", honeypot_address);

    // Infinite loop to keep fetching logs
    loop {
        // Fetch logs from the Ethereum node
        let logs = match get_logs(&eth_node, &honeypot_address).await {
            Ok(logs) => logs,
            Err(e) => {
                error!("Failed to fetch logs: {}", e);
                sleep(Duration::from_secs(10)).await;
                continue;
            }
        };

        // Iterate through the logs
        for log in logs {
            // Extract log data from the raw response
            let log_data_str = match log.get("data").and_then(|d| d.as_str()) {
                Some(data) => data,
                None => {
                    warn!("Failed to extract log data as string");
                    continue;
                }
            };

            // Decode the hex-encoded log data
            let data_bytes = match hex::decode(log_data_str) {
                Ok(bytes) => bytes,
                Err(e) => {
                    warn!("Failed to decode log data: {}", e);
                    continue;
                }
            };

            // Initialize the ABI decoder
            let mut decoder = Decoder::new(&data_bytes, true);

            // Decode the log data using the defined Rust type
            match decoder.decode::<LogData::RustType>() {
                Ok(decoded) => {
                    let (bot_address, action, amount, success) = decoded;

                    // Create a log structure for output
                    let log_data = serde_json::json!({
                        "bot_address": format!("{:?}", bot_address),   // Bot address as string
                        "action": hex::encode(action),                 // Convert action (bytes) to hex string
                        "timestamp": log.get("timestamp").unwrap_or(&Value::Null), // Log timestamp
                        "amount": format!("{}", amount),               // Convert amount to string
                        "success": success                             // Success or failure of the exploit as boolean
                    });

                    // Compress and save the log as JSON
                    if let Err(e) = compress_log(&log_data.to_string(), "logs/current/log.gz") {
                        warn!("Failed to compress and save log: {}", e);
                    } else {
                        info!("Log data compressed and saved successfully");
                    }
                },
                Err(e) => {
                    warn!("Failed to decode log data: {}", e);
                    continue;
                }
            }
        }

        // Sleep for a while before fetching logs again
        sleep(Duration::from_secs(10)).await;
    }
}

// Fetch logs from Ethereum node
async fn get_logs(eth_node: &str, honeypot_address: &Address) -> Result<Vec<Value>, Box<dyn std::error::Error>> {
    let client = Client::new();
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "eth_getLogs",
        "params": [{
            "address": format!("{:?}", honeypot_address),
            "fromBlock": "latest",
            "topics": []
        }],
        "id": 1
    });

    let response = client
        .post(eth_node)
        .json(&payload)  // Send request with the payload
        .send()
        .await?;

    // Check if the response is successful
    if !response.status().is_success() {
        return Err(format!("Non-success status code: {}", response.status()).into());
    }

    // Parse the logs from the response
    let logs: Vec<Value> = response.json().await?;
    Ok(logs)
}

// Function to compress log data and save it
fn compress_log(log_data: &str, compressed_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create(compressed_file)?;
    let mut encoder = GzEncoder::new(file, Compression::default());
    encoder.write_all(log_data.as_bytes())?;
    encoder.finish()?;
    Ok(())
}
