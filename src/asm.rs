use core::arch::{asm, global_asm};

// halt spins forever.
pub fn halt() -> ! {
    loop {
        unsafe { asm!("wfe") }
    }
}

// power_off shuts down using qemu semihosting feature.
pub fn power_off() -> ! {
    // TODO: any way to affect the exit code?
    // I always get exit(1) from qemu, even with other exit reasons.
    unsafe {
        asm!(
            "mov w0, 0x18",     // exit
            "ldr x1, =0x20026", // application exit
            "hlt #0xF000",      // semihosting call
        )
    }
    halt();
}

pub fn delay(cycles: u64) {
    unsafe {
        // TODO: can this be optimized away? how to prevent that?
        asm!(
            "1:",
            "sub {cnt}, {cnt}, #1",
            "cbnz {cnt}, 1b",
            cnt = inout(reg) cycles => _,
        );
    }
}

// _start is the initial entry point.
// Qemu calls it on all four cores, with no stack pointer set.
// It sets up a stack for each core and tail-calls _start_rust.
global_asm!(
    "
    .globl _start
    _start:
        mrs x0, MPIDR_EL1
        and x0, x0, 0xff        // x0 = core id
        ldr x30, =0x400000
        mov x1, 0x10000         // x1 = STACKSIZE = 16 pages
        msub x30, x0, x1, x30
        mov sp, x30             // sp = STACKTOP - STACKSIZE * core id
        b _start_rust
"
);
