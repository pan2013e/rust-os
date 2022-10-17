use core::arch::asm;

use crate::println;
use crate::sbi::ecall::{sbi_ecall, SBI_SET_TIMER};
use crate::riscv::privileged::rdtime;

const QEMU_CLK_FREQ : u64 = 10000000;

#[inline(always)]
pub fn clock_set_next_event() {
    sbi_ecall(SBI_SET_TIMER, 0, 
        rdtime() + QEMU_CLK_FREQ, 0, 0, 0, 0, 0);
}

#[inline(always)]
pub fn timer_init() {
    unsafe {
        asm!(
            "li a7, 0",
            "li a6, 0",
            "add a0, {time}, {freq}",
            "ecall",
            time = in(reg) rdtime(),
            freq = in(reg) QEMU_CLK_FREQ
        );
    }
    println!("timer_init complete");
}