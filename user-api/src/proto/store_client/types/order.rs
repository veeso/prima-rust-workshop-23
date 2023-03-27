use std::str::FromStr;

use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use uuid::Uuid;

use super::{Article, SyntaxError};

pub struct Order {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub transaction_id: Option<String>,
    pub status: OrderStatus,
    pub articles: Vec<OrderArticle>,
}

impl TryFrom<super::store::Order> for Order {
    type Error = SyntaxError;

    fn try_from(value: super::store::Order) -> Result<Self, Self::Error> {
        let mut articles = Vec::with_capacity(value.articles.len());
        for article in value.articles.into_iter() {
            articles.push(OrderArticle::try_from(article)?);
        }

        Ok(Self {
            id: Uuid::from_str(&value.id)?,
            created_at: NaiveDateTime::parse_from_str(
                &value.created_at.map(|x| x.timestamp).unwrap_or_default(),
                "%Y-%m-%d %H:%M:%S",
            )?,
            transaction_id: value.transaction_id,
            status: OrderStatus::try_from(value.status)?,
            articles,
        })
    }
}

pub enum OrderStatus {
    Created,
    Preparing,
    PaymentFailed,
    Shipped,
}

impl TryFrom<i32> for OrderStatus {
    type Error = SyntaxError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Created),
            1 => Ok(Self::Preparing),
            2 => Ok(Self::PaymentFailed),
            3 => Ok(Self::Shipped),
            _ => Err(SyntaxError::UnknownValue),
        }
    }
}

/// Article inside a order (order x quantity)
pub struct OrderArticle {
    pub article: Article,
    pub quantity: u32,
}

impl TryFrom<super::store::OrderArticle> for OrderArticle {
    type Error = SyntaxError;

    fn try_from(value: super::store::OrderArticle) -> Result<Self, Self::Error> {
        Ok(Self {
            quantity: value.quantity,
            article: Article {
                id: Uuid::from_str(&value.id)?,
                name: value.name,
                description: value.description,
                unit_price: Decimal::from_str(
                    &value.unit_price.map(|x| x.value).unwrap_or_default(),
                )?,
            },
        })
    }
}

pub enum SubmitOrderResponse {
    Ok(Uuid),
    Err(SubmitOrderError),
}

impl TryFrom<super::store::SubmitOrderResponse> for SubmitOrderResponse {
    type Error = SyntaxError;

    fn try_from(value: super::store::SubmitOrderResponse) -> Result<Self, Self::Error> {
        match value.status {
            Some(super::store::submit_order_response::Status::OrderId(id)) => {
                Ok(Self::Ok(Uuid::from_str(&id)?))
            }
            Some(super::store::submit_order_response::Status::Error(err)) => {
                Ok(Self::Err(SubmitOrderError::try_from(err)?))
            }
            None => Err(SyntaxError::ValueIsMissing),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum SubmitOrderError {
    Unknown,
    InvalidArticle,
}

impl TryFrom<i32> for SubmitOrderError {
    type Error = SyntaxError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Unknown),
            1 => Ok(Self::InvalidArticle),
            _ => Err(SyntaxError::UnknownValue),
        }
    }
}
