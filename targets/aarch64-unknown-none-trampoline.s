	.section .text.trampoline
	.align 2
	.global _start_trampoline

_start_trampoline:
	b	_start_kernel
