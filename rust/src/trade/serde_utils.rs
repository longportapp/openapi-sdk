#![allow(dead_code)]

use std::fmt::Display;

use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};
use time::{Date, OffsetDateTime};

pub(crate) mod timestamp {
    use super::*;

    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<OffsetDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        let value = value
            .parse::<i64>()
            .map_err(|_| Error::custom("invalid timestamp"))?;
        OffsetDateTime::from_unix_timestamp(value).map_err(|_| Error::custom("invalid timestamp"))
    }

    pub(crate) fn serialize<S>(datetime: &OffsetDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&datetime.unix_timestamp().to_string())
    }
}

pub(crate) mod timestamp_opt {
    use super::*;

    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Option<OffsetDateTime>, D::Error>
    where
        D: Deserializer<'de>,
    {
        match <Option<String>>::deserialize(deserializer)? {
            Some(value) if !value.is_empty() => {
                let value = value
                    .parse::<i64>()
                    .map_err(|_| Error::custom("invalid timestamp"))?;
                let datetime = OffsetDateTime::from_unix_timestamp(value)
                    .map_err(|_| Error::custom("invalid timestamp"))?;
                Ok(Some(datetime))
            }
            _ => Ok(None),
        }
    }

    pub(crate) fn serialize<S>(
        datetime: &Option<OffsetDateTime>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match datetime {
            Some(datetime) => serializer.serialize_str(&datetime.unix_timestamp().to_string()),
            None => serializer.serialize_none(),
        }
    }
}

pub(crate) mod date_opt {
    use super::*;

    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Option<Date>, D::Error>
    where
        D: Deserializer<'de>,
    {
        match <Option<String>>::deserialize(deserializer)? {
            Some(value) if !value.is_empty() => {
                let datetime = Date::parse(
                    &value,
                    time::macros::format_description!("[year]-[month]-[day]"),
                )
                .map_err(|_| Error::custom("invalid timestamp"))?;
                Ok(Some(datetime))
            }
            _ => Ok(None),
        }
    }

    pub(crate) fn serialize<S>(datetime: &Option<Date>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match datetime {
            Some(date) => serializer.serialize_str(
                &date
                    .format(time::macros::format_description!("[year]-[month]-[day]"))
                    .unwrap(),
            ),
            None => serializer.serialize_none(),
        }
    }
}

pub(crate) mod number_str_opt {
    use num_traits::Num;

    use super::*;

    pub(crate) fn serialize<S, T>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: Num + Display,
    {
        match value {
            Some(value) => serializer.collect_str(value),
            None => serializer.serialize_none(),
        }
    }

    pub(crate) fn deserialize<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
    where
        D: Deserializer<'de>,
        T: Num,
        T::FromStrRadixErr: Display,
    {
        let value = String::deserialize(deserializer)?;
        if !value.is_empty() {
            let n = <T as Num>::from_str_radix(&value, 10)
                .map_err(|err| D::Error::custom(err.to_string()))?;
            Ok(Some(n))
        } else {
            Ok(None)
        }
    }
}

pub(crate) mod decimal_opt {
    use rust_decimal::Decimal;

    use super::*;

    pub(crate) fn serialize<S>(value: &Option<Decimal>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(value) => Serialize::serialize(&value, serializer),
            None => serializer.serialize_none(),
        }
    }

    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Option<Decimal>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        if !value.is_empty() {
            let n = value
                .parse::<Decimal>()
                .map_err(|err| Error::custom(err.to_string()))?;
            Ok(Some(n))
        } else {
            Ok(None)
        }
    }
}
