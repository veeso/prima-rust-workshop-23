use crate::{graphql::types::Order, proto::StoreClient};

use uuid::Uuid;

/// Orders query
pub struct Orders {
    store_server_url: String,
}

impl Orders {
    /// Instantiates a new `Orders`
    pub fn new(store_server_url: &str) -> Self {
        Self {
            store_server_url: store_server_url.to_string(),
        }
    }

    /// Resolve query orders
    pub async fn resolve(
        &self,
        user_id: Uuid,
        page: u32,
        count: u32,
    ) -> async_graphql::Result<Vec<Order>> {
        let mut client = StoreClient::connect(self.store_server_url.clone()).await?;

        let orders = client
            .query_orders(user_id, page, count)
            .await?
            .into_iter()
            .map(Order::from)
            .collect();

        Ok(orders)
    }
}
