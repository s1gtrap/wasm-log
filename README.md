# A simple logger for front-end wasm web app

`Cargo.toml`
```
[dependencies]
log = "0.4"
wasm-logger = "0.1.0"
```
In your app:

```rust
#[macro_use]
extern crate log;
extern crate wasm_logger;
```

Initialize `wasm-logger` when your app start:
```rust
wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
```

# License
[MIT](http://opensource.org/licenses/MIT)