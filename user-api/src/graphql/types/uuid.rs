use async_graphql::{InputValueError, InputValueResult, Scalar, ScalarType, Value};
use std::str::FromStr;
use uuid::Uuid as UuidV4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Uuid(UuidV4);

impl From<Uuid> for UuidV4 {
    fn from(uuid: Uuid) -> Self {
        uuid.0
    }
}

impl From<&Uuid> for UuidV4 {
    fn from(uuid: &Uuid) -> Self {
        uuid.0
    }
}

impl From<UuidV4> for Uuid {
    fn from(uuid: UuidV4) -> Self {
        Self(uuid)
    }
}

#[Scalar]
impl ScalarType for Uuid {
    fn parse(value: Value) -> InputValueResult<Self> {
        if let Value::String(value_str) = &value {
            match UuidV4::from_str(value_str) {
                Err(_) => Err(InputValueError::expected_type(value)),
                Ok(uuid) => Ok(Uuid(uuid)),
            }
        } else {
            Err(InputValueError::expected_type(value))
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.0.to_string())
    }
}
