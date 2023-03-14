//! # User-friendly types for store_client with no ambiguous nulls

use super::store;
use crate::proto::SyntaxError;

mod article;
mod auth_response;
mod order;

pub use article::{Article, OrderedArticle};
pub use auth_response::{AuthError, AuthResponse};
pub use order::{Order, OrderArticle, OrderStatus, SubmitOrderError, SubmitOrderResponse};
