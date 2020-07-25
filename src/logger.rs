use core::fmt;
use winapi::km::wdm;

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => ($crate::logger::_log_message(format_args!($($arg)*)));
}

pub fn _log_message(args: fmt::Arguments) {

    let logger = Logger::new();
    let s = format!("{}", args);
    logger.write_string(s.as_str());
}

struct Logger {}

impl fmt::Write for Logger {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

impl Logger {
    pub fn new() -> Self {
        Logger {}
    }

    pub fn write_string(&self, s : &str) {
        let log_message = format!("{}\0", s);
        unsafe {
            wdm::DbgPrint(
                "%s".as_bytes().as_ptr(),
                log_message.as_str().as_ptr());
        }
    }
}