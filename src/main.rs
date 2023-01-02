#![no_std]
#![no_main]

mod asm;
mod board;
mod cpu;
mod gpio;
mod mmio;
mod reg;
mod uart;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    asm::halt();
}

fn init_heap(_base: usize, _top: usize) {
    // TODO
}

// _start_rust is called from _start (in asm) with the stack set up.
#[no_mangle]
pub extern "C" fn _start_rust() -> ! {
    println!("EL {:x}", cpu::current_el());
    if cpu::core_id() != 0 {
        println!("halting");
        asm::halt();
    }

    asm::init_exceptions();
    init_heap(board::HEAP_BASE, board::HEAP_TOP);
    main();
    println!("Powering Off");
    asm::power_off();
}

// main is the first full rust function called.
fn main() {
    println!("Hello World!");
    //panic!("Test panic");
}
