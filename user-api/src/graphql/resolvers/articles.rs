use crate::{graphql::types::Article, proto::ProtobufClient};

use std::sync::Arc;

/// Articles query
pub struct Articles {
    protobuf_client: Arc<ProtobufClient>,
}

impl Articles {
    /// Instantiates a new `Articles`
    pub fn new(protobuf_client: Arc<ProtobufClient>) -> Self {
        Self { protobuf_client }
    }

    /// Resolve query articles
    pub async fn resolve(
        &self,
        query: Option<String>,
        page: u64,
        count: u64,
    ) -> async_graphql::Result<Vec<Article>> {
        todo!()
    }
}
