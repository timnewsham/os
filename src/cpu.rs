/*
 * CPU register access.
 */

use crate::define_bit;
use crate::reg::Reg;
use core::arch::asm;

#[macro_export]
macro_rules! msr_imm {
    ($reg:ident, $imm:expr) => {
        unsafe {
            asm!(core::concat!("msr ", stringify!($reg), ", ", stringify!($imm)));
        }
    };
}

#[macro_export]
macro_rules! cpu_reg64 {
    ($struct_name:ident, $reg:ident) => {
        pub struct $struct_name {
            cached: u64,
        }

        impl $struct_name {
            // new creates a new instances with preset cached value.
            #[allow(dead_code)]
            pub fn new(value: u64) -> Self {
                $struct_name { cached: value }
            }

            // new creates a new instances with zeroed cached value.
            #[allow(dead_code)]
            pub fn zero() -> Self {
                Self::new(0)
            }

            // new creates a new instances with fetched value.
            #[allow(dead_code)]
            pub fn fetch() -> Self {
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
cpu_reg64!(EsrEl3, ESR_EL3);
cpu_reg64!(ElrEl3, ELR_EL3);
cpu_reg64!(FarEl3, FAR_EL3);
cpu_reg64!(MpidrEl1, MPIDR_EL1);
cpu_reg64!(ScrEl3, SCR_EL3);
cpu_reg64!(SpSel, SPSel);
cpu_reg64!(VBarEl3, VBAR_EL3);

impl ScrEl3 {
    define_bit!(1, set_irq, get_irq);
    define_bit!(2, set_fiq, get_fiq);
    define_bit!(3, set_ea, get_ea);
    define_bit!(10, set_rw, get_rw);
}

impl SpSel {
    define_bit!(0, set_sp, get_sp);
}

pub fn current_el() -> u64 {
    return CurrentEl::fetch().get_value() >> 2;
}

pub fn core_id() -> u64 {
    return MpidrEl1::fetch().get_value() & 0xff;
}
