#[doc = "I2S0"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Control Register"]
    pub CTRL: crate::RWRegister<u32>,
    #[doc = "Rx FIFO Filling Level"]
    pub RFIFO_FILLINGS: crate::RWRegister<u32>,
    #[doc = "Tx FIFO Filling Level"]
    pub TFIFO_FILLINGS: crate::RWRegister<u32>,
    #[doc = "TX/RX FIFO Threshold setting."]
    pub FIFO_THRESH: crate::RWRegister<u32>,
    #[doc = "Status Registers"]
    pub STA: crate::RWRegister<u32>,
    _reserved0: [u8; 0x0c],
    #[doc = "Rx Data0"]
    pub RXD_DATA0: crate::RWRegister<u32>,
    #[doc = "Rx Data1"]
    pub RXD_DATA1: crate::RWRegister<u32>,
    #[doc = "Rx Data2"]
    pub RXD_DATA2: crate::RWRegister<u32>,
    #[doc = "Rx Data3"]
    pub RXD_DATA3: crate::RWRegister<u32>,
    #[doc = "Tx Data0"]
    pub TXD_DATA0: crate::RWRegister<u32>,
    #[doc = "Tx Data1"]
    pub TXD_DATA1: crate::RWRegister<u32>,
    #[doc = "Tx Data2"]
    pub TXD_DATA2: crate::RWRegister<u32>,
    #[doc = "Tx Data3"]
    pub TXD_DATA3: crate::RWRegister<u32>,
    _reserved1: [u8; 0x10],
    #[doc = "Configruation Regsiters"]
    pub CFGR: crate::RWRegister<u32>,
    _reserved2: [u8; 0x04],
    #[doc = "Misc configuration Registers"]
    pub MISC_CFGR: crate::RWRegister<u32>,
    _reserved3: [u8; 0x04],
    #[doc = "Rx Slots Enable for Rx Data0"]
    pub RXDSLOT_DATA0: crate::RWRegister<u32>,
    #[doc = "Rx Slots Enable for Rx Data1"]
    pub RXDSLOT_DATA1: crate::RWRegister<u32>,
    #[doc = "Rx Slots Enable for Rx Data2"]
    pub RXDSLOT_DATA2: crate::RWRegister<u32>,
    #[doc = "Rx Slots Enable for Rx Data3"]
    pub RXDSLOT_DATA3: crate::RWRegister<u32>,
    #[doc = "Tx Slots Enable for Tx Data0."]
    pub TXDSLOT_DATA0: crate::RWRegister<u32>,
    #[doc = "Tx Slots Enable for Tx Data1."]
    pub TXDSLOT_DATA1: crate::RWRegister<u32>,
    #[doc = "Tx Slots Enable for Tx Data2."]
    pub TXDSLOT_DATA2: crate::RWRegister<u32>,
    #[doc = "Tx Slots Enable for Tx Data3."]
    pub TXDSLOT_DATA3: crate::RWRegister<u32>,
}
#[doc = "Control Register"]
pub mod CTRL {
    #[doc = "enable for the module"]
    pub mod I2S_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "enable for each RX data pad"]
    pub mod RX_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "enable for each TX data pad"]
    pub mod TX_EN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Self-clear"]
    pub mod RXFIFOCLR {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Self-clear"]
    pub mod TXFIFOCLR {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Asserted to use DMA, else to use interrupt"]
    pub mod RX_DMA_EN {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Asserted to use DMA, else to use interrupt"]
    pub mod TX_DMA_EN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error interrupt enable This bit controls the generation of an interrupt when an error condition (UD, OV) occurs. 0: Error interrupt is masked 1: Error interrupt is enabled"]
    pub mod ERRIE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RX buffer data available interrupt enable 0: RXNE interrupt masked 1: RXNE interrupt not masked. Used to generate an interrupt request when the RXNE flag is set."]
    pub mod RXDAIE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TX buffer data needed interrupt enable 0: TXE interrupt masked 1: TXE interrupt not masked. Used to generate an interrupt request when the TXE flag is set."]
    pub mod TXDNIE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "software reset the CLK GEN module if asserted to be 1'b1. Self-clear."]
    pub mod SFTRST_CLKGEN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "software reset the TX module if asserted to be 1'b1. Self-clear."]
    pub mod SFTRST_TX {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "software reset the RX module if asserted to be 1'b1. Self-clear."]
    pub mod SFTRST_RX {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Rx FIFO Filling Level"]
pub mod RFIFO_FILLINGS {
    #[doc = "RX0 fifo fillings"]
    pub mod RX0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RX1 fifo fillings"]
    pub mod RX1 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RX2 fifo fillings"]
    pub mod RX2 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RX3 fifo fillings"]
    pub mod RX3 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Tx FIFO Filling Level"]
pub mod TFIFO_FILLINGS {
    #[doc = "TX0 fifo fillings"]
    pub mod TX0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TX1 fifo fillings"]
    pub mod TX1 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TX2 fifo fillings"]
    pub mod TX2 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TX3 fifo fillings"]
    pub mod TX3 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "TX/RX FIFO Threshold setting."]
pub mod FIFO_THRESH {
    #[doc = "RX fifo threshold to trigger STA\\[rx_da\\]. When rx fifo filling is greater than or equal to the threshold, assert the rx_da flag."]
    pub mod RX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TX fifo threshold to trigger STA\\[tx_dn\\]. When tx fifo filling is smaller than or equal to the threshold, assert the tx_dn flag."]
    pub mod TX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status Registers"]
pub mod STA {
    #[doc = "Asserted when rx fifo data are available."]
    pub mod RX_DA {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Asserted when tx fifo data are needed."]
    pub mod TX_DN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Asserted when rx fifo is overflow. Write 1 to any of these 4 bits will clear the overflow error."]
    pub mod RX_OV {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Asserted when tx fifo is underflow. Should be ANDed with CTRL\\[tx_en\\] the for correct value. Write 1 to any of these 4 bits will clear the underflow error."]
    pub mod TX_UD {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Rx Data0"]
pub mod RXD_DATA0 {
    #[doc = "No description avaiable"]
    pub mod D {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Rx Data1"]
pub mod RXD_DATA1 {
    #[doc = "No description avaiable"]
    pub mod D {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Rx Data2"]
pub mod RXD_DATA2 {
    #[doc = "No description avaiable"]
    pub mod D {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Rx Data3"]
pub mod RXD_DATA3 {
    #[doc = "No description avaiable"]
    pub mod D {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Tx Data0"]
pub mod TXD_DATA0 {
    #[doc = "No description avaiable"]
    pub mod D {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Tx Data1"]
pub mod TXD_DATA1 {
    #[doc = "No description avaiable"]
    pub mod D {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Tx Data2"]
pub mod TXD_DATA2 {
    #[doc = "No description avaiable"]
    pub mod D {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Tx Data3"]
pub mod TXD_DATA3 {
    #[doc = "No description avaiable"]
    pub mod D {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Configruation Regsiters"]
pub mod CFGR {
    #[doc = "Channel length (number of bits per audio channel) 0: 16-bit wide 1: 32-bit wide The bit write operation has a meaning only if DATLEN = 00 otherwise the channel length is fixed to 32-bit by hardware whatever the value filled in. Note: For correct operation, this bit should be configured when the I2S is disabled."]
    pub mod CHSIZ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data length to be transferred 00: 16-bit data length 01: 24-bit data length 10: 32-bit data length 11: Not allowed Note: For correct operation, these bits should be configured when the I2S is disabled."]
    pub mod DATSIZ {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "I2S standard selection 00: I2S Philips standard. 01: MSB justified standard (left justified) 10: LSB justified standard (right justified) 11: PCM standard Note: For correct operation, these bits should be configured when the I2S is disabled."]
    pub mod STD {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TDM mode 0: not TDM mode 1: TDM mode"]
    pub mod TDM_EN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CH_MAX\\[3:0\\] s the number of channels supported in TDM mode. When not in TDM mode, it must be set as 2. It must be an even number, so CH_MAX\\[0\\] is always 0. 4'h2: 2 channels 4'h4: 4 channels ..."]
    pub mod CH_MAX {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The start edge of a frame 0: Falling edge indicates a new frame (Just like standard I2S Philips standard) 1: Rising edge indicates a new frame"]
    pub mod FRAME_EDGE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "asserted to use external clk source"]
    pub mod MCK_SEL_OP {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "asserted to use external clk source"]
    pub mod FCLK_SEL_OP {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "asserted to use external clk source"]
    pub mod BCLK_SEL_OP {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Invert the MCLK pad input before using it internally. Only valid in MCLK slave mode"]
    pub mod INV_MCLK_IN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Invert the MCLK before sending it out to pad. Only valid in MCLK master mode"]
    pub mod INV_MCLK_OUT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Invert the FCLK pad input before using it internally. Only valid in FCLK slave mode"]
    pub mod INV_FCLK_IN {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Invert the FCLK before sending it out to pad. Only valid in FCLK master mode"]
    pub mod INV_FCLK_OUT {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Invert the BCLK pad input before using it internally. Only valid in BCLK slave mode"]
    pub mod INV_BCLK_IN {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Invert the BCLK before sending it out to pad. Only valid in BCLK master mode"]
    pub mod INV_BCLK_OUT {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Linear prescaler to generate BCLK from MCLK. BCLK_DIV \\[8:0\\] = 0: BCLK=No CLK. BCLK_DIV \\[8:0\\] = 1: BCLK=MCLK/1 BCLK_DIV \\[8:0\\] = n: BCLK=MCLK/(n). Note: These bits should be configured when the I2S is disabled. It is used only when the I2S is in master mode."]
    pub mod BCLK_DIV {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Gate off the bclk. Asserted to gate-off the BCLK."]
    pub mod BCLK_GATEOFF {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Misc configuration Registers"]
pub mod MISC_CFGR {
    #[doc = "Master clock output to pad enable 0: Master clock output is disabled 1: Master clock output is enabled Note: This bit should be configured when the I2S is disabled. It is used only when the I2S is in master mode."]
    pub mod MCLKOE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Gate off the mclk. This mclk is the output of a glitch prone mux, so every time to switch the mclk, the gate off clock should be asserted at first. After the clock is switched, de-assert this bit to ungate off the mclk."]
    pub mod MCLK_GATEOFF {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Rx Slots Enable for Rx Data0"]
pub mod RXDSLOT_DATA0 {
    #[doc = "No description avaiable"]
    pub mod EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Rx Slots Enable for Rx Data1"]
pub mod RXDSLOT_DATA1 {
    #[doc = "No description avaiable"]
    pub mod EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Rx Slots Enable for Rx Data2"]
pub mod RXDSLOT_DATA2 {
    #[doc = "No description avaiable"]
    pub mod EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Rx Slots Enable for Rx Data3"]
pub mod RXDSLOT_DATA3 {
    #[doc = "No description avaiable"]
    pub mod EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Tx Slots Enable for Tx Data0."]
pub mod TXDSLOT_DATA0 {
    #[doc = "No description avaiable"]
    pub mod EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Tx Slots Enable for Tx Data1."]
pub mod TXDSLOT_DATA1 {
    #[doc = "No description avaiable"]
    pub mod EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Tx Slots Enable for Tx Data2."]
pub mod TXDSLOT_DATA2 {
    #[doc = "No description avaiable"]
    pub mod EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Tx Slots Enable for Tx Data3."]
pub mod TXDSLOT_DATA3 {
    #[doc = "No description avaiable"]
    pub mod EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
