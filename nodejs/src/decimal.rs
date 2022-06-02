use std::fmt::{self, Debug, Formatter};

use napi::{Either, Error, Result};
use rust_decimal::prelude::{FromPrimitive, ToPrimitive};

#[napi_derive::napi]
#[derive(Copy, Clone, Default)]
pub struct Decimal(pub(crate) rust_decimal::Decimal);

impl From<rust_decimal::Decimal> for Decimal {
    #[inline]
    fn from(value: rust_decimal::Decimal) -> Self {
        Self(value)
    }
}

impl Debug for Decimal {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

macro_rules! decimal_constants {
    ($($id:ident),*) => {
        $(
        #[napi_derive::napi]
        pub const $id: Decimal = Decimal(rust_decimal::Decimal::$id);
        )*
    };
}

decimal_constants!(
    E,
    E_INVERSE,
    HALF_PI,
    MAX,
    MIN,
    NEGATIVE_ONE,
    ONE,
    ONE_HUNDRED,
    ONE_THOUSAND,
    PI,
    QUARTER_PI,
    TEN,
    TWO,
    TWO_PI,
    ZERO
);

#[napi_derive::napi]
impl Decimal {
    #[napi(constructor)]
    pub fn new(value: Either<String, f64>) -> Result<Self> {
        match value {
            Either::A(value) => {
                Ok(Self(value.parse().map_err(|err| {
                    Error::from_reason(format!("invalid decimal: {}", err))
                })?))
            }
            Either::B(value) => Ok(Self(rust_decimal::Decimal::from_f64(value).ok_or_else(
                || Error::from_reason(format!("can not create decimal from number: {}", value)),
            )?)),
        }
    }

    #[napi]
    #[allow(clippy::wrong_self_convention)]
    #[allow(clippy::inherent_to_string)]
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }

    #[napi]
    #[allow(clippy::wrong_self_convention)]
    pub fn to_number(&self) -> f64 {
        self.0.to_f64().unwrap_or_default()
    }
}
