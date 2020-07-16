pub enum Logger { }

impl Logger {
	// TODO: Feature flag
	const VERBOSITY: LogLevel = LogLevel::Debug;
	
	pub fn log(level: LogLevel, msg: &str) {
		if level >= Self::VERBOSITY {
			println!("[{}]: {}", <&str>::from(level), msg)
		}
	}
	
	pub fn debug(msg: &str) {
		Self::log(LogLevel::Debug, msg);
	}
	
	pub fn info(msg: &str) {
		Self::log(LogLevel::Info, msg);
	}
	
	pub fn warning(msg: &str) {
		Self::log(LogLevel::Warning, msg);
	}
	
	pub fn error(msg: &str) {
		Self::log(LogLevel::Error, msg);
	}
	
	pub fn critical(msg: &str) {
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
