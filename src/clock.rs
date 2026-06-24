use core::time::Duration;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CDuration {
    secs: u64,
    nanos: u32,
}

impl From<Duration> for CDuration {
    fn from(d: Duration) -> Self {
        CDuration {
            secs: d.as_secs(),
            nanos: d.subsec_nanos(),
        }
    }
}

impl Into<Duration> for CDuration {
    fn into(self) -> Duration {
        Duration::new(self.secs, self.nanos)
    }
}

impl CDuration {
    pub const ZERO: Self = Self { secs: 0, nanos: 0 };
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Clock {
    /// Contains duration relative to UNIX epoch. (UTC)
    RTC = 0,
    /// Contains duration since boot.
    Monotonic = 1,
}

impl Clock {
    pub const fn try_from(value: u32) -> Option<Self> {
        match value {
            0 => Some(Self::RTC),
            1 => Some(Self::Monotonic),
            _ => None,
        }
    }
}
