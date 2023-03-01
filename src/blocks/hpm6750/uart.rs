#[doc = "UART0"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    #[doc = "Configuration Register"]
    pub CFG: crate::RWRegister<u32>,
    #[doc = "Over Sample Control Register"]
    pub OSCR: crate::RWRegister<u32>,
    _reserved1: [u8; 0x08],
    #[doc = "Divisor Latch LSB (when DLAB = 1)"]
    pub DLL: crate::RWRegister<u32>,
    #[doc = "Divisor Latch MSB (when DLAB = 1)"]
    pub DLM: crate::RWRegister<u32>,
    #[doc = "FIFO Control Register"]
    pub FCR: crate::RWRegister<u32>,
    #[doc = "Line Control Register"]
    pub LCR: crate::RWRegister<u32>,
    #[doc = "Modem Control Register ("]
    pub MCR: crate::RWRegister<u32>,
    #[doc = "Line Status Register"]
    pub LSR: crate::RWRegister<u32>,
    #[doc = "Modem Status Register"]
    pub MSR: crate::RWRegister<u32>,
}
#[doc = "Configuration Register"]
pub mod CFG {
    #[doc = "The depth of RXFIFO and TXFIFO 0: 16-byte FIFO 1: 32-byte FIFO 2: 64-byte FIFO 3: 128-byte FIFO"]
    pub mod FIFOSIZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "16-byte FIFO"]
            pub const Bytes16: u32 = 0;
            #[doc = "32-byte FIFO"]
            pub const Bytes32: u32 = 0x01;
            #[doc = "64-byte FIFO"]
            pub const Bytes64: u32 = 0x02;
            #[doc = "128-byte FIFO"]
            pub const Bytes128: u32 = 0x03;
        }
    }
}
#[doc = "Over Sample Control Register"]
pub mod OSCR {
    #[doc = "Over-sample control The value must be an even number; any odd value writes to this field will be converted to an even value. OSC=0: The over-sample ratio is 32 OSC<=8: The over-sample ratio is 8 8 < OSC< 32: The over sample ratio is OSC"]
    pub mod OSC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Divisor Latch LSB (when DLAB = 1)"]
pub mod DLL {
    #[doc = "Least significant byte of the Divisor Latch"]
    pub mod DLL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Divisor Latch MSB (when DLAB = 1)"]
pub mod DLM {
    #[doc = "Most significant byte of the Divisor Latch"]
    pub mod DLM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "FIFO Control Register"]
pub mod FCR {
    #[doc = "FIFO enable Write 1 to enable both the transmitter and receiver FIFOs. The FIFOs are reset when the value of this bit toggles."]
    pub mod FIFOE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receiver FIFO reset Write 1 to clear all bytes in the RXFIFO and resets its counter. The Receiver Shift Register is not cleared. This bit will automatically be cleared."]
    pub mod RFIFORST {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmitter FIFO reset Write 1 to clear all bytes in the TXFIFO and resets its counter. The Transmitter Shift Register is not cleared. This bit will automatically be cleared."]
    pub mod TFIFORST {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA enable 0: Disable 1: Enable"]
    pub mod DMAE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmitter FIFO trigger level"]
    pub mod TFIFOT {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receiver FIFO trigger level"]
    pub mod RFIFOT {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Line Control Register"]
pub mod LCR {
    #[doc = "Word length setting 0: 5 bits 1: 6 bits 2: 7 bits 3: 8 bits"]
    pub mod WLS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "5 bits"]
            pub const Bits5: u32 = 0;
            #[doc = "6 bits"]
            pub const Bits6: u32 = 0x01;
            #[doc = "7 bits"]
            pub const Bits7: u32 = 0x02;
            #[doc = "8 bits"]
            pub const Bits8: u32 = 0x03;
        }
    }
    #[doc = "Number of STOP bits 0: 1 bits 1: The number of STOP bit is based on the WLS setting When WLS = 0, STOP bit is 1.5 bits When WLS = 1, 2, 3, STOP bit is 2 bits"]
    pub mod STB {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1 bit"]
            pub const Stop1bit: u32 = 0;
            #[doc = "The number of STOP bit is based on the WLS setting. When WLS = 0, STOP bit is 1.5 bits. When WLS = 1, 2, 3, STOP bit is 2 bits"]
            pub const Stop1p5bit: u32 = 0x01;
        }
    }
    #[doc = "Parity enable When this bit is set, a parity bit is generated in transmitted data before the first STOP bit and the parity bit would be checked for the received data."]
    pub mod PEN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable parity check"]
            pub const Disable: u32 = 0;
            #[doc = "Enable parity check"]
            pub const Enable: u32 = 0x01;
        }
    }
    #[doc = "Even parity select 1: Even parity (an even number of logic-1 is in the data and parity bits) 0: Old parity."]
    pub mod EPS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Old parity"]
            pub const Odd: u32 = 0;
            #[doc = "Even parity (an even number of logic-1 is in the data and parity bits)"]
            pub const Even: u32 = 0x01;
        }
    }
    #[doc = "Stick parity 1: Parity bit is constant 0 or 1, depending on bit4 (EPS). 0: Disable the sticky bit parity."]
    pub mod SPS {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable the sticky bit parity"]
            pub const Free: u32 = 0;
            #[doc = "Parity bit is constant 0 or 1, depending on bit4 (EPS)"]
            pub const Sticky: u32 = 0x01;
        }
    }
    #[doc = "Break control"]
    pub mod BC {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Divisor latch access bit"]
    pub mod DLAB {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Modem Control Register ("]
pub mod MCR {
    #[doc = "Request to send This bit controls the modem_rtsn output. 0: The modem_rtsn output signal will be driven HIGH 1: The modem_rtsn output signal will be driven LOW"]
    pub mod RTS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable loopback mode 0: Disable 1: Enable"]
    pub mod LOOP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Auto flow control enable 0: Disable 1: The auto-CTS and auto-RTS setting is based on the RTS bit setting: When RTS = 0, auto-CTS only When RTS = 1, auto-CTS and auto-RTS"]
    pub mod AFE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Line Status Register"]
pub mod LSR {
    #[doc = "Data ready. This bit is set when there are incoming received data in the Receiver Buffer Register (RBR). It is cleared when all of the received data are read."]
    pub mod DR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Overrun error This bit indicates that data in the Receiver Buffer Register (RBR) is overrun."]
    pub mod OE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Parity error This bit is set when the received parity does not match with the parity selected in the LCR\\[5:4\\]. It is cleared when this register is read. In the FIFO mode, this bit indicates the parity error for the received data at the top of the RXFIFO."]
    pub mod PE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Framing error This bit is set when the received STOP bit is not HIGH. It is cleared when this register is read. In the FIFO mode, this bit indicates the framing error for the received data at the top of the RXFIFO."]
    pub mod FE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Line break This bit is set when the uart_sin input signal was held LOWfor longer than the time for a full-word transmission. A full-word transmission is the transmission of the START, data, parity, and STOP bits. It is cleared when this register is read. In the FIFO mode, this bit indicates the line break for the received data at the top of the RXFIFO."]
    pub mod LBREAK {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmitter Holding Register empty This bit is 1 when the THR (TXFIFO in the FIFO mode) is empty. Otherwise, it is zero. If the THRE interrupt is enabled, an interrupt is triggered when THRE becomes 1."]
    pub mod THRE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmitter empty This bit is 1 when the THR (TXFIFO in the FIFO mode) and the Transmitter Shift Register (TSR) are both empty. Otherwise, it is zero."]
    pub mod TEMT {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error in RXFIFO In the FIFO mode, this bit is set when there is at least one parity error, framing error, or line break associated with data in the RXFIFO. It is cleared when this register is read and there is no more error for the rest of data in the RXFIFO."]
    pub mod ERRF {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Modem Status Register"]
pub mod MSR {
    #[doc = "Delta clear to send This bit is set when the state of the modem_ctsn input signal has been changed since the last time this register is read."]
    pub mod DCTS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Clear to send 0: The modem_ctsn input signal is HIGH. 1: The modem_ctsn input signal is LOW."]
    pub mod CTS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
