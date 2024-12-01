use axum_poc::config::config_loader;

use tracing::{error, info};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
    .with_max_level(tracing::Level::DEBUG)
    .init();

    let dotenny_env = match config_loader::load() {
        Ok(env) => env,
        Err(err) => {
            error!("Failed to load config: {}", err);
            std::process::exit(1);
        }
    };

    info!("Loaded config: {:#?}", dotenny_env);
}
