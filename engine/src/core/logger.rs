//! # Logger
//!
//! A custom logger for all important information, still in development.

use LogLevel::{Debug, Error, Fatal, Info, Trace, Warn};

/// Log Level definition
enum LogLevel {
    Fatal,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

/// Logger: Printing
fn log(color: &str, code: &str, message: String) {
    println!("{}{}: {}\n", color, code, message);
    println!("\x1b[1;37m");
}

/// Output handler
fn log_output(log_level: LogLevel, message: String) {
    match log_level {
        Fatal => {
            log("\x1b[1;31m", "[FATAL]", message);
        }
        Error => {
            log("\x1b[1;31m", "[ERROR]", message);
        }
        Warn => {
            log("\x1b[1;33m", "[WARN]", message);
        }
        Info => {
            log("\x1b[1;37m", "[INFO]", message);
        }
        Debug => {
            log("\x1b[1;37m", "[DEBUG]", message);
        }
        Trace => {
            log("\x1b[1;37m", "[TRACE]", message);
        }
    }
}

pub fn fatal(message: String) {
    log_output(Fatal, message);
}

pub fn error(message: String) {
    log_output(Error, message);
}

pub fn warn(message: String) {
    log_output(Warn, message);
}

pub fn info(message: String) {
    log_output(Info, message);
}

pub fn debug(message: String) {
    log_output(Debug, message);
}

pub fn trace(message: String) {
    log_output(Trace, message);
}

/// Start logging with examples
pub fn initialize_logging() {
    // TODO: create log file
    fatal("Fatal example".to_string());
    error("Error example".to_string());
    warn("Warning example".to_string());
    info("Info example".to_string());
    debug("Debug example".to_string());
    trace("Trace example".to_string());
}

/// Ends logger
pub fn shutdown() {
    // TODO: clean up logging/write
}
