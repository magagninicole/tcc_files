.option norvc # disable generation of compressed instructions

.align 4
.global _switch_to_user
_switch_to_user:
        # a0 - Frame address
        # a1 - Program counter
        # a2 - SATP Register
        csrw    mscratch, a0

        // load program counter
        ld a1,  520(a0)
        // Load satp
        ld a2,  512(a0)
        # Load processor mode
        ld a3,  552(a0)

        # 1 << 7 is MPIE
        # Since user mode is 00, we don't need to set anything
        # in MPP (bits 12:11)
        li t0,  1 << 7 | 1 << 5 | 1 << 13

        # Combine enable bits with mode bits.
        slli    a3, a3, 11
        or      t0, t0, a3
        csrw    mstatus, t0
        csrw    mepc, a1
        csrw    satp, a2
        li      t1, 0xaaa
        csrw    mie, t1
        la      t2, _trap_vector
        csrw    mtvec, t2
        mv t6, a0
1:
        .set    i, 1
        .rept   31
            load_gp %i, t6
            .set    i, i+1
        .endr

        mret

.global _make_syscall
_make_syscall:
        # We're setting this up to work with libgloss
        # They want a7 to be the system call number and all parameters
        # in a0 - a5
        mv      a7, a0
        mv      a0, a1
        mv      a1, a2
        mv      a2, a3
        mv      a3, a4
        mv      a4, a5
        mv      a5, a6
        ecall
        ret
