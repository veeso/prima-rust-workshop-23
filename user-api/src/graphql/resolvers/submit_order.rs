use uuid::Uuid;

use crate::{
    graphql::types::{Order, OrderArticle},
    proto::ProtobufClient,
};

use std::sync::Arc;

/// Submit order mutation
pub struct SubmitOrder {
    protobuf_client: Arc<ProtobufClient>,
}

impl SubmitOrder {
    /// Instantiates a new `SubmitOrder`
    pub fn new(protobuf_client: Arc<ProtobufClient>) -> Self {
        Self { protobuf_client }
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
