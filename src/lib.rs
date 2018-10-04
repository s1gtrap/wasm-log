//! A simple logger for front end wasm web app.
//!
//! For more information about how to use loggers in Rust, see [log](https://crates.io/crates/log).
//!
//! ```rust
//!     #[macro_use]
//!     extern crate log;
//!     extern crate wasm_logger;
//! ```
//! Add the following line to the initialization code of your app:
//! ```rust
//!     wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
//! ```

#[deny(missing_docs)]
extern crate log;
extern crate wasm_bindgen;
extern crate web_sys;

use log::{Level, Log, Metadata, Record};
use wasm_bindgen::JsValue;
use web_sys::console;

/// Specifies what to be logged
pub struct Config {
    level: Level,
    path_prefix: Option<String>,
}

impl Config {
    /// Specify the maximum level you want to log
    pub fn new(level: Level) -> Self {
        Self {
            level,
            path_prefix: None,
        }
    }

    /// Both maximum level and path_prefix
    pub fn with_prefix(level: Level, path_prefix: &str) -> Self {
        Self {
            level,
            path_prefix: Some(path_prefix.to_string()),
        }
    }
}

/// The logger
struct WasmLogger {
    config: Config,
}

impl Log for WasmLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        if let Some(ref prefix) = self.config.path_prefix {
            metadata.target().starts_with(prefix)
        } else {
            true
        }
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let s = JsValue::from(format!(
                "{} {}\n{}",
                record.level(),
                record.target(),
                record.args()
            ));
            match record.level() {
                Level::Trace => console::log_1(&s),
                Level::Debug => console::debug_1(&s),
                Level::Info => console::info_1(&s),
                Level::Warn => console::warn_1(&s),
                Level::Error => console::error_1(&s),
            }
        }
    }

    fn flush(&self) {}
}

/// Initialize the logger which the given config. If failed, it will log a message to the the browser console.
pub fn init(config: Config) {
    let max_level = config.level;
    let wl = WasmLogger { config };

    match log::set_boxed_logger(Box::new(wl)) {
        Ok(_) => log::set_max_level(max_level.to_level_filter()),
        Err(e) => console::error_1(&JsValue::from(e.to_string())),
    }
}
