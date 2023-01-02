pub const NCPU: usize = 4;
pub const STACK_SIZE: usize = 0x10000;

/*
 * BCM2837 phys IOBASE
 * bus address 0x7Exx.xxxx lives at cpu phys address 0x3Fxx.xxxx
 * Ref: BCM2837 ARM Peripherals, section 1.2.3.
 */
pub const IOBASE: usize = 0x3f00_0000;
pub const AUX_BASE: usize = IOBASE + 0x21_5000;
pub const GPIO_BASE: usize = IOBASE + 0x20_0000;

// RAMTOP is at 0x4000_0000, but overlaps the IO region at 0x3f00_0000.
// During boot it some ram is stolen for the VC SDRAM which specifies a
// split between what the ARM claims and what the GPU claims.
// I'm guess this is normally passed in some config struct by the bootloader,
// but in my qemu environment I've got nothing.
// So.. wild guesses here, lets say the VC SDRAM split gives 256MB to the GPU,
// and we'll claim the rest.
pub const RAM_TOP: usize = 0x4000_0000 - 256 * 1024 * 1024;

// I'm not using a linker script yet, and I have no access to a symbol
// specifying the end of the .bss section...  So lets fake it.
// Arbitrarily declaring that the text/data/bss will fit in 64MB!
pub const PROG_SIZE: usize = 64 * 1024 * 1024;
pub const HEAP_BASE: usize = 0 + PROG_SIZE;
pub const HEAP_TOP: usize = RAM_TOP - STACK_SIZE * NCPU;
