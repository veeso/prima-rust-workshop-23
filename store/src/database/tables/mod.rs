//! # Tables
//!
//! Database table entities

use super::{DatabaseError, DatabaseResult, StoreDb};

mod article;
mod customer;
mod order;
mod order_article;

pub use article::Article;
pub use customer::Customer;
pub use order::CustomerOrder;
pub use order_article::OrderArticle;
