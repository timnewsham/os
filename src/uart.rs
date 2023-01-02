/*
 * uart.rs
 * BCM2837 AUX UART (UART1) support.
 */

use crate::reg::Reg;
use crate::{board, define_bit, define_bit_wo, define_bits, gpio, mmio_reg32};
use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;

mmio_reg32!(AuxEnables, board::AUX_BASE + 4);
mmio_reg32!(AuxMuIo, board::AUX_BASE + 0x40);
mmio_reg32!(AuxMuIer, board::AUX_BASE + 0x44);
mmio_reg32!(AuxMuIir, board::AUX_BASE + 0x48);
mmio_reg32!(AuxMuLcr, board::AUX_BASE + 0x4c);
mmio_reg32!(AuxMuMcr, board::AUX_BASE + 0x50);
mmio_reg32!(AuxMuLsr, board::AUX_BASE + 0x54);
mmio_reg32!(AuxMuCntl, board::AUX_BASE + 0x60);
mmio_reg32!(AuxMuBaud, board::AUX_BASE + 0x68);

impl AuxEnables {
    define_bit!(0, set_enable, get_enable);
}

impl AuxMuIir {
    define_bit_wo!(1, set_clear_recv_fifo);
    define_bit_wo!(2, set_clear_xmit_fifo);
}

impl AuxMuLcr {
    const DATA_SIZE_8B: u32 = 3;

    define_bits!(0, 2, u32, set_data_size, get_data_size);
}

impl AuxMuLsr {
    define_bit!(5, _set_tx_empty, get_tx_empty);
}

impl AuxMuCntl {
    define_bit!(0, set_recv_enb, get_recv_enb);
    define_bit!(1, set_xmit_enb, get_xmit_enb);
}

impl AuxMuBaud {
    fn set_baud(&mut self, baud: u32) -> &mut Self {
        self.set_bits(0, 16, (board::AUX_UART_CLOCK / (baud * 8)) - 1)
    }
}

// init enables and initializes the aux UART (uart1).
fn init() {
    gpio::pin_use_as_alt5(board::AUX_UART_TX_PIN);
    gpio::pin_use_as_alt5(board::AUX_UART_RX_PIN);

    AuxEnables::zero().set_enable(true).store(); // uart enabled
    AuxMuIer::new(0).store(); // reset interupts
    AuxMuCntl::new(0).store(); // recv/xmit disabled
    AuxMuLcr::new(0).set_data_size(AuxMuLcr::DATA_SIZE_8B).store(); // 8bit mode
    AuxMuMcr::new(0).store(); // reset interupts
    AuxMuIer::new(0).store(); // reset interupts again
    AuxMuIir::new(0).set_clear_recv_fifo(true).set_clear_xmit_fifo(true).store(); // clear both fifos
    AuxMuBaud::zero().set_baud(115200).store();
    AuxMuCntl::zero().set_recv_enb(true).set_xmit_enb(true).store(); // recv/xmit enabled
}

// write_char writes a single character. It uses polling to wait
// for the uart to be writable.
fn write_char(ch: u8) {
    let mut aux_mu_lsr = AuxMuLsr::zero();
    while !aux_mu_lsr.fetch().get_tx_empty() { /* wait */ }
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
    ($($arg:tt)*) => ($crate::print!("core {}: {}\n", cpu::core_id(), format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}
