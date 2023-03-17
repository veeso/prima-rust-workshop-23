use async_graphql::SimpleObject;

use super::{ArticleInOrder, NaiveDateTime, OrderStatus, Uuid};
use crate::proto::store_client::types::Order as ProtoOrder;

#[derive(SimpleObject, Clone, PartialEq, Eq)]
pub struct Order {
    id: Uuid,
    created_at: NaiveDateTime,
    status: OrderStatus,
    articles: Vec<ArticleInOrder>,
}

impl From<ProtoOrder> for Order {
    fn from(value: ProtoOrder) -> Self {
        Self {
            id: value.id.into(),
            created_at: value.created_at.into(),
            status: value.status.into(),
            articles: value
                .articles
                .into_iter()
                .map(ArticleInOrder::from)
                .collect(),
        }
    }
}
