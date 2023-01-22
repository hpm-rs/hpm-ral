#![allow(dead_code)]

use core::fmt::Write;
use core::ops::Deref;

use hpm_ral::uart;
use hpm_ral::{modify_reg, read_reg, write_reg};

use super::dma::{self, Dma};

pub struct Uart<'a, const N: u8> {
    inner: uart::Instance<N>,
    dma: Option<&'a Dma>,
}

impl<'a, const N: u8> Uart<'a, N> {
    pub fn new(uart: uart::Instance<N>, dma: Option<&'a Dma>) -> Self {
        Self { inner: uart, dma }
    }

    pub fn init(&self, buadrate: u32, clock_src_freq: u32) {
        // Disable all interrupt
        write_reg!(uart, self.inner, DLM, 0);
        // Set DLAB to 1
        modify_reg!(uart, self.inner, LCR, DLAB: 1);

        let div = clock_src_freq / (buadrate * 16);
        modify_reg!(uart, self.inner, DLL, DLL: div);
        modify_reg!(uart, self.inner, DLM, DLM: div >> 8);

        // Set DLAB to 0
        modify_reg!(uart, self.inner, LCR, DLAB: 0);
        // Word length to 8 bits
        modify_reg!(uart, self.inner, LCR, WLS: Bits8);
        // Enable TX and RX FIFO and DMA
        modify_reg!(uart, self.inner, FCR, FIFOE: 1);
        if self.dma.is_some() {
            modify_reg!(uart, self.inner, FCR, DMAE: 1);
        }
    }

    pub fn send_byte(&self, byte: u8) {
        while read_reg!(uart, self.inner, LSR, THRE) == 0 {}
        write_reg!(uart, self.inner, DLL, DLL: byte as u32);
    }
}

impl<const N: u8> Write for Uart<'_, N> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        static mut FIRST_FLAG: bool = false;
        match self.inner.deref() as *const uart::RegisterBlock {
            uart::UART0 => unsafe {
                if let Some(dma) = self.dma {
                    // TODO: When the transfer completion flag is set, it's not actually completed
                    while FIRST_FLAG && !dma.is_complete(dma::UART0_TX_CH) {}
                    dma.uart0_start_tx(s.as_bytes(), s.len());
                    FIRST_FLAG = true;
                } else {
                    for ch in s.bytes() {
                        self.send_byte(ch);
                    }
                }
                Ok(())
            },
            _ => Err(core::fmt::Error),
        }
    }
}
