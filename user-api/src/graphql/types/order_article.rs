//! # Order article

use async_graphql::InputObject;

use super::Uuid;

#[derive(InputObject)]
pub struct OrderArticle {
    id: Uuid,
    quantity: u64,
}

impl OrderArticle {
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn quantity(&self) -> u64 {
        self.quantity
    }
}
