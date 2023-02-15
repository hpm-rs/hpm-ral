#![allow(unused)]
#![allow(dead_code)]

use hpm_ral::spi;
use hpm_ral::{modify_reg, read_reg, write_reg};

use crate::hal::spi::FullDuplex;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Polarity {
    IdleLow = 0,
    IdleHigh,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    SamplingOddEdge = 0,
    SamplingEvenEdge,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct TimingConfig {
    pub clk_src_freq: u32,
    pub sclk_freq: u32,
    pub csht: u32,
    pub cs2sclk: u32,
}

impl TimingConfig {
    pub fn with_sclk_freq(sclk_freq: u32) -> Self {
        Self {
            clk_src_freq: 24_000_000,
            sclk_freq,
            csht: 12,
            cs2sclk: 4,
        }
    }
}

impl Default for TimingConfig {
    fn default() -> Self {
        Self {
            clk_src_freq: 24_000_000,
            sclk_freq: 4_000_000,
            csht: 12,
            cs2sclk: 4,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MosiDirection {
    UniDirection = 0,
    BiDirection,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ControllerMode {
    Master = 0,
    Slave,
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TransferMode {
    ReadWhileWrite = 0,
    WriteOnly = 1,
    ReadOnly = 2,
    ReadAfterWrite = 3,
    WriteAfterRead = 4,
    WriteDummyRead = 5,
    ReadDummyWrite = 6,
    NoneData = 7,
    DummyWrite = 8,
    DummyRead = 9,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct FormatConfig {
    pub addr_len_in_bytes: u8,
    pub data_len_in_bits: u8,
    pub data_merge: bool,
    pub mosi_dir: MosiDirection,
    pub is_lsb: bool,
    pub mode: ControllerMode,
    pub cpol: Polarity,
    pub cpha: Phase,
}

impl Default for FormatConfig {
    fn default() -> Self {
        Self {
            addr_len_in_bytes: 1,
            data_len_in_bits: 8,
            data_merge: false,
            mosi_dir: MosiDirection::UniDirection,
            is_lsb: false,
            mode: ControllerMode::Master,
            cpol: Polarity::IdleLow,
            cpha: Phase::SamplingOddEdge,
        }
    }
}

#[derive(Debug)]
pub enum Error {
    Timeout,
}

pub struct Spi<const N: u8> {
    inner: spi::Instance<N>,
}

impl<const N: u8> Spi<N> {
    pub fn new(spi: spi::Instance<N>, timing: TimingConfig, format: FormatConfig) -> Self {
        // Timing config
        let sclk_div = (timing.clk_src_freq / timing.sclk_freq) / 2 - 1;
        write_reg!(
            spi,
            &spi,
            TIMING,
            SCLK_DIV: sclk_div,
            CSHT: timing.csht - 1,
            CS2SCLK: timing.cs2sclk - 1
        );
        // Format config
        write_reg!(
            spi,
            &spi,
            TRANSFMT,
            ADDRLEN: format.addr_len_in_bytes as u32 - 1,
            DATALEN: format.data_len_in_bits as u32 - 1,
            DATAMERGE: format.data_merge as u32,
            MOSIBIDIR: format.mosi_dir as u32,
            LSB: format.is_lsb as u32,
            SLVMODE: format.mode as u32,
            CPOL: format.cpol as u32,
            CPHA: format.cpha as u32
        );
        // Transfer control
        write_reg!(
            spi,
            &spi,
            TRANSCTRL,
            SLVDATAONLY: Disable,
            ADDRFMT: Single,
            DUALQUAD: Single,
            TOKENEN: Disable,
            TOKENVALUE: Token0x00
        );

        Spi { inner: spi }
    }

    pub fn write(&self, buf: &[u8]) {
        // Reset TX FIFO, RX FIFO and control
        modify_reg!(
            spi,
            &self.inner,
            CTRL,
            TXFIFORST: 1,
            RXFIFORST: 1,
            SPIRST: 1
        );
        // Transfer control
        modify_reg!(
            spi,
            &self.inner,
            TRANSCTRL,
            CMDEN: Disable,
            ADDREN: Disable,
            TRANSMODE: WriteOnly,
            WRTRANCNT: buf.len() as u32 - 1
        );
        // Write dummy command to start transfer
        write_reg!(spi, &self.inner, CMD, 0xff);
        // Write data
        for &data in buf {
            while self.is_tx_fifo_full() {}
            write_reg!(spi, &self.inner, DATA, data as u32);
        }
        // Wait transfer complete
        while !self.is_transfer_complete() {}
    }

    #[inline]
    pub fn is_tx_fifo_full(&self) -> bool {
        read_reg!(spi, &self.inner, STATUS, TXFULL) != 0
    }

    #[inline]
    pub fn is_rx_fifo_empty(&self) -> bool {
        read_reg!(spi, &self.inner, STATUS, RXEMPTY) != 0
    }

    #[inline]
    pub fn is_transfer_complete(&self) -> bool {
        read_reg!(spi, &self.inner, STATUS, SPIACTIVE) == 0
    }
}

impl<const N: u8> FullDuplex<u8> for Spi<N> {
    type Error = Error;

    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        if !self.is_rx_fifo_empty() {
            Ok(read_reg!(spi, &self.inner, DATA) as u8)
        } else {
            Err(nb::Error::WouldBlock)
        }
    }

    fn send(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        self.write(&[word]);
        Ok(())
    }
}

impl<const N: u8> crate::hal::blocking::spi::Write<u8> for Spi<N> {
    type Error = Error;

    fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
        // Disambiguation with spi::Write<W>::write
        Spi::<N>::write(&self, words);
        Ok(())
    }
}
