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
    lvl_trace: String,
    lvl_debug: String,
    lvl_info: String,
    lvl_warn: String,
    lvl_error: String,
    tgt: String,
    args: String,
}

impl Style {
    fn new() -> Style {
        let base = String::from("color: white; padding: 0 3px; background:");
        Style {
            lvl_trace: format!("{} darkgray;", base),
            lvl_debug: format!("{} darkgreen;", base),
            lvl_info: format!("{} green;", base),
            lvl_warn: format!("{} gold;", base),
            lvl_error: format!("{} red;", base),
            tgt: String::from("font-weight: bold; color: inherit"),
            args: String::from("background: inherit; color: inherit"),
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
            let tgt_style = JsValue::from_str(&style.tgt);
            let args_style = JsValue::from_str(&style.args);

            match record.level() {
                Level::Trace => console::log_4(&s,
                                               &JsValue::from_str(&style.lvl_trace),
                                               &tgt_style,
                                               &args_style),
                Level::Debug => console::debug_4(&s,
                                                 &JsValue::from(&style.lvl_debug),
                                                 &tgt_style,
                                                 &args_style),
                Level::Info => console::info_4(&s,
                                               &JsValue::from(&style.lvl_info),
                                               &tgt_style,
                                               &args_style),
                Level::Warn => console::warn_4(&s,
                                               &JsValue::from(&style.lvl_warn),
                                               &tgt_style,
                                               &args_style),
                Level::Error => console::error_4(&s,
                                                 &JsValue::from(&style.lvl_error),
                                                 &tgt_style,
                                                 &args_style),
            }
        }
    }

    fn flush(&self) {}
}

/// Initialize the logger which the given config. If failed, it will log a message to the the browser console.
pub fn init(config: Config) {
    let max_level = config.level;
    let wl = WasmLogger { config, style: Style::new() };

    match log::set_boxed_logger(Box::new(wl)) {
        Ok(_) => log::set_max_level(max_level.to_level_filter()),
        Err(e) => console::error_1(&JsValue::from(e.to_string())),
    }
}
