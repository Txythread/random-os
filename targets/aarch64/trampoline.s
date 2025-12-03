	.section .text.trampoline
	.align 2
	.global _start_trampoline

_start_trampoline:
	mov	x0, 0x4010
	mov	sp, x0
	b	_kernel_start
