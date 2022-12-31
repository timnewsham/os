#![no_std]
#![no_main]

mod asm;
mod gpio;
mod mmio;
mod uart;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    asm::halt();
}

// _start_rust is called from _start (in asm) with the stack set up and a cpuid.
#[no_mangle]
pub extern "C" fn _start_rust(cpuid: u64) -> ! {
    if cpuid != 0 {
        println!("core {} halting", cpuid);
        asm::halt();
    }

    main();
    println!("Powering Off");
    asm::power_off();
}

// main is the first full rust function called.
fn main() {
    println!("Hello World!");
    //panic!("Test panic");
}
