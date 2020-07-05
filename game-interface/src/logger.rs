pub trait Logger {
	const VERBOSITY: LogLevel;
	
	fn print(raw: &str);
	
	fn log(level: LogLevel, msg: &str) {
		if level >= Self::VERBOSITY {
			Self::print(&format!("[{}]: {}", <&str>::from(level), msg))
		}
	}
	
	fn debug(msg: &str) {
		Self::log(LogLevel::Debug, msg);
	}
	
	fn info(msg: &str) {
		Self::log(LogLevel::Info, msg);
	}
	
	fn warning(msg: &str) {
		Self::log(LogLevel::Warning, msg);
	}
	
	fn error(msg: &str) {
		Self::log(LogLevel::Error, msg);
	}
	
	fn critical(msg: &str) {
		Self::log(LogLevel::Critical, msg);
	}
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum LogLevel {
	Debug = 0,
	Info = 1,
	Warning = 2,
	Error = 3,
	Critical = 4,
}

impl From<LogLevel> for &str {
	fn from(level: LogLevel) -> Self {
		match level {
			LogLevel::Debug => "DEBUG",
			LogLevel::Info => "INFO",
			LogLevel::Warning => "WARNING",
			LogLevel::Error => "ERROR",
			LogLevel::Critical => "CRITICAL",
		}
	}
}
