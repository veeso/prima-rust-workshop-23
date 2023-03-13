use thiserror::Error;
use tonic::transport::Error as TonicError;

/// Service error
#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("database error: {0}")]
    Database(crate::database::DatabaseError),
    #[error("tonic server error: {0}")]
    Tonic(TonicError),
    #[error("invalid listener address")]
    InvalidAddress,
}

impl From<crate::database::DatabaseError> for ServiceError {
    fn from(value: crate::database::DatabaseError) -> Self {
        Self::Database(value)
    }
}

impl From<TonicError> for ServiceError {
    fn from(value: TonicError) -> Self {
        Self::Tonic(value)
    }
}

// -- database error to tonic status

impl From<crate::database::DatabaseError> for tonic::Status {
    fn from(value: crate::database::DatabaseError) -> Self {
        Self::new(tonic::Code::Internal, value.to_string())
    }
}
