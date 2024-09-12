use core::fmt;
use atomic_refcell::AtomicRefCell;

pub use crate::port::PortWriteOnly;
pub use log;

pub struct Serial;

// port com_1 0x3f8, port com_2 0x2f8
pub static PORT: AtomicRefCell<PortWriteOnly<u8>> = AtomicRefCell::new(PortWriteOnly::new(0x3f8));

static LOG_LEVEL_NAMES: [&str; 5] = ["ERROR", "WARN", "INFO", "DEBUG", "TRACE"];

#[derive(Clone, Copy)]
pub enum LogLevel {
    Error = 0,
    Warn,
    Info,
    Debug,
    Trace,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.pad(LOG_LEVEL_NAMES[*self as usize])
    }
}

impl fmt::Write for Serial {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let mut port = PORT.borrow_mut();
        for b in s.bytes() {
            unsafe { port.write(b) }
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! serial_log {
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        writeln!(crate::logger::Serial, $($arg)*).unwrap();
    }};
}

#[macro_export]
macro_rules! info {
    ($format:expr) => (
        log::info!($format);
        crate::serial_log!(concat!("[{:>5}]: {:>20}@{:<3}: ", $format), $crate::logger::LogLevel::Info, file!(), line!());
    );
    ($format:expr, $($args:tt)*) => (
        log::info!($format, $($args)*);
        crate::serial_log!(concat!("[{:>5}]: {:>20}@{:<3}: ", $format), $crate::logger::LogLevel::Info, file!(), line!(), $($args)*);
    )
}

#[macro_export]
macro_rules! debug {
    ($format:expr) => (
        log::debug!($format);
        crate::serial_log!(concat!("[{:>5}]: {:>20}@{:<3}: ", $format), $crate::logger::LogLevel::Debug, file!(), line!());
    );
    ($format:expr, $($args:tt)*) => (
        log::debug!($format, $($args)*);
        crate::serial_log!(concat!("[{:>5}]: {:>20}@{:<3}: ", $format), $crate::logger::LogLevel::Debug, file!(), line!(), $($args)*);
    )
}

#[macro_export(local_inner_macros)]
macro_rules! warn {
    ($format:expr) => (
        log::warn!($format);
        serial_log!(concat!("[{:>5}]: {:>20}@{:<3}: ", $format), $crate::logger::LogLevel::Warn, file!(), line!());
    );
    ($format:expr, $($args:tt)*) => (
        log::warn!($format, $($args)*);
        crate::serial_log!(concat!("[{:>5}]: {:>20}@{:<3}: ", $format), $crate::logger::LogLevel::Info, file!(), line!(), $($args)*);
    )
}

#[macro_export]
macro_rules! error {
    ($format:expr) => (
        log::error!($format);
        crate::serial_log!(concat!("[{:>5}]: {:>20}@{:<3}: ", $format), $crate::logger::LogLevel::Error, file!(), line!());
    );
    ($format:expr, $($args:tt)*) => (
        log::error!($format, $($args)*);
        crate::serial_log!(concat!("[{:>5}]: {:>20}@{:<3}: ", $format), $crate::logger::LogLevel::Error, file!(), line!(), $($args)*);
    )
}

#[macro_export]
macro_rules! trace {
    ($format:expr) => (
        log::trace!($format);
        crate::serial_log!(concat!("[{:>5}]: {:>20}@{:<3}: ", $format), $crate::logger::LogLevel::Debug, file!(), line!());
    );
    ($format:expr, $($args:tt)*) => (
        log::trace!($format, $($args)*);
        crate::serial_log!(concat!("[{:>5}]: {:>20}@{:<3}: ", $format), $crate::logger::LogLevel::Trace, file!(), line!(), $($args)*);
    )
}

#[warn(unused_imports)]
pub(crate) use info;
pub(crate) use debug;
pub(crate) use crate::warn;
pub(crate) use crate::error;
pub(crate) use crate::trace;
