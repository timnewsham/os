#[macro_export]
macro_rules! mmio_reg32 {
    ($struct_name:ident, $addr:expr) => {
        struct $struct_name {
            cached: u32,
        }

        impl $struct_name {
            // new creates a new instances with pre-set cached value.
            #[allow(dead_code)]
            fn new(value: u32) -> Self {
                $struct_name { cached: value }
            }

            // zero creates a new instance with zeroed cached value.
            #[allow(dead_code)]
            fn zero() -> Self {
                Self::new(0)
            }

            // fetch creates a new instances with fetched value.
            #[allow(dead_code)]
            fn fetch() -> Self {
                let mut x = Self::zero();
                x.fetch();
                x
            }
        }

        impl Reg<u32> for $struct_name {
            fn store(&self) {
                let val = self.get_value();
                unsafe {
                    core::ptr::write_volatile($addr as *mut u32, val);
                }
            }

            fn fetch(&mut self) -> &mut Self {
                let val = unsafe { core::ptr::read_volatile($addr as *mut u32) };
                self.set_value(val);
                self
            }

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
    ($struct_name:ident, $size:expr, $addr:expr) => {
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
