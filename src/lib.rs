use chrono::Local;
use colored::*;

#[derive(PartialEq, PartialOrd)]
pub enum LogLevel {
    Info = 2,
    Debug = 1,
    Warn = 3,
    Error = 4,
    Fatal = 5,
    Trace = 0,
}

pub struct Hagja {
    id: &'static str,
    log_level: LogLevel,
}

impl Hagja {
    pub const fn new(id: &'static str, log_level: LogLevel) -> Self {
        Self { id, log_level }
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

    pub fn info(&self, msg: &str)  { self.emit(LogLevel::Info, msg); }
    pub fn debug(&self, msg: &str) { self.emit(LogLevel::Debug, msg); }
    pub fn warn(&self, msg: &str)  { self.emit(LogLevel::Warn, msg); }
    pub fn error(&self, msg: &str) { self.emit(LogLevel::Error, msg); }
    pub fn fatal(&self, msg: &str) { self.emit(LogLevel::Fatal, msg); }
    pub fn trace(&self, msg: &str) { self.emit(LogLevel::Trace, msg); }
}