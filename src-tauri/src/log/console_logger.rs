pub struct ConsoleLogger;

impl log::Log for ConsoleLogger {
    /// Always returns `true`, enabling logging for all metadata.
    ///
    /// This method allows all log records to be processed by the logger.
    ///
    /// # Examples
    ///
    /// ```
    /// let logger = ConsoleLogger;
    /// assert!(logger.enabled(&log::Metadata::builder().build()));
    /// ```
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    /// Outputs a formatted log message to the console if logging is enabled for the given record.
    ///
    /// The log entry includes the first character of the log level, the source file name, the line number, and the log message content.
    ///
    /// # Examples
    ///
    /// ```
    /// use log::{Record, Level, Metadata};
    /// let logger = ConsoleLogger;
    /// let record = Record::builder()
    ///     .level(Level::Info)
    ///     .file(Some("main.rs"))
    ///     .line(Some(42))
    ///     .args(format_args!("Hello, world!"))
    ///     .build();
    /// logger.log(&record);
    /// // Output: I - main.rs::42 - Hello, world!
    /// ```
    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            println!(
                "{} - {}::{} - {}",
                record.level().as_str().chars().next().unwrap_or('?'),
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                record.args()
            )
        }
    }

    /// Flushes any buffered log records.
///
/// This implementation does nothing, as log messages are written directly to the console and require no flushing.
fn flush(&self) {}
}
