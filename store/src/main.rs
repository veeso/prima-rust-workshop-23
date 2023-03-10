const APP_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

#[macro_use]
extern crate tracing;

mod database;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    info!("producer v{} - developed by {}", APP_VERSION, APP_AUTHORS);

    Ok(())
}
