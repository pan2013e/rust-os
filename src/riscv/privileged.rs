use core::arch::asm;

#[inline(always)]
pub fn sfence_vma() {
    unsafe {
        asm!("sfence.vma");
    }
}

#[inline(always)]
pub fn rdtime() -> u64 {
    let tm : u64;
    unsafe {
        asm!(
            "rdtime {tm}",
            tm = out(reg) tm
        );
    }
    return tm;
}