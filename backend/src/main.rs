use dotenv::dotenv;
use std::error::Error;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables
    dotenv().ok();

    // Verify all required environment variables are set
    if let Err(e) = backend::verify_environment() {
        eprintln!("Environment setup error: {}", e);
        std::process::exit(1);
    }

    let configs = backend::load_chain_configs()?;

    println!("Starting HoneySnare log monitor...");
    println!("Monitoring chains:");
    for config in &configs {
        if config.enabled {
            let status = match &config.honeypot_address {
                Some(addr) => format!("configured with address {}", addr),
                None => "waiting for deployment".to_string(),
            };
            println!("- {} ({}) using {}", config.chain, status, config.rpc_url_env);
        }
    }

    loop {
        for config in &configs {
            if !config.enabled {
                continue;
            }

            match backend::fetch_chain_logs(config).await {
                Ok(logs) => {
                    for log in logs {
                        if let Err(e) = backend::write_log(&log) {
                            eprintln!("Error writing log from {}: {}", config.chain, e);
                        }
                    }
                }
                Err(e) => eprintln!("Error fetching logs from {}: {}", config.chain, e),
            }
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
    }
}