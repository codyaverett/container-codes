mod server;
mod handlers;
mod middleware;
mod static_files;

use anyhow::Result;
use container_codes_shared::{config::Config, logging::init_logging};
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::load_from_env()?;
    
    init_logging(&config.logging)?;
    
    info!("Starting Container Codes Server v{}", env!("CARGO_PKG_VERSION"));
    
    if let Err(e) = server::start(config).await {
        error!("Server failed to start: {}", e);
        std::process::exit(1);
    }
    
    Ok(())
}