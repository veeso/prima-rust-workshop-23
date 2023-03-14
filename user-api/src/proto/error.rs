//! # Error

use chrono::ParseError;
use rust_decimal::Error as DecimalError;
use thiserror::Error;
use tonic::transport::Error as TransportError;
use tonic::Status;
use uuid::Error as UuidError;

#[derive(Debug, Error)]
pub enum ProtobufError {
    #[error("transport error: {0}")]
    Transport(TransportError),
    #[error("protobuf error: {0}")]
    Protobuf(Status),
    #[error("syntax error: {0}")]
    Syntax(SyntaxError),
}

impl From<TransportError> for ProtobufError {
    fn from(value: TransportError) -> Self {
        Self::Transport(value)
    }
}

impl From<Status> for ProtobufError {
    fn from(value: Status) -> Self {
        Self::Protobuf(value)
    }
}

impl From<SyntaxError> for ProtobufError {
    fn from(value: SyntaxError) -> Self {
        Self::Syntax(value)
    }
}

// syntax

#[derive(Debug, Error)]
pub enum SyntaxError {
    #[error("bad decimal: {0}")]
    Decimal(DecimalError),
    #[error("bad timestamp: {0}")]
    DateTime(ParseError),
    #[error("bad uuid: {0}")]
    Uuid(UuidError),
    #[error("unknown value")]
    UnknownValue,
    #[error("a value is missing")]
    ValueIsMissing,
}

impl From<DecimalError> for SyntaxError {
    fn from(value: DecimalError) -> Self {
        Self::Decimal(value)
    }
}

impl From<ParseError> for SyntaxError {
    fn from(value: ParseError) -> Self {
        Self::DateTime(value)
    }
}

impl From<UuidError> for SyntaxError {
    fn from(value: UuidError) -> Self {
        Self::Uuid(value)
    }
}
