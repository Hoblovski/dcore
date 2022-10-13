# Trap assembly.
# On any trap, the control flow jumps to __alltraps.

.altmacro
.macro SAVE_GP n
    sd x\n, \n*8(sp)
.endm
.macro LOAD_GP n
    ld x\n, \n*8(sp)
.endm

    .section .text
    .globl __alltraps
    .globl __restore
    .align 2
__alltraps:
    # FROM: user-mode traps (synchronous exception/ecall or asynchronous interrupt)
    #       or kernel-mode interrupts (only in scheduler context)
    # REQUIRES:
    #       sscratch = kernel sp
    #           TODO what bout kernel intr
    # HW SIDE EFFECTS:
    #       sie = OFF
    csrrw sp, sscratch, sp
    # ASSERT:
    #   sp = kernel sp
    #   sscratch = user sp
    addi sp, sp, -34*8
    # save general-purpose registers
    sd x1, 1*8(sp)
    # skip sp(x2): done at last step
    sd x3, 3*8(sp)
    # skip tp(x4): application does not use it
    # save x5~x31
    .set n, 5
    .rept 27
        SAVE_GP %n
        .set n, n+1
    .endr
    csrr t0, sstatus
    sd t0, 32*8(sp)
    csrr t1, sepc
    sd t1, 33*8(sp)
    # save sp(x2): on trap context user sp should be saved
    csrr t2, sscratch
    sd t2, 2*8(sp)
    # call trap_handler(cx: &mut TrapContext)
    mv a0, sp
    call trap_handler

__restore:
    # FROM: the end of interrupt handling
    #       or the beginning of a new thread
    # REQUIRES:
    #
    # now sp->kernel stack(after allocated), sscratch->user stack
    # restore sstatus/sepc
    ld t0, 32*8(sp)
    csrw sstatus, t0
    ld t1, 33*8(sp)
    csrw sepc, t1
    ld t2, 2*8(sp)
    csrw sscratch, t2
    ld x1, 1*8(sp)
    # skip sp: done at last step
    ld x3, 3*8(sp)
    # skip tp.
    .set n, 5
    .rept 27
        LOAD_GP %n
        .set n, n+1
    .endr
    addi sp, sp, 34*8
    csrrw sp, sscratch, sp
    # ASSERT:
    #   sscratch = kernel sp
    sret
    # TO: where control flow was before trap
