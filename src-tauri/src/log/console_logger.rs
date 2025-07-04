pub struct ConsoleLogger;

impl log::Log for ConsoleLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            println!(
                "{} - {}::{} - {}",
                record.level().as_str().chars().nth(0).unwrap(),
                record.file().unwrap(),
                record.line().unwrap(),
                record.args()
            )
        }
    }

    fn flush(&self) {}
}
