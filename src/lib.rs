extern crate log;
extern crate term_painter;

use std::io;
use std::io::Write;
use term_painter::{Painted, ToStyle};
use term_painter::Color::{NotSet, Yellow};
use log::{Log, LogLevel, LogLevelFilter, LogRecord, SetLoggerError, LogMetadata};
use std::ascii::AsciiExt;


pub struct Logger {
    level: LogLevelFilter, 
}


impl Logger {
    pub fn new() -> Logger {
        Logger {
            level: LogLevelFilter::Info,
        }
    }

    pub fn set_level(&mut self, level: LogLevel) -> &mut Self {
        self.level = level.to_log_level_filter();
        self
    }

    fn format_level(&self, record: &LogRecord) -> Painted<String> {
        let (level_style, print_level) = match record.level() {
            LogLevel::Error => (Yellow.to_style(), true),
            LogLevel::Warn  => (Yellow.to_style(), true),
            LogLevel::Debug => (Yellow.to_style(), true),
            LogLevel::Trace => (Yellow.dim(), true),
            _ => (NotSet.to_style(), cfg!(ndebug)),
        };


        if print_level {
            let level_name = format!("{}", record.level()).to_ascii_lowercase();
            let (head, tail) = level_name.split_at(1);
            let mut level_name2 = String::with_capacity(head.len() + tail.len());
            level_name2.push_str(&head.to_ascii_uppercase());
            level_name2.push_str(&tail.to_ascii_lowercase());
            level_name2.push_str(": ");
            level_style.paint(level_name2)
        } else {
            NotSet.paint(String::new())
        }
    }

    fn format_location(&self, record: &LogRecord) -> String {
        if cfg!(ndebug) {
            format!("{}:{} ", record.location().file(), record.location().line())
        } else {
            String::new()
        }
    }

    pub fn enabled(&self, level: LogLevel, _target: &str) -> bool {
        level <= self.level
    } 
}


impl Log for Logger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        self.enabled(metadata.level(), metadata.target())
    }

    fn log(&self, record: &LogRecord) {
        if !Log::enabled(self, record.metadata()) {
            return;
        }

        if record.level() <= LogLevel::Warn {
            let _ = writeln!(&mut io::stderr(), "{}{}{}", self.format_level(record), self.format_location(record), record.args());
        } else {
            let _ = writeln!(&mut io::stdout(), "{}{}{}", self.format_level(record), self.format_location(record), record.args());
        };
    }
}


pub fn init(level: LogLevel) -> Result<(), SetLoggerError> {
    let mut logger = Box::new(Logger::new());
    logger.set_level(level);

    log::set_logger(|max_level| {
        max_level.set(level.to_log_level_filter());
        logger as Box<Log + 'static>
    })
}



#[cfg(test)]
mod tests {
    use log::{Log, LogLevel};
    use super::{Logger};
    
    #[test]
    fn log_level() {
        let mut logger = Logger::new();
        assert!(logger.enabled(LogLevel::Error, "crate1"));
        assert!(logger.enabled(LogLevel::Info, "crate1"));
        assert!(!logger.enabled(LogLevel::Debug, "crate1"));
        assert!(!logger.enabled(LogLevel::Trace, "crate1"));

        logger.set_level(LogLevel::Debug);
        assert!(logger.enabled(LogLevel::Error, "crate1"));
        assert!(logger.enabled(LogLevel::Debug, "crate1"));
        assert!(!logger.enabled(LogLevel::Trace, "crate1"));

        logger.set_level(LogLevel::Trace);
        assert!(logger.enabled(LogLevel::Debug, "crate1"));
        assert!(logger.enabled(LogLevel::Trace, "crate1"));
    }

}
