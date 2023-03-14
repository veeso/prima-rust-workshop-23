//! # Proto
//!
//! Protobuf client

mod error;
mod store_client;

pub use error::{ProtobufError, SyntaxError};
pub use store_client::StoreClient;

pub type ProtobufResult<T> = Result<T, ProtobufError>;
