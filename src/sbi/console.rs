#[macro_use]

use core::fmt::{self, Write};

use crate::sbi::ecall::{sbi_ecall, SBI_PUTCHAR};

fn console_puts(_str : &str) {
    for _ch in _str.chars() {
        console_putchar(_ch);
    }
    console_putchar('\n');
}

fn console_putchar(ch : char) {
    sbi_ecall(SBI_PUTCHAR, 0, 
        ch as u64, 0, 0, 0, 0, 0);
}

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for _ch in s.chars() {
            console_putchar(_ch);
        }
        Ok(())
    }
}

pub fn print(args : fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::sbi::console::print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::sbi::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}