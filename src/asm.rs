use crate::{cpu, msr_imm};
use crate::reg::Reg;
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
        //asm!("svc 0"); // test exceptions.

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

global_asm!(
    "
    .global _vector_table
    .balign 2048
    _vector_table:

    // Lower EL, AArch32
    .balign 128
    _vector_0_serror:
        mov x0, #0x0
        b _exception
    .balign 128
    _vector_0_fiq:
        mov x0, #0x1
        b _exception
    .balign 128
    _vector_0_irq:
        mov x0, #0x2
        b _exception
    .balign 128
    _vector_0_synch:
        mov x0, #0x3
        b _exception

    // Lower EL, AArch64
    .balign 128
    _vector_1_serror:
        mov x0, #0x10
        b _exception
    .balign 128
    _vector_1_fiq:
        mov x0, #0x11
        b _exception
    .balign 128
    _vector_1_irq:
        mov x0, #0x12
        b _exception
    .balign 128
    _vector_1_synch:
        mov x0, #0x13
        b _exception

    // Current EL, SPx
    .balign 128
    _vector_2_serror:
        mov x0, #0x20
        b _exception
    .balign 128
    _vector_2_fiq:
        mov x0, #0x21
        b _exception
    .balign 128
    _vector_2_irq:
        mov x0, #0x22
        b _exception
    .balign 128
    _vector_2_synch:
        mov x0, #0x23
        b _exception

    // Current EL, SP0
    .balign 128
    _vector_3_serror:
        mov x0, #0x30
        b _exception
    .balign 128
    _vector_3_fiq:
        mov x0, #0x31
        b _exception
    .balign 128
    _vector_3_irq:
        mov x0, #0x32
        b _exception
    .balign 128
    _vector_3_synch:
        mov x0, #0x33
        b _exception
"
);

extern "C" {
    fn _vector_table();
}

// _exception is called by the cpu via vector_table to handle exceptions.
#[no_mangle]
pub extern "C" fn _exception(num: u64) -> ! {
    let group = num >> 4;
    let index = num & 0xf;
    let esr = cpu::EsrEl3::fetch().get_value();
    let far = cpu::FarEl3::fetch().get_value();
    panic!("got exception group {} index {}, ESR {:x}, FAR {:x}", group, index, esr, far);
}

pub fn init_exceptions() {
    // TODO: setup EL3 interrupt config
    let vbar = _vector_table as u64;
    cpu::VBarEl3::new(vbar).store();
    msr_imm!(DAIFClr, 0b1111);
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
