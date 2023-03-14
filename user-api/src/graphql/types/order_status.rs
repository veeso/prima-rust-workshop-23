use async_graphql::Enum;

#[derive(Enum, Clone, Copy, PartialEq, Eq)]
pub enum OrderStatus {
    Created,
    Preparing,
    Shipped,
    PaymentFailed,
}
