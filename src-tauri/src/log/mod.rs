use lazy_static::lazy_static;
use log::SetLoggerError;

use crate::log::console_logger::ConsoleLogger;

mod console_logger;

lazy_static! {
    static ref CONSOLE_LOGGER: Box<dyn log::Log> = Box::new(ConsoleLogger);
}

pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(CONSOLE_LOGGER.as_ref())
        .map(|()| log::set_max_level(log::LevelFilter::Debug))
}
