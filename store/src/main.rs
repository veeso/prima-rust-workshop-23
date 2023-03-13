const APP_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

#[macro_use]
extern crate serde;
#[macro_use]
extern crate tracing;

mod config;
mod database;
mod service;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    info!("producer v{} - developed by {}", APP_VERSION, APP_AUTHORS);

    let config = config::Config::try_from_env()?;
    info!("configuration parsed");

    info!("initializing store service");
    let service =
        service::StoreService::configure(&config.server_url, &config.database_url).await?;
    info!("store service is ready");
    service.run().await?;

    Ok(())
}
