### Purpose
A simple helper trait for writing durations in a human-readable way.

~30 lines of code, zero dependencies, zero unsafe, zero macros. 

```rust
200.nanoseconds()  == Duration::from_nanos(100)
100.milliseconds() == Duration::from_millis(100)
10.seconds()       == Duration::from_secs(10)
5.minutes()        == Duration::from_secs(5*60)
2.hours()          == Duration::from_secs(2*3600)
3.days()           == Duration::from_secs(3*86400)
```