use std::time::Duration;
pub trait Durr {
    fn nanoseconds(&self) -> Duration;
    fn milliseconds(&self) -> Duration;
    fn seconds(&self) -> Duration;
    fn minutes(&self) -> Duration;
    fn hours(&self) -> Duration;
    fn days(&self) -> Duration;
}

impl Durr for u64 {
    fn nanoseconds(&self) -> Duration {
        Duration::from_nanos(*self)
    }
    fn milliseconds(&self) -> Duration {
        Duration::from_millis(*self)
    }
    fn seconds(&self) -> Duration {
        Duration::from_secs(*self)
    }
    fn minutes(&self) -> Duration {
        Duration::from_secs(*self * 60)
    }
    fn hours(&self) -> Duration {
        Duration::from_secs(*self * 3600)
    }
    fn days(&self) -> Duration {
        Duration::from_secs(*self * 86400)
    }
}