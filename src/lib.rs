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
use wasm_bindgen::prelude::*;
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

/// The log styles
struct Style {
    lvl_trace: JsValue,
    lvl_debug: JsValue,
    lvl_info: JsValue,
    lvl_warn: JsValue,
    lvl_error: JsValue,
    tgt: JsValue,
    args: JsValue,
}

impl Style {
    fn new() -> Style {
        let base = String::from("color: white; padding: 0 3px; background:");
        Style {
            lvl_trace: JsValue::from_str(&(format!("{} darkgray;", base))),
            lvl_debug: JsValue::from_str(&(format!("{} darkgreen;", base))),
            lvl_info: JsValue::from_str(&(format!("{} green;", base))),
            lvl_warn: JsValue::from_str(&(format!("{} gold;", base))),
            lvl_error: JsValue::from_str(&(format!("{} red;", base))),
            tgt: JsValue::from_str("font-weight: bold; color: inherit"),
            args: JsValue::from_str("background: inherit; color: inherit"),
        }
    }
}

/// The logger
struct WasmLogger {
    config: Config,
    style: Style,
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
            let style = &self.style;
            let s = JsValue::from_str(&format!(
                "[%c{: <5}%c {}%c] {}",
                record.level(),
                record.target(),
                record.args()
            ));
            match record.level() {
                Level::Trace => console::log_4(&s,
                                               &style.lvl_trace,
                                               &style.tgt,
                                               &style.args),
                Level::Debug => console::debug_4(&s,
                                                 &style.lvl_debug,
                                                 &style.tgt,
                                                 &style.args),
                Level::Info => console::info_4(&s,
                                               &style.lvl_info,
                                               &style.tgt,
                                               &style.args),
                Level::Warn => console::warn_4(&s,
                                               &style.lvl_warn,
                                               &style.tgt,
                                               &style.args),
                Level::Error => console::error_4(&s,
                                                 &style.lvl_error,
                                                 &style.tgt,
                                                 &style.args),
            }
        }
    }

    fn flush(&self) {}
}

/// Since wasm doesn't support thread yet, no worry about Send and Sync for now
unsafe impl Send for WasmLogger {}
unsafe impl Sync for WasmLogger {}

/// Initialize the logger which the given config. If failed, it will log a message to the the browser console.
pub fn init(config: Config) {
    let max_level = config.level;
    let wl = WasmLogger { config, style: Style::new() };

    match log::set_boxed_logger(Box::new(wl)) {
        Ok(_) => log::set_max_level(max_level.to_level_filter()),
        Err(e) => console::error_1(&JsValue::from(e.to_string())),
    }
}
