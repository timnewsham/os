/*
 * BCM2837 phys IOBASE
 * bus address 0x7Exx.xxxx lives at cpu phys address 0x3Fxx.xxxx
 * Ref: BCM2837 ARM Peripherals, section 1.2.3.
 */
pub const IOBASE: usize = 0x3f00_0000;

// Reg32 is a 32-bit MMIO register with "safe" fetch and store methods.
// The implementer must ensure that the associated ADDR is safe to read and write.
pub trait Reg32 {
    // ADDR is a fixed address for this register.
    const ADDR: usize;

    // store writes the current value into the hardware register.
    fn store(&self) {
        let val = self.get_value();
        unsafe {
            core::ptr::write_volatile(Self::ADDR as *mut u32, val);
        }
    }

    // fetch reads the hardware register and returns it.
    fn fetch(&mut self) -> &mut Self {
        let val = unsafe { core::ptr::read_volatile(Self::ADDR as *mut u32) };
        self.set_value(val);
        self
    }

    // get_value gets the currently cached value.
    fn get_value(&self) -> u32;

    // set_value sets the currently cached value.
    // Use store() to commit it to the register.
    fn set_value(&mut self, val: u32) -> &mut Self;

    fn set_bits(&mut self, shift: u32, sz: u32, val: u32) -> &mut Self {
        let mask: u32 = (1 << sz) - 1;
        if val & !mask != 0 {
            panic!("{} is too big", val);
        }

        let orig = self.get_value();
        let new = (orig & (mask << shift)) | (val << shift);
        self.set_value(new)
    }
}

#[macro_export]
macro_rules! mmio_reg32 {
    ($struct_name:ident, $addr:expr) => {
        struct $struct_name {
            cached: u32,
        }

        impl $struct_name {
            #[allow(dead_code)]
            fn new(value: u32) -> Self {
                $struct_name { cached: value }
            }

            #[allow(dead_code)]
            fn zero() -> Self {
                Self::new(0)
            }
        }

        impl Reg32 for $struct_name {
            const ADDR: usize = $addr;
            fn get_value(&self) -> u32 {
                self.cached
            }
            fn set_value(&mut self, val: u32) -> &mut Self {
                self.cached = val;
                self
            }
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
macro_rules! mmio_reg32_array {
    ($struct_name:ident, $addr:expr, $size:expr) => {
        struct $struct_name {}

        impl $struct_name {
            fn new() -> Self {
                $struct_name {}
            }
        }

        impl Reg32Array for $struct_name {
            const ADDR: usize = $addr;
            const SIZE: usize = $size;
        }
    };
}
