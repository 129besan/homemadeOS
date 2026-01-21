use core::fmt;
use crate::drivers::serial;

pub struct SerialWriter;

impl fmt::Write for SerialWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        serial::write_string(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! kprint {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        let _ = write!($crate::log::SerialWriter, $($arg)*);
    });
}

#[macro_export]
macro_rules! kprintln {
    () => ($crate::kprint!("\n"));
    ($($arg:tt)*) => ($crate::kprint!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => ($crate::kprint!("[INFO] {}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => ($crate::kprint!("[WARN] {}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => ($crate::kprint!("[ERROR] {}\n", format_args!($($arg)*)));
}
