use crate::sbi::ecall::{sbi_ecall, SBI_SET_TIMER};
use crate::riscv::privileged::rdtime;

const QEMU_CLK_FREQ : u64 = 10000000;

#[inline(always)]
pub fn clock_set_next_event() {
    sbi_ecall(SBI_SET_TIMER, 0, 
        rdtime() + QEMU_CLK_FREQ, 0, 0, 0, 0, 0);
}