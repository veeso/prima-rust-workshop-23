use uuid::Uuid;

use crate::{
    graphql::types::{Order, OrderArticle},
    proto::StoreClient,
};

use std::sync::Arc;

/// Submit order mutation
pub struct SubmitOrder {
    store_client: Arc<StoreClient>,
}

impl SubmitOrder {
    /// Instantiates a new `SubmitOrder`
    pub fn new(store_client: Arc<StoreClient>) -> Self {
        Self { store_client }
    }

    /// Resolve mutation for submit order
    pub async fn resolve(
        &self,
        user_id: Uuid,
        articles: Vec<OrderArticle>,
    ) -> async_graphql::Result<Order> {
        todo!()
    }
}
