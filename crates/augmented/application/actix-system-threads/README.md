# actix-system-threads

This is a wrapper struct for `actix` that provides a single global dispatcher into several actix
`Arbiter` threads.

### Usage
```rust
use actix_system_threads::ActorSystem;
let system = ActorSystem::current();

system.spawn(async move {
    // ...
});
let x = system.spawn_result(async move {
    10
});
assert_eq!(x, 10);
```

License: MIT
