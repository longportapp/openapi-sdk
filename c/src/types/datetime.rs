use time::{Date, Month, Time};

use crate::types::ToFFI;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct CDate {
    pub year: i32,
    pub month: u8,
    pub day: u8,
}

impl From<Date> for CDate {
    fn from(date: Date) -> Self {
        Self {
            year: date.year(),
            month: date.month() as u8,
            day: date.day(),
        }
    }
}

impl From<CDate> for Date {
    fn from(date: CDate) -> Self {
        Date::from_calendar_date(
            date.year,
            Month::try_from(date.month).expect("invalid month"),
            date.day,
        )
        .expect("invalid date")
    }
}

impl ToFFI for CDate {
    type FFIType = Self;

    #[inline]
    fn to_ffi_type(&self) -> Self::FFIType {
        *self
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct CTime {
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

impl From<Time> for CTime {
    fn from(t: Time) -> Self {
        Self {
            hour: t.hour(),
            minute: t.minute(),
            second: t.second(),
        }
    }
}

impl ToFFI for CTime {
    type FFIType = Self;

    #[inline]
    fn to_ffi_type(&self) -> Self::FFIType {
        *self
    }
}
