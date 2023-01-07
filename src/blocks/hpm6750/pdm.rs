#[doc = "PDM"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Control Register"]
    pub CTRL: crate::RWRegister<u32>,
    #[doc = "Channel Control Register"]
    pub CH_CTRL: crate::RWRegister<u32>,
    #[doc = "Status Register"]
    pub ST: crate::RWRegister<u32>,
    #[doc = "Channel Configuration Register"]
    pub CH_CFG: crate::RWRegister<u32>,
    #[doc = "CIC configuration register"]
    pub CIC_CFG: crate::RWRegister<u32>,
    #[doc = "In Buf Control Register"]
    pub CTRL_INBUF: crate::RWRegister<u32>,
    #[doc = "Filter 0 Control Register"]
    pub CTRL_FILT0: crate::RWRegister<u32>,
    #[doc = "Filter 1 Control Register"]
    pub CTRL_FILT1: crate::RWRegister<u32>,
    #[doc = "Run Register"]
    pub RUN: crate::RWRegister<u32>,
    #[doc = "Memory Access Address"]
    pub MEMADDR: crate::RWRegister<u32>,
    #[doc = "Memory Access Data"]
    pub MEMDATA: crate::RWRegister<u32>,
    #[doc = "HPF A Coef Register"]
    pub HPF_MA: crate::RWRegister<u32>,
    #[doc = "HPF B Coef Register"]
    pub HPF_B: crate::RWRegister<u32>,
}
#[doc = "Control Register"]
pub mod CTRL {
    #[doc = "pdm high pass filter enable. This order-1 HPF only applies to the PDM mic data."]
    pub mod HPF_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "pdm_clk_output_en"]
    pub mod PDM_CLK_OE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "asserted to bypass the pdm clock divider"]
    pub mod PDM_CLK_DIV_BYPASS {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The clock divider will work at least 4. 0: div-by-2, 1: div-by-4 . . . n: div-by-2*(n+1)"]
    pub mod PDM_CLK_HFDIV {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Capture cycle delay>=0, should be less than PDM_CLK_HFDIV"]
    pub mod CAPT_DLY {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "decimation rate after CIC. Now it is forced to be 3."]
    pub mod DEC_AFT_CIC {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error interrupt enable This bit controls the generation of an interrupt when an error condition (CIC saturation) occurs. 0: Error interrupt is masked 1: Error interrupt is enabled"]
    pub mod CIC_SAT_ERR_IE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CIC overload error interrupt enable"]
    pub mod CIC_OVLD_ERR_IE {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "output fifo overflow error interrupt enable"]
    pub mod OFIFO_OVFL_ERR_IE {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "data accessed out of boundary error interruput enable. The error happens when the module cannot calculate the enough number of data in time."]
    pub mod FILT_CRX_ERR_IE {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Asserted to use Coef RAM instead of Coef ROM"]
    pub mod USE_COEF_RAM {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "asserted if the falling edge of the ref fclk from DAO is the start of a new frame. This is used to to align DAO feedback signal."]
    pub mod SOF_FEDGE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "software reset the module. Self-clear."]
    pub mod SFTRST {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel Control Register"]
pub mod CH_CTRL {
    #[doc = "Asserted to enable the channel. Ch8 & 9 are refs. Ch0-7 are pdm mics."]
    pub mod CH_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Asserted to select PDM_CLK high level captured, otherwise to select PDM_CLK low level captured."]
    pub mod CH_POL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status Register"]
pub mod ST {
    #[doc = "CIC saturation. Write 1 clear"]
    pub mod CIC_SAT_ERR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CIC overload error. write 1 clear"]
    pub mod CIC_OVLD_ERR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "output fifo overflow error. The reason may be sampling frequency mismatch, either fast or slow."]
    pub mod OFIFO_OVFL_ERR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "data accessed out of boundary error"]
    pub mod FILT_CRX_ERR {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel Configuration Register"]
pub mod CH_CFG {
    #[doc = "Type of Channel 0 2'b00: dec-by-3 wiith filter type0 (CIC Compenstation+norm filter) 2'b01: dec-by-3 with filter type 1 (No CIC compenstation, only norm filter)"]
    pub mod CH0_TYPE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "No description avaiable"]
    pub mod CH1_TYPE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "No description avaiable"]
    pub mod CH2_TYPE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "No description avaiable"]
    pub mod CH3_TYPE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "No description avaiable"]
    pub mod CH4_TYPE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "No description avaiable"]
    pub mod CH5_TYPE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "No description avaiable"]
    pub mod CH6_TYPE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "No description avaiable"]
    pub mod CH7_TYPE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "No description avaiable"]
    pub mod CH8_TYPE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "No description avaiable"]
    pub mod CH9_TYPE {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CIC configuration register"]
pub mod CIC_CFG {
    #[doc = "CIC decimation factor"]
    pub mod CIC_DEC_RATIO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Sigma_delta_order\\[1:0\\] 2'b00: 7 2'b01: 6 2'b10: 5 Others: unused"]
    pub mod SGD {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "the shift value after CIC results."]
    pub mod POST_SCALE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "In Buf Control Register"]
pub mod CTRL_INBUF {
    #[doc = "The starting address of channel 0 in filter data buffer"]
    pub mod START_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The spacing between starting address of adjacent channels"]
    pub mod PITCH {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The buf size-1 for each channel"]
    pub mod MAX_PTR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Filter 0 Control Register"]
pub mod CTRL_FILT0 {
    #[doc = "Starting address of Coef of filter type 2'b00 in coef memory"]
    pub mod COEF_START_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Coef length of filter type 2'b00 in coef memory"]
    pub mod COEF_LEN_M0 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Filter 1 Control Register"]
pub mod CTRL_FILT1 {
    #[doc = "Starting address of Coef of filter type 2'b01 in coef memory"]
    pub mod COEF_START_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Coef length of filter type 2'b01 in coef memory"]
    pub mod COEF_LEN_M1 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Run Register"]
pub mod RUN {
    #[doc = "Asserted to enable the module"]
    pub mod PDM_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Access Address"]
pub mod MEMADDR {
    #[doc = "0--0x0FFFFFFF: COEF_RAM 0x10000000--0x1FFFFFFF: DATA_RAM"]
    pub mod ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Memory Access Data"]
pub mod MEMDATA {
    #[doc = "The data write-to/read-from buffer"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "HPF A Coef Register"]
pub mod HPF_MA {
    #[doc = "Composite value of coef A of the Order-1 HPF"]
    pub mod COEF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "HPF B Coef Register"]
pub mod HPF_B {
    #[doc = "coef B of the Order-1 HPF"]
    pub mod COEF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
