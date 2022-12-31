/*
 * uart.rs
 * BCM2837 AUX UART (UART1) support.
 */

use crate::gpio;
use crate::make_reg32;
use crate::mmio;
use crate::mmio::Reg32;
use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;

const AUX_UART_CLOCK: u32 = 50_000_000;
const AUX_BASE: usize = mmio::IOBASE + 0x21_5000;

// TODO: figure out a way to do this generically with proper u32-based structures
// with bit fields to better document each bit field access.
make_reg32!(AuxEnables, AUX_BASE + 4); // bit 0 enables aux's mini uart (uart1)
make_reg32!(AuxMuIo, AUX_BASE + 0x40);
make_reg32!(AuxMuIer, AUX_BASE + 0x44);
make_reg32!(AuxMuIir, AUX_BASE + 0x48);
make_reg32!(AuxMuLcr, AUX_BASE + 0x4c);
make_reg32!(AuxMuMcr, AUX_BASE + 0x50);
make_reg32!(AuxMuLsr, AUX_BASE + 0x54);
make_reg32!(AuxMuCntl, AUX_BASE + 0x60);
make_reg32!(AuxMuBaud, AUX_BASE + 0x68);

// aux_mu_baud computes the baud register value for the intended baud rate
// based on the clock speed of the SoC.
fn aux_mu_baud(baud: u32) -> u32 {
    (AUX_UART_CLOCK / (baud * 8)) - 1
}

// init enables and initializes the aux UART (uart1).
fn init() {
    gpio::pin_use_as_alt5(14);
    gpio::pin_use_as_alt5(15);

    AuxEnables {}.store(1); // uart enabled
    AuxMuIer {}.store(0); // reset interupts
    AuxMuCntl {}.store(0); // recv/xmit disabled
    AuxMuLcr {}.store(3); // 8bit mode
    AuxMuMcr {}.store(0); // reset interupts
    AuxMuIer {}.store(0); // reset interupts again
    AuxMuIir {}.store(6); // clear both fifos
    AuxMuBaud {}.store(aux_mu_baud(115200));
    AuxMuCntl {}.store(3); // recv/xmit enabled
}

// is_write_ready returns true if the uart is ready to transmit.
fn is_write_ready() -> bool {
    (AuxMuLsr {}.fetch() & 0x20) != 0
}

// write_char writes a single character. It uses polling to wait
// for the uart to be writable.
fn write_char(ch: u8) {
    while !is_write_ready() { /* wait */ }
    AuxMuIo {}.store(ch as u32);
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
