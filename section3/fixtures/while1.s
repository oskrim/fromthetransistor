	.text
	.syntax unified
	.eabi_attribute	67, "2.09"
	.eabi_attribute	6, 2
	.eabi_attribute	8, 1
	.eabi_attribute	9, 1
	.eabi_attribute	34, 1
	.eabi_attribute	17, 1
	.eabi_attribute	20, 1
	.eabi_attribute	21, 0
	.eabi_attribute	23, 3
	.eabi_attribute	24, 1
	.eabi_attribute	25, 1
	.eabi_attribute	38, 1
	.eabi_attribute	14, 0
	.file	"my cool jit"
	.globl	main
	.p2align	2
	.type	main,%function
	.code	32
main:
	.fnstart
	.pad	#4
	sub	sp, sp, #4
	mov	r0, #0
	mov	r1, #134217728
	str	r0, [sp]
	mov	r0, #65536
	orr	r0, r0, #134217728
.LBB0_1:
	ldr	r2, [r0]
	add	r2, r2, #1
	str	r2, [r1]
	b	.LBB0_1
.Lfunc_end0:
	.size	main, .Lfunc_end0-main
	.fnend

	.section	".note.GNU-stack","",%progbits
	.eabi_attribute	30, 1
