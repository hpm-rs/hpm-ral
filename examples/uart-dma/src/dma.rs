#![allow(dead_code)]

use hpm_ral::{dma, dmamux};
use hpm_ral::{modify_reg, read_reg, write_reg};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Channel {
    Channel0 = 0,
    Channel1 = 1,
    Channel2 = 2,
    Channel3 = 3,
    Channel4 = 4,
    Channel5 = 5,
    Channel6 = 6,
    Channel7 = 7,
}

pub enum Source {
    Uart0Rx = 8,
    Uart0Tx = 9,
}

const UART_THR_OFFSET: u32 = 0x20;
pub(crate) const UART0_TX_CH: Channel = Channel::Channel0;

pub struct Dma {
    hdma: dma::HDMA0,
    dmamux: dmamux::DMAMUX,
}

impl Dma {
    pub fn new(hdma: dma::HDMA0, dmamux: dmamux::DMAMUX) -> Self {
        let dma = Dma { hdma, dmamux };
        dma.setup();
        dma
    }

    fn setup(&self) {
        // Setup channel 0 for for UART0 TX
        write_reg!(
            dma,
            self.hdma,
            CHCTRL_CH0_CTRL,
            SRCBUSINFIDX: 0,
            DSTBUSINFIDX: 0,
            PRIORITY: Lower,
            SRCBURSTSIZE: Transfer1,
            SRCWIDTH: Byte,
            DSTWIDTH: Byte,
            SRCMODE: Normal,
            DSTMODE: Handshake,
            SRCADDRCTRL: Increment,
            DSTADDRCTRL: Fixed,
            SRCREQSEL: UART0_TX_CH as u32,
            DSTREQSEL: UART0_TX_CH as u32,
            INTABTMASK: Disable,
            INTERRMASK: Disable,
            INTTCMASK: Disable,
            ENABLE: Disable
        );
        write_reg!(
            dma,
            self.hdma,
            CHCTRL_CH0_DSTADDR,
            hpm_ral::uart::UART0 as u32 + UART_THR_OFFSET
        );
        // Setup DMAMUX
        write_reg!(
            dmamux,
            self.dmamux,
            MUXCFG_HDMA_MUX0,
            ENABLE: Enable,
            SOURCE: Source::Uart0Tx as u32
        );
    }

    #[inline(always)]
    pub fn is_enabled(&self, ch: Channel) -> bool {
        (read_reg!(dma, self.hdma, CHEN) >> (ch as u32)) & 0x01 != 0
    }

    #[inline(always)]
    pub fn is_complete(&self, ch: Channel) -> bool {
        (read_reg!(dma, self.hdma, INTSTATUS, TC) >> (ch as u32)) & 0x01 != 0
    }

    #[inline(always)]
    pub fn abort(&self, ch: Channel) {
        write_reg!(dma, self.hdma, CHABORT, 1 << (ch as u32));
    }

    pub fn uart0_start_tx(&self, tx: &[u8], len: usize) {
        self.abort(UART0_TX_CH);
        // Write 1 to clear flag
        write_reg!(
            dma,
            self.hdma,
            INTSTATUS,
            TC: 1 << (UART0_TX_CH as u32),
            ABORT: 1 << (UART0_TX_CH as u32),
            ERROR: 1 << (UART0_TX_CH as u32)
        );
        write_reg!(dma, self.hdma, CHCTRL_CH0_SRCADDR, tx.as_ptr() as u32);
        write_reg!(dma, self.hdma, CHCTRL_CH0_TRANSIZE, len as u32);
        modify_reg!(dma, self.hdma, CHCTRL_CH0_CTRL, ENABLE: 1);
    }
}
