use async_graphql::{Enum, SimpleObject, Union};
use thiserror::Error;

use super::Uuid;
use crate::proto::store_client::types::{SubmitOrderError, SubmitOrderResponse};

#[derive(Union)]
pub enum OrderSubmission {
    OrderAccepted(OrderAccepted),
    OrderRejected(OrderRejected),
}

#[derive(SimpleObject)]
pub struct OrderAccepted {
    id: Uuid,
}

#[derive(SimpleObject)]
pub struct OrderRejected {
    code: OrderRejectedCode,
    message: String,
}

#[derive(Enum, Clone, Copy, PartialEq, Eq, Error, Debug)]
pub enum OrderRejectedCode {
    #[error("unknown error")]
    UnknownError,
    #[error("an invalid article was found in the order articles")]
    InvalidArticle,
}

impl From<SubmitOrderResponse> for OrderSubmission {
    fn from(value: SubmitOrderResponse) -> Self {
        match value {
            SubmitOrderResponse::Ok(id) => Self::OrderAccepted(OrderAccepted { id: id.into() }),
            SubmitOrderResponse::Err(err) => Self::OrderRejected(OrderRejected {
                message: OrderRejectedCode::from(err).to_string(),
                code: err.into(),
            }),
        }
    }
}

impl From<SubmitOrderError> for OrderRejectedCode {
    fn from(value: SubmitOrderError) -> Self {
        match value {
            SubmitOrderError::InvalidArticle => Self::InvalidArticle,
            SubmitOrderError::Unknown => Self::UnknownError,
        }
    }
}
