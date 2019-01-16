# wasm-logger

`Cargo.toml`
```
[dependencies]
log = "0.4"
wasm-logger = "0.1.0"
```
## Rust 2015

```rust
#[macro_use]
extern crate log;
extern crate wasm_logger;
```

Initialize `wasm-logger` when your app start:
```rust
wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
```

Logging:
```rust
    info!("Some info");
    error!("Error message");
```

## Rust 2018
In Rust 2018 you don't have to `extern crate`, just initialize `wasm-logger` when your app start:
```rust
wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
```

Logging:
```rust
    log::info!("Some info");
    log::error!("Error message");
```

## Log for specific module only

You can provide a path prefix:
```
wasm_logger::init(wasm_logger::Config::with_prefix(log::Level::Debug, "some::module"));
```

then, `wasm-logger` only logs message from `some::module` 

## Logging in Rust

For more information about how to use loggers in Rust, see [log](https://crates.io/crates/log).

## License
[MIT](http://opensource.org/licenses/MIT)
