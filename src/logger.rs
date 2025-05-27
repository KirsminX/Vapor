use chrono::{Local, TimeZone};
use chrono_tz::Tz;
use colored::Colorize;
use lazy_static::lazy_static;
use rust_i18n::t;
use std::sync::Mutex;

lazy_static! {
    pub static ref LOGGER: Mutex<Logger> = Mutex::new(Logger::new());
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

pub struct Logger {
    timezone: Option<Tz>,
    language: Option<String>,
    min_level: LogLevel,
}

impl Logger {
    fn new() -> Self {
        Self {
            timezone: None,
            language: Some("en".to_string()),
            min_level: LogLevel::Debug,
        }
    }

    pub fn set_min_level(&mut self, level: LogLevel) {
        self.min_level = level;
    }

    pub fn set_timezone(&mut self, tz: Tz) {
        self.timezone = Some(tz);
    }

    pub fn set_language(&mut self, lang: &str) {
        rust_i18n::set_locale(lang);
        self.language = Some(lang.to_string());
    }

    fn format_time(&self) -> String {
        match self.timezone {
            Some(ref tz) => {
                let dt = tz.from_local_datetime(&Local::now().naive_local()).unwrap();
                dt.format("%Y/%-m/%-d %H:%M:%S").to_string()
            }
            None => {
                let dt = Local::now();
                dt.format("%Y/%-m/%-d %H:%M:%S").to_string()
            }
        }
    }

    pub fn log(&self, level: LogLevel, key: &str) {
        if level < self.min_level {
            return;
        }

        let time = self.format_time();

        let (level_color, message_color, level_str) = match level {
            LogLevel::Error => (
                (255, 46, 99),
                (255, 46, 99),
                t!("error").to_string(),
            ),
            LogLevel::Warning => (
                (249, 237, 105),
                (249, 237, 105),
                t!("warning").to_string(),
            ),
            LogLevel::Info => (
                (48, 227, 202),
                (255, 255, 255),
                t!("info").to_string(),
            ),
            LogLevel::Debug => (
                (82, 97, 107),
                (82, 97, 107),
                t!("debug").to_string(),
            ),
        };

        let level_display = format!("[{}] ", level_str).truecolor(level_color.0, level_color.1, level_color.2);

        let raw_message = t!(key);
        let final_message = if raw_message == key {
            let lang = self.language.as_ref().unwrap();
            let tz_str = self.timezone.as_ref().map(|tz| tz.name()).unwrap_or("unknown");
            format!("翻译失败！Translate Failed! | 语言 Lang {} | 时区 Tz {} | 内容 Value {}", lang, tz_str, key)
        } else {
            raw_message.into_owned()
        };
        let colored_message = final_message.truecolor(message_color.0, message_color.1, message_color.2);

        println!("{} {}{}", time, level_display, colored_message);
    }
}

#[macro_export]
macro_rules! tz {
    ($tz_str:expr) => {{
        use std::str::FromStr;
        chrono_tz::Tz::from_str($tz_str).unwrap_or_else(|_| panic!("TimeZoneError: Invalid time zone: {}", $tz_str))
    }};
}

#[macro_export]
macro_rules! init_logger {
    ( 
        min_level = $level:expr,
        language = $lang:expr,
        timezone = $tz_str:expr $(,)?
    ) => {
        {
            let mut logger = $crate::logger::LOGGER.lock().unwrap();
            logger.set_min_level($level);
            logger.set_language($lang);
            let tz = $crate::tz!($tz_str);
            logger.set_timezone(tz);
        }
    };
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