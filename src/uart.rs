/*
 * uart.rs
 * BCM2837 AUX UART (UART1) support.
 */

use crate::mmio::Reg32;
use crate::{gpio, mmio, mmio_reg32};
use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;

const AUX_UART_CLOCK: u32 = 50_000_000;
const AUX_BASE: usize = mmio::IOBASE + 0x21_5000;

// TODO: figure out a way to do this generically with proper u32-based structures
// with bit fields to better document each bit field access.
mmio_reg32!(AuxEnables, AUX_BASE + 4); // bit 0 enables aux's mini uart (uart1)
mmio_reg32!(AuxMuIo, AUX_BASE + 0x40);
mmio_reg32!(AuxMuIer, AUX_BASE + 0x44);
mmio_reg32!(AuxMuIir, AUX_BASE + 0x48);
mmio_reg32!(AuxMuLcr, AUX_BASE + 0x4c);
mmio_reg32!(AuxMuMcr, AUX_BASE + 0x50);
mmio_reg32!(AuxMuLsr, AUX_BASE + 0x54);
mmio_reg32!(AuxMuCntl, AUX_BASE + 0x60);
mmio_reg32!(AuxMuBaud, AUX_BASE + 0x68);

impl AuxEnables {
    fn set_enable(&mut self, enable: bool) -> &mut Self {
        self.set_bits(0, 1, enable as u32)
    }
}

impl AuxMuIir {
    fn clear_fifos(&mut self) -> &mut Self {
        self.set_bits(1, 2, 3)
    }
}

impl AuxMuLcr {
    fn set_8bit(&mut self) -> &mut Self {
        self.set_value(3)
    }
}

impl AuxMuLsr {
    fn is_ready(&self) -> bool {
        self.get_value() & 0x20 != 0
    }
}

impl AuxMuCntl {
    fn enable_recv(&mut self, enab: bool) -> &mut Self {
        self.set_bits(0, 1, enab as u32)
    }
    fn enable_xmit(&mut self, enab: bool) -> &mut Self {
        self.set_bits(1, 1, enab as u32)
    }
}

impl AuxMuBaud {
    fn set_baud(&mut self, baud: u32) -> &mut Self {
        self.set_value((AUX_UART_CLOCK / (baud * 8)) - 1)
    }
}

// init enables and initializes the aux UART (uart1).
fn init() {
    gpio::pin_use_as_alt5(14);
    gpio::pin_use_as_alt5(15);

    AuxEnables::zero().set_enable(true).store(); // uart enabled
    AuxMuIer::new(0).store(); // reset interupts
    AuxMuCntl::new(0).store(); // recv/xmit disabled
    AuxMuLcr::new(0).set_8bit().store(); // 8bit mode
    AuxMuMcr::new(0).store(); // reset interupts
    AuxMuIer::new(0).store(); // reset interupts again
    AuxMuIir::new(0).clear_fifos().store(); // clear both fifos
    AuxMuBaud::zero().set_baud(115200).store();
    AuxMuCntl::zero().enable_xmit(true).enable_recv(true).store(); // recv/xmit enabled
}

// write_char writes a single character. It uses polling to wait
// for the uart to be writable.
fn write_char(ch: u8) {
    let mut aux_mu_lsr = AuxMuLsr::zero();
    while !aux_mu_lsr.fetch().is_ready() { /* wait */ }
    AuxMuIo::new(ch as u32).store();
}

pub struct Writer {
    initialized: bool,
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer { initialized: false });
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        if !self.initialized {
            self.initialized = true;
            init();
        }

        for ch in s.bytes() {
            write_char(ch);
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::uart::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}
