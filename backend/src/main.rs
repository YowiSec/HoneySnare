#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    
    let configs = backend::load_chain_configs()?;

    println!("Starting HoneySnare daily log check...");
    
    // Single check of all enabled chains
    for config in &configs {
        if config.enabled {
            if let Ok(logs) = backend::fetch_chain_logs(config).await {
                for log in logs {
                    if let Err(e) = backend::write_log(&log) {
                        eprintln!("Error writing log: {}", e);
                    }
                }
            }
        }
    }

    println!("Log check complete.");
    Ok(())
}
