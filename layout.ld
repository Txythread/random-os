ENTRY(_start)

SECTIONS
{
    . = 0x80000;             /* Load address for QEMU virt */

    .text : { *(.text*) } :text
    .rodata : { *(.rodata*) } :text
    .data : { *(.data*) } :data
    .bss : { *(.bss*) } :data
}

PHDRS
{
    text PT_LOAD FILEHDR PHDRS;
    data PT_LOAD FILEHDR PHDRS;
}

