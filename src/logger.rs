// src/logger.rs
use chrono::{Local, Utc};
use chrono_tz::Tz;
use colored::Colorize;
use lazy_static::lazy_static;
use rust_i18n::t;
use std::sync::Mutex;

lazy_static! {
    static ref LOGGER: Mutex<Logger> = Mutex::new(Logger::new());
}

#[derive(Clone, Copy)]
pub enum LogLevel {
    Error,
    Warning,
    Info,
    Debug,
}

pub struct Logger {
    timezone: Option<Tz>,
}

impl Logger {
    fn new() -> Self {
        Self { timezone: None }
    }

    pub fn set_timezone(&mut self, tz: Tz) {
        self.timezone = Some(tz);
    }

    fn format_time(&self) -> String {
        // 修改为分离处理时区分支，避免DateTime类型冲突
        match self.timezone {
            Some(ref tz) => {
                let dt = Utc::now().with_timezone(tz);
                dt.format("%Y-%-m-%-d").to_string()
            }
            None => {
                let dt = Local::now();
                dt.format("%Y-%-m-%-d").to_string()
            }
        }
    }

    fn log(&self, level: LogLevel, key: &str) {
        let time = self.format_time();

        let (level_color, message_color, level_str) = match level {
            LogLevel::Error => (
                (255, 46, 99), // 红色 #ff2e63
                (255, 46, 99), // 红色 #ff2e63
                t!("error").to_string(),
            ),
            LogLevel::Warning => (
                (255, 222, 125), // 黄色 #ffde7d
                (255, 222, 125), // 黄色 #ffde7d
                t!("warning").to_string(),
            ),
            LogLevel::Info => (
                (48, 227, 202),  // 蓝色 #30e3ca
                (248, 243, 212), // 白色 #f8f3d4
                t!("info").to_string(),
            ),
            LogLevel::Debug => (
                (82, 97, 107), // 灰色 #52616b
                (82, 97, 107), // 灰色 #52616b
                t!("debug").to_string(),
            ),
        };

        let level_display = level_str.truecolor(level_color.0, level_color.1, level_color.2);
        let message = t!(key).truecolor(message_color.0, message_color.1, message_color.2);

        println!("{} [{}] {}", time, level_display, message);
    }
}

pub fn set_timezone(tz: Tz) {
    LOGGER.lock().unwrap().set_timezone(tz);
}

#[macro_export]
macro_rules! error {
    ($key:expr) => {{
        $crate::logger::LOGGER.lock().unwrap().log(
            $crate::logger::LogLevel::Error,
            $key
        );
    }};
}

#[macro_export]
macro_rules! warn {
    ($key:expr) => {{
        $crate::logger::LOGGER.lock().unwrap().log(
            $crate::logger::LogLevel::Warning,
            $key
        );
    }};
}

#[macro_export]
macro_rules! info {
    ($key:expr) => {{
        $crate::logger::LOGGER.lock().unwrap().log(
            $crate::logger::LogLevel::Info,
            $key
        );
    }};
}

#[macro_export]
macro_rules! debug {
    ($key:expr) => {{
        $crate::logger::LOGGER.lock().unwrap().log(
            $crate::logger::LogLevel::Debug,
            $key
        );
    }};
}