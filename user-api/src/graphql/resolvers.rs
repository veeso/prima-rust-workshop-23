//! # GraphQL resolvers

pub const INTERNAL_SERVER_ERROR: &str = "INTERNAL SERVER ERROR";
pub const UNAUTHORIZED: &str = "UNAUTHORIZED";

mod articles;
mod order;
mod submit_order;

pub use articles::Articles;
pub use order::Orders;
pub use submit_order::SubmitOrder;
