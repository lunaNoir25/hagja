use chrono::Local;

pub enum LogLevel {
    Info,
    Debug,
    Warn,
    Error,
    Fatal,
    Trace,
}

impl LogLevel {
    fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Info  => "INFO",
            LogLevel::Debug => "DEBUG",
            LogLevel::Warn  => "WARN",
            LogLevel::Error => "ERROR",
            LogLevel::Fatal => "FATAL",
            LogLevel::Trace => "TRACE",
        }
    }
}

pub struct Hagja {
    id: &'static str,
}

impl Hagja {
    pub const fn new(id: &'static str) -> Self {
        Self { id }
    }

    fn emit(&self, level: LogLevel, msg: &str) {
        let now = Local::now();
        let time = now.format("%H:%M:%S");
        let level_str = level.as_str();
        
        println!("[{}] [{}] [{}]: {}", time, level_str, self.id, msg);
    }

    pub fn info(&self, msg: &str)  { self.emit(LogLevel::Info, msg); }
    pub fn debug(&self, msg: &str) { self.emit(LogLevel::Debug, msg); }
    pub fn warn(&self, msg: &str)  { self.emit(LogLevel::Warn, msg); }
    pub fn error(&self, msg: &str) { self.emit(LogLevel::Error, msg); }
    pub fn fatal(&self, msg: &str) { self.emit(LogLevel::Fatal, msg); }
    pub fn trace(&self, msg: &str) { self.emit(LogLevel::Trace, msg); }
}