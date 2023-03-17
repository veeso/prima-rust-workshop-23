use uuid::Uuid;

use crate::{
    graphql::types::{OrderArticle, OrderSubmission},
    proto::{store_client::types::OrderedArticle, StoreClient},
};

/// Submit order mutation
pub struct SubmitOrder {
    store_server_url: String,
}

impl SubmitOrder {
    /// Instantiates a new `SubmitOrder`
    pub fn new(store_server_url: String) -> Self {
        Self {
            store_server_url: store_server_url.to_string(),
        }
    }

    /// Resolve mutation for submit order
    pub async fn resolve(
        &self,
        user_id: Uuid,
        articles: Vec<OrderArticle>,
    ) -> async_graphql::Result<OrderSubmission> {
        let mut client = StoreClient::connect(self.store_server_url.clone()).await?;
        let submit_result = client
            .submit_order(
                user_id,
                articles.into_iter().map(OrderedArticle::from).collect(),
            )
            .await?;

        Ok(submit_result.into())
    }
}
