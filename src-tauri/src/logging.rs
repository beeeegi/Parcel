use log::{Level, LevelFilter, Log, Metadata, Record};
use once_cell::sync::Lazy;
use serde::Serialize;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Serialize)]
pub struct LogEntry {
    pub timestamp: u64,
    pub level: String,
    pub message: String,
}

static LOG_BUFFER: Lazy<Mutex<Vec<LogEntry>>> = Lazy::new(|| Mutex::new(Vec::new()));

pub struct TauriLogger;

impl TauriLogger {
    pub fn init() {
        log::set_logger(&LOGGER)
            .map(|()| log::set_max_level(LevelFilter::Info))
            .expect("Failed to set logger");
    }

    pub fn get_logs() -> Vec<LogEntry> {
        LOG_BUFFER.lock().unwrap().clone()
    }

    pub fn clear_logs() {
        LOG_BUFFER.lock().unwrap().clear();
    }

    pub fn add_log(level: Level, message: String) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let entry = LogEntry {
            timestamp,
            level: level.to_string(),
            message,
        };

        LOG_BUFFER.lock().unwrap().push(entry);
    }
}

static LOGGER: TauriLogger = TauriLogger;

impl Log for TauriLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let message = format!("{}", record.args());
            TauriLogger::add_log(record.level(), message);
            
            #[cfg(debug_assertions)]
            println!("[{}] {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}
