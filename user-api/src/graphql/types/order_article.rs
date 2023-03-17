//! # Order article

use async_graphql::InputObject;

use super::Uuid;
use crate::proto::store_client::types::OrderedArticle as ProtoOrderArticle;

#[derive(InputObject)]
pub struct OrderArticle {
    id: Uuid,
    quantity: u32,
}

impl From<ProtoOrderArticle> for OrderArticle {
    fn from(value: ProtoOrderArticle) -> Self {
        Self {
            id: value.id.into(),
            quantity: value.quantity,
        }
    }
}

impl From<OrderArticle> for ProtoOrderArticle {
    fn from(value: OrderArticle) -> Self {
        Self {
            id: value.id.uuid(),
            quantity: value.quantity,
        }
    }
}
