// Reg<T> is a T (some primitive integer/bit vector) MMIO register with "safe" fetch and store methods.
pub trait Reg<T>
where
    T: Copy
        + From<bool>
        + From<u8>
        + core::cmp::Eq
        + core::ops::BitAnd<Output = T>
        + core::ops::BitOr<Output = T>
        + core::ops::Shl<u8, Output = T>
        + core::ops::Shr<u8, Output = T>
        + core::ops::Sub<Output = T>
        + core::ops::Not<Output = T>
        + core::fmt::Display,
{
    // store writes the current value into the hardware register.
    fn store(&self);

    // fetch reads the hardware register and returns it.
    fn fetch(&mut self) -> &mut Self;

    // get_value gets the currently cached value.
    fn get_value(&self) -> T;

    // set_value sets the currently cached value.
    // Use store() to commit it to the register.
    fn set_value(&mut self, val: T) -> &mut Self;

    // set_bits sets bits [shift .. shift+sz] to val.
    fn set_bits(&mut self, shift: u8, sz: u8, val: T) -> &mut Self {
        let mask = (T::from(1) << sz) - T::from(1);
        if val & !mask != T::from(0) {
            panic!("{} is too big", val);
        }

        let orig = self.get_value();
        let new = (orig & !(mask << shift)) | (val << shift);
        self.set_value(new)
    }

    // set_bit sets bit bitpos to val.
    fn set_bit(&mut self, bitpos: u8, val: bool) -> &mut Self {
        self.set_bits(bitpos, 1, T::from(val))
    }

    // get_bits returns bits [shift .. shift+sz].
    fn get_bits(&self, shift: u8, sz: u8) -> T {
        let mask = (T::from(1) << sz) - T::from(1);
        let val = self.get_value();
        return (val >> shift) & mask;
    }

    // get_bit returns bit bitpos.
    fn get_bit(&self, bitpos: u8) -> bool {
        self.get_bits(bitpos, 1) != T::from(0)
    }
}

/*
#[macro_export]
macro_rules! define_bit {
    ($bitpos:expr, $name:ident) => {
        pub fn concat_idents!(get_, $name)(&self) -> bool {
            self.get_bit($bitpos)
        }
        pub fn concat_idents!(set_, $name)(&mut self, val: bool) -> &mut Self {
            self.set_bit(bitpos, val)
        }
    };
}
*/
