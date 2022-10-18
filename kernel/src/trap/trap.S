# Trap assembly.
# On any trap, the control flow jumps to __alltraps.

# NOTE: these macros require sp to point to TRAP_CONTEXT
.altmacro
.macro SAVE_GPR n
    sd x\n, \n*8(sp)
.endm
.macro LOAD_GPR n
    ld x\n, \n*8(sp)
.endm

# NOTE: CSR save/load will clobber t0.
.macro SAVE_CSR_ csr n
    csrr t0, \csr
    sd t0, \n*8(sp)
.endm
.macro LOAD_CSR_ csr n
    ld t0, \n*8(sp)
    csrw \csr, t0
.endm

    .section .text.trampoline
    .globl __alltraps
    .globl __restore
    .align 2
__alltraps:
    # FROM: user-mode traps (synchronous exception/ecall or asynchronous interrupt)
    #       or kernel-mode interrupts (only in scheduler context)
    # TODO: kernel traps
    # REQUIRES:
    #       sscratch = TrapContext state save area virtual address (in user address space)
    # HW SIDE EFFECTS:
    #       sie = OFF
    csrrw sp, sscratch, sp
    # ASSERT:
    #   sp = TrapContext state save area (the LSB i.e. byte 0) = TRAP_CONTEXT
    #   sscratch = user sp
    # save general-purpose registers
    SAVE_GPR 1
    SAVE_GPR 3
    .set n, 5
    .rept 27
        SAVE_GPR %n
        .set n, n+1
    .endr
    # save sp. must be placed after SAVE_GPR since it clobbers t0
    SAVE_CSR_ sscratch 2
    # save CSRs
    SAVE_CSR_ sstatus 32
    SAVE_CSR_ sepc 33

    # load trap handler address
    ld t0, 36*8(sp)
    # load kernel satp
    ld t1, 34*8(sp)
    # switch to kernel stack
    # CORRECTNESS:
    #       this only happens after all `*(sp)` uses,
    #       or data can be loaded/stored from wrong address i.e. not within the TRAP_CONTEXT page.
    ld sp, 35*8(sp)
    # switch to kernel satp
    # CORRECTNESS:
    #       this only happens after all `*(sp)` uses
    #       since they are on the TRAP_CONTEXT page which is not mapped in kernel address space.
    csrw satp, t1
    sfence.vma
    # jump to trap handler
    jr t0

__restore:
    # FROM: the end of interrupt handling
    #       or the beginning of a new thread
    # REQUIRES:
    #       a0 = TrapContext state save area = TRAP_CONTEXT
    #       a1 = user_satp
    # switch to user satp
    csrw satp, a1
    sfence.vma
    # restore GPR and CSR from TrapContext
    csrw sscratch, a0
    mv sp, a0
    LOAD_CSR_ sstatus 32
    LOAD_CSR_ sepc 33
    LOAD_GPR 1
    LOAD_GPR 3
    .set n, 5
    .rept 27
        LOAD_GPR %n
        .set n, n+1
    .endr
    # switch to user stack
    ld sp, 2*8(sp)
    sret
