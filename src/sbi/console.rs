#[macro_use]

use core::fmt::{self, Write};

use crate::sbi::ecall::{sbi_ecall, SBI_PUTCHAR, SBI_GETCHAR, SbiRet};

const LF: u8  = 0x0a;
const CR: u8  = 0x0d;
const _BS: u8 = 0x08; // '\b'
const BS: u8  = 0x7f;

const BUF_SIZE: usize = 4096;
pub static mut BUF_PTR: usize = 0;
pub static mut BUF: [u8; BUF_SIZE] = [0; BUF_SIZE];

#[inline(always)]
fn ctrl_x_to_ascii(ch: char) -> u8 {
    ch as u8 - '@' as u8
}

fn console_puts(_str: &str) {
    for _ch in _str.chars() {
        console_putchar(_ch);
    }
    console_putchar('\n');
}

pub fn console_putchar(ch : char) {
    sbi_ecall(SBI_PUTCHAR, 0, 
        ch as u64, 0, 0, 0, 0, 0);
}

pub fn console_echo(ch: u8) {
    match ch {
        LF | CR => {
            crate::print!("{}{}", LF as char, CR as char);
        }
        BS => {
            console_putchar(_BS as char);
            console_putchar(' ');
            console_putchar(_BS as char);
        }
        _ => {
            console_putchar(ch as char);
        }
    }
}

pub fn console_getchar() -> u8 {
    let ret = sbi_ecall(SBI_GETCHAR, 0, 0, 0, 0, 0, 0, 0);
    return ret.get_value() as u8
}

pub fn console_readline() {

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