use async_graphql::{InputValueError, InputValueResult, Scalar, ScalarType, Value};
use rust_decimal::Decimal as DecimalRs;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Decimal(DecimalRs);

impl From<Decimal> for DecimalRs {
    fn from(value: Decimal) -> Self {
        value.0
    }
}

impl From<&Decimal> for DecimalRs {
    fn from(value: &Decimal) -> Self {
        value.0
    }
}

impl From<DecimalRs> for Decimal {
    fn from(value: DecimalRs) -> Self {
        Self(value)
    }
}

#[Scalar]
impl ScalarType for Decimal {
    fn parse(value: Value) -> InputValueResult<Self> {
        if let Value::String(value_str) = &value {
            match DecimalRs::from_str(value_str) {
                Err(_) => Err(InputValueError::expected_type(value)),
                Ok(val) => Ok(Decimal(val)),
            }
        } else {
            Err(InputValueError::expected_type(value))
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.0.to_string())
    }
}
