/*
 * CPU register access.
 */

use crate::reg::Reg;
use core::arch::asm;

#[macro_export]
macro_rules! cpu_reg64 {
    ($struct_name:ident, $reg:ident) => {
        struct $struct_name {
            cached: u64,
        }

        impl $struct_name {
			// new creates a new instances with preset cached value.
            #[allow(dead_code)]
            fn new(value: u64) -> Self {
                $struct_name { cached: value }
            }

			// new creates a new instances with zeroed cached value.
            #[allow(dead_code)]
            fn zero() -> Self {
                Self::new(0)
            }

			// new creates a new instances with fetched value.
            #[allow(dead_code)]
            fn fetch() -> Self {
                let mut x = Self::zero();
				x.fetch();
				x
            }
        }

        impl Reg<u64> for $struct_name {
            fn store(&self) {
                let val = self.cached;
                unsafe {
        			asm!(core::concat!("msr ", stringify!($reg), ", {}"), in(reg) val);
                }
            }

            fn fetch(&mut self) -> &mut Self {
				let val: u64;
				unsafe {
        			asm!(core::concat!("mrs {}, ", stringify!($reg)), out(reg) val);
				}
				self.cached = val;
                self
            }

            fn get_value(&self) -> u64 {
                self.cached
            }

            fn set_value(&mut self, val: u64) -> &mut Self {
                self.cached = val;
                self
            }
        }
    };
}

cpu_reg64!(CurrentEl, CurrentEl);
cpu_reg64!(MpidrEl1, MPIDR_EL1);

pub fn current_el() -> u64 {
    return CurrentEl::fetch().get_value() >> 2;
}

pub fn core_id() -> u64 {
    return MpidrEl1::fetch().get_value() & 0xff;
}
