use lazy_static::lazy_static;
use log::SetLoggerError;

use crate::log::console_logger::ConsoleLogger;

mod console_logger;

lazy_static! {
    static ref CONSOLE_LOGGER: Box<dyn log::Log> = Box::new(ConsoleLogger);
}

/// Initializes the global logger with a console-based logger and sets the maximum log level to `Debug`.
///
/// Returns an error if the logger has already been set or if initialization fails.
///
/// # Examples
///
/// ```
/// log::init().expect("Logger initialization failed");
/// ```
pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(CONSOLE_LOGGER.as_ref()).map(|()| log::set_max_level(log::LevelFilter::Debug))
}
