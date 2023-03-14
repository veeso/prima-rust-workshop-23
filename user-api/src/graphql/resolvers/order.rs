use crate::{graphql::types::Order, proto::StoreClient};

use std::sync::Arc;
use uuid::Uuid;

/// Orders query
pub struct Orders {
    store_client: Arc<StoreClient>,
}

impl Orders {
    /// Instantiates a new `Orders`
    pub fn new(store_client: Arc<StoreClient>) -> Self {
        Self { store_client }
    }

    /// Resolve query orders
    pub async fn resolve(
        &self,
        user_id: Uuid,
        page: u64,
        count: u64,
    ) -> async_graphql::Result<Vec<Order>> {
        todo!()
    }
}
