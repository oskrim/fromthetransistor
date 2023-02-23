	.section	__TEXT,__text,regular,pure_instructions
	.syntax unified
	.globl	_main
	.p2align	2
	.code	32
_main:
	sub	sp, sp, #8
	mov	r0, #0
	str	r0, [sp, #4]
	mov	r0, #1024
	ldr	r0, [r0]
	str	r0, [sp]
	cmp	r0, #123
	ble	LBB0_2
	mov	r0, #42
	str	r0, [sp, #4]
	b	LBB0_3
LBB0_2:
	ldr	r0, [sp]
	cmp	r0, #99
	mov	r0, #20
	movle	r0, #10
	str	r0, [sp]
LBB0_3:
	add	r0, sp, #4
	add	sp, sp, #8
	bx	lr

.subsections_via_symbols
