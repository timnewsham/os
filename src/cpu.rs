/*
 * CPU register access.
 */

#[macro_export]
macro_rules! cpu_reg64 {
    ($struct_name:ident, $reg:ident) => {
        struct $struct_name {
            cached: u64,
        }

        impl $struct_name {
            #[allow(dead_code)]
            fn new(value: u64) -> Self {
                $struct_name { cached: value }
            }

            #[allow(dead_code)]
            fn zero() -> Self {
                Self::new(0)
            }
        }

        impl Reg<u64> for $struct_name {
            fn store(&self) {
                let val = self.cached;
                unsafe {
        			//asm!("msr MPIDR_EL1, {src}", src = in(reg) val);
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
