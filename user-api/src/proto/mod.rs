//! # Proto
//!
//! Protobuf client

mod error;
pub mod store_client;

pub use error::{ProtobufError, SyntaxError};
pub use store_client::StoreClient;

pub type ProtobufResult<T> = Result<T, ProtobufError>;
