use async_graphql::SimpleObject;

use super::{Decimal, Uuid};

#[derive(SimpleObject, Clone, PartialEq, Eq)]
pub struct ArticleInOrder {
    id: Uuid,
    name: String,
    description: String,
    unit_price: Decimal,
    quantity: u64,
}
