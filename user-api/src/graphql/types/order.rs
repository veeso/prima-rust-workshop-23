use async_graphql::SimpleObject;

use super::{ArticleInOrder, NaiveDateTime, OrderStatus, Uuid};

#[derive(SimpleObject, Clone, PartialEq, Eq)]
pub struct Order {
    id: Uuid,
    created_at: NaiveDateTime,
    status: OrderStatus,
    articles: Vec<ArticleInOrder>,
}
