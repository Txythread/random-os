	.section .text.trampoline
	.align 2
	.global _start_trampoline

_start_trampoline:
	ldr	x0, =kernel_stack
	mov	sp, x0
	b	_kernel_start
