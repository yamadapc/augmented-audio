# wisual-logger
![crates.io](https://img.shields.io/crates/v/wisual-logger.svg)
- - -
Just a pretty printer configuration for `env_logger`.

```rust
fn main() {
    wisual_logger::init_from_env();
    log::info!("Hello world");
}
```

Will output:
![](screenshot.png)