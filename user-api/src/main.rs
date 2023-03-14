const APP_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

#[macro_use]
extern crate serde;
#[macro_use]
extern crate tracing;

mod config;
mod graphql;
mod proto;
mod web;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    info!("user-api v{} - developed by {}", APP_VERSION, APP_AUTHORS);
    let config = config::Config::try_from_env()?;

    Ok(())
}
