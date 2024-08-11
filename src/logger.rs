use crate::config::cli_config::CliConfig;
use chrono::Local;

pub struct Logger {
    config: CliConfig,
}

impl Logger {
    pub fn new(config: CliConfig) -> Self {
        Self { config }
    }

    fn timestamp() -> String {
        let now = Local::now();
        now.format("%Y-%m-%d %H:%M:%S").to_string()
    }

    fn should_log(&self, level: &str) -> bool {
        let levels = ["error", "warn", "info", "log", "debug"];
        let current_level = levels.iter().position(|&l| l == self.config.log_level);
        let message_level = levels.iter().position(|&l| l == level);
        match (current_level, message_level) {
            (Some(current), Some(message)) => message <= current,
            _ => false,
        }
    }

    pub fn log_info(&self, message: &str) {
        if self.should_log("info") {
            println!("[{} INFO] {}", Self::timestamp(), message);
        }
    }

    pub fn log_warn(&self, message: &str) {
        if self.should_log("warn") {
            println!("[{} WARN] {}", Self::timestamp(), message);
        }
    }

    pub fn log_error(&self, message: &str) {
        if self.should_log("error") {
            println!("[{} ERROR] {}", Self::timestamp(), message);
        }
    }

    pub fn log_debug(&self, message: &str) {
        if self.should_log("debug") {
            println!("[{} DEBUG] {}", Self::timestamp(), message);
        }
    }

    pub fn log(&self, message: &str) {
        if self.should_log("log") {
            println!("[{} LOG] {}", Self::timestamp(), message);
        }
    }
}
