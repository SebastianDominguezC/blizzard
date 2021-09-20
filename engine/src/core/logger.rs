// warn - trace are optional
enum LogLevel {
    Fatal,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

use LogLevel::{Debug, Error, Fatal, Info, Trace, Warn};

fn log(color: &str, code: &str, message: String) {
    println!("{}{}: {}\n", color, code, message);
    println!("\x1b[1;37m");
}

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

pub fn log_fatal(message: String) {
    log_output(Fatal, message);
}

pub fn log_error(message: String) {
    log_output(Error, message);
}

pub fn log_warning(message: String) {
    log_output(Warn, message);
}

pub fn log_info(message: String) {
    log_output(Info, message);
}

pub fn log_debug(message: String) {
    log_output(Debug, message);
}

pub fn log_trace(message: String) {
    log_output(Trace, message);
}

pub fn initalize_logging() {
    // TODO: create log file
}

pub fn shutdown() {
    // TODO: clean up logging/write
}
