use crate::reg::Reg;
use crate::{board, cpu, msr_imm, println};
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
        b _unhandled_exception
    .balign 128
    _vector_0_fiq:
        mov x0, #0x1
        b _unhandled_exception
    .balign 128
    _vector_0_irq:
        mov x0, #0x2
        b _unhandled_exception
    .balign 128
    _vector_0_synch:
        mov x0, #0x3
        b _unhandled_exception

    // Lower EL, AArch64
    .balign 128
    _vector_1_serror:
        mov x0, #0x10
        b _unhandled_exception
    .balign 128
    _vector_1_fiq:
        mov x0, #0x11
        b _unhandled_exception
    .balign 128
    _vector_1_irq:
        mov x0, #0x12
        b _unhandled_exception
    .balign 128
    _vector_1_synch:
        mov x0, #0x13
        b _unhandled_exception

    // Current EL, SPx
    .balign 128
    _vector_2_serror:
        mov x0, #0x20
        b _unhandled_exception
    .balign 128
    _vector_2_fiq:
        mov x0, #0x21
        b _unhandled_exception
    .balign 128
    _vector_2_irq:
        mov x0, #0x22
        b _unhandled_exception
    .balign 128
    _vector_2_synch:
        mov x0, #0x23
        b _unhandled_exception

    // Current EL, SP0
    .balign 128
    _vector_3_serror:
        mov x0, #0x30
        b _unhandled_exception
    .balign 128
    _vector_3_fiq:
        mov x0, #0x31
        b _unhandled_exception
    .balign 128
    _vector_3_irq:
        mov x0, #0x32
        b _unhandled_exception
    .balign 128
    _vector_3_synch:
        mov x0, #0x33
        b _unhandled_exception
"
);

extern "C" {
    fn _vector_table();
}

// _unhandled_exception is called by the cpu via vector_table to handle exceptions.
#[no_mangle]
pub extern "C" fn _unhandled_exception(num: u64) -> ! {
    let group = num >> 4;
    let index = num & 0xf;
    let elr = cpu::ElrEl3::fetch().get_value();
    let esr = cpu::EsrEl3::fetch().get_value();
    let far = cpu::FarEl3::fetch().get_value();
    println!("got exception group {} index {}", group, index);
    println!("  ELR {:x} ESR {:x} FAR {:x}", elr, esr, far);
    panic!("unhandled exception");
}

pub fn init_exceptions() {
    let vbar = _vector_table as u64;
    cpu::ScrEl3::zero()
        .set_ea(true) // EA unmasked
        .set_irq(true) // IRQ unmasked
        .set_fiq(true) // FIQ unmasked
        .set_rw(true) // RW - next level AArch64
        .store();
    cpu::SpSel::zero()
        .set_sp(true) // SP - use SP_ELx for exceptions, not SP_EL0
        .store();
    cpu::VBarEl3::new(vbar).store();
    msr_imm!(DAIFClr, 0b1111); // clear interrupt disables
}

// _start is the initial entry point.
// Qemu calls it on all four cores, with no stack pointer set.
// It sets up a stack for each core and tail calls _start_rust.
#[no_mangle]
#[naked]
pub extern "C" fn _start() -> ! {
    unsafe {
        asm!(
            "mrs x0, MPIDR_EL1",
            "and x0, x0, 0xff        // x0 = core id",
            "ldr x1, ={stack_size}",
            "ldr x2, ={ram_top}",
            "msub x30, x0, x1, x2",
            "mov sp, x30             // sp = ram_top - core_id * stack_size",
            "b _start_rust",
            stack_size = const board::STACK_SIZE,
            ram_top = const board::RAM_TOP,
            options(noreturn),
        );
    }
}
