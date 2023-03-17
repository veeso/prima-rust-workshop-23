//! # Web server

use crate::proto::StoreClient;

use std::sync::Arc;

pub struct WebServer {
    store_client_url: String,
}

impl WebServer {
    /// Initialize web server
    pub async fn init(protobuf_url: &str) -> anyhow::Result<Self> {
        debug!("webserver initialized");

        Ok(Self {
            store_client_url: protobuf_url.to_string(),
        })
    }
}
