.altmacro

.set NUM_GP_REGS, 32  # Number of registers per context
.set NUM_FP_REGS, 32
.set REG_SIZE, 8   # Register size (in bytes)
.set MAX_CPUS, 8   # Maximum number of CPUs

.align 4
.macro SAVE_CONTEXT
        addi sp, sp, -32*(REG_SIZE)

        .set i, 0
        .rept 31
                save_gp %i
                .set i, i+1
        .endr
.endm

.align 4
.macro RESTORE_CONTEXT
        mv t6, a0
        .set i, 1
        .rept 31
                load_gp %i, t6
                .set i, i+1
        .endr
.endm

# Use macros for saving and restoring multiple registers
.macro save_gp i, basereg=t6
        STORE_X x\i, ((\i)*REG_SIZE)(\basereg)
.endm

.macro load_gp i, basereg=t6
        LOAD_X x\i, ((\i)*REG_SIZE)(\basereg)
.endm

.macro save_fp i, basereg=t6
        fsd f\i, ((NUM_GP_REGS+(\i))*REG_SIZE)(\basereg)
.endm

.macro load_fp i, basereg=t6
        fld f\i, ((NUM_GP_REGS+(\i))*REG_SIZE)(\basereg)
.endm
