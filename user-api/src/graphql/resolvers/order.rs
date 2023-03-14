use crate::{graphql::types::Order, proto::ProtobufClient};

use std::sync::Arc;
use uuid::Uuid;

/// Orders query
pub struct Orders {
    protobuf_client: Arc<ProtobufClient>,
}

impl Orders {
    /// Instantiates a new `Orders`
    pub fn new(protobuf_client: Arc<ProtobufClient>) -> Self {
        Self { protobuf_client }
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
