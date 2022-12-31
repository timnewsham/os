/*
 * gpio.rs
 * BCM2837 GPIO support.
 */

use crate::mmio::{Reg32, Reg32Array};
use crate::{asm, mmio, mmio_reg32, mmio_reg32_array};

const GPIO_MAXPIN: u32 = 53;
const GPIO_BASE: usize = mmio::IOBASE + 0x20_0000;

mmio_reg32_array!(GpFSel, GPIO_BASE, 6);
mmio_reg32!(GpPud, GPIO_BASE + 0x94);
mmio_reg32_array!(GpPupdClk, GPIO_BASE + 0x98, 2);

// bitvec_write writes val to a GPIO bit vector reg_vec having sz-bit entries.
// These are arrays of sz-bit elements that are packed into u32 registers
// starting iwth the lowest bits.  Elements never span a register, and the
// upper bits of the register are left unused if the element size doesnt equally
// divide 32.
fn bitvec_write<T: Reg32Array>(reg_vec: T, sz: u32, pin: u32, val: u32) {
    if pin >= GPIO_MAXPIN {
        panic!("pin {} is too large", pin);
    }

    let mask = (1 << sz) - 1;
    if (val & !mask) != 0 {
        panic!("val {} is too big", val);
    }

    let fields_per_u32 = 32 / sz;
    let reg_index = (pin / fields_per_u32) as usize;
    let reg_offset = pin % fields_per_u32;

    let shift = reg_offset * sz;

    let curval = reg_vec.fetch(reg_index);
    let newval = (curval & !(mask << shift)) | (val << shift);
    reg_vec.store(reg_index, newval);
}

// pin_set_function sets the function bit field for pin to val.
fn pin_set_function(pin: u32, val: u32) {
    bitvec_write(GpFSel {}, 3, pin, val);
}

// pin_set_pupd_clk sets the pupd_clk bit field for pin to val.
fn pin_set_pupd_clk(pin: u32, val: u32) {
    bitvec_write(GpPupdClk {}, 1, pin, val);
}

// pin_set_pull sets the pullup behavior of a GPIO pin.
fn pin_set_pull(pin: u32, val: u32) {
    // See BCM2837 ARM Peripherals pg 101.
    // Write intended value
    GpPud {}.store(val);

    // wait
    asm::delay(150);

    // assert clock for the right pin
    pin_set_pupd_clk(pin, 1);

    // wait
    asm::delay(150);

    // clear GPPUD, and de-assert clock
    GpPud {}.store(0);
    pin_set_pupd_clk(pin, 0);
}

// pin_use_as_alt5 sets a GPIO pin to an ALT5 alternative function.
pub fn pin_use_as_alt5(pin: u32) {
    pin_set_pull(pin, 0); // pull none
    pin_set_function(pin, 2); // ALT5
}
