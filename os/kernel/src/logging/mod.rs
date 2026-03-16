use core::fmt;

use x86::io::outb;

use crate::vga;

#[derive(Clone, Copy, Debug)]
pub enum LogLevel {
    Trace,
    Info,
    Warn,
    Error,
}

impl LogLevel {
    fn as_str(self) -> &'static str {
        match self {
            Self::Trace => "TRACE",
            Self::Info => "INFO",
            Self::Warn => "WARN",
            Self::Error => "ERROR",
        }
    }
}

pub fn init() {
    serial_write_str("[INFO] serial logger initialized\n");
}

pub fn log(level: LogLevel, args: fmt::Arguments<'_>) {
    let mut line_buf = [0u8; 512];
    let mut cursor = 0usize;

    let _ = fmt::write(
        &mut BufferWriter {
            buffer: &mut line_buf,
            cursor: &mut cursor,
        },
        format_args!("[{}] {}", level.as_str(), args),
    );

    let msg = core::str::from_utf8(&line_buf[..cursor]).unwrap_or("[log utf8 err]");
    serial_write_str(msg);
    serial_write_str("\n");

    vga::_print(format_args!("{}\n", msg));
}

pub fn serial_write_str(s: &str) {
    for b in s.bytes() {
        serial_write_byte(b);
    }
}

pub fn serial_write_byte(byte: u8) {
    unsafe {
        outb(0x3F8, byte);
    }
}

struct BufferWriter<'a> {
    buffer: &'a mut [u8],
    cursor: &'a mut usize,
}

impl fmt::Write for BufferWriter<'_> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for b in s.bytes() {
            if *self.cursor >= self.buffer.len() {
                return Ok(());
            }
            self.buffer[*self.cursor] = b;
            *self.cursor += 1;
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! klog {
    ($lvl:expr, $($arg:tt)*) => {
        $crate::logging::log($lvl, format_args!($($arg)*))
    };
}
