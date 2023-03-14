use async_graphql::SimpleObject;

use super::{Decimal, Uuid};

#[derive(SimpleObject, Clone, PartialEq, Eq)]
pub struct Article {
    id: Uuid,
    name: String,
    description: String,
    unit_price: Decimal,
}
