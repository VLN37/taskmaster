#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum LogLevel {
    DEBUG,
    INFO,
    WARN,
    ERROR,
}

impl From<&str> for LogLevel {
    fn from(value: &str) -> Self {
        match value {
            "ERROR" => LogLevel::ERROR,
            "INFO" => LogLevel::INFO,
            "WARN" => LogLevel::WARN,
            "WARNING" => LogLevel::WARN,
            "DEBUG" => LogLevel::DEBUG,
            _ => panic!("{value} is not a valid LogLevel"),
        }
    }
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            LogLevel::ERROR => "ERROR",
            LogLevel::WARN => "WARN",
            LogLevel::INFO => "INFO",
            LogLevel::DEBUG => "DEBUG",
        };
        formatter.pad(val)
    }
}
