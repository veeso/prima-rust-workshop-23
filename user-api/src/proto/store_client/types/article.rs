use rust_decimal::Decimal;
use std::str::FromStr;
use uuid::Uuid;

use crate::proto::SyntaxError;

pub struct Article {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub unit_price: Decimal,
}

impl TryFrom<super::store::Article> for Article {
    type Error = SyntaxError;

    fn try_from(value: super::store::Article) -> Result<Self, Self::Error> {
        Ok(Self {
            id: Uuid::from_str(&value.id)?,
            name: value.name,
            description: value.description,
            unit_price: Decimal::from_str(&value.unit_price.map(|x| x.value).unwrap_or_default())?,
        })
    }
}

pub struct OrderedArticle {
    pub id: Uuid,
    pub quantity: u32,
}
