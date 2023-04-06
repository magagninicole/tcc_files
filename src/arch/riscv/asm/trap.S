.option norvc # disable generation of compressed instructions
.section .text

.align 4
_trap_vector:
        # All registers are volatile here, we need to save them
        # before we do anything.
        csrrw   t6, mscratch, t6

        # csrrw will atomically swap t6 into mscratch and the old
        # value of mscratch into t6. This is nice because we just
        # switched values and didn't destroy anything -- all atomically!
        # in cpu.rs we have a structure of:
        #  32 gp regs           0
        #  32 fp regs           256
        #  SATP register        512
        #  Trap stack       520
        #  CPU HARTID           528
        # We use t6 as the temporary register because it is the very
        # bottom register (x31)
        .set    i, 0
        .rept   31
                save_gp %i
                .set    i, i+1
        .endr

        # Save the actual t6 register, which we swapped into
        # mscratch
        mv      t5, t6
        csrr    t6, mscratch
        save_gp 31, t5

        # Restore the kernel trap frame into mscratch
        csrw    mscratch, t5

        csrr    t1, mstatus
        srli    t0, t1, 13
        andi    t0, t0, 3
        li      t3, 3
        bne     t0, t3, 1f
1:
        # Get ready to go into Rust (trap.rs)
        # We don't want to write into the user's stack or whomever
        # messed with us here.
        csrr    a0, mepc
        sd      a0, 520(t5)
        csrr    a1, mtval
        csrr    a2, mcause
        csrr    a3, mhartid
        csrr    a4, mstatus
        csrr    a5, mscratch
        la      t0, KERNEL_STACK_END
        ld      sp, 0(t0)
        call    trap_handler

        # When we get here, we've returned from m_trap, restore registers
        # and return.
        # trap_handler will return the return address via a0.

        # I removed mepc here, we want to go back to the same place we
        # left off since we're here only with asynchronous traps.
        # csrw    mepc, a0

        # Now load the trap frame back into t6
        csrr    t6, mscratch

        csrr    t1, mstatus
        srli    t0, t1, 13
        andi    t0, t0, 3
        li      t3, 3
        bne     t0, t3, 1f
1:
        # Restore all GP registers
        .set    i, 1
        .rept   31
            load_gp %i
            .set    i, i+1
        .endr

        # Since we ran this loop 31 times starting with i = 1,
        # the last one loaded t6 back to its original value.
        mret
