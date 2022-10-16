#![no_std]
#![no_main]
#![allow(unused)]
#![feature(panic_info_message)]

pub mod sbi;
pub mod misc;
pub mod riscv;
pub mod kernel;

use core::arch::global_asm;

use sbi::console::*;
use sbi::device::*;
use misc::rust_lang::panic;
use riscv::privileged::*;
use kernel::traps::trap_handler;

global_asm!(include_str!("kernel/head.S"));
global_asm!(include_str!("kernel/entry.S"));

#[no_mangle]
pub extern "C" fn start_kernel() {
    loop {}
    shutdown();
}