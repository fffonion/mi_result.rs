# MiResult

This project is not affiliated with or endorsed by the XiaoMi company.

Small extension trait for Rust's `Result` providing a friendlier `are_you_ok()` method that mirrors `is_ok()`.

## Usage

```rust
use mi_result::MiResult;

fn main() {
    let mi_fans: Result<i32, &str> = Ok(42);
    assert!(mi_fans.are_you_ok());

    let mi_fans: Result<i32, &str> = Err("nope");
    assert!(!mi_fans.are_you_ok());
}
```

## Development

- Run tests: `cargo test`

## Notes

- Crate is published as `mi_result`; the internal trait is `MiResult`.
