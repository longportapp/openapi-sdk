use std::{
    fmt::{self, Display, Formatter},
    time::{Duration, SystemTime},
};

#[derive(Debug, Copy, Clone)]
pub(crate) struct Timestamp(Duration);

impl Timestamp {
    pub(crate) fn now() -> Self {
        Timestamp(
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap(),
        )
    }
}

impl Display for Timestamp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{:07.03}",
            self.0.as_secs(),
            self.0.subsec_micros() as f32 / 1000.0
        )
    }
}
