ENTRY(_start_trampoline)

SECTIONS
{
	/* The trampoline, code that jumps to the kernel, sits here */
	. = 0x00000;
	.text.boot : {
		KEEP(*(.text.trampoline))
	}

	/* The actual kernel sits here */
	. = 0x80000;             /* Load address for QEMU virt if possible */
	.text : { *(.text*) } :text
	.rodata : { *(.rodata*) } :text
	.data : { *(.data*) } :data
	.bss : { *(.bss*) } :data
}

PHDRS
{
    text PT_LOAD;
    data PT_LOAD;
}
