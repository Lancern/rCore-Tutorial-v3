use core::fmt::Write;

use log::{Level, LevelFilter, Log, Metadata, Record};

use crate::sbi::console_putchar;

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.chars() {
            console_putchar(c as usize);
        }
        Ok(())
    }
}

pub fn print(args: core::fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}

struct Logger;

impl Log for Logger {
    fn enabled(&self, _: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        match record.level() {
            Level::Error => {
                print!("\x1b[31m[ERR] ");
            },
            Level::Warn => {
                print!("\x1b[93m[WRN] ");
            },
            Level::Info => {
                print!("\x1b[34m[INF] ");
            },
            Level::Debug => {
                print!("\x1b[32m[DBG] ");
            },
            Level::Trace => {
                print!("\x1b[90m[TRC] ");
            },
        };
        print(*record.args());
        println!("\x1b[0m");
    }

    fn flush(&self) {}
}

static LOGGER_SINGLETON: Logger = Logger;

pub fn init_log() {
    log::set_logger(&LOGGER_SINGLETON).expect("failed to set logger");
    log::set_max_level(LevelFilter::Info);
}
