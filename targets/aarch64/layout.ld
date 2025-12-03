ENTRY(_kernel_start)

SECTIONS
{
	/* The trampoline, code that jumps to the kernel, sits here */
	. = 0x00000;
	.text.boot : {
		KEEP(*(.text.trampoline))
	}

	. = 0x00010;

	.stack (NOLOAD) : {
        	stack_start = .;            /* address of start (bottom) of stack */
        	. += 4K;                   /* size of supervisor stack */
        	kernel_stack = .;              /* address of top of stack, 0x4010 */
    	} :data

	/* The actual kernel sits here */
	. = 0x80000;             /* Load address for QEMU virt if possible */
	.text : { *(.text*) } :text
	.rodata : { *(.rodata*) } :text
	.data : { *(.data*) } :data
	.bss : { *(.bss*) } :data
	.eh_frame_hdr : { *(.eh_frame_hdr*) } :data
	.strtab : { *(.strtab*) } :data
	
}
PHDRS
{
    text PT_LOAD;
    data PT_LOAD;
}
