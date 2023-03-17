use async_graphql::SimpleObject;

use super::{Decimal, Uuid};
use crate::proto::store_client::types::Article as ProtoArticle;

#[derive(SimpleObject, Clone, PartialEq, Eq)]
pub struct Article {
    id: Uuid,
    name: String,
    description: String,
    unit_price: Decimal,
}

impl From<ProtoArticle> for Article {
    fn from(value: ProtoArticle) -> Self {
        Self {
            id: value.id.into(),
            name: value.name,
            description: value.description,
            unit_price: value.unit_price.into(),
        }
    }
}
