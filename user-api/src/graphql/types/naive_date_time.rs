use async_graphql::{InputValueError, InputValueResult, Scalar, ScalarType, Value};
use chrono::NaiveDateTime as ChronoNaiveDateTime;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NaiveDateTime(ChronoNaiveDateTime);

impl From<NaiveDateTime> for ChronoNaiveDateTime {
    fn from(value: NaiveDateTime) -> Self {
        value.0
    }
}

impl From<&NaiveDateTime> for ChronoNaiveDateTime {
    fn from(value: &NaiveDateTime) -> Self {
        value.0
    }
}

impl From<ChronoNaiveDateTime> for NaiveDateTime {
    fn from(value: ChronoNaiveDateTime) -> Self {
        Self(value)
    }
}

#[Scalar]
impl ScalarType for NaiveDateTime {
    fn parse(value: Value) -> InputValueResult<Self> {
        if let Value::String(value_str) = &value {
            match ChronoNaiveDateTime::from_str(value_str) {
                Err(_) => Err(InputValueError::expected_type(value)),
                Ok(val) => Ok(NaiveDateTime(val)),
            }
        } else {
            Err(InputValueError::expected_type(value))
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.0.to_string())
    }
}
