use rust_decimal::Decimal;
use time::OffsetDateTime;

use crate::UpdateFields;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Period {
    Min_1,
    Min_2,
    Min_3,
    Min_5,
    Min_10,
    Min_15,
    Min_20,
    Min_30,
    Min_45,
    Min_60,
    Min_120,
    Min_180,
    Min_240,
    Day,
    Week,
    Month,
    Quarter,
    Year,
}

impl Period {
    #[inline]
    pub(crate) fn minutes(&self) -> u8 {
        match self {
            Period::Min_1 => 1,
            Period::Min_2 => 2,
            Period::Min_3 => 3,
            Period::Min_5 => 5,
            Period::Min_10 => 10,
            Period::Min_15 => 15,
            Period::Min_20 => 20,
            Period::Min_30 => 30,
            Period::Min_45 => 45,
            Period::Min_60 => 60,
            Period::Min_120 => 120,
            Period::Min_180 => 180,
            Period::Min_240 => 240,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Trade {
    pub time: OffsetDateTime,
    pub price: Decimal,
    pub volume: i64,
    pub update_fields: UpdateFields,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Quote {
    pub time: OffsetDateTime,
    pub open: Decimal,
    pub high: Decimal,
    pub low: Decimal,
    pub last_done: Decimal,
    pub volume: i64,
    pub turnover: Decimal,
}
