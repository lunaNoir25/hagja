//! # Hagja (학자) - Rust's Scribe logger
//! 
//! A lightweight and robust logger system for rust.
//! Hagja provides clean outputs, logging to files, and coloured logs.
//! 
//! Copyright (c) 2025 Luna Moonlit Noir <lunaNoir.sk@gmail.com>
//! GNU General Public License v3.0

use std::fs::File;
use std::sync::{Arc, Mutex, OnceLock};
use std::io::Write;
use chrono::Local;
use colored::*;

#[macro_export]
macro_rules! info {
    ($logger:expr, $($arg:tt)*) => {
        $logger.info(&format!($($arg)*))
    };
    ($($arg:tt)+) => {
        $crate::get_default_logger().info(&format!($($arg)*))
    };
}
#[macro_export]
macro_rules! debug {
    ($logger:expr, $($arg:tt)*) => {
        $logger.debug(&format!($($arg)*))
    };
    ($($arg:tt)+) => {
        $crate::get_default_logger().debug(&format!($($arg)*))
    };
}
#[macro_export]
macro_rules! warn {
    ($logger:expr, $($arg:tt)*) => {
        $logger.warn(&format!($($arg)*))
    };
    ($($arg:tt)+) => {
        $crate::get_default_logger().warn(&format!($($arg)*))
    };
}
#[macro_export]
macro_rules! error {
    ($logger:expr, $($arg:tt)*) => {
        $logger.error(&format!($($arg)*))
    };
    ($($arg:tt)+) => {
        $crate::get_default_logger().error(&format!($($arg)*))
    };
}
#[macro_export]
macro_rules! fatal {
    ($logger:expr, $($arg:tt)*) => {
        $logger.fatal(&format!($($arg)*))
    };
    ($($arg:tt)+) => {
        $crate::get_default_logger().fatal(&format!($($arg)*))
    };
}
#[macro_export]
macro_rules! trace {
    ($logger:expr, $($arg:tt)*) => {
        $logger.trace(&format!($($arg)*))
    };
    ($($arg:tt)+) => {
        $crate::get_default_logger().trace(&format!($($arg)*))
    };
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum LogLevel {
    /// Log levels from lowest to highest.
    Trace = 0,
    Debug = 1,
    Info = 2,
    Warn = 3,
    Error = 4,
    Fatal = 5,
}

pub struct Hagja {
    id: &'static str,
    log_level: LogLevel,
    write_file: bool,
    file: Option<Arc<Mutex<File>>>,
}

impl Hagja {
    /// Create a new logger instance.
    /// 
    /// # Example
    /// ```
    /// use hagja::*;
    /// let logger = Hagja::new("MyModule", LogLevel::Info, false, None);
    /// info!("Intializing...");
    /// ```
    pub const fn new(id: &'static str, log_level: LogLevel, write_file: bool, file: Option<Arc<Mutex<File>>>) -> Self {
        Self { id, log_level, file, write_file }
    }

    fn emit(&self, level: LogLevel, msg: &str) {
        if level < self.log_level {
            return;
        }

        let time = Local::now().format("%H:%M:%S");
        
        let log = format!("[{}] [{: <5}] [{}]: {}", time, self.get_level_name(&level), self.id, msg);
        
        let clog = match level {
            LogLevel::Info  => log.white(),
            LogLevel::Debug => log.blue(),
            LogLevel::Warn  => log.yellow(),
            LogLevel::Error => log.bright_red(),
            LogLevel::Fatal => log.red().underline(),
            LogLevel::Trace => log.bright_black(),
        };

        println!("{}", clog);

        if self.write_file {
            if let Some(file_mutex) = &self.file {
                if let Ok(mut file) = file_mutex.lock() {
                    let _ = writeln!(file, "{}", log);
                }
            }
        }
    }

    fn get_level_name(&self, level: &LogLevel) -> &'static str {
        match level {
            LogLevel::Info  => "INFO",
            LogLevel::Debug => "DEBUG",
            LogLevel::Warn  => "WARN",
            LogLevel::Error => "ERROR",
            LogLevel::Fatal => "FATAL",
            LogLevel::Trace => "TRACE",
        }
    }

    /// Standard log, use for information or standard procedures.
    pub fn info(&self, msg: &str)  { self.emit(LogLevel::Info, msg); }
    /// Lower than info, use for deeper logging or excessive logs.
    pub fn debug(&self, msg: &str) { self.emit(LogLevel::Debug, msg); }
    /// Warnings, use when something unexpected happens, but doesn't directly warrant an error.
    pub fn warn(&self, msg: &str)  { self.emit(LogLevel::Warn, msg); }
    /// Errors, self-explanatory.
    pub fn error(&self, msg: &str) { self.emit(LogLevel::Error, msg); }
    /// Fatal Errors, use like errors, but when the process needs to exit to avoid damage.
    pub fn fatal(&self, msg: &str) { self.emit(LogLevel::Fatal, msg); }
    /// Lower than debug, use when precise, step-by-step logging is needed.
    pub fn trace(&self, msg: &str) { self.emit(LogLevel::Trace, msg); }
}

impl std::fmt::Debug for Hagja {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Hagja")
            .field("id", &self.id)
            .field("level", &self.log_level)
            .finish()
    }
}

static DEFAULT_LOGGER: OnceLock<Hagja> = OnceLock::new();

/// Set the default logger instance for macros.
pub fn set_default_logger(logger: Hagja) -> Result<(), Hagja> {
    DEFAULT_LOGGER.set(logger).map_err(|e| e)
}

/// Get the current default logger instance.
pub fn get_default_logger() -> &'static Hagja {
    DEFAULT_LOGGER.get_or_init(|| Hagja::new("default", LogLevel::Info, false, None))
}