#![no_std]
#![no_main]

mod asm;
mod cpu;
mod gpio;
mod mmio;
mod reg;
mod uart;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    asm::halt();
}

// _start_rust is called from _start (in asm) with the stack set up.
#[no_mangle]
pub extern "C" fn _start_rust() -> ! {
    println!("EL {:x}", asm::current_el());
    if asm::core_id() != 0 {
        println!("halting");
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
