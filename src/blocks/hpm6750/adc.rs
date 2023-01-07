#[doc = "ADC0"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "No description avaiable"]
    pub CONFIG_TRG0A: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CONFIG_TRG0B: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CONFIG_TRG0C: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CONFIG_TRG1A: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CONFIG_TRG1B: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CONFIG_TRG1C: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CONFIG_TRG2A: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CONFIG_TRG2B: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CONFIG_TRG2C: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CONFIG_TRG3A: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CONFIG_TRG3B: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CONFIG_TRG3C: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub TRG_DMA_ADDR: crate::RWRegister<u32>,
    _reserved0: [u8; 0x03cc],
    #[doc = "No description avaiable"]
    pub BUS_RESULT_CHN0: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub BUS_RESULT_CHN1: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub BUS_RESULT_CHN2: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub BUS_RESULT_CHN3: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub BUS_RESULT_CHN4: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub BUS_RESULT_CHN5: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub BUS_RESULT_CHN6: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub BUS_RESULT_CHN7: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub BUS_RESULT_CHN8: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub BUS_RESULT_CHN9: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub BUS_RESULT_CHN10: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub BUS_RESULT_CHN11: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub BUS_RESULT_CHN12: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub BUS_RESULT_CHN13: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub BUS_RESULT_CHN14: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub BUS_RESULT_CHN15: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub BUS_RESULT_CHN16: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub BUS_RESULT_CHN17: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub BUS_RESULT_CHN18: crate::RWRegister<u32>,
    _reserved1: [u8; 0xb4],
    #[doc = "No description avaiable"]
    pub BUF_CFG0: crate::RWRegister<u32>,
    _reserved2: [u8; 0x02fc],
    #[doc = "No description avaiable"]
    pub SEQ_CFG0: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub SEQ_DMA_ADDR: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub SEQ_WR_ADDR: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub SEQ_DMA_CFG: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub SEQ_QUE_CFG0: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub SEQ_QUE_CFG1: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub SEQ_QUE_CFG2: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub SEQ_QUE_CFG3: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub SEQ_QUE_CFG4: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub SEQ_QUE_CFG5: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub SEQ_QUE_CFG6: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub SEQ_QUE_CFG7: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub SEQ_QUE_CFG8: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub SEQ_QUE_CFG9: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub SEQ_QUE_CFG10: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub SEQ_QUE_CFG11: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub SEQ_QUE_CFG12: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub SEQ_QUE_CFG13: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub SEQ_QUE_CFG14: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub SEQ_QUE_CFG15: crate::RWRegister<u32>,
    _reserved3: [u8; 0x03b0],
    #[doc = "No description avaiable"]
    pub PRD_CFG_CHN0_PRD_CFG: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub PRD_CFG_CHN0_PRD_THSHD_CFG: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub PRD_CFG_CHN0_PRD_RESULT: crate::RWRegister<u32>,
    _reserved4: [u8; 0x04],
    #[doc = "No description avaiable"]
    pub PRD_CFG_CHN1_PRD_CFG: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub PRD_CFG_CHN1_PRD_THSHD_CFG: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub PRD_CFG_CHN1_PRD_RESULT: crate::RWRegister<u32>,
    _reserved5: [u8; 0x04],
    #[doc = "No description avaiable"]
    pub PRD_CFG_CHN2_PRD_CFG: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub PRD_CFG_CHN2_PRD_THSHD_CFG: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub PRD_CFG_CHN2_PRD_RESULT: crate::RWRegister<u32>,
    _reserved6: [u8; 0x04],
    #[doc = "No description avaiable"]
    pub PRD_CFG_CHN3_PRD_CFG: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub PRD_CFG_CHN3_PRD_THSHD_CFG: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub PRD_CFG_CHN3_PRD_RESULT: crate::RWRegister<u32>,
    _reserved7: [u8; 0x04],
    #[doc = "No description avaiable"]
    pub PRD_CFG_CHN4_PRD_CFG: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub PRD_CFG_CHN4_PRD_THSHD_CFG: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub PRD_CFG_CHN4_PRD_RESULT: crate::RWRegister<u32>,
    _reserved8: [u8; 0x04],
    #[doc = "No description avaiable"]
    pub PRD_CFG_CHN5_PRD_CFG: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub PRD_CFG_CHN5_PRD_THSHD_CFG: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub PRD_CFG_CHN5_PRD_RESULT: crate::RWRegister<u32>,
    _reserved9: [u8; 0x04],
    #[doc = "No description avaiable"]
    pub PRD_CFG_CHN6_PRD_CFG: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub PRD_CFG_CHN6_PRD_THSHD_CFG: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub PRD_CFG_CHN6_PRD_RESULT: crate::RWRegister<u32>,
    _reserved10: [u8; 0x04],
    #[doc = "No description avaiable"]
    pub PRD_CFG_CHN7_PRD_CFG: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub PRD_CFG_CHN7_PRD_THSHD_CFG: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub PRD_CFG_CHN7_PRD_RESULT: crate::RWRegister<u32>,
    _reserved11: [u8; 0x04],
    #[doc = "No description avaiable"]
    pub PRD_CFG_CHN8_PRD_CFG: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub PRD_CFG_CHN8_PRD_THSHD_CFG: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub PRD_CFG_CHN8_PRD_RESULT: crate::RWRegister<u32>,
    _reserved12: [u8; 0x04],
    #[doc = "No description avaiable"]
    pub PRD_CFG_CHN9_PRD_CFG: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub PRD_CFG_CHN9_PRD_THSHD_CFG: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub PRD_CFG_CHN9_PRD_RESULT: crate::RWRegister<u32>,
    _reserved13: [u8; 0x04],
    #[doc = "No description avaiable"]
    pub PRD_CFG_CHN10_PRD_CFG: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub PRD_CFG_CHN10_PRD_THSHD_CFG: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub PRD_CFG_CHN10_PRD_RESULT: crate::RWRegister<u32>,
    _reserved14: [u8; 0x04],
    #[doc = "No description avaiable"]
    pub PRD_CFG_CHN11_PRD_CFG: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub PRD_CFG_CHN11_PRD_THSHD_CFG: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub PRD_CFG_CHN11_PRD_RESULT: crate::RWRegister<u32>,
    _reserved15: [u8; 0x04],
    #[doc = "No description avaiable"]
    pub PRD_CFG_CHN12_PRD_CFG: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub PRD_CFG_CHN12_PRD_THSHD_CFG: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub PRD_CFG_CHN12_PRD_RESULT: crate::RWRegister<u32>,
    _reserved16: [u8; 0x04],
    #[doc = "No description avaiable"]
    pub PRD_CFG_CHN13_PRD_CFG: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub PRD_CFG_CHN13_PRD_THSHD_CFG: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub PRD_CFG_CHN13_PRD_RESULT: crate::RWRegister<u32>,
    _reserved17: [u8; 0x04],
    #[doc = "No description avaiable"]
    pub PRD_CFG_CHN14_PRD_CFG: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub PRD_CFG_CHN14_PRD_THSHD_CFG: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub PRD_CFG_CHN14_PRD_RESULT: crate::RWRegister<u32>,
    _reserved18: [u8; 0x04],
    #[doc = "No description avaiable"]
    pub PRD_CFG_CHN15_PRD_CFG: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub PRD_CFG_CHN15_PRD_THSHD_CFG: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub PRD_CFG_CHN15_PRD_RESULT: crate::RWRegister<u32>,
    _reserved19: [u8; 0x04],
    #[doc = "No description avaiable"]
    pub PRD_CFG_CHN16_PRD_CFG: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub PRD_CFG_CHN16_PRD_THSHD_CFG: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub PRD_CFG_CHN16_PRD_RESULT: crate::RWRegister<u32>,
    _reserved20: [u8; 0x04],
    #[doc = "No description avaiable"]
    pub PRD_CFG_CHN17_PRD_CFG: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub PRD_CFG_CHN17_PRD_THSHD_CFG: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub PRD_CFG_CHN17_PRD_RESULT: crate::RWRegister<u32>,
    _reserved21: [u8; 0x04],
    #[doc = "No description avaiable"]
    pub PRD_CFG_CHN18_PRD_CFG: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub PRD_CFG_CHN18_PRD_THSHD_CFG: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub PRD_CFG_CHN18_PRD_RESULT: crate::RWRegister<u32>,
    _reserved22: [u8; 0x02d4],
    #[doc = "No description avaiable"]
    pub SAMPLE_CFG_CHN0: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub SAMPLE_CFG_CHN1: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub SAMPLE_CFG_CHN2: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub SAMPLE_CFG_CHN3: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub SAMPLE_CFG_CHN4: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub SAMPLE_CFG_CHN5: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub SAMPLE_CFG_CHN6: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub SAMPLE_CFG_CHN7: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub SAMPLE_CFG_CHN8: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub SAMPLE_CFG_CHN9: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub SAMPLE_CFG_CHN10: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub SAMPLE_CFG_CHN11: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub SAMPLE_CFG_CHN12: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub SAMPLE_CFG_CHN13: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub SAMPLE_CFG_CHN14: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub SAMPLE_CFG_CHN15: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub SAMPLE_CFG_CHN16: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub SAMPLE_CFG_CHN17: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub SAMPLE_CFG_CHN18: crate::RWRegister<u32>,
    _reserved23: [u8; 0xb8],
    #[doc = "No description avaiable"]
    pub CONV_CFG1: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub ADC_CFG0: crate::RWRegister<u32>,
    _reserved24: [u8; 0x04],
    #[doc = "No description avaiable"]
    pub INT_STS: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub INT_EN: crate::RWRegister<u32>,
    _reserved25: [u8; 0xe8],
    #[doc = "No description avaiable"]
    pub ANA_CTRL0: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub ANA_CTRL1: crate::RWRegister<u32>,
    _reserved26: [u8; 0x08],
    #[doc = "No description avaiable"]
    pub ANA_STATUS: crate::RWRegister<u32>,
}
#[doc = "No description avaiable"]
pub mod CONFIG_TRG0A {
    #[doc = "channel number for 1st conversion"]
    pub mod CHAN0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "interupt enable for 1st conversion"]
    pub mod INTEN0 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "channel number for 2nd conversion"]
    pub mod CHAN1 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "interupt enable for 2nd conversion"]
    pub mod INTEN1 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "channel number for 3rd conversion"]
    pub mod CHAN2 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "interupt enable for 3rd conversion"]
    pub mod INTEN2 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "channel number for 4th conversion"]
    pub mod CHAN3 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "interupt enable for 4th conversion"]
    pub mod INTEN3 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "length for current trigger, can up to 4 conversions for one trigger, from 0 to 3"]
    pub mod TRIG_LEN {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CONFIG_TRG0B {
    #[doc = "channel number for 1st conversion"]
    pub mod CHAN0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "interupt enable for 1st conversion"]
    pub mod INTEN0 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "channel number for 2nd conversion"]
    pub mod CHAN1 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "interupt enable for 2nd conversion"]
    pub mod INTEN1 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "channel number for 3rd conversion"]
    pub mod CHAN2 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "interupt enable for 3rd conversion"]
    pub mod INTEN2 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "channel number for 4th conversion"]
    pub mod CHAN3 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "interupt enable for 4th conversion"]
    pub mod INTEN3 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "length for current trigger, can up to 4 conversions for one trigger, from 0 to 3"]
    pub mod TRIG_LEN {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CONFIG_TRG0C {
    #[doc = "channel number for 1st conversion"]
    pub mod CHAN0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "interupt enable for 1st conversion"]
    pub mod INTEN0 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "channel number for 2nd conversion"]
    pub mod CHAN1 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "interupt enable for 2nd conversion"]
    pub mod INTEN1 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "channel number for 3rd conversion"]
    pub mod CHAN2 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "interupt enable for 3rd conversion"]
    pub mod INTEN2 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "channel number for 4th conversion"]
    pub mod CHAN3 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "interupt enable for 4th conversion"]
    pub mod INTEN3 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "length for current trigger, can up to 4 conversions for one trigger, from 0 to 3"]
    pub mod TRIG_LEN {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CONFIG_TRG1A {
    #[doc = "channel number for 1st conversion"]
    pub mod CHAN0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "interupt enable for 1st conversion"]
    pub mod INTEN0 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "channel number for 2nd conversion"]
    pub mod CHAN1 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "interupt enable for 2nd conversion"]
    pub mod INTEN1 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "channel number for 3rd conversion"]
    pub mod CHAN2 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "interupt enable for 3rd conversion"]
    pub mod INTEN2 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "channel number for 4th conversion"]
    pub mod CHAN3 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "interupt enable for 4th conversion"]
    pub mod INTEN3 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "length for current trigger, can up to 4 conversions for one trigger, from 0 to 3"]
    pub mod TRIG_LEN {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CONFIG_TRG1B {
    #[doc = "channel number for 1st conversion"]
    pub mod CHAN0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "interupt enable for 1st conversion"]
    pub mod INTEN0 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "channel number for 2nd conversion"]
    pub mod CHAN1 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "interupt enable for 2nd conversion"]
    pub mod INTEN1 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "channel number for 3rd conversion"]
    pub mod CHAN2 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "interupt enable for 3rd conversion"]
    pub mod INTEN2 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "channel number for 4th conversion"]
    pub mod CHAN3 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "interupt enable for 4th conversion"]
    pub mod INTEN3 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "length for current trigger, can up to 4 conversions for one trigger, from 0 to 3"]
    pub mod TRIG_LEN {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CONFIG_TRG1C {
    #[doc = "channel number for 1st conversion"]
    pub mod CHAN0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "interupt enable for 1st conversion"]
    pub mod INTEN0 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "channel number for 2nd conversion"]
    pub mod CHAN1 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "interupt enable for 2nd conversion"]
    pub mod INTEN1 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "channel number for 3rd conversion"]
    pub mod CHAN2 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "interupt enable for 3rd conversion"]
    pub mod INTEN2 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "channel number for 4th conversion"]
    pub mod CHAN3 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "interupt enable for 4th conversion"]
    pub mod INTEN3 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "length for current trigger, can up to 4 conversions for one trigger, from 0 to 3"]
    pub mod TRIG_LEN {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CONFIG_TRG2A {
    #[doc = "channel number for 1st conversion"]
    pub mod CHAN0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "interupt enable for 1st conversion"]
    pub mod INTEN0 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "channel number for 2nd conversion"]
    pub mod CHAN1 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "interupt enable for 2nd conversion"]
    pub mod INTEN1 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "channel number for 3rd conversion"]
    pub mod CHAN2 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "interupt enable for 3rd conversion"]
    pub mod INTEN2 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "channel number for 4th conversion"]
    pub mod CHAN3 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "interupt enable for 4th conversion"]
    pub mod INTEN3 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "length for current trigger, can up to 4 conversions for one trigger, from 0 to 3"]
    pub mod TRIG_LEN {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CONFIG_TRG2B {
    #[doc = "channel number for 1st conversion"]
    pub mod CHAN0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "interupt enable for 1st conversion"]
    pub mod INTEN0 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "channel number for 2nd conversion"]
    pub mod CHAN1 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "interupt enable for 2nd conversion"]
    pub mod INTEN1 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "channel number for 3rd conversion"]
    pub mod CHAN2 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "interupt enable for 3rd conversion"]
    pub mod INTEN2 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "channel number for 4th conversion"]
    pub mod CHAN3 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "interupt enable for 4th conversion"]
    pub mod INTEN3 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "length for current trigger, can up to 4 conversions for one trigger, from 0 to 3"]
    pub mod TRIG_LEN {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CONFIG_TRG2C {
    #[doc = "channel number for 1st conversion"]
    pub mod CHAN0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "interupt enable for 1st conversion"]
    pub mod INTEN0 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "channel number for 2nd conversion"]
    pub mod CHAN1 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "interupt enable for 2nd conversion"]
    pub mod INTEN1 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "channel number for 3rd conversion"]
    pub mod CHAN2 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "interupt enable for 3rd conversion"]
    pub mod INTEN2 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "channel number for 4th conversion"]
    pub mod CHAN3 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "interupt enable for 4th conversion"]
    pub mod INTEN3 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "length for current trigger, can up to 4 conversions for one trigger, from 0 to 3"]
    pub mod TRIG_LEN {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CONFIG_TRG3A {
    #[doc = "channel number for 1st conversion"]
    pub mod CHAN0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "interupt enable for 1st conversion"]
    pub mod INTEN0 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "channel number for 2nd conversion"]
    pub mod CHAN1 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "interupt enable for 2nd conversion"]
    pub mod INTEN1 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "channel number for 3rd conversion"]
    pub mod CHAN2 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "interupt enable for 3rd conversion"]
    pub mod INTEN2 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "channel number for 4th conversion"]
    pub mod CHAN3 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "interupt enable for 4th conversion"]
    pub mod INTEN3 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "length for current trigger, can up to 4 conversions for one trigger, from 0 to 3"]
    pub mod TRIG_LEN {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CONFIG_TRG3B {
    #[doc = "channel number for 1st conversion"]
    pub mod CHAN0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "interupt enable for 1st conversion"]
    pub mod INTEN0 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "channel number for 2nd conversion"]
    pub mod CHAN1 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "interupt enable for 2nd conversion"]
    pub mod INTEN1 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "channel number for 3rd conversion"]
    pub mod CHAN2 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "interupt enable for 3rd conversion"]
    pub mod INTEN2 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "channel number for 4th conversion"]
    pub mod CHAN3 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "interupt enable for 4th conversion"]
    pub mod INTEN3 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "length for current trigger, can up to 4 conversions for one trigger, from 0 to 3"]
    pub mod TRIG_LEN {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CONFIG_TRG3C {
    #[doc = "channel number for 1st conversion"]
    pub mod CHAN0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "interupt enable for 1st conversion"]
    pub mod INTEN0 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "channel number for 2nd conversion"]
    pub mod CHAN1 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "interupt enable for 2nd conversion"]
    pub mod INTEN1 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "channel number for 3rd conversion"]
    pub mod CHAN2 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "interupt enable for 3rd conversion"]
    pub mod INTEN2 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "channel number for 4th conversion"]
    pub mod CHAN3 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "interupt enable for 4th conversion"]
    pub mod INTEN3 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "length for current trigger, can up to 4 conversions for one trigger, from 0 to 3"]
    pub mod TRIG_LEN {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod TRG_DMA_ADDR {
    #[doc = "buffer start address for trigger queue, 192byte total, 16 bytes for each trigger (4 bytes for each conversion)"]
    pub mod TRG_DMA_ADDR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x3fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod BUS_RESULT_CHN0 {
    #[doc = "read this register will trigger one adc conversion. If wait_dis bit is set, SW will get the latest conversion result(not current one) with valid bit is 0, SW need polling valid bit till it's set to get current result If wait_dis bit is 0, SW can get the current conversion result with holding the bus, valid bit is always set at this mode. this is not recommended if channel sample time is too long"]
    pub mod CHAN_RESULT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set after conversion finished if wait_dis is set, cleared after software read. The first time read with 0 will trigger one new conversion. If SW read other channel when one channel conversion is in progress, it will not trigger new conversion at other channel, and will get old result with valid 0, also with read_cflct interrupt status bit set. the result may not realtime if software read once and wait long time to read again"]
    pub mod VALID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod BUS_RESULT_CHN1 {
    #[doc = "read this register will trigger one adc conversion. If wait_dis bit is set, SW will get the latest conversion result(not current one) with valid bit is 0, SW need polling valid bit till it's set to get current result If wait_dis bit is 0, SW can get the current conversion result with holding the bus, valid bit is always set at this mode. this is not recommended if channel sample time is too long"]
    pub mod CHAN_RESULT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set after conversion finished if wait_dis is set, cleared after software read. The first time read with 0 will trigger one new conversion. If SW read other channel when one channel conversion is in progress, it will not trigger new conversion at other channel, and will get old result with valid 0, also with read_cflct interrupt status bit set. the result may not realtime if software read once and wait long time to read again"]
    pub mod VALID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod BUS_RESULT_CHN2 {
    #[doc = "read this register will trigger one adc conversion. If wait_dis bit is set, SW will get the latest conversion result(not current one) with valid bit is 0, SW need polling valid bit till it's set to get current result If wait_dis bit is 0, SW can get the current conversion result with holding the bus, valid bit is always set at this mode. this is not recommended if channel sample time is too long"]
    pub mod CHAN_RESULT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set after conversion finished if wait_dis is set, cleared after software read. The first time read with 0 will trigger one new conversion. If SW read other channel when one channel conversion is in progress, it will not trigger new conversion at other channel, and will get old result with valid 0, also with read_cflct interrupt status bit set. the result may not realtime if software read once and wait long time to read again"]
    pub mod VALID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod BUS_RESULT_CHN3 {
    #[doc = "read this register will trigger one adc conversion. If wait_dis bit is set, SW will get the latest conversion result(not current one) with valid bit is 0, SW need polling valid bit till it's set to get current result If wait_dis bit is 0, SW can get the current conversion result with holding the bus, valid bit is always set at this mode. this is not recommended if channel sample time is too long"]
    pub mod CHAN_RESULT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set after conversion finished if wait_dis is set, cleared after software read. The first time read with 0 will trigger one new conversion. If SW read other channel when one channel conversion is in progress, it will not trigger new conversion at other channel, and will get old result with valid 0, also with read_cflct interrupt status bit set. the result may not realtime if software read once and wait long time to read again"]
    pub mod VALID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod BUS_RESULT_CHN4 {
    #[doc = "read this register will trigger one adc conversion. If wait_dis bit is set, SW will get the latest conversion result(not current one) with valid bit is 0, SW need polling valid bit till it's set to get current result If wait_dis bit is 0, SW can get the current conversion result with holding the bus, valid bit is always set at this mode. this is not recommended if channel sample time is too long"]
    pub mod CHAN_RESULT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set after conversion finished if wait_dis is set, cleared after software read. The first time read with 0 will trigger one new conversion. If SW read other channel when one channel conversion is in progress, it will not trigger new conversion at other channel, and will get old result with valid 0, also with read_cflct interrupt status bit set. the result may not realtime if software read once and wait long time to read again"]
    pub mod VALID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod BUS_RESULT_CHN5 {
    #[doc = "read this register will trigger one adc conversion. If wait_dis bit is set, SW will get the latest conversion result(not current one) with valid bit is 0, SW need polling valid bit till it's set to get current result If wait_dis bit is 0, SW can get the current conversion result with holding the bus, valid bit is always set at this mode. this is not recommended if channel sample time is too long"]
    pub mod CHAN_RESULT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set after conversion finished if wait_dis is set, cleared after software read. The first time read with 0 will trigger one new conversion. If SW read other channel when one channel conversion is in progress, it will not trigger new conversion at other channel, and will get old result with valid 0, also with read_cflct interrupt status bit set. the result may not realtime if software read once and wait long time to read again"]
    pub mod VALID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod BUS_RESULT_CHN6 {
    #[doc = "read this register will trigger one adc conversion. If wait_dis bit is set, SW will get the latest conversion result(not current one) with valid bit is 0, SW need polling valid bit till it's set to get current result If wait_dis bit is 0, SW can get the current conversion result with holding the bus, valid bit is always set at this mode. this is not recommended if channel sample time is too long"]
    pub mod CHAN_RESULT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set after conversion finished if wait_dis is set, cleared after software read. The first time read with 0 will trigger one new conversion. If SW read other channel when one channel conversion is in progress, it will not trigger new conversion at other channel, and will get old result with valid 0, also with read_cflct interrupt status bit set. the result may not realtime if software read once and wait long time to read again"]
    pub mod VALID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod BUS_RESULT_CHN7 {
    #[doc = "read this register will trigger one adc conversion. If wait_dis bit is set, SW will get the latest conversion result(not current one) with valid bit is 0, SW need polling valid bit till it's set to get current result If wait_dis bit is 0, SW can get the current conversion result with holding the bus, valid bit is always set at this mode. this is not recommended if channel sample time is too long"]
    pub mod CHAN_RESULT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set after conversion finished if wait_dis is set, cleared after software read. The first time read with 0 will trigger one new conversion. If SW read other channel when one channel conversion is in progress, it will not trigger new conversion at other channel, and will get old result with valid 0, also with read_cflct interrupt status bit set. the result may not realtime if software read once and wait long time to read again"]
    pub mod VALID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod BUS_RESULT_CHN8 {
    #[doc = "read this register will trigger one adc conversion. If wait_dis bit is set, SW will get the latest conversion result(not current one) with valid bit is 0, SW need polling valid bit till it's set to get current result If wait_dis bit is 0, SW can get the current conversion result with holding the bus, valid bit is always set at this mode. this is not recommended if channel sample time is too long"]
    pub mod CHAN_RESULT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set after conversion finished if wait_dis is set, cleared after software read. The first time read with 0 will trigger one new conversion. If SW read other channel when one channel conversion is in progress, it will not trigger new conversion at other channel, and will get old result with valid 0, also with read_cflct interrupt status bit set. the result may not realtime if software read once and wait long time to read again"]
    pub mod VALID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod BUS_RESULT_CHN9 {
    #[doc = "read this register will trigger one adc conversion. If wait_dis bit is set, SW will get the latest conversion result(not current one) with valid bit is 0, SW need polling valid bit till it's set to get current result If wait_dis bit is 0, SW can get the current conversion result with holding the bus, valid bit is always set at this mode. this is not recommended if channel sample time is too long"]
    pub mod CHAN_RESULT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set after conversion finished if wait_dis is set, cleared after software read. The first time read with 0 will trigger one new conversion. If SW read other channel when one channel conversion is in progress, it will not trigger new conversion at other channel, and will get old result with valid 0, also with read_cflct interrupt status bit set. the result may not realtime if software read once and wait long time to read again"]
    pub mod VALID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod BUS_RESULT_CHN10 {
    #[doc = "read this register will trigger one adc conversion. If wait_dis bit is set, SW will get the latest conversion result(not current one) with valid bit is 0, SW need polling valid bit till it's set to get current result If wait_dis bit is 0, SW can get the current conversion result with holding the bus, valid bit is always set at this mode. this is not recommended if channel sample time is too long"]
    pub mod CHAN_RESULT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set after conversion finished if wait_dis is set, cleared after software read. The first time read with 0 will trigger one new conversion. If SW read other channel when one channel conversion is in progress, it will not trigger new conversion at other channel, and will get old result with valid 0, also with read_cflct interrupt status bit set. the result may not realtime if software read once and wait long time to read again"]
    pub mod VALID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod BUS_RESULT_CHN11 {
    #[doc = "read this register will trigger one adc conversion. If wait_dis bit is set, SW will get the latest conversion result(not current one) with valid bit is 0, SW need polling valid bit till it's set to get current result If wait_dis bit is 0, SW can get the current conversion result with holding the bus, valid bit is always set at this mode. this is not recommended if channel sample time is too long"]
    pub mod CHAN_RESULT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set after conversion finished if wait_dis is set, cleared after software read. The first time read with 0 will trigger one new conversion. If SW read other channel when one channel conversion is in progress, it will not trigger new conversion at other channel, and will get old result with valid 0, also with read_cflct interrupt status bit set. the result may not realtime if software read once and wait long time to read again"]
    pub mod VALID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod BUS_RESULT_CHN12 {
    #[doc = "read this register will trigger one adc conversion. If wait_dis bit is set, SW will get the latest conversion result(not current one) with valid bit is 0, SW need polling valid bit till it's set to get current result If wait_dis bit is 0, SW can get the current conversion result with holding the bus, valid bit is always set at this mode. this is not recommended if channel sample time is too long"]
    pub mod CHAN_RESULT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set after conversion finished if wait_dis is set, cleared after software read. The first time read with 0 will trigger one new conversion. If SW read other channel when one channel conversion is in progress, it will not trigger new conversion at other channel, and will get old result with valid 0, also with read_cflct interrupt status bit set. the result may not realtime if software read once and wait long time to read again"]
    pub mod VALID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod BUS_RESULT_CHN13 {
    #[doc = "read this register will trigger one adc conversion. If wait_dis bit is set, SW will get the latest conversion result(not current one) with valid bit is 0, SW need polling valid bit till it's set to get current result If wait_dis bit is 0, SW can get the current conversion result with holding the bus, valid bit is always set at this mode. this is not recommended if channel sample time is too long"]
    pub mod CHAN_RESULT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set after conversion finished if wait_dis is set, cleared after software read. The first time read with 0 will trigger one new conversion. If SW read other channel when one channel conversion is in progress, it will not trigger new conversion at other channel, and will get old result with valid 0, also with read_cflct interrupt status bit set. the result may not realtime if software read once and wait long time to read again"]
    pub mod VALID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod BUS_RESULT_CHN14 {
    #[doc = "read this register will trigger one adc conversion. If wait_dis bit is set, SW will get the latest conversion result(not current one) with valid bit is 0, SW need polling valid bit till it's set to get current result If wait_dis bit is 0, SW can get the current conversion result with holding the bus, valid bit is always set at this mode. this is not recommended if channel sample time is too long"]
    pub mod CHAN_RESULT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set after conversion finished if wait_dis is set, cleared after software read. The first time read with 0 will trigger one new conversion. If SW read other channel when one channel conversion is in progress, it will not trigger new conversion at other channel, and will get old result with valid 0, also with read_cflct interrupt status bit set. the result may not realtime if software read once and wait long time to read again"]
    pub mod VALID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod BUS_RESULT_CHN15 {
    #[doc = "read this register will trigger one adc conversion. If wait_dis bit is set, SW will get the latest conversion result(not current one) with valid bit is 0, SW need polling valid bit till it's set to get current result If wait_dis bit is 0, SW can get the current conversion result with holding the bus, valid bit is always set at this mode. this is not recommended if channel sample time is too long"]
    pub mod CHAN_RESULT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set after conversion finished if wait_dis is set, cleared after software read. The first time read with 0 will trigger one new conversion. If SW read other channel when one channel conversion is in progress, it will not trigger new conversion at other channel, and will get old result with valid 0, also with read_cflct interrupt status bit set. the result may not realtime if software read once and wait long time to read again"]
    pub mod VALID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod BUS_RESULT_CHN16 {
    #[doc = "read this register will trigger one adc conversion. If wait_dis bit is set, SW will get the latest conversion result(not current one) with valid bit is 0, SW need polling valid bit till it's set to get current result If wait_dis bit is 0, SW can get the current conversion result with holding the bus, valid bit is always set at this mode. this is not recommended if channel sample time is too long"]
    pub mod CHAN_RESULT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set after conversion finished if wait_dis is set, cleared after software read. The first time read with 0 will trigger one new conversion. If SW read other channel when one channel conversion is in progress, it will not trigger new conversion at other channel, and will get old result with valid 0, also with read_cflct interrupt status bit set. the result may not realtime if software read once and wait long time to read again"]
    pub mod VALID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod BUS_RESULT_CHN17 {
    #[doc = "read this register will trigger one adc conversion. If wait_dis bit is set, SW will get the latest conversion result(not current one) with valid bit is 0, SW need polling valid bit till it's set to get current result If wait_dis bit is 0, SW can get the current conversion result with holding the bus, valid bit is always set at this mode. this is not recommended if channel sample time is too long"]
    pub mod CHAN_RESULT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set after conversion finished if wait_dis is set, cleared after software read. The first time read with 0 will trigger one new conversion. If SW read other channel when one channel conversion is in progress, it will not trigger new conversion at other channel, and will get old result with valid 0, also with read_cflct interrupt status bit set. the result may not realtime if software read once and wait long time to read again"]
    pub mod VALID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod BUS_RESULT_CHN18 {
    #[doc = "read this register will trigger one adc conversion. If wait_dis bit is set, SW will get the latest conversion result(not current one) with valid bit is 0, SW need polling valid bit till it's set to get current result If wait_dis bit is 0, SW can get the current conversion result with holding the bus, valid bit is always set at this mode. this is not recommended if channel sample time is too long"]
    pub mod CHAN_RESULT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set after conversion finished if wait_dis is set, cleared after software read. The first time read with 0 will trigger one new conversion. If SW read other channel when one channel conversion is in progress, it will not trigger new conversion at other channel, and will get old result with valid 0, also with read_cflct interrupt status bit set. the result may not realtime if software read once and wait long time to read again"]
    pub mod VALID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod BUF_CFG0 {
    #[doc = "set to disable read waiting, get result immediately but maybe not current conversion result."]
    pub mod WAIT_DIS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod SEQ_CFG0 {
    #[doc = "set to enable external HW trigger, only trigger on posedge"]
    pub mod HW_TRIG_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set to enable SW trigger"]
    pub mod SW_TRIG_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SW trigger, pulse signal, cleared by HW one cycle later"]
    pub mod SW_TRIG {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "if set, HW will continue process the queue till end(seq_len) after trigger once"]
    pub mod CONT_EN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "if set together with cont_en, HW will continue process the whole queue after trigger once. If cont_en is 0, this bit is not used"]
    pub mod RESTART_EN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "sequence queue length, 0 for one, 0xF for 16"]
    pub mod SEQ_LEN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "current dma write cycle bit"]
    pub mod CYCLE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod SEQ_DMA_ADDR {
    #[doc = "dma target address, should be 4-byte aligned"]
    pub mod TAR_ADDR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x3fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod SEQ_WR_ADDR {
    #[doc = "HW update this field after each dma write, it indicate the next dma write pointer. dma write address is (tar_addr+seq_wr_pointer)*4"]
    pub mod SEQ_WR_POINTER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod SEQ_DMA_CFG {
    #[doc = "dma buffer length, after write to (tar_addr\\[31:2\\]+buf_len)*4, the next dma address will be tar_addr\\[31:2\\]*4 0 for 4byte; 0xFFF for 16kbyte."]
    pub mod BUF_LEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set to stop dma if reach the stop_pos"]
    pub mod STOP_EN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set this bit will reset HW dma write pointer to seq_dma_addr, and set HW cycle bit to 1. dma is halted if this bit is set. SW should clear all cycle bit in buffer to 0 before clear dma_rst"]
    pub mod DMA_RST {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "if stop_en is set, SW is responsible to udpate this field to the next read point, HW should not write data to this point since it's not read out by SW yet"]
    pub mod STOP_POS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod SEQ_QUE_CFG0 {
    #[doc = "channel number for current conversion"]
    pub mod CHAN_NUM_4_0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "interrupt enable for current conversion"]
    pub mod SEQ_INT_EN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod SEQ_QUE_CFG1 {
    #[doc = "channel number for current conversion"]
    pub mod CHAN_NUM_4_0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "interrupt enable for current conversion"]
    pub mod SEQ_INT_EN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod SEQ_QUE_CFG2 {
    #[doc = "channel number for current conversion"]
    pub mod CHAN_NUM_4_0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "interrupt enable for current conversion"]
    pub mod SEQ_INT_EN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod SEQ_QUE_CFG3 {
    #[doc = "channel number for current conversion"]
    pub mod CHAN_NUM_4_0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "interrupt enable for current conversion"]
    pub mod SEQ_INT_EN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod SEQ_QUE_CFG4 {
    #[doc = "channel number for current conversion"]
    pub mod CHAN_NUM_4_0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "interrupt enable for current conversion"]
    pub mod SEQ_INT_EN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod SEQ_QUE_CFG5 {
    #[doc = "channel number for current conversion"]
    pub mod CHAN_NUM_4_0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "interrupt enable for current conversion"]
    pub mod SEQ_INT_EN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod SEQ_QUE_CFG6 {
    #[doc = "channel number for current conversion"]
    pub mod CHAN_NUM_4_0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "interrupt enable for current conversion"]
    pub mod SEQ_INT_EN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod SEQ_QUE_CFG7 {
    #[doc = "channel number for current conversion"]
    pub mod CHAN_NUM_4_0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "interrupt enable for current conversion"]
    pub mod SEQ_INT_EN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod SEQ_QUE_CFG8 {
    #[doc = "channel number for current conversion"]
    pub mod CHAN_NUM_4_0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "interrupt enable for current conversion"]
    pub mod SEQ_INT_EN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod SEQ_QUE_CFG9 {
    #[doc = "channel number for current conversion"]
    pub mod CHAN_NUM_4_0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "interrupt enable for current conversion"]
    pub mod SEQ_INT_EN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod SEQ_QUE_CFG10 {
    #[doc = "channel number for current conversion"]
    pub mod CHAN_NUM_4_0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "interrupt enable for current conversion"]
    pub mod SEQ_INT_EN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod SEQ_QUE_CFG11 {
    #[doc = "channel number for current conversion"]
    pub mod CHAN_NUM_4_0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "interrupt enable for current conversion"]
    pub mod SEQ_INT_EN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod SEQ_QUE_CFG12 {
    #[doc = "channel number for current conversion"]
    pub mod CHAN_NUM_4_0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "interrupt enable for current conversion"]
    pub mod SEQ_INT_EN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod SEQ_QUE_CFG13 {
    #[doc = "channel number for current conversion"]
    pub mod CHAN_NUM_4_0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "interrupt enable for current conversion"]
    pub mod SEQ_INT_EN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod SEQ_QUE_CFG14 {
    #[doc = "channel number for current conversion"]
    pub mod CHAN_NUM_4_0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "interrupt enable for current conversion"]
    pub mod SEQ_INT_EN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod SEQ_QUE_CFG15 {
    #[doc = "channel number for current conversion"]
    pub mod CHAN_NUM_4_0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "interrupt enable for current conversion"]
    pub mod SEQ_INT_EN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PRD_CFG_CHN0_PRD_CFG {
    #[doc = "conver period, with prescale. Set to 0 means disable current channel"]
    pub mod PRD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "0: 1xclock, 1: 2x, 2: 4x, 3: 8x,…,15: 32768x,…,31: 2Gx"]
    pub mod PRESCALE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PRD_CFG_CHN0_PRD_THSHD_CFG {
    #[doc = "threshold low"]
    pub mod THSHDL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "threshold high, assert interrupt(if enabled) if result exceed high or low."]
    pub mod THSHDH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PRD_CFG_CHN0_PRD_RESULT {
    #[doc = "adc convert result, update after each valid conversion. it may be updated period according to config, also may be updated due to other queue convert the same channel"]
    pub mod CHAN_RESULT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PRD_CFG_CHN1_PRD_CFG {
    #[doc = "conver period, with prescale. Set to 0 means disable current channel"]
    pub mod PRD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "0: 1xclock, 1: 2x, 2: 4x, 3: 8x,…,15: 32768x,…,31: 2Gx"]
    pub mod PRESCALE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PRD_CFG_CHN1_PRD_THSHD_CFG {
    #[doc = "threshold low"]
    pub mod THSHDL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "threshold high, assert interrupt(if enabled) if result exceed high or low."]
    pub mod THSHDH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PRD_CFG_CHN1_PRD_RESULT {
    #[doc = "adc convert result, update after each valid conversion. it may be updated period according to config, also may be updated due to other queue convert the same channel"]
    pub mod CHAN_RESULT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PRD_CFG_CHN2_PRD_CFG {
    #[doc = "conver period, with prescale. Set to 0 means disable current channel"]
    pub mod PRD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "0: 1xclock, 1: 2x, 2: 4x, 3: 8x,…,15: 32768x,…,31: 2Gx"]
    pub mod PRESCALE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PRD_CFG_CHN2_PRD_THSHD_CFG {
    #[doc = "threshold low"]
    pub mod THSHDL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "threshold high, assert interrupt(if enabled) if result exceed high or low."]
    pub mod THSHDH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PRD_CFG_CHN2_PRD_RESULT {
    #[doc = "adc convert result, update after each valid conversion. it may be updated period according to config, also may be updated due to other queue convert the same channel"]
    pub mod CHAN_RESULT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PRD_CFG_CHN3_PRD_CFG {
    #[doc = "conver period, with prescale. Set to 0 means disable current channel"]
    pub mod PRD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "0: 1xclock, 1: 2x, 2: 4x, 3: 8x,…,15: 32768x,…,31: 2Gx"]
    pub mod PRESCALE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PRD_CFG_CHN3_PRD_THSHD_CFG {
    #[doc = "threshold low"]
    pub mod THSHDL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "threshold high, assert interrupt(if enabled) if result exceed high or low."]
    pub mod THSHDH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PRD_CFG_CHN3_PRD_RESULT {
    #[doc = "adc convert result, update after each valid conversion. it may be updated period according to config, also may be updated due to other queue convert the same channel"]
    pub mod CHAN_RESULT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PRD_CFG_CHN4_PRD_CFG {
    #[doc = "conver period, with prescale. Set to 0 means disable current channel"]
    pub mod PRD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "0: 1xclock, 1: 2x, 2: 4x, 3: 8x,…,15: 32768x,…,31: 2Gx"]
    pub mod PRESCALE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PRD_CFG_CHN4_PRD_THSHD_CFG {
    #[doc = "threshold low"]
    pub mod THSHDL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "threshold high, assert interrupt(if enabled) if result exceed high or low."]
    pub mod THSHDH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PRD_CFG_CHN4_PRD_RESULT {
    #[doc = "adc convert result, update after each valid conversion. it may be updated period according to config, also may be updated due to other queue convert the same channel"]
    pub mod CHAN_RESULT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PRD_CFG_CHN5_PRD_CFG {
    #[doc = "conver period, with prescale. Set to 0 means disable current channel"]
    pub mod PRD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "0: 1xclock, 1: 2x, 2: 4x, 3: 8x,…,15: 32768x,…,31: 2Gx"]
    pub mod PRESCALE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PRD_CFG_CHN5_PRD_THSHD_CFG {
    #[doc = "threshold low"]
    pub mod THSHDL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "threshold high, assert interrupt(if enabled) if result exceed high or low."]
    pub mod THSHDH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PRD_CFG_CHN5_PRD_RESULT {
    #[doc = "adc convert result, update after each valid conversion. it may be updated period according to config, also may be updated due to other queue convert the same channel"]
    pub mod CHAN_RESULT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PRD_CFG_CHN6_PRD_CFG {
    #[doc = "conver period, with prescale. Set to 0 means disable current channel"]
    pub mod PRD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "0: 1xclock, 1: 2x, 2: 4x, 3: 8x,…,15: 32768x,…,31: 2Gx"]
    pub mod PRESCALE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PRD_CFG_CHN6_PRD_THSHD_CFG {
    #[doc = "threshold low"]
    pub mod THSHDL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "threshold high, assert interrupt(if enabled) if result exceed high or low."]
    pub mod THSHDH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PRD_CFG_CHN6_PRD_RESULT {
    #[doc = "adc convert result, update after each valid conversion. it may be updated period according to config, also may be updated due to other queue convert the same channel"]
    pub mod CHAN_RESULT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PRD_CFG_CHN7_PRD_CFG {
    #[doc = "conver period, with prescale. Set to 0 means disable current channel"]
    pub mod PRD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "0: 1xclock, 1: 2x, 2: 4x, 3: 8x,…,15: 32768x,…,31: 2Gx"]
    pub mod PRESCALE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PRD_CFG_CHN7_PRD_THSHD_CFG {
    #[doc = "threshold low"]
    pub mod THSHDL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "threshold high, assert interrupt(if enabled) if result exceed high or low."]
    pub mod THSHDH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PRD_CFG_CHN7_PRD_RESULT {
    #[doc = "adc convert result, update after each valid conversion. it may be updated period according to config, also may be updated due to other queue convert the same channel"]
    pub mod CHAN_RESULT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PRD_CFG_CHN8_PRD_CFG {
    #[doc = "conver period, with prescale. Set to 0 means disable current channel"]
    pub mod PRD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "0: 1xclock, 1: 2x, 2: 4x, 3: 8x,…,15: 32768x,…,31: 2Gx"]
    pub mod PRESCALE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PRD_CFG_CHN8_PRD_THSHD_CFG {
    #[doc = "threshold low"]
    pub mod THSHDL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "threshold high, assert interrupt(if enabled) if result exceed high or low."]
    pub mod THSHDH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PRD_CFG_CHN8_PRD_RESULT {
    #[doc = "adc convert result, update after each valid conversion. it may be updated period according to config, also may be updated due to other queue convert the same channel"]
    pub mod CHAN_RESULT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PRD_CFG_CHN9_PRD_CFG {
    #[doc = "conver period, with prescale. Set to 0 means disable current channel"]
    pub mod PRD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "0: 1xclock, 1: 2x, 2: 4x, 3: 8x,…,15: 32768x,…,31: 2Gx"]
    pub mod PRESCALE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PRD_CFG_CHN9_PRD_THSHD_CFG {
    #[doc = "threshold low"]
    pub mod THSHDL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "threshold high, assert interrupt(if enabled) if result exceed high or low."]
    pub mod THSHDH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PRD_CFG_CHN9_PRD_RESULT {
    #[doc = "adc convert result, update after each valid conversion. it may be updated period according to config, also may be updated due to other queue convert the same channel"]
    pub mod CHAN_RESULT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PRD_CFG_CHN10_PRD_CFG {
    #[doc = "conver period, with prescale. Set to 0 means disable current channel"]
    pub mod PRD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "0: 1xclock, 1: 2x, 2: 4x, 3: 8x,…,15: 32768x,…,31: 2Gx"]
    pub mod PRESCALE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PRD_CFG_CHN10_PRD_THSHD_CFG {
    #[doc = "threshold low"]
    pub mod THSHDL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "threshold high, assert interrupt(if enabled) if result exceed high or low."]
    pub mod THSHDH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PRD_CFG_CHN10_PRD_RESULT {
    #[doc = "adc convert result, update after each valid conversion. it may be updated period according to config, also may be updated due to other queue convert the same channel"]
    pub mod CHAN_RESULT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PRD_CFG_CHN11_PRD_CFG {
    #[doc = "conver period, with prescale. Set to 0 means disable current channel"]
    pub mod PRD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "0: 1xclock, 1: 2x, 2: 4x, 3: 8x,…,15: 32768x,…,31: 2Gx"]
    pub mod PRESCALE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PRD_CFG_CHN11_PRD_THSHD_CFG {
    #[doc = "threshold low"]
    pub mod THSHDL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "threshold high, assert interrupt(if enabled) if result exceed high or low."]
    pub mod THSHDH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PRD_CFG_CHN11_PRD_RESULT {
    #[doc = "adc convert result, update after each valid conversion. it may be updated period according to config, also may be updated due to other queue convert the same channel"]
    pub mod CHAN_RESULT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PRD_CFG_CHN12_PRD_CFG {
    #[doc = "conver period, with prescale. Set to 0 means disable current channel"]
    pub mod PRD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "0: 1xclock, 1: 2x, 2: 4x, 3: 8x,…,15: 32768x,…,31: 2Gx"]
    pub mod PRESCALE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PRD_CFG_CHN12_PRD_THSHD_CFG {
    #[doc = "threshold low"]
    pub mod THSHDL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "threshold high, assert interrupt(if enabled) if result exceed high or low."]
    pub mod THSHDH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PRD_CFG_CHN12_PRD_RESULT {
    #[doc = "adc convert result, update after each valid conversion. it may be updated period according to config, also may be updated due to other queue convert the same channel"]
    pub mod CHAN_RESULT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PRD_CFG_CHN13_PRD_CFG {
    #[doc = "conver period, with prescale. Set to 0 means disable current channel"]
    pub mod PRD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "0: 1xclock, 1: 2x, 2: 4x, 3: 8x,…,15: 32768x,…,31: 2Gx"]
    pub mod PRESCALE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PRD_CFG_CHN13_PRD_THSHD_CFG {
    #[doc = "threshold low"]
    pub mod THSHDL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "threshold high, assert interrupt(if enabled) if result exceed high or low."]
    pub mod THSHDH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PRD_CFG_CHN13_PRD_RESULT {
    #[doc = "adc convert result, update after each valid conversion. it may be updated period according to config, also may be updated due to other queue convert the same channel"]
    pub mod CHAN_RESULT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PRD_CFG_CHN14_PRD_CFG {
    #[doc = "conver period, with prescale. Set to 0 means disable current channel"]
    pub mod PRD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "0: 1xclock, 1: 2x, 2: 4x, 3: 8x,…,15: 32768x,…,31: 2Gx"]
    pub mod PRESCALE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PRD_CFG_CHN14_PRD_THSHD_CFG {
    #[doc = "threshold low"]
    pub mod THSHDL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "threshold high, assert interrupt(if enabled) if result exceed high or low."]
    pub mod THSHDH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PRD_CFG_CHN14_PRD_RESULT {
    #[doc = "adc convert result, update after each valid conversion. it may be updated period according to config, also may be updated due to other queue convert the same channel"]
    pub mod CHAN_RESULT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PRD_CFG_CHN15_PRD_CFG {
    #[doc = "conver period, with prescale. Set to 0 means disable current channel"]
    pub mod PRD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "0: 1xclock, 1: 2x, 2: 4x, 3: 8x,…,15: 32768x,…,31: 2Gx"]
    pub mod PRESCALE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PRD_CFG_CHN15_PRD_THSHD_CFG {
    #[doc = "threshold low"]
    pub mod THSHDL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "threshold high, assert interrupt(if enabled) if result exceed high or low."]
    pub mod THSHDH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PRD_CFG_CHN15_PRD_RESULT {
    #[doc = "adc convert result, update after each valid conversion. it may be updated period according to config, also may be updated due to other queue convert the same channel"]
    pub mod CHAN_RESULT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PRD_CFG_CHN16_PRD_CFG {
    #[doc = "conver period, with prescale. Set to 0 means disable current channel"]
    pub mod PRD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "0: 1xclock, 1: 2x, 2: 4x, 3: 8x,…,15: 32768x,…,31: 2Gx"]
    pub mod PRESCALE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PRD_CFG_CHN16_PRD_THSHD_CFG {
    #[doc = "threshold low"]
    pub mod THSHDL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "threshold high, assert interrupt(if enabled) if result exceed high or low."]
    pub mod THSHDH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PRD_CFG_CHN16_PRD_RESULT {
    #[doc = "adc convert result, update after each valid conversion. it may be updated period according to config, also may be updated due to other queue convert the same channel"]
    pub mod CHAN_RESULT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PRD_CFG_CHN17_PRD_CFG {
    #[doc = "conver period, with prescale. Set to 0 means disable current channel"]
    pub mod PRD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "0: 1xclock, 1: 2x, 2: 4x, 3: 8x,…,15: 32768x,…,31: 2Gx"]
    pub mod PRESCALE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PRD_CFG_CHN17_PRD_THSHD_CFG {
    #[doc = "threshold low"]
    pub mod THSHDL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "threshold high, assert interrupt(if enabled) if result exceed high or low."]
    pub mod THSHDH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PRD_CFG_CHN17_PRD_RESULT {
    #[doc = "adc convert result, update after each valid conversion. it may be updated period according to config, also may be updated due to other queue convert the same channel"]
    pub mod CHAN_RESULT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PRD_CFG_CHN18_PRD_CFG {
    #[doc = "conver period, with prescale. Set to 0 means disable current channel"]
    pub mod PRD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "0: 1xclock, 1: 2x, 2: 4x, 3: 8x,…,15: 32768x,…,31: 2Gx"]
    pub mod PRESCALE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PRD_CFG_CHN18_PRD_THSHD_CFG {
    #[doc = "threshold low"]
    pub mod THSHDL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "threshold high, assert interrupt(if enabled) if result exceed high or low."]
    pub mod THSHDH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PRD_CFG_CHN18_PRD_RESULT {
    #[doc = "adc convert result, update after each valid conversion. it may be updated period according to config, also may be updated due to other queue convert the same channel"]
    pub mod CHAN_RESULT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod SAMPLE_CFG_CHN0 {
    #[doc = "sample clock number, base on clock_period, default one period"]
    pub mod SAMPLE_CLOCK_NUMBER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "shift for sample_clock_number"]
    pub mod SAMPLE_CLOCK_NUMBER_SHIFT {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set to 1 to select differential channel"]
    pub mod DIFF_SEL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod SAMPLE_CFG_CHN1 {
    #[doc = "sample clock number, base on clock_period, default one period"]
    pub mod SAMPLE_CLOCK_NUMBER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "shift for sample_clock_number"]
    pub mod SAMPLE_CLOCK_NUMBER_SHIFT {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set to 1 to select differential channel"]
    pub mod DIFF_SEL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod SAMPLE_CFG_CHN2 {
    #[doc = "sample clock number, base on clock_period, default one period"]
    pub mod SAMPLE_CLOCK_NUMBER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "shift for sample_clock_number"]
    pub mod SAMPLE_CLOCK_NUMBER_SHIFT {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set to 1 to select differential channel"]
    pub mod DIFF_SEL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod SAMPLE_CFG_CHN3 {
    #[doc = "sample clock number, base on clock_period, default one period"]
    pub mod SAMPLE_CLOCK_NUMBER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "shift for sample_clock_number"]
    pub mod SAMPLE_CLOCK_NUMBER_SHIFT {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set to 1 to select differential channel"]
    pub mod DIFF_SEL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod SAMPLE_CFG_CHN4 {
    #[doc = "sample clock number, base on clock_period, default one period"]
    pub mod SAMPLE_CLOCK_NUMBER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "shift for sample_clock_number"]
    pub mod SAMPLE_CLOCK_NUMBER_SHIFT {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set to 1 to select differential channel"]
    pub mod DIFF_SEL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod SAMPLE_CFG_CHN5 {
    #[doc = "sample clock number, base on clock_period, default one period"]
    pub mod SAMPLE_CLOCK_NUMBER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "shift for sample_clock_number"]
    pub mod SAMPLE_CLOCK_NUMBER_SHIFT {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set to 1 to select differential channel"]
    pub mod DIFF_SEL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod SAMPLE_CFG_CHN6 {
    #[doc = "sample clock number, base on clock_period, default one period"]
    pub mod SAMPLE_CLOCK_NUMBER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "shift for sample_clock_number"]
    pub mod SAMPLE_CLOCK_NUMBER_SHIFT {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set to 1 to select differential channel"]
    pub mod DIFF_SEL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod SAMPLE_CFG_CHN7 {
    #[doc = "sample clock number, base on clock_period, default one period"]
    pub mod SAMPLE_CLOCK_NUMBER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "shift for sample_clock_number"]
    pub mod SAMPLE_CLOCK_NUMBER_SHIFT {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set to 1 to select differential channel"]
    pub mod DIFF_SEL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod SAMPLE_CFG_CHN8 {
    #[doc = "sample clock number, base on clock_period, default one period"]
    pub mod SAMPLE_CLOCK_NUMBER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "shift for sample_clock_number"]
    pub mod SAMPLE_CLOCK_NUMBER_SHIFT {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set to 1 to select differential channel"]
    pub mod DIFF_SEL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod SAMPLE_CFG_CHN9 {
    #[doc = "sample clock number, base on clock_period, default one period"]
    pub mod SAMPLE_CLOCK_NUMBER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "shift for sample_clock_number"]
    pub mod SAMPLE_CLOCK_NUMBER_SHIFT {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set to 1 to select differential channel"]
    pub mod DIFF_SEL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod SAMPLE_CFG_CHN10 {
    #[doc = "sample clock number, base on clock_period, default one period"]
    pub mod SAMPLE_CLOCK_NUMBER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "shift for sample_clock_number"]
    pub mod SAMPLE_CLOCK_NUMBER_SHIFT {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set to 1 to select differential channel"]
    pub mod DIFF_SEL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod SAMPLE_CFG_CHN11 {
    #[doc = "sample clock number, base on clock_period, default one period"]
    pub mod SAMPLE_CLOCK_NUMBER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "shift for sample_clock_number"]
    pub mod SAMPLE_CLOCK_NUMBER_SHIFT {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set to 1 to select differential channel"]
    pub mod DIFF_SEL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod SAMPLE_CFG_CHN12 {
    #[doc = "sample clock number, base on clock_period, default one period"]
    pub mod SAMPLE_CLOCK_NUMBER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "shift for sample_clock_number"]
    pub mod SAMPLE_CLOCK_NUMBER_SHIFT {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set to 1 to select differential channel"]
    pub mod DIFF_SEL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod SAMPLE_CFG_CHN13 {
    #[doc = "sample clock number, base on clock_period, default one period"]
    pub mod SAMPLE_CLOCK_NUMBER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "shift for sample_clock_number"]
    pub mod SAMPLE_CLOCK_NUMBER_SHIFT {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set to 1 to select differential channel"]
    pub mod DIFF_SEL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod SAMPLE_CFG_CHN14 {
    #[doc = "sample clock number, base on clock_period, default one period"]
    pub mod SAMPLE_CLOCK_NUMBER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "shift for sample_clock_number"]
    pub mod SAMPLE_CLOCK_NUMBER_SHIFT {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set to 1 to select differential channel"]
    pub mod DIFF_SEL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod SAMPLE_CFG_CHN15 {
    #[doc = "sample clock number, base on clock_period, default one period"]
    pub mod SAMPLE_CLOCK_NUMBER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "shift for sample_clock_number"]
    pub mod SAMPLE_CLOCK_NUMBER_SHIFT {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set to 1 to select differential channel"]
    pub mod DIFF_SEL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod SAMPLE_CFG_CHN16 {
    #[doc = "sample clock number, base on clock_period, default one period"]
    pub mod SAMPLE_CLOCK_NUMBER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "shift for sample_clock_number"]
    pub mod SAMPLE_CLOCK_NUMBER_SHIFT {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set to 1 to select differential channel"]
    pub mod DIFF_SEL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod SAMPLE_CFG_CHN17 {
    #[doc = "sample clock number, base on clock_period, default one period"]
    pub mod SAMPLE_CLOCK_NUMBER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "shift for sample_clock_number"]
    pub mod SAMPLE_CLOCK_NUMBER_SHIFT {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set to 1 to select differential channel"]
    pub mod DIFF_SEL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod SAMPLE_CFG_CHN18 {
    #[doc = "sample clock number, base on clock_period, default one period"]
    pub mod SAMPLE_CLOCK_NUMBER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "shift for sample_clock_number"]
    pub mod SAMPLE_CLOCK_NUMBER_SHIFT {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set to 1 to select differential channel"]
    pub mod DIFF_SEL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CONV_CFG1 {
    #[doc = "clock_period, N half clock cycle per half adc cycle 0 for same adc_clk and bus_clk, 1 for 1:2, 2 for 1:3. set to 2 can genenerate 66.7MHz adc_clk at 200MHz bus_clk"]
    pub mod CLOCK_DIVIDER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "convert clock numbers, set to 13 (0xD) for 12bit mode, which means convert need 14 adc clock cycles(based on clock after divider); set to 11 for 10bit mode; set to 9 for 8bit mode; set to 7 or 6bit mode; Ex: use 200MHz bus clock for adc, set sample_clock_number to 4, sample_clock_number_shift to 0, covert_clk_number to 13 for 12bit mode, clock_divder to 2, then each ADC convertion(plus sample) need 18(14 convert, 4 sample) cycles(66MHz)."]
    pub mod CONVERT_CLOCK_NUMBER {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod ADC_CFG0 {
    #[doc = "set to 1 to enable ADC DMA to write data to soc memory bus, for trig queue and seq queue;"]
    pub mod ADC_AHB_EN {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set to 1 will enable sync AHB bus, to get better bus performance. Adc_clk must to be set to same as bus clock at this mode"]
    pub mod SEL_SYNC_AHB {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod INT_STS {
    #[doc = "set if one chanel watch dog event triggered"]
    pub mod WDOG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0007_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set if got hresp=1"]
    pub mod AHB_ERR {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "No description avaiable"]
    pub mod DMA_FIFO_FULL {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "one conversion complete in seq_queue if related seq_int_en is set"]
    pub mod SEQ_CVC {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "the whole sequence complete interrupt"]
    pub mod SEQ_CMPT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "dma abort interrupt, set if seqence dma write pointer reachs sw read pointer if stop_en is set"]
    pub mod SEQ_DMAABT {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "No description avaiable"]
    pub mod SEQ_HW_CFLCT {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "sequence queue conflict interrup, set if HW or SW trigger received during conversion"]
    pub mod SEQ_SW_CFLCT {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "read conflict interrup, set if wait_dis is set, one conversion is in progress, SW read another channel"]
    pub mod READ_CFLCT {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "No description avaiable"]
    pub mod TRIG_HW_CFLCT {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "No description avaiable"]
    pub mod TRIG_SW_CFLCT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "interrupt for one trigger conversion complete if enabled"]
    pub mod TRIG_CMPT {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod INT_EN {
    #[doc = "set if one chanel watch dog event triggered"]
    pub mod WDOG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0007_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set if got hresp=1, generally caused by wrong trg_dma_addr or seq_dma_addr"]
    pub mod AHB_ERR {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA fifo full interrupt, user need to check clock frequency if it's set."]
    pub mod DMA_FIFO_FULL {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "one conversion complete in seq_queue if related seq_int_en is set"]
    pub mod SEQ_CVC {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "the whole sequence complete interrupt"]
    pub mod SEQ_CMPT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "dma abort interrupt, set if seqence dma write pointer reachs sw read pointer if stop_en is set"]
    pub mod SEQ_DMAABT {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "No description avaiable"]
    pub mod SEQ_HW_CFLCT {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "sequence queue conflict interrup, set if HW or SW trigger received during conversion"]
    pub mod SEQ_SW_CFLCT {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "read conflict interrup, set if wait_dis is set, one conversion is in progress, SW read another channel"]
    pub mod READ_CFLCT {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "No description avaiable"]
    pub mod TRIG_HW_CFLCT {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "No description avaiable"]
    pub mod TRIG_SW_CFLCT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "interrupt for one trigger conversion complete if enabled"]
    pub mod TRIG_CMPT {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod ANA_CTRL0 {
    #[doc = "Signal that loads the offset calibration word into the internal registers (Active H)"]
    pub mod LOADCAL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set to start the offset calibration cycle (Active H). user need to clear it after setting it."]
    pub mod STARTCAL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set to 1 to reset calibration logic; default high."]
    pub mod RESETCAL {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set to 1 to reset adc analog; default high."]
    pub mod RESETADC {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set to enable adc analog function. user need set it after LDO stable, or wait at least 20us after setting enldo, then set this bit."]
    pub mod ENADC {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set to enable adc LDO, need at least 20us for LDO to be stable."]
    pub mod ENLDO {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Defines the range for the LDO reference (vdd_soc) selrange_ldo = 0: LDO reference dvdd or vref_ldo in range \\[0.81;0.99\\] selrange_ldo = 1: LDO reference dvdd or vref_ldo in range \\[0.99;1.21\\]"]
    pub mod SELRANGE_LDO {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set will insert one adc cycle rearm before sample, user need to increase one to sample_clock_number"]
    pub mod REARM_EN {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "calibration value for single-end mode"]
    pub mod CAL_VAL_SE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "calibration value for differential mode"]
    pub mod CAL_VAL_DIFF {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod ANA_CTRL1 {
    #[doc = "11-12bit 10-10bit 01-8bit 00-6bit"]
    pub mod SELRES {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod ANA_STATUS {
    #[doc = "No description avaiable"]
    pub mod CAL_OUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates if the ADC is in calibration mode (Active H)."]
    pub mod CALON {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
