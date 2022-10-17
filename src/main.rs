#![no_std]
#![no_main]
#![allow(unused)]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

pub mod sbi;
pub mod misc;
pub mod riscv;
pub mod kernel;

extern crate alloc;

use core::arch::asm;
use core::arch::global_asm;
use alloc::boxed::Box;

use sbi::console::*;
use sbi::device::*;
use sbi::timer::timer_init;
use misc::rust_lang::panic;
use riscv::privileged::*;
use kernel::traps::trap_handler;
use kernel::mm::mm_init;

use virtio_drivers::*;

use crate::kernel::interrupt::irq_enable;

global_asm!(include_str!("kernel/head.S"));
global_asm!(include_str!("kernel/entry.S"));

#[no_mangle]
pub extern "C" fn start_kernel() {
    println!("rust-os booting...");
    irq_enable();
    timer_init();
    mm_init();
    loop {}
    shutdown();
}