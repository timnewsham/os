/*
 * BCM2837 phys IOBASE
 * bus address 0x7Exx.xxxx lives at cpu phys address 0x3Fxx.xxxx
 * Ref: BCM2837 ARM Peripherals, section 1.2.3.
 */
pub const IOBASE: usize = 0x3f00_0000;

// Reg32 is a 32-bit MMIO register with "safe" fetch and store methods.
// The implementer must ensure that the associated ADDR is safe to read and write.
pub trait Reg32 {
    const ADDR: usize;
    fn store(&self, val: u32) {
        unsafe { core::ptr::write_volatile(Self::ADDR as *mut u32, val) }
    }
    fn fetch(&self) -> u32 {
        unsafe { core::ptr::read_volatile(Self::ADDR as *mut u32) }
    }
}

#[macro_export]
macro_rules! make_reg32 {
    ($struct_name:ident, $addr:expr) => {
        struct $struct_name {}
        impl Reg32 for $struct_name {
            const ADDR: usize = $addr;
        }
    };
}

// Reg32Array is an array of 32-bit MMIO registers with "safe" fetch and store methods.
// The implementer must ensure that the associated ADDR and LEN are safe to read and write.
pub trait Reg32Array {
    const ADDR: usize;
    const SIZE: usize;
    fn store(&self, idx: usize, val: u32) {
        if idx >= Self::SIZE {
            panic!("idx {} is too big!", idx);
        }
        let addr = Self::ADDR + 4 * idx;
        unsafe { core::ptr::write_volatile(addr as *mut u32, val) }
    }
    fn fetch(&self, idx: usize) -> u32 {
        if idx >= Self::SIZE {
            panic!("idx {} is too big!", idx);
        }
        let addr = Self::ADDR + 4 * idx;
        unsafe { core::ptr::read_volatile(addr as *mut u32) }
    }
}

#[macro_export]
macro_rules! make_reg32_array {
    ($struct_name:ident, $addr:expr, $size:expr) => {
        struct $struct_name {}
        impl Reg32Array for $struct_name {
            const ADDR: usize = $addr;
            const SIZE: usize = $size;
        }
    };
}
