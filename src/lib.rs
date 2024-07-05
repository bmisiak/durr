#[forbid(unsafe_code)]
use std::time::{Duration, Instant};
pub trait Durr {
    fn nanoseconds(&self) -> Duration;
    fn milliseconds(&self) -> Duration;
    fn seconds(&self) -> Duration;
    fn minutes(&self) -> Duration;
    fn hours(&self) -> Duration;
}

impl Durr for u64 {
    #[inline]
    fn nanoseconds(&self) -> Duration {
        Duration::from_nanos(*self)
    }
    #[inline]
    fn milliseconds(&self) -> Duration {
        Duration::from_millis(*self)
    }
    #[inline]
    fn seconds(&self) -> Duration {
        Duration::from_secs(*self)
    }
    #[inline]
    fn minutes(&self) -> Duration {
        Duration::from_secs(*self * 60)
    }
    #[inline]
    fn hours(&self) -> Duration {
        Duration::from_secs(*self * 3600)
    }
}

#[inline]
pub fn now() -> Instant {
    Instant::now()
}