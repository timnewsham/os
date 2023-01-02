/*
 * gpio.rs
 * BCM2837 GPIO support.
 */

use crate::define_bits;
use crate::mmio::Reg32Array;
use crate::reg::Reg;
use crate::{asm, board, mmio_reg32, mmio_reg32_array};

const GPIO_MAXPIN: u32 = 53;
const ALT5: u32 = 2;

mmio_reg32_array!(GpFSel, 6, board::GPIO_BASE);
mmio_reg32!(GpPud, board::GPIO_BASE + 0x94);
mmio_reg32_array!(GpPupdClk, 2, board::GPIO_BASE + 0x98);

impl GpFSel {
    fn store_pin_function(self, pin: u32, val: u32) {
        _bitvec_write(self, 3, pin, val);
    }
}

impl GpPud {
    const PUD_DISABLED: u32 = 0;

    define_bits!(0, 2, u32, set_pud, get_pud);
}

impl GpPupdClk {
    fn store_pin_clk(self, pin: u32, val: u32) {
        _bitvec_write(self, 1, pin, val);
    }
}

// bitvec_write writes val to a GPIO bit vector reg_vec having sz-bit entries.
// These are arrays of sz-bit elements that are packed into u32 registers
// starting iwth the lowest bits.  Elements never span a register, and the
// upper bits of the register are left unused if the element size doesnt equally
// divide 32.
fn _bitvec_write<T: Reg32Array>(reg_vec: T, sz: u8, pin: u32, val: u32) {
    if pin >= GPIO_MAXPIN {
        panic!("pin {} is too large", pin);
    }

    let fields_per_u32 = 32 / (sz as u32);
    let reg_index = (pin / fields_per_u32) as usize;
    let reg_offset = (pin % fields_per_u32) as u8;

    reg_vec.index_fetch(reg_index).set_bits(reg_offset, sz, val).store();
}

// pin_disable_pull sets the pullup behavior of a GPIO pin to disable.
fn pin_disable_pull(pin: u32) {
    // See BCM2837 ARM Peripherals pg 101.
    // Write intended value
    GpPud::zero().set_pud(GpPud::PUD_DISABLED).store();

    // wait
    asm::delay(150);

    // assert clock for the right pin
    GpPupdClk::new().store_pin_clk(pin, 1);

    // wait
    asm::delay(150);

    // clear GPPUD, and de-assert clock
    GpPud::zero().store();
    GpPupdClk::new().store_pin_clk(pin, 0);
}

// pin_use_as_alt5 sets a GPIO pin to an ALT5 alternative function.
pub fn pin_use_as_alt5(pin: u32) {
    pin_disable_pull(pin);
    GpFSel::new().store_pin_function(pin, ALT5); // ALT5
}
