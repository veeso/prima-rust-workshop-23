//! # User-friendly types for store_client with no ambiguous nulls

use std::str::FromStr;

use thiserror::Error;
use uuid::Uuid;

use crate::proto::SyntaxError;

/// Authentication response
pub enum AuthResponse {
    Authenticated(Uuid),
    Failed(AuthError),
}

impl TryFrom<super::store::AuthResponse> for AuthResponse {
    type Error = SyntaxError;

    fn try_from(value: super::store::AuthResponse) -> Result<Self, Self::Error> {
        match value.status {
            None => Err(SyntaxError::UnknownValue),
            Some(super::store::auth_response::Status::UserId(id)) => match Uuid::from_str(&id) {
                Err(e) => Err(SyntaxError::Uuid(e)),
                Ok(uuid) => Ok(Self::Authenticated(uuid)),
            },
            Some(super::store::auth_response::Status::Error(error)) => {
                Ok(Self::Failed(AuthError::try_from(error)?))
            }
        }
    }
}

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("email already taken")]
    EmailAlreadyTaken,
    #[error("bad email syntax")]
    BadEmailSyntax,
    #[error("invalid email or password")]
    InvalidEmailOrPassword,
}

impl TryFrom<i32> for AuthError {
    type Error = SyntaxError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::EmailAlreadyTaken),
            1 => Ok(Self::BadEmailSyntax),
            2 => Ok(Self::InvalidEmailOrPassword),
            _ => Err(SyntaxError::UnknownValue),
        }
    }
}
