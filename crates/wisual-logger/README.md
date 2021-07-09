# wisual-logger
[![crates.io](https://img.shields.io/crates/v/wisual-logger.svg)](https://crates.io/crates/wisual-logger)
[![docs.rs](https://docs.rs/wisual-logger/badge.svg)](https://docs.rs/wisual-logger/)
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

```shell
INFO [2021-07-09T02:26:16.239338+00:00] (main@hello_world) Hello world
```