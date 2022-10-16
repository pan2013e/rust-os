OUTPUT_ARCH("riscv")
ENTRY(_start)
BASE_ADDR = 0x80200000;

SECTIONS
{
    . = BASE_ADDR;

    _skernel = .;

    .text : ALIGN(0x1000){
        _stext = .;

        *(.text.entry)
        *(.text .text.*)
        
        _etext = .;
    }

    .rodata : ALIGN(0x1000){
        _srodata = .;

        *(.rodata .rodata.*)
        
        _erodata = .;
    }

    .data : ALIGN(0x1000){
        _sdata = .;

        *(.data .data.*)
        
        _edata = .;
    }

    .bss : ALIGN(0x1000){
        _sbss = .;

        *(.bss.stack)
        *(.sbss .sbss.*)
        *(.bss .bss.*)
        
        _ebss = .;
    }

    _ekernel = .;
}
