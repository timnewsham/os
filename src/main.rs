#![no_std]
#![no_main]

mod gpio;
mod mmio;
mod uart;

use core::arch::{asm, global_asm};
use core::panic::PanicInfo;

// halt spins forever.
fn halt() -> ! {
    loop {
        unsafe { asm!("wfe") }
    }
}

// power_off shuts down using qemu semihosting feature.
fn power_off() -> ! {
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

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// _start is the initial entry point.
// Qemu calls it on all four cores, with no stack pointer set.
// It sets up a stack for each core and tail-calls _start_rust.
global_asm!(
    "
	.globl _start
    _start:
        mrs x0, MPIDR_EL1
        and x0, x0, 0xff		// x0 = cpuid
        ldr x30, =0x400000
        mov x1, 0x10000			// x1 = STACKSIZE = 16 pages
        msub x30, x0, x1, x30
        mov sp, x30				// sp = STACKTOP - STACKSIZE * cpuid
        b _start_rust
"
);

// _start_rust is called from _start (in asm) with the stack set up and a cpuid.
#[no_mangle]
pub extern "C" fn _start_rust(cpuid: u64) -> ! {
    if cpuid != 0 {
        println!("core {} halting", cpuid);
        halt();
    }

    main();
    println!("Powering Off");
    power_off();
}

// main is the first full rust function called.
fn main() {
    println!("Hello World!");
    //panic!("Test panic");
}
