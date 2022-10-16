.extern start_kernel, _traps

    .section .text.entry
    .globl _start
_start:
    la t0, _traps
    csrw stvec, t0
    li t0, 32
    csrs sie, t0
    li t0, 10000000
    li a7, 0
    li a6, 0 
    rdtime t1
    add a0, t1, t0
    ecall
    csrsi sstatus, 2
    la sp, boot_stack_top
    call start_kernel

    .section .bss.stack
    .globl boot_stack
boot_stack:
    .space 0x1000

    .globl boot_stack_top
boot_stack_top: