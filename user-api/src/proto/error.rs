//! # Error

use thiserror::Error;
use tonic::transport::Error as TransportError;

#[derive(Debug, Error)]
pub enum ProtobufError {
    #[error("transport error: {0}")]
    Transport(TransportError),
}

impl From<TransportError> for ProtobufError {
    fn from(value: TransportError) -> Self {
        Self::Transport(value)
    }
}
