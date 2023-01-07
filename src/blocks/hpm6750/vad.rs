#[doc = "VAD"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Control Register"]
    pub CTRL: crate::RWRegister<u32>,
    #[doc = "Filter Control Register"]
    pub FILTCTRL: crate::RWRegister<u32>,
    #[doc = "Decision Control Register 0"]
    pub DEC_CTRL0: crate::RWRegister<u32>,
    #[doc = "Decision Control Register 1"]
    pub DEC_CTRL1: crate::RWRegister<u32>,
    #[doc = "Decision Control Register 2"]
    pub DEC_CTRL2: crate::RWRegister<u32>,
    _reserved0: [u8; 0x04],
    #[doc = "Status"]
    pub ST: crate::RWRegister<u32>,
    #[doc = "Out FIFO"]
    pub OFIFO: crate::RWRegister<u32>,
    #[doc = "Run Command Register"]
    pub RUN: crate::RWRegister<u32>,
    #[doc = "Out FIFO Control Register"]
    pub OFIFO_CTRL: crate::RWRegister<u32>,
    #[doc = "CIC Configuration Register"]
    pub CIC_CFG: crate::RWRegister<u32>,
    _reserved1: [u8; 0x74],
    #[doc = "Short Time Energy Register"]
    pub COEF_STE_ACT: crate::RWRegister<u32>,
}
#[doc = "Control Register"]
pub mod CTRL {
    #[doc = "the number of channels to be stored in buffer. Asserted to enable 2 channels."]
    pub mod CHNUM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Asserted to select PDM_CLK high level captured, otherwise to select PDM_CLK low level captured."]
    pub mod CH_POL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "pdm_clk_output_en"]
    pub mod PDM_CLK_OE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "asserted to bypass the pdm clock divider"]
    pub mod PDM_CLK_DIV_BYPASS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "OFIFO threshold to generate ofifo_av (when fillings >= threshold) (fifo size: max 16 items, 16*32bits)"]
    pub mod FIFO_THRSH {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "asserted to disable membuf"]
    pub mod MEMBUF_DISABLE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CIC saturation Interrupt Enable"]
    pub mod CIC_SAT_ERR_IE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CIC overload Interrupt Enable"]
    pub mod CIC_OVLD_ERR_IE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IIR overflow error interrupt enable"]
    pub mod IIR_OVFL_ERR_IE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IIR overload error interrupt enable"]
    pub mod IIR_OVLD_ERR_IE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "OFIFO overflow error interrupt enable"]
    pub mod OFIFO_OVFL_ERR_IE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Buf empty interrupt enable"]
    pub mod MEMBUF_EMPTY_IE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "OFIFO data available interrupt enable"]
    pub mod OFIFO_AV_IE {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "VAD event interrupt enable"]
    pub mod VAD_IE {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The clock divider will work at least 4. 0: div-by-2, 1: div-by-4 . . . n: div-by-2*(n+1)"]
    pub mod PDM_CLK_HFDIV {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Capture cycle delay>=0, should be less than PDM_CLK_HFDIV"]
    pub mod CAPT_DLY {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Filter Control Register"]
pub mod FILTCTRL {
    #[doc = "IIR slot enable"]
    pub mod IIR_SLOT_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "the decimation ratio of iir after CIC -1 2: means dec-by-3"]
    pub mod DECRATIO {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Decision Control Register 0"]
pub mod DEC_CTRL0 {
    #[doc = "length of sub-block"]
    pub mod SUBBLK_LEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "asserted to have 3 sub-blocks, otherwise to have 2 sub-blocks"]
    pub mod BLK_CFG {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "the value of amplitude for noise determination when calculationg ZCR"]
    pub mod NOISE_TOL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Decision Control Register 1"]
pub mod DEC_CTRL1 {
    #[doc = "ZCR low limit"]
    pub mod ZCR_LOW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ZCR high limit"]
    pub mod ZCR_HIGH {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Decision Control Register 2"]
pub mod DEC_CTRL2 {
    #[doc = "amplitude low limit"]
    pub mod AMP_LOW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "amplitude high limit"]
    pub mod AMP_HIGH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status"]
pub mod ST {
    #[doc = "CIC saturation"]
    pub mod CIC_SAT_ERR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CIC overload"]
    pub mod CIC_OVLD_ERR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IIR oberflow"]
    pub mod IIR_OVFL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IIR overloading"]
    pub mod IIR_OVLD {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "OFIFO overflow"]
    pub mod OFIFO_OVFL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Buf empty"]
    pub mod MEMBUF_EMPTY {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "OFIFO data available"]
    pub mod OFIFO_AV {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "VAD event found"]
    pub mod VAD {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Out FIFO"]
pub mod OFIFO {
    #[doc = "The PCM data. When there is only one channel, the samples are from Ch0, and the 2 samples in the 32-bits are: bit \\[31:16\\]: the samples earlier in time (\\[T-1\\]). Bit \\[15:0\\]: the samples later in time (\\[T\\]). When there is two channels, the samples in the 32-bits are: bit \\[31:16\\]: the samples belong to Ch 1 (when ch_pol\\[1:0\\]==2, the data is captured at the positive part of the pdm clk). bit \\[15:0\\]: the samples belong to Ch 0 (when ch_pol\\[1:0\\]==2, the data is captured at the negtive part of the pdm clk)."]
    pub mod D {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Run Command Register"]
pub mod RUN {
    #[doc = "module enable"]
    pub mod VAD_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "software reset. Self-clear"]
    pub mod SFTRST {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Out FIFO Control Register"]
pub mod OFIFO_CTRL {
    #[doc = "Asserted to enable OFIFO"]
    pub mod EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CIC Configuration Register"]
pub mod CIC_CFG {
    #[doc = "the shift value after CIC results."]
    pub mod POST_SCALE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Short Time Energy Register"]
pub mod COEF_STE_ACT {
    #[doc = "The current detected short time energy"]
    pub mod VAL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
