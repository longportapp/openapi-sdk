use std::{
    fmt::{self, Display, Formatter},
    num::ParseIntError,
    str::FromStr,
    time::SystemTime,
};

#[derive(Debug, Copy, Clone)]
pub(crate) struct Timestamp(u64);

impl Timestamp {
    pub(crate) fn now() -> Self {
        Timestamp(
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        )
    }
}

impl FromStr for Timestamp {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<u64>().map(Self)
    }
}

impl Display for Timestamp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
