    .section .text.init
    .globl _start
    
_start:
    la sp, boot_stack_top
    j start_kernel