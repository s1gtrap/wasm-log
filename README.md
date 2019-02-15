# wasm-logger

A pretty logger for wasm front-end app based on [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen)

`Cargo.toml`
```
[dependencies]
log = "0.4"
wasm-logger = "0.1.2"
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

// Logging
info!("Some info");
error!("Error message");
```

## Rust 2018
In Rust 2018 you don't have to `extern crate`, just initialize `wasm-logger` when your app start:
```rust
wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));

// Logging
log::info!("Some info");
log::error!("Error message");
```

## Log for a specific module only

You can provide a path prefix:
```rust
wasm_logger::init(wasm_logger::Config::with_prefix(log::Level::Debug, "some::module"));
```

then, `wasm-logger` only logs message from `some::module` 

## Logging in Rust

For more information about how to use loggers in Rust, see [log](https://crates.io/crates/log).

## Mapping from `log` to console's methods
`log::error!`, `log::warn!` and `log::info!` call theirs equivalent methods of the browser console. The `console.trace` method outputs some extra trace from the generated JS glue code which we don't want. Therefore, we choose to map `log::debug!` to `console.log` and `log::trace!` to `console.debug`.

## Note for Chromium/Chrome users

Chromium/Chrome filters out `console.debug` (execute by `log::trace!`) by default. You must check the `Verbose` filter in your browser console to see trace entries.

## License
[MIT](http://opensource.org/licenses/MIT)
or
[Apache-2.0](http://www.apache.org/licenses/LICENSE-2.0)
