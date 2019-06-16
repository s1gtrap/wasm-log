# wasm-logger

A logger that sends a message with its Rust source's line and filename to the browser console. 

![screenshot](screenshot.png)

In v0.1.4 and up, the default Config will put the message on the same line as other info ([see more](https://gitlab.com/limira-rs/wasm-logger/issues/1)).
If you want them on a separate line as the screenshot above, you must config like this:
```rust
wasm_logger::init(
    wasm_logger::Config::new(log::Level::Debug)
        .message_on_new_line()
);
```

## Usage

**Note**: For more information about how to use loggers in Rust, see [log](https://crates.io/crates/log).

`Cargo.toml`
```
[dependencies]
log = "0.4.6"
wasm-logger = "0.1.5"
```
Initialize `wasm-logger` when your app start:
```rust
wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));

// Logging
log::info!("Some info");
log::error!("Error message");
```

### Log for a specific module only

You can provide a path prefix:
```rust
wasm_logger::init(wasm_logger::Config::with_prefix(log::Level::Debug, "some::module"));
```

then, `wasm-logger` only logs message from `some::module` 

## Mapping from `log` to console's methods
`log::error!`, `log::warn!` and `log::info!` call theirs equivalent methods of the browser console. The `console.trace` method outputs some extra trace from the generated JS glue code which we don't want. Therefore, we choose to map:
* `log::debug!` to `console.log`
* `log::trace!` to `console.debug`.

## Note for Chromium/Chrome users

Chromium/Chrome filters out `console.debug` (execute by `log::trace!`) by default. You must check the `Verbose` filter in your browser console to see trace entries.

## License
[MIT](http://opensource.org/licenses/MIT)
or
[Apache-2.0](http://www.apache.org/licenses/LICENSE-2.0)
