	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.private_extern	__ZN3std2rt10lang_start17h8f3f1d2dc6734b7fE
	.globl	__ZN3std2rt10lang_start17h8f3f1d2dc6734b7fE
	.p2align	2
__ZN3std2rt10lang_start17h8f3f1d2dc6734b7fE:
	.cfi_startproc
	sub	sp, sp, #32
	.cfi_def_cfa_offset 32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	mov	x4, x3
	mov	x3, x2
	mov	x2, x1
	str	x0, [sp, #8]
Lloh0:
	adrp	x1, l_anon.85c3667675896acf9cc0e42e0cf0ca3e.0@PAGE
Lloh1:
	add	x1, x1, l_anon.85c3667675896acf9cc0e42e0cf0ca3e.0@PAGEOFF
	add	x0, sp, #8
	bl	__ZN3std2rt19lang_start_internal17h7e788da8c79e20dcE
	.cfi_def_cfa wsp, 32
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.loh AdrpAdd	Lloh0, Lloh1
	.cfi_endproc

	.p2align	2
__ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hc0676eddb8145381E:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	ldr	x0, [x0]
	bl	__ZN3std3sys9backtrace28__rust_begin_short_backtrace17h42045ea58f9d9c4dE
	mov	w0, #0
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN3std3sys9backtrace28__rust_begin_short_backtrace17h42045ea58f9d9c4dE:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	blr	x0
	; InlineAsm Start
	; InlineAsm End
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h53e7a5429aeb5b8dE:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	ldr	x0, [x0]
	bl	__ZN3std3sys9backtrace28__rust_begin_short_backtrace17h42045ea58f9d9c4dE
	mov	w0, #0
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.private_extern	__ZN4main4main17hfa203070a8b0b79bE
	.globl	__ZN4main4main17hfa203070a8b0b79bE
	.p2align	2
__ZN4main4main17hfa203070a8b0b79bE:
	.cfi_startproc
	sub	sp, sp, #80
	.cfi_def_cfa_offset 80
	stp	x29, x30, [sp, #64]
	add	x29, sp, #64
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
Lloh2:
	adrp	x8, __ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$3fmt17hf71ca2f276637ae9E@GOTPAGE
Lloh3:
	ldr	x8, [x8, __ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$3fmt17hf71ca2f276637ae9E@GOTPAGEOFF]
Lloh4:
	adrp	x9, l_anon.85c3667675896acf9cc0e42e0cf0ca3e.1@PAGE
Lloh5:
	add	x9, x9, l_anon.85c3667675896acf9cc0e42e0cf0ca3e.1@PAGEOFF
	stp	x9, x8, [x29, #-16]
Lloh6:
	adrp	x8, l_anon.85c3667675896acf9cc0e42e0cf0ca3e.4@PAGE
Lloh7:
	add	x8, x8, l_anon.85c3667675896acf9cc0e42e0cf0ca3e.4@PAGEOFF
	mov	w9, #2
	stp	x8, x9, [sp]
	sub	x8, x29, #16
	mov	w9, #1
	str	x8, [sp, #16]
	stp	x9, xzr, [sp, #24]
	mov	x0, sp
	bl	__ZN3std2io5stdio6_print17h657677ef6fd356f5E
	.cfi_def_cfa wsp, 80
	ldp	x29, x30, [sp, #64]
	add	sp, sp, #80
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.loh AdrpAdd	Lloh6, Lloh7
	.loh AdrpAdd	Lloh4, Lloh5
	.loh AdrpLdrGot	Lloh2, Lloh3
	.cfi_endproc

	.globl	_get_k
	.p2align	2
_get_k:
	.cfi_startproc
	mov	w0, #45
	ret
	.cfi_endproc

	.globl	_main
	.p2align	2
_main:
	.cfi_startproc
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	mov	x3, x1
	sxtw	x2, w0
Lloh8:
	adrp	x8, __ZN4main4main17hfa203070a8b0b79bE@PAGE
Lloh9:
	add	x8, x8, __ZN4main4main17hfa203070a8b0b79bE@PAGEOFF
	str	x8, [sp, #8]
Lloh10:
	adrp	x1, l_anon.85c3667675896acf9cc0e42e0cf0ca3e.0@PAGE
Lloh11:
	add	x1, x1, l_anon.85c3667675896acf9cc0e42e0cf0ca3e.0@PAGEOFF
	add	x0, sp, #8
	mov	w4, #0
	bl	__ZN3std2rt19lang_start_internal17h7e788da8c79e20dcE
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
	.loh AdrpAdd	Lloh10, Lloh11
	.loh AdrpAdd	Lloh8, Lloh9
	.cfi_endproc

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.85c3667675896acf9cc0e42e0cf0ca3e.0:
	.asciz	"\000\000\000\000\000\000\000\000\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	__ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h53e7a5429aeb5b8dE
	.quad	__ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hc0676eddb8145381E
	.quad	__ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hc0676eddb8145381E

	.section	__TEXT,__literal4,4byte_literals
	.p2align	2, 0x0
l_anon.85c3667675896acf9cc0e42e0cf0ca3e.1:
	.asciz	"-\000\000"

	.section	__TEXT,__literal8,8byte_literals
l_anon.85c3667675896acf9cc0e42e0cf0ca3e.2:
	.ascii	"result: "

	.section	__TEXT,__const
l_anon.85c3667675896acf9cc0e42e0cf0ca3e.3:
	.byte	10

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.85c3667675896acf9cc0e42e0cf0ca3e.4:
	.quad	l_anon.85c3667675896acf9cc0e42e0cf0ca3e.2
	.asciz	"\b\000\000\000\000\000\000"
	.quad	l_anon.85c3667675896acf9cc0e42e0cf0ca3e.3
	.asciz	"\001\000\000\000\000\000\000"

.subsections_via_symbols
