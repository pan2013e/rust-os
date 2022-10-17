use core::arch::asm;

#[inline(always)]
pub fn irq_enable() {
    unsafe {
        asm!(
            "la t0, _traps",
            "csrw stvec, t0",
            "li t0, 544",     // 2^9 + 2^5
            "csrs sie, t0",   // set sie[9] & sie[5]
            "li t0, 1 << 10",
            "li t1, 0x0c002080",  // enable sext in OpenSBI
            "sw t0, 0(t1)",
            "li t1, 0x10000004",  // enable uart serial
            "sb x0, 0(t1)",
            "li t0, 1",
            "li t1, 0x10000001",
            "sb t0, 0(t1)",
            "li t0, 7",
            "li t1, 0x0c000028", // QEMU magic numbers
            "sw t0, 0(t1)",
            "li t1, 0x0c201000",
            "sw x0, 0(t1)",
            "csrsi sstatus, 2"
        );
    }
}