//! # Web server

use crate::proto::StoreClient;

use std::sync::Arc;

pub struct WebServer {
    store_client: Arc<StoreClient>,
}

impl WebServer {
    /// Initialize web server
    pub async fn init(protobuf_url: &str) -> anyhow::Result<Self> {
        debug!("initializing web server");
        let store_client = StoreClient::connect(protobuf_url.to_string()).await?;
        debug!("webserver initialized");

        Ok(Self {
            store_client: Arc::new(store_client),
        })
    }
}
