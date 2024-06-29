### Purpose
A simple helper trait for writing durations in a human-readable way.

```rust
10.seconds()
5.minutes()
3.days()
```
instead of
```rust
Duration::from_secs(10)
Duration::from_secs(5*60)
Duration::from_secs(3*86400)
```