use async_graphql::SimpleObject;

use super::{Decimal, Uuid};
use crate::proto::store_client::types::OrderArticle as ProtoArticleInOrder;

#[derive(SimpleObject, Clone, PartialEq, Eq)]
pub struct ArticleInOrder {
    id: Uuid,
    name: String,
    description: String,
    unit_price: Decimal,
    quantity: u32,
}

impl From<ProtoArticleInOrder> for ArticleInOrder {
    fn from(value: ProtoArticleInOrder) -> Self {
        Self {
            id: value.article.id.into(),
            name: value.article.name,
            description: value.article.description,
            unit_price: value.article.unit_price.into(),
            quantity: value.quantity,
        }
    }
}
