use core::arch::asm;

#[inline(always)]
pub fn ecall() {
    unsafe {
        asm!("ecall");
    }
}