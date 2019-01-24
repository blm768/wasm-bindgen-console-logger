//! # `wasm-bindgen` console logger
//!
//! This small utility crate integrates the [`log`](https://crates.io/crates/log)
//! crate with the JavaScript console logging functions with the help of
//! [`wasm-bindgen`](https://crates.io/crates/wasm-bindgen).

//! ## Example
//!
//! ```
//! use log::{error, info, warn};
//! use wasm_bindgen::prelude::*;
//! use wasm_bindgen_console_logger::DEFAULT_LOGGER;
//!
//! #[wasm_bindgen]
//! pub fn start() {
//!     log::set_logger(&DEFAULT_LOGGER).unwrap();
//!     log::set_max_level(log::LevelFilter::Info);
//!
//!     error!("Error message");
//!     warn!("Warning message");
//!     info!("Informational message");
//! }
//! ```

use log::{Level, Log, Metadata, Record};
use wasm_bindgen::prelude::*;

pub const DEFAULT_LOGGER: ConsoleLogger = ConsoleLogger {
    formatter: &format_message,
    log_level: Level::Trace,
};

fn format_message(record: &Record) -> String {
    if record.level() >= Level::Debug {
        format!("{}: {}", record.level(), record.args())
    } else {
        format!("{}", record.args())
    }
}

/// Formats a `log::Record` as a `String`
pub type RecordFormatter = Fn(&Record) -> String + Send + Sync;

/// Logs messages to the Web browser's console
///
/// Error and warning messages will be logged with `console.error()` and `console.warn()`, respectively.
/// All other messages will be logged with `console.log()`.
pub struct ConsoleLogger {
    formatter: &'static RecordFormatter,
    log_level: Level,
}

impl ConsoleLogger {
    /// Constructs a new `ConsoleLogger`
    ///
    /// The given function will be used to format the logged messages.
    pub fn new(formatter: &'static RecordFormatter, log_level: Level) -> Self {
        ConsoleLogger { formatter, log_level }
    }

    /// Constructs a `ConsoleLogger` with the given maximum log level
    /// 
    /// A default log formatter will be used.
    pub fn with_level(level: Level) -> Self {
        ConsoleLogger::new(&format_message, level)
    }
}

impl Default for ConsoleLogger {
    fn default() -> Self {
        DEFAULT_LOGGER
    }
}

impl Log for ConsoleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.log_level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let msg = (self.formatter)(record);
            match record.level() {
                Level::Error => error(&msg),
                Level::Warn => warn(&msg),
                _ => log(&msg),
            }
        }
    }

    fn flush(&self) {}
}

// Bindings to console functions
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace=console)]
    fn log(text: &str);

    #[wasm_bindgen(js_namespace=console)]
    fn warn(text: &str);

    #[wasm_bindgen(js_namespace=console)]
    fn error(text: &str);
}
