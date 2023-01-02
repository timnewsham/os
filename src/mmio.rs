use crate::reg;
use crate::reg::Reg;

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

// Generic mmio with an explicit address.
pub struct Mmio<T> {
    addr: usize,
    cached: T,
}

impl<T: reg::Trait<T>> Mmio<T> {
    #[allow(dead_code)]
    pub fn new(addr: usize, val: T) -> Self {
        Mmio::<T> { addr: addr, cached: val }
    }

    #[allow(dead_code)]
    pub fn zero(addr: usize) -> Self {
        Self::new(addr, T::from(0))
    }

    #[allow(dead_code)]
    pub fn fetch(addr: usize) -> Self {
        let mut x = Self::new(addr, T::from(0));
        x.fetch();
        x
    }
}

impl<T: reg::Trait<T>> Reg<T> for Mmio<T> {
    fn store(&self) {
        let val = self.get_value();
        unsafe {
            core::ptr::write_volatile(self.addr as *mut T, val);
        }
    }

    fn fetch(&mut self) -> &mut Self {
        let val = unsafe { core::ptr::read_volatile(self.addr as *mut T) };
        self.set_value(val);
        self
    }

    fn get_value(&self) -> T {
        self.cached
    }

    fn set_value(&mut self, val: T) -> &mut Self {
        self.cached = val;
        self
    }
}

// Reg32Array is an array of 32-bit MMIO registers with "safe" fetch and store methods.
// The implementer must ensure that the associated ADDR and LEN are safe to read and write.
// TODO: genericize this for types other than u32 if ever needed.
pub trait Reg32Array {
    const ADDR: usize;
    const SIZE: usize;

    // get returns a Reg<u32> for this index with the cached value zeroed.
    #[allow(dead_code)]
    fn get(&self, idx: usize) -> Mmio<u32> {
        if idx >= Self::SIZE {
            panic!("idx {} is too big!", idx);
        }
        let addr = Self::ADDR + 4 * idx;
        Mmio::<u32>::zero(addr)
    }

    // get returns a Reg<u32> for this index with the cached value fetched.
    #[allow(dead_code)]
    fn fetch(&self, idx: usize) -> Mmio<u32> {
        if idx >= Self::SIZE {
            panic!("idx {} is too big!", idx);
        }
        let addr = Self::ADDR + 4 * idx;
        Mmio::<u32>::fetch(addr)
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
