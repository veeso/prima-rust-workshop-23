use crate::{graphql::types::Article, proto::StoreClient};

use std::sync::Arc;

/// Articles query
pub struct Articles {
    store_client: Arc<StoreClient>,
}

impl Articles {
    /// Instantiates a new `Articles`
    pub fn new(store_client: Arc<StoreClient>) -> Self {
        Self { store_client }
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
