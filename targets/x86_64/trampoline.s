	.section .text.trampoline
	.align 2
	.global _start_trampoline

_start_trampoline:
	jmp	_start_kernel
