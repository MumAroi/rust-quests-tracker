use quests_tracker::config::config_loader;
use tracing::{error, info};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let dotenvy = match config_loader::load() {
        Ok(env) => env,
        Err(err) => {
            error!("Error loading ENV: {}", err);
            std::process::exit(1);
        }
    };

    info!("ENV has been loaded");
}
