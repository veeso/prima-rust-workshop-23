use async_graphql::Enum;

use crate::proto::store_client::types::OrderStatus as ProtoOrderStatus;

#[derive(Enum, Clone, Copy, PartialEq, Eq)]
pub enum OrderStatus {
    Created,
    Preparing,
    Shipped,
    PaymentFailed,
}

impl From<ProtoOrderStatus> for OrderStatus {
    fn from(value: ProtoOrderStatus) -> Self {
        match value {
            ProtoOrderStatus::Created => Self::Created,
            ProtoOrderStatus::PaymentFailed => Self::PaymentFailed,
            ProtoOrderStatus::Preparing => Self::Preparing,
            ProtoOrderStatus::Shipped => Self::Shipped,
        }
    }
}
