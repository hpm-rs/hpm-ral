#[doc = "PWM0"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Shadow registers unlock register"]
    pub UNLK: crate::RWRegister<u32>,
    #[doc = "Counter start register"]
    pub STA: crate::RWRegister<u32>,
    #[doc = "Counter reload register"]
    pub RLD: crate::RWRegister<u32>,
    #[doc = "Comparator register"]
    pub CMP_0: crate::RWRegister<u32>,
    #[doc = "Comparator register"]
    pub CMP_1: crate::RWRegister<u32>,
    #[doc = "Comparator register"]
    pub CMP_2: crate::RWRegister<u32>,
    #[doc = "Comparator register"]
    pub CMP_3: crate::RWRegister<u32>,
    #[doc = "Comparator register"]
    pub CMP_4: crate::RWRegister<u32>,
    #[doc = "Comparator register"]
    pub CMP_5: crate::RWRegister<u32>,
    #[doc = "Comparator register"]
    pub CMP_6: crate::RWRegister<u32>,
    #[doc = "Comparator register"]
    pub CMP_7: crate::RWRegister<u32>,
    #[doc = "Comparator register"]
    pub CMP_8: crate::RWRegister<u32>,
    #[doc = "Comparator register"]
    pub CMP_9: crate::RWRegister<u32>,
    #[doc = "Comparator register"]
    pub CMP_10: crate::RWRegister<u32>,
    #[doc = "Comparator register"]
    pub CMP_11: crate::RWRegister<u32>,
    #[doc = "Comparator register"]
    pub CMP_12: crate::RWRegister<u32>,
    #[doc = "Comparator register"]
    pub CMP_13: crate::RWRegister<u32>,
    #[doc = "Comparator register"]
    pub CMP_14: crate::RWRegister<u32>,
    #[doc = "Comparator register"]
    pub CMP_15: crate::RWRegister<u32>,
    #[doc = "Comparator register"]
    pub CMP_16: crate::RWRegister<u32>,
    #[doc = "Comparator register"]
    pub CMP_17: crate::RWRegister<u32>,
    #[doc = "Comparator register"]
    pub CMP_18: crate::RWRegister<u32>,
    #[doc = "Comparator register"]
    pub CMP_19: crate::RWRegister<u32>,
    #[doc = "Comparator register"]
    pub CMP_20: crate::RWRegister<u32>,
    #[doc = "Comparator register"]
    pub CMP_21: crate::RWRegister<u32>,
    #[doc = "Comparator register"]
    pub CMP_22: crate::RWRegister<u32>,
    #[doc = "Comparator register"]
    pub CMP_23: crate::RWRegister<u32>,
    _reserved0: [u8; 0x0c],
    #[doc = "Force output mode register"]
    pub FRCMD: crate::RWRegister<u32>,
    #[doc = "Shadow registers lock register"]
    pub SHLK: crate::RWRegister<u32>,
    #[doc = "Output channel configure register"]
    pub CHCFG_0: crate::RWRegister<u32>,
    #[doc = "Output channel configure register"]
    pub CHCFG_1: crate::RWRegister<u32>,
    #[doc = "Output channel configure register"]
    pub CHCFG_2: crate::RWRegister<u32>,
    #[doc = "Output channel configure register"]
    pub CHCFG_3: crate::RWRegister<u32>,
    #[doc = "Output channel configure register"]
    pub CHCFG_4: crate::RWRegister<u32>,
    #[doc = "Output channel configure register"]
    pub CHCFG_5: crate::RWRegister<u32>,
    #[doc = "Output channel configure register"]
    pub CHCFG_6: crate::RWRegister<u32>,
    #[doc = "Output channel configure register"]
    pub CHCFG_7: crate::RWRegister<u32>,
    #[doc = "Output channel configure register"]
    pub CHCFG_8: crate::RWRegister<u32>,
    #[doc = "Output channel configure register"]
    pub CHCFG_9: crate::RWRegister<u32>,
    #[doc = "Output channel configure register"]
    pub CHCFG_10: crate::RWRegister<u32>,
    #[doc = "Output channel configure register"]
    pub CHCFG_11: crate::RWRegister<u32>,
    #[doc = "Output channel configure register"]
    pub CHCFG_12: crate::RWRegister<u32>,
    #[doc = "Output channel configure register"]
    pub CHCFG_13: crate::RWRegister<u32>,
    #[doc = "Output channel configure register"]
    pub CHCFG_14: crate::RWRegister<u32>,
    #[doc = "Output channel configure register"]
    pub CHCFG_15: crate::RWRegister<u32>,
    #[doc = "Output channel configure register"]
    pub CHCFG_16: crate::RWRegister<u32>,
    #[doc = "Output channel configure register"]
    pub CHCFG_17: crate::RWRegister<u32>,
    #[doc = "Output channel configure register"]
    pub CHCFG_18: crate::RWRegister<u32>,
    #[doc = "Output channel configure register"]
    pub CHCFG_19: crate::RWRegister<u32>,
    #[doc = "Output channel configure register"]
    pub CHCFG_20: crate::RWRegister<u32>,
    #[doc = "Output channel configure register"]
    pub CHCFG_21: crate::RWRegister<u32>,
    #[doc = "Output channel configure register"]
    pub CHCFG_22: crate::RWRegister<u32>,
    #[doc = "Output channel configure register"]
    pub CHCFG_23: crate::RWRegister<u32>,
    _reserved1: [u8; 0x10],
    #[doc = "Global control register"]
    pub GCR: crate::RWRegister<u32>,
    #[doc = "Shadow register control register"]
    pub SHCR: crate::RWRegister<u32>,
    _reserved2: [u8; 0x08],
    #[doc = "Capture rising edge register"]
    pub CAPPOS_0: crate::RWRegister<u32>,
    #[doc = "Capture rising edge register"]
    pub CAPPOS_1: crate::RWRegister<u32>,
    #[doc = "Capture rising edge register"]
    pub CAPPOS_2: crate::RWRegister<u32>,
    #[doc = "Capture rising edge register"]
    pub CAPPOS_3: crate::RWRegister<u32>,
    #[doc = "Capture rising edge register"]
    pub CAPPOS_4: crate::RWRegister<u32>,
    #[doc = "Capture rising edge register"]
    pub CAPPOS_5: crate::RWRegister<u32>,
    #[doc = "Capture rising edge register"]
    pub CAPPOS_6: crate::RWRegister<u32>,
    #[doc = "Capture rising edge register"]
    pub CAPPOS_7: crate::RWRegister<u32>,
    #[doc = "Capture rising edge register"]
    pub CAPPOS_8: crate::RWRegister<u32>,
    #[doc = "Capture rising edge register"]
    pub CAPPOS_9: crate::RWRegister<u32>,
    #[doc = "Capture rising edge register"]
    pub CAPPOS_10: crate::RWRegister<u32>,
    #[doc = "Capture rising edge register"]
    pub CAPPOS_11: crate::RWRegister<u32>,
    #[doc = "Capture rising edge register"]
    pub CAPPOS_12: crate::RWRegister<u32>,
    #[doc = "Capture rising edge register"]
    pub CAPPOS_13: crate::RWRegister<u32>,
    #[doc = "Capture rising edge register"]
    pub CAPPOS_14: crate::RWRegister<u32>,
    #[doc = "Capture rising edge register"]
    pub CAPPOS_15: crate::RWRegister<u32>,
    #[doc = "Capture rising edge register"]
    pub CAPPOS_16: crate::RWRegister<u32>,
    #[doc = "Capture rising edge register"]
    pub CAPPOS_17: crate::RWRegister<u32>,
    #[doc = "Capture rising edge register"]
    pub CAPPOS_18: crate::RWRegister<u32>,
    #[doc = "Capture rising edge register"]
    pub CAPPOS_19: crate::RWRegister<u32>,
    #[doc = "Capture rising edge register"]
    pub CAPPOS_20: crate::RWRegister<u32>,
    #[doc = "Capture rising edge register"]
    pub CAPPOS_21: crate::RWRegister<u32>,
    #[doc = "Capture rising edge register"]
    pub CAPPOS_22: crate::RWRegister<u32>,
    #[doc = "Capture rising edge register"]
    pub CAPPOS_23: crate::RWRegister<u32>,
    _reserved3: [u8; 0x10],
    #[doc = "Counter"]
    pub CNT: crate::RWRegister<u32>,
    _reserved4: [u8; 0x0c],
    #[doc = "Capture falling edge register"]
    pub CAPNEG_0: crate::RWRegister<u32>,
    #[doc = "Capture falling edge register"]
    pub CAPNEG_1: crate::RWRegister<u32>,
    #[doc = "Capture falling edge register"]
    pub CAPNEG_2: crate::RWRegister<u32>,
    #[doc = "Capture falling edge register"]
    pub CAPNEG_3: crate::RWRegister<u32>,
    #[doc = "Capture falling edge register"]
    pub CAPNEG_4: crate::RWRegister<u32>,
    #[doc = "Capture falling edge register"]
    pub CAPNEG_5: crate::RWRegister<u32>,
    #[doc = "Capture falling edge register"]
    pub CAPNEG_6: crate::RWRegister<u32>,
    #[doc = "Capture falling edge register"]
    pub CAPNEG_7: crate::RWRegister<u32>,
    #[doc = "Capture falling edge register"]
    pub CAPNEG_8: crate::RWRegister<u32>,
    #[doc = "Capture falling edge register"]
    pub CAPNEG_9: crate::RWRegister<u32>,
    #[doc = "Capture falling edge register"]
    pub CAPNEG_10: crate::RWRegister<u32>,
    #[doc = "Capture falling edge register"]
    pub CAPNEG_11: crate::RWRegister<u32>,
    #[doc = "Capture falling edge register"]
    pub CAPNEG_12: crate::RWRegister<u32>,
    #[doc = "Capture falling edge register"]
    pub CAPNEG_13: crate::RWRegister<u32>,
    #[doc = "Capture falling edge register"]
    pub CAPNEG_14: crate::RWRegister<u32>,
    #[doc = "Capture falling edge register"]
    pub CAPNEG_15: crate::RWRegister<u32>,
    #[doc = "Capture falling edge register"]
    pub CAPNEG_16: crate::RWRegister<u32>,
    #[doc = "Capture falling edge register"]
    pub CAPNEG_17: crate::RWRegister<u32>,
    #[doc = "Capture falling edge register"]
    pub CAPNEG_18: crate::RWRegister<u32>,
    #[doc = "Capture falling edge register"]
    pub CAPNEG_19: crate::RWRegister<u32>,
    #[doc = "Capture falling edge register"]
    pub CAPNEG_20: crate::RWRegister<u32>,
    #[doc = "Capture falling edge register"]
    pub CAPNEG_21: crate::RWRegister<u32>,
    #[doc = "Capture falling edge register"]
    pub CAPNEG_22: crate::RWRegister<u32>,
    #[doc = "Capture falling edge register"]
    pub CAPNEG_23: crate::RWRegister<u32>,
    _reserved5: [u8; 0x10],
    #[doc = "Counter copy"]
    pub CNTCOPY: crate::RWRegister<u32>,
    _reserved6: [u8; 0x0c],
    #[doc = "PWM channel configure register"]
    pub PWMCFG_0: crate::RWRegister<u32>,
    #[doc = "PWM channel configure register"]
    pub PWMCFG_1: crate::RWRegister<u32>,
    #[doc = "PWM channel configure register"]
    pub PWMCFG_2: crate::RWRegister<u32>,
    #[doc = "PWM channel configure register"]
    pub PWMCFG_3: crate::RWRegister<u32>,
    #[doc = "PWM channel configure register"]
    pub PWMCFG_4: crate::RWRegister<u32>,
    #[doc = "PWM channel configure register"]
    pub PWMCFG_5: crate::RWRegister<u32>,
    #[doc = "PWM channel configure register"]
    pub PWMCFG_6: crate::RWRegister<u32>,
    #[doc = "PWM channel configure register"]
    pub PWMCFG_7: crate::RWRegister<u32>,
    #[doc = "Status register"]
    pub SR: crate::RWRegister<u32>,
    #[doc = "Interrupt request enable register"]
    pub IRQEN: crate::RWRegister<u32>,
    _reserved7: [u8; 0x04],
    #[doc = "DMA request enable register"]
    pub DMAEN: crate::RWRegister<u32>,
    #[doc = "Comparator configure register"]
    pub CMPCFG_CMPCFG0: crate::RWRegister<u32>,
    #[doc = "Comparator configure register"]
    pub CMPCFG_1: crate::RWRegister<u32>,
    #[doc = "Comparator configure register"]
    pub CMPCFG_2: crate::RWRegister<u32>,
    #[doc = "Comparator configure register"]
    pub CMPCFG_3: crate::RWRegister<u32>,
    #[doc = "Comparator configure register"]
    pub CMPCFG_4: crate::RWRegister<u32>,
    #[doc = "Comparator configure register"]
    pub CMPCFG_5: crate::RWRegister<u32>,
    #[doc = "Comparator configure register"]
    pub CMPCFG_6: crate::RWRegister<u32>,
    #[doc = "Comparator configure register"]
    pub CMPCFG_7: crate::RWRegister<u32>,
    #[doc = "Comparator configure register"]
    pub CMPCFG_8: crate::RWRegister<u32>,
    #[doc = "Comparator configure register"]
    pub CMPCFG_9: crate::RWRegister<u32>,
    #[doc = "Comparator configure register"]
    pub CMPCFG_10: crate::RWRegister<u32>,
    #[doc = "Comparator configure register"]
    pub CMPCFG_11: crate::RWRegister<u32>,
    #[doc = "Comparator configure register"]
    pub CMPCFG_12: crate::RWRegister<u32>,
    #[doc = "Comparator configure register"]
    pub CMPCFG_13: crate::RWRegister<u32>,
    #[doc = "Comparator configure register"]
    pub CMPCFG_14: crate::RWRegister<u32>,
    #[doc = "Comparator configure register"]
    pub CMPCFG_15: crate::RWRegister<u32>,
    #[doc = "Comparator configure register"]
    pub CMPCFG_16: crate::RWRegister<u32>,
    #[doc = "Comparator configure register"]
    pub CMPCFG_17: crate::RWRegister<u32>,
    #[doc = "Comparator configure register"]
    pub CMPCFG_18: crate::RWRegister<u32>,
    #[doc = "Comparator configure register"]
    pub CMPCFG_19: crate::RWRegister<u32>,
    #[doc = "Comparator configure register"]
    pub CMPCFG_20: crate::RWRegister<u32>,
    #[doc = "Comparator configure register"]
    pub CMPCFG_21: crate::RWRegister<u32>,
    #[doc = "Comparator configure register"]
    pub CMPCFG_22: crate::RWRegister<u32>,
    #[doc = "Comparator configure register"]
    pub CMPCFG_23: crate::RWRegister<u32>,
}
#[doc = "Shadow registers unlock register"]
pub mod UNLK {
    #[doc = "write 0xB0382607 to unlock the shadow registers of register offset from 0x04 to 0x78, otherwise the shadow registers can not be written."]
    pub mod SHUNLK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Counter start register"]
pub mod STA {
    #[doc = "pwm timer counter start value sta/rld will be loaded from shadow register to work register at main counter reload time, or software write unlk.shunlk"]
    pub mod STA {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "pwm timer counter extended start point, should back to this value after reach xrld"]
    pub mod XSTA {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Counter reload register"]
pub mod RLD {
    #[doc = "pwm timer counter reload value"]
    pub mod RLD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "timeout counter extended reload point, counter will reload to xsta after reach this point"]
    pub mod XRLD {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Comparator register"]
pub mod CMP_0 {
    #[doc = "jitter counter compare value"]
    pub mod CMPJIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "half clock counter compare value"]
    pub mod CMPHLF {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock counter compare value, the compare output is 0 at default, set to 1 when compare value meet, and clr to 0 when timer reload. Software can invert the output by setting chan_cfg.out_polarity."]
    pub mod CMP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "extended counter compare value"]
    pub mod XCMP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Comparator register"]
pub mod CMP_1 {
    #[doc = "jitter counter compare value"]
    pub mod CMPJIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "half clock counter compare value"]
    pub mod CMPHLF {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock counter compare value, the compare output is 0 at default, set to 1 when compare value meet, and clr to 0 when timer reload. Software can invert the output by setting chan_cfg.out_polarity."]
    pub mod CMP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "extended counter compare value"]
    pub mod XCMP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Comparator register"]
pub mod CMP_2 {
    #[doc = "jitter counter compare value"]
    pub mod CMPJIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "half clock counter compare value"]
    pub mod CMPHLF {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock counter compare value, the compare output is 0 at default, set to 1 when compare value meet, and clr to 0 when timer reload. Software can invert the output by setting chan_cfg.out_polarity."]
    pub mod CMP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "extended counter compare value"]
    pub mod XCMP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Comparator register"]
pub mod CMP_3 {
    #[doc = "jitter counter compare value"]
    pub mod CMPJIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "half clock counter compare value"]
    pub mod CMPHLF {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock counter compare value, the compare output is 0 at default, set to 1 when compare value meet, and clr to 0 when timer reload. Software can invert the output by setting chan_cfg.out_polarity."]
    pub mod CMP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "extended counter compare value"]
    pub mod XCMP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Comparator register"]
pub mod CMP_4 {
    #[doc = "jitter counter compare value"]
    pub mod CMPJIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "half clock counter compare value"]
    pub mod CMPHLF {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock counter compare value, the compare output is 0 at default, set to 1 when compare value meet, and clr to 0 when timer reload. Software can invert the output by setting chan_cfg.out_polarity."]
    pub mod CMP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "extended counter compare value"]
    pub mod XCMP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Comparator register"]
pub mod CMP_5 {
    #[doc = "jitter counter compare value"]
    pub mod CMPJIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "half clock counter compare value"]
    pub mod CMPHLF {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock counter compare value, the compare output is 0 at default, set to 1 when compare value meet, and clr to 0 when timer reload. Software can invert the output by setting chan_cfg.out_polarity."]
    pub mod CMP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "extended counter compare value"]
    pub mod XCMP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Comparator register"]
pub mod CMP_6 {
    #[doc = "jitter counter compare value"]
    pub mod CMPJIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "half clock counter compare value"]
    pub mod CMPHLF {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock counter compare value, the compare output is 0 at default, set to 1 when compare value meet, and clr to 0 when timer reload. Software can invert the output by setting chan_cfg.out_polarity."]
    pub mod CMP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "extended counter compare value"]
    pub mod XCMP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Comparator register"]
pub mod CMP_7 {
    #[doc = "jitter counter compare value"]
    pub mod CMPJIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "half clock counter compare value"]
    pub mod CMPHLF {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock counter compare value, the compare output is 0 at default, set to 1 when compare value meet, and clr to 0 when timer reload. Software can invert the output by setting chan_cfg.out_polarity."]
    pub mod CMP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "extended counter compare value"]
    pub mod XCMP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Comparator register"]
pub mod CMP_8 {
    #[doc = "jitter counter compare value"]
    pub mod CMPJIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "half clock counter compare value"]
    pub mod CMPHLF {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock counter compare value, the compare output is 0 at default, set to 1 when compare value meet, and clr to 0 when timer reload. Software can invert the output by setting chan_cfg.out_polarity."]
    pub mod CMP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "extended counter compare value"]
    pub mod XCMP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Comparator register"]
pub mod CMP_9 {
    #[doc = "jitter counter compare value"]
    pub mod CMPJIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "half clock counter compare value"]
    pub mod CMPHLF {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock counter compare value, the compare output is 0 at default, set to 1 when compare value meet, and clr to 0 when timer reload. Software can invert the output by setting chan_cfg.out_polarity."]
    pub mod CMP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "extended counter compare value"]
    pub mod XCMP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Comparator register"]
pub mod CMP_10 {
    #[doc = "jitter counter compare value"]
    pub mod CMPJIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "half clock counter compare value"]
    pub mod CMPHLF {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock counter compare value, the compare output is 0 at default, set to 1 when compare value meet, and clr to 0 when timer reload. Software can invert the output by setting chan_cfg.out_polarity."]
    pub mod CMP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "extended counter compare value"]
    pub mod XCMP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Comparator register"]
pub mod CMP_11 {
    #[doc = "jitter counter compare value"]
    pub mod CMPJIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "half clock counter compare value"]
    pub mod CMPHLF {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock counter compare value, the compare output is 0 at default, set to 1 when compare value meet, and clr to 0 when timer reload. Software can invert the output by setting chan_cfg.out_polarity."]
    pub mod CMP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "extended counter compare value"]
    pub mod XCMP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Comparator register"]
pub mod CMP_12 {
    #[doc = "jitter counter compare value"]
    pub mod CMPJIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "half clock counter compare value"]
    pub mod CMPHLF {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock counter compare value, the compare output is 0 at default, set to 1 when compare value meet, and clr to 0 when timer reload. Software can invert the output by setting chan_cfg.out_polarity."]
    pub mod CMP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "extended counter compare value"]
    pub mod XCMP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Comparator register"]
pub mod CMP_13 {
    #[doc = "jitter counter compare value"]
    pub mod CMPJIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "half clock counter compare value"]
    pub mod CMPHLF {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock counter compare value, the compare output is 0 at default, set to 1 when compare value meet, and clr to 0 when timer reload. Software can invert the output by setting chan_cfg.out_polarity."]
    pub mod CMP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "extended counter compare value"]
    pub mod XCMP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Comparator register"]
pub mod CMP_14 {
    #[doc = "jitter counter compare value"]
    pub mod CMPJIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "half clock counter compare value"]
    pub mod CMPHLF {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock counter compare value, the compare output is 0 at default, set to 1 when compare value meet, and clr to 0 when timer reload. Software can invert the output by setting chan_cfg.out_polarity."]
    pub mod CMP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "extended counter compare value"]
    pub mod XCMP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Comparator register"]
pub mod CMP_15 {
    #[doc = "jitter counter compare value"]
    pub mod CMPJIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "half clock counter compare value"]
    pub mod CMPHLF {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock counter compare value, the compare output is 0 at default, set to 1 when compare value meet, and clr to 0 when timer reload. Software can invert the output by setting chan_cfg.out_polarity."]
    pub mod CMP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "extended counter compare value"]
    pub mod XCMP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Comparator register"]
pub mod CMP_16 {
    #[doc = "jitter counter compare value"]
    pub mod CMPJIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "half clock counter compare value"]
    pub mod CMPHLF {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock counter compare value, the compare output is 0 at default, set to 1 when compare value meet, and clr to 0 when timer reload. Software can invert the output by setting chan_cfg.out_polarity."]
    pub mod CMP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "extended counter compare value"]
    pub mod XCMP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Comparator register"]
pub mod CMP_17 {
    #[doc = "jitter counter compare value"]
    pub mod CMPJIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "half clock counter compare value"]
    pub mod CMPHLF {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock counter compare value, the compare output is 0 at default, set to 1 when compare value meet, and clr to 0 when timer reload. Software can invert the output by setting chan_cfg.out_polarity."]
    pub mod CMP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "extended counter compare value"]
    pub mod XCMP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Comparator register"]
pub mod CMP_18 {
    #[doc = "jitter counter compare value"]
    pub mod CMPJIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "half clock counter compare value"]
    pub mod CMPHLF {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock counter compare value, the compare output is 0 at default, set to 1 when compare value meet, and clr to 0 when timer reload. Software can invert the output by setting chan_cfg.out_polarity."]
    pub mod CMP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "extended counter compare value"]
    pub mod XCMP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Comparator register"]
pub mod CMP_19 {
    #[doc = "jitter counter compare value"]
    pub mod CMPJIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "half clock counter compare value"]
    pub mod CMPHLF {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock counter compare value, the compare output is 0 at default, set to 1 when compare value meet, and clr to 0 when timer reload. Software can invert the output by setting chan_cfg.out_polarity."]
    pub mod CMP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "extended counter compare value"]
    pub mod XCMP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Comparator register"]
pub mod CMP_20 {
    #[doc = "jitter counter compare value"]
    pub mod CMPJIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "half clock counter compare value"]
    pub mod CMPHLF {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock counter compare value, the compare output is 0 at default, set to 1 when compare value meet, and clr to 0 when timer reload. Software can invert the output by setting chan_cfg.out_polarity."]
    pub mod CMP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "extended counter compare value"]
    pub mod XCMP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Comparator register"]
pub mod CMP_21 {
    #[doc = "jitter counter compare value"]
    pub mod CMPJIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "half clock counter compare value"]
    pub mod CMPHLF {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock counter compare value, the compare output is 0 at default, set to 1 when compare value meet, and clr to 0 when timer reload. Software can invert the output by setting chan_cfg.out_polarity."]
    pub mod CMP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "extended counter compare value"]
    pub mod XCMP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Comparator register"]
pub mod CMP_22 {
    #[doc = "jitter counter compare value"]
    pub mod CMPJIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "half clock counter compare value"]
    pub mod CMPHLF {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock counter compare value, the compare output is 0 at default, set to 1 when compare value meet, and clr to 0 when timer reload. Software can invert the output by setting chan_cfg.out_polarity."]
    pub mod CMP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "extended counter compare value"]
    pub mod XCMP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Comparator register"]
pub mod CMP_23 {
    #[doc = "jitter counter compare value"]
    pub mod CMPJIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "half clock counter compare value"]
    pub mod CMPHLF {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock counter compare value, the compare output is 0 at default, set to 1 when compare value meet, and clr to 0 when timer reload. Software can invert the output by setting chan_cfg.out_polarity."]
    pub mod CMP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "extended counter compare value"]
    pub mod XCMP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Force output mode register"]
pub mod FRCMD {
    #[doc = "2bit for each PWM output channel (0~7); 00: force output 0 01: force output 1 10: output highz 11: no force"]
    pub mod FRCMD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Shadow registers lock register"]
pub mod SHLK {
    #[doc = "write 1 to lock all shawdow register, wirte access is not permitted"]
    pub mod SHLK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Output channel configure register"]
pub mod CHCFG_0 {
    #[doc = "output polarity, set to 1 will invert the output"]
    pub mod OUTPOL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "assign the first comparator for this output channel"]
    pub mod CMPSELBEG {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "assign the last comparator for this output channel"]
    pub mod CMPSELEND {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Output channel configure register"]
pub mod CHCFG_1 {
    #[doc = "output polarity, set to 1 will invert the output"]
    pub mod OUTPOL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "assign the first comparator for this output channel"]
    pub mod CMPSELBEG {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "assign the last comparator for this output channel"]
    pub mod CMPSELEND {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Output channel configure register"]
pub mod CHCFG_2 {
    #[doc = "output polarity, set to 1 will invert the output"]
    pub mod OUTPOL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "assign the first comparator for this output channel"]
    pub mod CMPSELBEG {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "assign the last comparator for this output channel"]
    pub mod CMPSELEND {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Output channel configure register"]
pub mod CHCFG_3 {
    #[doc = "output polarity, set to 1 will invert the output"]
    pub mod OUTPOL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "assign the first comparator for this output channel"]
    pub mod CMPSELBEG {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "assign the last comparator for this output channel"]
    pub mod CMPSELEND {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Output channel configure register"]
pub mod CHCFG_4 {
    #[doc = "output polarity, set to 1 will invert the output"]
    pub mod OUTPOL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "assign the first comparator for this output channel"]
    pub mod CMPSELBEG {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "assign the last comparator for this output channel"]
    pub mod CMPSELEND {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Output channel configure register"]
pub mod CHCFG_5 {
    #[doc = "output polarity, set to 1 will invert the output"]
    pub mod OUTPOL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "assign the first comparator for this output channel"]
    pub mod CMPSELBEG {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "assign the last comparator for this output channel"]
    pub mod CMPSELEND {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Output channel configure register"]
pub mod CHCFG_6 {
    #[doc = "output polarity, set to 1 will invert the output"]
    pub mod OUTPOL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "assign the first comparator for this output channel"]
    pub mod CMPSELBEG {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "assign the last comparator for this output channel"]
    pub mod CMPSELEND {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Output channel configure register"]
pub mod CHCFG_7 {
    #[doc = "output polarity, set to 1 will invert the output"]
    pub mod OUTPOL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "assign the first comparator for this output channel"]
    pub mod CMPSELBEG {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "assign the last comparator for this output channel"]
    pub mod CMPSELEND {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Output channel configure register"]
pub mod CHCFG_8 {
    #[doc = "output polarity, set to 1 will invert the output"]
    pub mod OUTPOL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "assign the first comparator for this output channel"]
    pub mod CMPSELBEG {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "assign the last comparator for this output channel"]
    pub mod CMPSELEND {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Output channel configure register"]
pub mod CHCFG_9 {
    #[doc = "output polarity, set to 1 will invert the output"]
    pub mod OUTPOL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "assign the first comparator for this output channel"]
    pub mod CMPSELBEG {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "assign the last comparator for this output channel"]
    pub mod CMPSELEND {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Output channel configure register"]
pub mod CHCFG_10 {
    #[doc = "output polarity, set to 1 will invert the output"]
    pub mod OUTPOL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "assign the first comparator for this output channel"]
    pub mod CMPSELBEG {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "assign the last comparator for this output channel"]
    pub mod CMPSELEND {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Output channel configure register"]
pub mod CHCFG_11 {
    #[doc = "output polarity, set to 1 will invert the output"]
    pub mod OUTPOL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "assign the first comparator for this output channel"]
    pub mod CMPSELBEG {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "assign the last comparator for this output channel"]
    pub mod CMPSELEND {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Output channel configure register"]
pub mod CHCFG_12 {
    #[doc = "output polarity, set to 1 will invert the output"]
    pub mod OUTPOL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "assign the first comparator for this output channel"]
    pub mod CMPSELBEG {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "assign the last comparator for this output channel"]
    pub mod CMPSELEND {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Output channel configure register"]
pub mod CHCFG_13 {
    #[doc = "output polarity, set to 1 will invert the output"]
    pub mod OUTPOL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "assign the first comparator for this output channel"]
    pub mod CMPSELBEG {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "assign the last comparator for this output channel"]
    pub mod CMPSELEND {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Output channel configure register"]
pub mod CHCFG_14 {
    #[doc = "output polarity, set to 1 will invert the output"]
    pub mod OUTPOL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "assign the first comparator for this output channel"]
    pub mod CMPSELBEG {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "assign the last comparator for this output channel"]
    pub mod CMPSELEND {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Output channel configure register"]
pub mod CHCFG_15 {
    #[doc = "output polarity, set to 1 will invert the output"]
    pub mod OUTPOL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "assign the first comparator for this output channel"]
    pub mod CMPSELBEG {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "assign the last comparator for this output channel"]
    pub mod CMPSELEND {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Output channel configure register"]
pub mod CHCFG_16 {
    #[doc = "output polarity, set to 1 will invert the output"]
    pub mod OUTPOL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "assign the first comparator for this output channel"]
    pub mod CMPSELBEG {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "assign the last comparator for this output channel"]
    pub mod CMPSELEND {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Output channel configure register"]
pub mod CHCFG_17 {
    #[doc = "output polarity, set to 1 will invert the output"]
    pub mod OUTPOL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "assign the first comparator for this output channel"]
    pub mod CMPSELBEG {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "assign the last comparator for this output channel"]
    pub mod CMPSELEND {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Output channel configure register"]
pub mod CHCFG_18 {
    #[doc = "output polarity, set to 1 will invert the output"]
    pub mod OUTPOL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "assign the first comparator for this output channel"]
    pub mod CMPSELBEG {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "assign the last comparator for this output channel"]
    pub mod CMPSELEND {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Output channel configure register"]
pub mod CHCFG_19 {
    #[doc = "output polarity, set to 1 will invert the output"]
    pub mod OUTPOL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "assign the first comparator for this output channel"]
    pub mod CMPSELBEG {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "assign the last comparator for this output channel"]
    pub mod CMPSELEND {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Output channel configure register"]
pub mod CHCFG_20 {
    #[doc = "output polarity, set to 1 will invert the output"]
    pub mod OUTPOL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "assign the first comparator for this output channel"]
    pub mod CMPSELBEG {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "assign the last comparator for this output channel"]
    pub mod CMPSELEND {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Output channel configure register"]
pub mod CHCFG_21 {
    #[doc = "output polarity, set to 1 will invert the output"]
    pub mod OUTPOL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "assign the first comparator for this output channel"]
    pub mod CMPSELBEG {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "assign the last comparator for this output channel"]
    pub mod CMPSELEND {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Output channel configure register"]
pub mod CHCFG_22 {
    #[doc = "output polarity, set to 1 will invert the output"]
    pub mod OUTPOL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "assign the first comparator for this output channel"]
    pub mod CMPSELBEG {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "assign the last comparator for this output channel"]
    pub mod CMPSELEND {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Output channel configure register"]
pub mod CHCFG_23 {
    #[doc = "output polarity, set to 1 will invert the output"]
    pub mod OUTPOL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "assign the first comparator for this output channel"]
    pub mod CMPSELBEG {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "assign the last comparator for this output channel"]
    pub mod CMPSELEND {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Global control register"]
pub mod GCR {
    #[doc = "1- write 1 to enable software force, if the frcsrcsel is set to 0, force will take effect"]
    pub mod SWFRC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit field select the force effective time 00: force immediately 01: force at main counter reload time 10: force at FRCSYNCI 11: no force"]
    pub mod FRCTIME {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- pwm timer extended counter (xcnt) reset to extended reload value (xrld) by synci is enabled"]
    pub mod XRLDSYNCEN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Write 1 to clear the fault condition. The output will recover if FAULTRECTIME is set to 2b'11. User should write 1 to this bit after the active FAULT signal de-assert and before it re-assert again."]
    pub mod FAULTCLR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- enable the pwm timer counter 0- stop the pwm timer counter"]
    pub mod CEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- pwm timer counter reset to reload value (rld) by synci is enabled"]
    pub mod RLDSYNCEN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "external fault polarity 1-active low 0-active high"]
    pub mod FAULTEXPOL {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- enable the external fault input 0"]
    pub mod FAULTE0EN {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- enable the external fault input 1"]
    pub mod FAULTE1EN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Selec one of the 24 comparators as fault output recover trigger."]
    pub mod FAULTRECHWSEL {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When hardware load is selected as output fault recover trigger and the selected channel is capture mode. This bit assign its effective edge of fault recover trigger. 1- Falling edge 0- Rising edge"]
    pub mod FAULTRECEDG {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield select one of the comparators as hardware event time to load comparator shadow registers"]
    pub mod CMPSHDWSEL {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When hardware event is selected as shawdow register effective time and the select comparator is configured as input capture mode. This bit assign its which edge is used as shadow register hardware load event. 1- Falling edge 0- Rising edge"]
    pub mod HWSHDWEDG {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "polarity of input pwm_force, 1- active low 0- active high"]
    pub mod FRCPOL {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- enable debug mode output protection"]
    pub mod DEBUGFAULT {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- enable the internal fault input 0"]
    pub mod FAULTI0EN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- enable the internal fault input 1"]
    pub mod FAULTI1EN {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- enable the internal fault input 2"]
    pub mod FAULTI2EN {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- enable the internal fault input 3"]
    pub mod FAULTI3EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Shadow register control register"]
pub mod SHCR {
    #[doc = "1- enable shadow registers lock feature, 0- disable shadow registers lock, shlk bit will always be 0"]
    pub mod SHLKEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield select when the counter related shadow registers (STA and RLD) will be loaded to its work register 00: after software set shlk bit of shlk register 01: immediately after the register being modified 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after SHSYNCI assert"]
    pub mod CNTSHDWUPT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield select one of the comparators as hardware event time to load the counter related shadow registers (STA and RLD)"]
    pub mod CNTSHDWSEL {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield select one of the comparators as hardware event time to load FRCMD shadow registers"]
    pub mod FRCSHDWSEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture rising edge register"]
pub mod CAPPOS_0 {
    #[doc = "counter value captured at input posedge"]
    pub mod CAPPOS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture rising edge register"]
pub mod CAPPOS_1 {
    #[doc = "counter value captured at input posedge"]
    pub mod CAPPOS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture rising edge register"]
pub mod CAPPOS_2 {
    #[doc = "counter value captured at input posedge"]
    pub mod CAPPOS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture rising edge register"]
pub mod CAPPOS_3 {
    #[doc = "counter value captured at input posedge"]
    pub mod CAPPOS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture rising edge register"]
pub mod CAPPOS_4 {
    #[doc = "counter value captured at input posedge"]
    pub mod CAPPOS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture rising edge register"]
pub mod CAPPOS_5 {
    #[doc = "counter value captured at input posedge"]
    pub mod CAPPOS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture rising edge register"]
pub mod CAPPOS_6 {
    #[doc = "counter value captured at input posedge"]
    pub mod CAPPOS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture rising edge register"]
pub mod CAPPOS_7 {
    #[doc = "counter value captured at input posedge"]
    pub mod CAPPOS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture rising edge register"]
pub mod CAPPOS_8 {
    #[doc = "counter value captured at input posedge"]
    pub mod CAPPOS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture rising edge register"]
pub mod CAPPOS_9 {
    #[doc = "counter value captured at input posedge"]
    pub mod CAPPOS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture rising edge register"]
pub mod CAPPOS_10 {
    #[doc = "counter value captured at input posedge"]
    pub mod CAPPOS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture rising edge register"]
pub mod CAPPOS_11 {
    #[doc = "counter value captured at input posedge"]
    pub mod CAPPOS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture rising edge register"]
pub mod CAPPOS_12 {
    #[doc = "counter value captured at input posedge"]
    pub mod CAPPOS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture rising edge register"]
pub mod CAPPOS_13 {
    #[doc = "counter value captured at input posedge"]
    pub mod CAPPOS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture rising edge register"]
pub mod CAPPOS_14 {
    #[doc = "counter value captured at input posedge"]
    pub mod CAPPOS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture rising edge register"]
pub mod CAPPOS_15 {
    #[doc = "counter value captured at input posedge"]
    pub mod CAPPOS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture rising edge register"]
pub mod CAPPOS_16 {
    #[doc = "counter value captured at input posedge"]
    pub mod CAPPOS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture rising edge register"]
pub mod CAPPOS_17 {
    #[doc = "counter value captured at input posedge"]
    pub mod CAPPOS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture rising edge register"]
pub mod CAPPOS_18 {
    #[doc = "counter value captured at input posedge"]
    pub mod CAPPOS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture rising edge register"]
pub mod CAPPOS_19 {
    #[doc = "counter value captured at input posedge"]
    pub mod CAPPOS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture rising edge register"]
pub mod CAPPOS_20 {
    #[doc = "counter value captured at input posedge"]
    pub mod CAPPOS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture rising edge register"]
pub mod CAPPOS_21 {
    #[doc = "counter value captured at input posedge"]
    pub mod CAPPOS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture rising edge register"]
pub mod CAPPOS_22 {
    #[doc = "counter value captured at input posedge"]
    pub mod CAPPOS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture rising edge register"]
pub mod CAPPOS_23 {
    #[doc = "counter value captured at input posedge"]
    pub mod CAPPOS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Counter"]
pub mod CNT {
    #[doc = "current clock counter value"]
    pub mod CNT {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "current extended counter value"]
    pub mod XCNT {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture falling edge register"]
pub mod CAPNEG_0 {
    #[doc = "counter value captured at input signal falling edge"]
    pub mod CAPNEG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture falling edge register"]
pub mod CAPNEG_1 {
    #[doc = "counter value captured at input signal falling edge"]
    pub mod CAPNEG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture falling edge register"]
pub mod CAPNEG_2 {
    #[doc = "counter value captured at input signal falling edge"]
    pub mod CAPNEG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture falling edge register"]
pub mod CAPNEG_3 {
    #[doc = "counter value captured at input signal falling edge"]
    pub mod CAPNEG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture falling edge register"]
pub mod CAPNEG_4 {
    #[doc = "counter value captured at input signal falling edge"]
    pub mod CAPNEG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture falling edge register"]
pub mod CAPNEG_5 {
    #[doc = "counter value captured at input signal falling edge"]
    pub mod CAPNEG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture falling edge register"]
pub mod CAPNEG_6 {
    #[doc = "counter value captured at input signal falling edge"]
    pub mod CAPNEG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture falling edge register"]
pub mod CAPNEG_7 {
    #[doc = "counter value captured at input signal falling edge"]
    pub mod CAPNEG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture falling edge register"]
pub mod CAPNEG_8 {
    #[doc = "counter value captured at input signal falling edge"]
    pub mod CAPNEG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture falling edge register"]
pub mod CAPNEG_9 {
    #[doc = "counter value captured at input signal falling edge"]
    pub mod CAPNEG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture falling edge register"]
pub mod CAPNEG_10 {
    #[doc = "counter value captured at input signal falling edge"]
    pub mod CAPNEG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture falling edge register"]
pub mod CAPNEG_11 {
    #[doc = "counter value captured at input signal falling edge"]
    pub mod CAPNEG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture falling edge register"]
pub mod CAPNEG_12 {
    #[doc = "counter value captured at input signal falling edge"]
    pub mod CAPNEG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture falling edge register"]
pub mod CAPNEG_13 {
    #[doc = "counter value captured at input signal falling edge"]
    pub mod CAPNEG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture falling edge register"]
pub mod CAPNEG_14 {
    #[doc = "counter value captured at input signal falling edge"]
    pub mod CAPNEG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture falling edge register"]
pub mod CAPNEG_15 {
    #[doc = "counter value captured at input signal falling edge"]
    pub mod CAPNEG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture falling edge register"]
pub mod CAPNEG_16 {
    #[doc = "counter value captured at input signal falling edge"]
    pub mod CAPNEG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture falling edge register"]
pub mod CAPNEG_17 {
    #[doc = "counter value captured at input signal falling edge"]
    pub mod CAPNEG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture falling edge register"]
pub mod CAPNEG_18 {
    #[doc = "counter value captured at input signal falling edge"]
    pub mod CAPNEG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture falling edge register"]
pub mod CAPNEG_19 {
    #[doc = "counter value captured at input signal falling edge"]
    pub mod CAPNEG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture falling edge register"]
pub mod CAPNEG_20 {
    #[doc = "counter value captured at input signal falling edge"]
    pub mod CAPNEG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture falling edge register"]
pub mod CAPNEG_21 {
    #[doc = "counter value captured at input signal falling edge"]
    pub mod CAPNEG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture falling edge register"]
pub mod CAPNEG_22 {
    #[doc = "counter value captured at input signal falling edge"]
    pub mod CAPNEG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture falling edge register"]
pub mod CAPNEG_23 {
    #[doc = "counter value captured at input signal falling edge"]
    pub mod CAPNEG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Counter copy"]
pub mod CNTCOPY {
    #[doc = "current clock counter value"]
    pub mod CNT {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "current extended counter value"]
    pub mod XCNT {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PWM channel configure register"]
pub mod PWMCFG_0 {
    #[doc = "This bitfield define the PWM pair deadarea length. The unit is 0.5 cycle. The minimum length of deadarea is 1 cycle. Note: user should configure pair bit and this bitfield before PWM output is enabled."]
    pub mod DEADAREA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- PWM output is in pair mode. Note the two PWM outputs need to be both set to pair mode. 0- PWM output is in indepandent mode."]
    pub mod PAIR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Select sources for force output 0- force output is enabled when FRCI assert 1- force output is enabled by software write swfrc to 1"]
    pub mod FRCSRCSEL {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield select when to recover PWM output after fault condition removed. 00: immediately 01: after pwm timer counter reload time 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after software write faultclr bit in GCR register"]
    pub mod FAULTRECTIME {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield defines the PWM output status when fault condition happen 00: force output 0 01: force output 1 1x: output highz"]
    pub mod FAULTMODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield select when the FRCMD shadow register will be loaded to its work register 00: after software set shlk bit of shlk register 01: immediately after the register being modified 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after SHSYNCI assert"]
    pub mod FRCSHDWUPT {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PWM output enable 1- output is enabled 0- output is disabled"]
    pub mod OEN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PWM channel configure register"]
pub mod PWMCFG_1 {
    #[doc = "This bitfield define the PWM pair deadarea length. The unit is 0.5 cycle. The minimum length of deadarea is 1 cycle. Note: user should configure pair bit and this bitfield before PWM output is enabled."]
    pub mod DEADAREA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- PWM output is in pair mode. Note the two PWM outputs need to be both set to pair mode. 0- PWM output is in indepandent mode."]
    pub mod PAIR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Select sources for force output 0- force output is enabled when FRCI assert 1- force output is enabled by software write swfrc to 1"]
    pub mod FRCSRCSEL {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield select when to recover PWM output after fault condition removed. 00: immediately 01: after pwm timer counter reload time 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after software write faultclr bit in GCR register"]
    pub mod FAULTRECTIME {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield defines the PWM output status when fault condition happen 00: force output 0 01: force output 1 1x: output highz"]
    pub mod FAULTMODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield select when the FRCMD shadow register will be loaded to its work register 00: after software set shlk bit of shlk register 01: immediately after the register being modified 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after SHSYNCI assert"]
    pub mod FRCSHDWUPT {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PWM output enable 1- output is enabled 0- output is disabled"]
    pub mod OEN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PWM channel configure register"]
pub mod PWMCFG_2 {
    #[doc = "This bitfield define the PWM pair deadarea length. The unit is 0.5 cycle. The minimum length of deadarea is 1 cycle. Note: user should configure pair bit and this bitfield before PWM output is enabled."]
    pub mod DEADAREA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- PWM output is in pair mode. Note the two PWM outputs need to be both set to pair mode. 0- PWM output is in indepandent mode."]
    pub mod PAIR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Select sources for force output 0- force output is enabled when FRCI assert 1- force output is enabled by software write swfrc to 1"]
    pub mod FRCSRCSEL {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield select when to recover PWM output after fault condition removed. 00: immediately 01: after pwm timer counter reload time 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after software write faultclr bit in GCR register"]
    pub mod FAULTRECTIME {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield defines the PWM output status when fault condition happen 00: force output 0 01: force output 1 1x: output highz"]
    pub mod FAULTMODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield select when the FRCMD shadow register will be loaded to its work register 00: after software set shlk bit of shlk register 01: immediately after the register being modified 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after SHSYNCI assert"]
    pub mod FRCSHDWUPT {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PWM output enable 1- output is enabled 0- output is disabled"]
    pub mod OEN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PWM channel configure register"]
pub mod PWMCFG_3 {
    #[doc = "This bitfield define the PWM pair deadarea length. The unit is 0.5 cycle. The minimum length of deadarea is 1 cycle. Note: user should configure pair bit and this bitfield before PWM output is enabled."]
    pub mod DEADAREA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- PWM output is in pair mode. Note the two PWM outputs need to be both set to pair mode. 0- PWM output is in indepandent mode."]
    pub mod PAIR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Select sources for force output 0- force output is enabled when FRCI assert 1- force output is enabled by software write swfrc to 1"]
    pub mod FRCSRCSEL {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield select when to recover PWM output after fault condition removed. 00: immediately 01: after pwm timer counter reload time 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after software write faultclr bit in GCR register"]
    pub mod FAULTRECTIME {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield defines the PWM output status when fault condition happen 00: force output 0 01: force output 1 1x: output highz"]
    pub mod FAULTMODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield select when the FRCMD shadow register will be loaded to its work register 00: after software set shlk bit of shlk register 01: immediately after the register being modified 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after SHSYNCI assert"]
    pub mod FRCSHDWUPT {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PWM output enable 1- output is enabled 0- output is disabled"]
    pub mod OEN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PWM channel configure register"]
pub mod PWMCFG_4 {
    #[doc = "This bitfield define the PWM pair deadarea length. The unit is 0.5 cycle. The minimum length of deadarea is 1 cycle. Note: user should configure pair bit and this bitfield before PWM output is enabled."]
    pub mod DEADAREA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- PWM output is in pair mode. Note the two PWM outputs need to be both set to pair mode. 0- PWM output is in indepandent mode."]
    pub mod PAIR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Select sources for force output 0- force output is enabled when FRCI assert 1- force output is enabled by software write swfrc to 1"]
    pub mod FRCSRCSEL {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield select when to recover PWM output after fault condition removed. 00: immediately 01: after pwm timer counter reload time 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after software write faultclr bit in GCR register"]
    pub mod FAULTRECTIME {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield defines the PWM output status when fault condition happen 00: force output 0 01: force output 1 1x: output highz"]
    pub mod FAULTMODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield select when the FRCMD shadow register will be loaded to its work register 00: after software set shlk bit of shlk register 01: immediately after the register being modified 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after SHSYNCI assert"]
    pub mod FRCSHDWUPT {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PWM output enable 1- output is enabled 0- output is disabled"]
    pub mod OEN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PWM channel configure register"]
pub mod PWMCFG_5 {
    #[doc = "This bitfield define the PWM pair deadarea length. The unit is 0.5 cycle. The minimum length of deadarea is 1 cycle. Note: user should configure pair bit and this bitfield before PWM output is enabled."]
    pub mod DEADAREA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- PWM output is in pair mode. Note the two PWM outputs need to be both set to pair mode. 0- PWM output is in indepandent mode."]
    pub mod PAIR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Select sources for force output 0- force output is enabled when FRCI assert 1- force output is enabled by software write swfrc to 1"]
    pub mod FRCSRCSEL {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield select when to recover PWM output after fault condition removed. 00: immediately 01: after pwm timer counter reload time 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after software write faultclr bit in GCR register"]
    pub mod FAULTRECTIME {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield defines the PWM output status when fault condition happen 00: force output 0 01: force output 1 1x: output highz"]
    pub mod FAULTMODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield select when the FRCMD shadow register will be loaded to its work register 00: after software set shlk bit of shlk register 01: immediately after the register being modified 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after SHSYNCI assert"]
    pub mod FRCSHDWUPT {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PWM output enable 1- output is enabled 0- output is disabled"]
    pub mod OEN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PWM channel configure register"]
pub mod PWMCFG_6 {
    #[doc = "This bitfield define the PWM pair deadarea length. The unit is 0.5 cycle. The minimum length of deadarea is 1 cycle. Note: user should configure pair bit and this bitfield before PWM output is enabled."]
    pub mod DEADAREA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- PWM output is in pair mode. Note the two PWM outputs need to be both set to pair mode. 0- PWM output is in indepandent mode."]
    pub mod PAIR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Select sources for force output 0- force output is enabled when FRCI assert 1- force output is enabled by software write swfrc to 1"]
    pub mod FRCSRCSEL {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield select when to recover PWM output after fault condition removed. 00: immediately 01: after pwm timer counter reload time 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after software write faultclr bit in GCR register"]
    pub mod FAULTRECTIME {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield defines the PWM output status when fault condition happen 00: force output 0 01: force output 1 1x: output highz"]
    pub mod FAULTMODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield select when the FRCMD shadow register will be loaded to its work register 00: after software set shlk bit of shlk register 01: immediately after the register being modified 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after SHSYNCI assert"]
    pub mod FRCSHDWUPT {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PWM output enable 1- output is enabled 0- output is disabled"]
    pub mod OEN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PWM channel configure register"]
pub mod PWMCFG_7 {
    #[doc = "This bitfield define the PWM pair deadarea length. The unit is 0.5 cycle. The minimum length of deadarea is 1 cycle. Note: user should configure pair bit and this bitfield before PWM output is enabled."]
    pub mod DEADAREA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- PWM output is in pair mode. Note the two PWM outputs need to be both set to pair mode. 0- PWM output is in indepandent mode."]
    pub mod PAIR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Select sources for force output 0- force output is enabled when FRCI assert 1- force output is enabled by software write swfrc to 1"]
    pub mod FRCSRCSEL {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield select when to recover PWM output after fault condition removed. 00: immediately 01: after pwm timer counter reload time 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after software write faultclr bit in GCR register"]
    pub mod FAULTRECTIME {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield defines the PWM output status when fault condition happen 00: force output 0 01: force output 1 1x: output highz"]
    pub mod FAULTMODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield select when the FRCMD shadow register will be loaded to its work register 00: after software set shlk bit of shlk register 01: immediately after the register being modified 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after SHSYNCI assert"]
    pub mod FRCSHDWUPT {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PWM output enable 1- output is enabled 0- output is disabled"]
    pub mod OEN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status register"]
pub mod SR {
    #[doc = "comparator output compare or input capture flag"]
    pub mod CMPFX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reload flag, this flag set when cnt count to rld value or when SYNCI assert"]
    pub mod RLDF {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "half reload flag, this flag set when cnt count to rld/2"]
    pub mod HALFRLDF {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "extended reload flag, this flag set when xcnt count to xrld value or when SYNCI assert"]
    pub mod XRLDF {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "fault condition flag"]
    pub mod FAULTF {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt request enable register"]
pub mod IRQEN {
    #[doc = "comparator output compare or input capture flag interrupt enable"]
    pub mod CMPIRQEX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reload flag interrupt enable"]
    pub mod RLDIRQE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "half reload flag interrupt enable"]
    pub mod HALFRLDIRQE {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "extended reload flag interrupt enable"]
    pub mod XRLDIRQE {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "fault condition interrupt enable"]
    pub mod FAULTIRQE {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA request enable register"]
pub mod DMAEN {
    #[doc = "comparator output compare or input capture flag DMA request enable"]
    pub mod CMPENX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reload flag DMA request enable"]
    pub mod RLDEN {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "half reload flag DMA request enable"]
    pub mod HALFRLDEN {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "extended reload flag DMA request enable"]
    pub mod XRLDEN {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "fault condition DMA request enable"]
    pub mod FAULTEN {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Comparator configure register"]
pub mod CMPCFG_CMPCFG0 {
    #[doc = "comparator mode 0- output compare mode 1- input capture mode"]
    pub mod CMPMODE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield select when the comparator shadow register will be loaded to its work register 00: after software set shlk bit of shlk register 01: immediately after the register being modified 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after SHSYNCI assert"]
    pub mod CMPSHDWUPT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield enable the comparator to compare xcmp with xcnt."]
    pub mod XCNTCMPEN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Comparator configure register"]
pub mod CMPCFG_1 {
    #[doc = "comparator mode 0- output compare mode 1- input capture mode"]
    pub mod CMPMODE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield select when the comparator shadow register will be loaded to its work register 00: after software set shlk bit of shlk register 01: immediately after the register being modified 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after SHSYNCI assert"]
    pub mod CMPSHDWUPT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield enable the comparator to compare xcmp with xcnt."]
    pub mod XCNTCMPEN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Comparator configure register"]
pub mod CMPCFG_2 {
    #[doc = "comparator mode 0- output compare mode 1- input capture mode"]
    pub mod CMPMODE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield select when the comparator shadow register will be loaded to its work register 00: after software set shlk bit of shlk register 01: immediately after the register being modified 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after SHSYNCI assert"]
    pub mod CMPSHDWUPT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield enable the comparator to compare xcmp with xcnt."]
    pub mod XCNTCMPEN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Comparator configure register"]
pub mod CMPCFG_3 {
    #[doc = "comparator mode 0- output compare mode 1- input capture mode"]
    pub mod CMPMODE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield select when the comparator shadow register will be loaded to its work register 00: after software set shlk bit of shlk register 01: immediately after the register being modified 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after SHSYNCI assert"]
    pub mod CMPSHDWUPT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield enable the comparator to compare xcmp with xcnt."]
    pub mod XCNTCMPEN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Comparator configure register"]
pub mod CMPCFG_4 {
    #[doc = "comparator mode 0- output compare mode 1- input capture mode"]
    pub mod CMPMODE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield select when the comparator shadow register will be loaded to its work register 00: after software set shlk bit of shlk register 01: immediately after the register being modified 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after SHSYNCI assert"]
    pub mod CMPSHDWUPT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield enable the comparator to compare xcmp with xcnt."]
    pub mod XCNTCMPEN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Comparator configure register"]
pub mod CMPCFG_5 {
    #[doc = "comparator mode 0- output compare mode 1- input capture mode"]
    pub mod CMPMODE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield select when the comparator shadow register will be loaded to its work register 00: after software set shlk bit of shlk register 01: immediately after the register being modified 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after SHSYNCI assert"]
    pub mod CMPSHDWUPT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield enable the comparator to compare xcmp with xcnt."]
    pub mod XCNTCMPEN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Comparator configure register"]
pub mod CMPCFG_6 {
    #[doc = "comparator mode 0- output compare mode 1- input capture mode"]
    pub mod CMPMODE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield select when the comparator shadow register will be loaded to its work register 00: after software set shlk bit of shlk register 01: immediately after the register being modified 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after SHSYNCI assert"]
    pub mod CMPSHDWUPT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield enable the comparator to compare xcmp with xcnt."]
    pub mod XCNTCMPEN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Comparator configure register"]
pub mod CMPCFG_7 {
    #[doc = "comparator mode 0- output compare mode 1- input capture mode"]
    pub mod CMPMODE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield select when the comparator shadow register will be loaded to its work register 00: after software set shlk bit of shlk register 01: immediately after the register being modified 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after SHSYNCI assert"]
    pub mod CMPSHDWUPT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield enable the comparator to compare xcmp with xcnt."]
    pub mod XCNTCMPEN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Comparator configure register"]
pub mod CMPCFG_8 {
    #[doc = "comparator mode 0- output compare mode 1- input capture mode"]
    pub mod CMPMODE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield select when the comparator shadow register will be loaded to its work register 00: after software set shlk bit of shlk register 01: immediately after the register being modified 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after SHSYNCI assert"]
    pub mod CMPSHDWUPT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield enable the comparator to compare xcmp with xcnt."]
    pub mod XCNTCMPEN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Comparator configure register"]
pub mod CMPCFG_9 {
    #[doc = "comparator mode 0- output compare mode 1- input capture mode"]
    pub mod CMPMODE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield select when the comparator shadow register will be loaded to its work register 00: after software set shlk bit of shlk register 01: immediately after the register being modified 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after SHSYNCI assert"]
    pub mod CMPSHDWUPT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield enable the comparator to compare xcmp with xcnt."]
    pub mod XCNTCMPEN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Comparator configure register"]
pub mod CMPCFG_10 {
    #[doc = "comparator mode 0- output compare mode 1- input capture mode"]
    pub mod CMPMODE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield select when the comparator shadow register will be loaded to its work register 00: after software set shlk bit of shlk register 01: immediately after the register being modified 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after SHSYNCI assert"]
    pub mod CMPSHDWUPT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield enable the comparator to compare xcmp with xcnt."]
    pub mod XCNTCMPEN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Comparator configure register"]
pub mod CMPCFG_11 {
    #[doc = "comparator mode 0- output compare mode 1- input capture mode"]
    pub mod CMPMODE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield select when the comparator shadow register will be loaded to its work register 00: after software set shlk bit of shlk register 01: immediately after the register being modified 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after SHSYNCI assert"]
    pub mod CMPSHDWUPT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield enable the comparator to compare xcmp with xcnt."]
    pub mod XCNTCMPEN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Comparator configure register"]
pub mod CMPCFG_12 {
    #[doc = "comparator mode 0- output compare mode 1- input capture mode"]
    pub mod CMPMODE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield select when the comparator shadow register will be loaded to its work register 00: after software set shlk bit of shlk register 01: immediately after the register being modified 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after SHSYNCI assert"]
    pub mod CMPSHDWUPT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield enable the comparator to compare xcmp with xcnt."]
    pub mod XCNTCMPEN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Comparator configure register"]
pub mod CMPCFG_13 {
    #[doc = "comparator mode 0- output compare mode 1- input capture mode"]
    pub mod CMPMODE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield select when the comparator shadow register will be loaded to its work register 00: after software set shlk bit of shlk register 01: immediately after the register being modified 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after SHSYNCI assert"]
    pub mod CMPSHDWUPT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield enable the comparator to compare xcmp with xcnt."]
    pub mod XCNTCMPEN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Comparator configure register"]
pub mod CMPCFG_14 {
    #[doc = "comparator mode 0- output compare mode 1- input capture mode"]
    pub mod CMPMODE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield select when the comparator shadow register will be loaded to its work register 00: after software set shlk bit of shlk register 01: immediately after the register being modified 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after SHSYNCI assert"]
    pub mod CMPSHDWUPT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield enable the comparator to compare xcmp with xcnt."]
    pub mod XCNTCMPEN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Comparator configure register"]
pub mod CMPCFG_15 {
    #[doc = "comparator mode 0- output compare mode 1- input capture mode"]
    pub mod CMPMODE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield select when the comparator shadow register will be loaded to its work register 00: after software set shlk bit of shlk register 01: immediately after the register being modified 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after SHSYNCI assert"]
    pub mod CMPSHDWUPT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield enable the comparator to compare xcmp with xcnt."]
    pub mod XCNTCMPEN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Comparator configure register"]
pub mod CMPCFG_16 {
    #[doc = "comparator mode 0- output compare mode 1- input capture mode"]
    pub mod CMPMODE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield select when the comparator shadow register will be loaded to its work register 00: after software set shlk bit of shlk register 01: immediately after the register being modified 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after SHSYNCI assert"]
    pub mod CMPSHDWUPT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield enable the comparator to compare xcmp with xcnt."]
    pub mod XCNTCMPEN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Comparator configure register"]
pub mod CMPCFG_17 {
    #[doc = "comparator mode 0- output compare mode 1- input capture mode"]
    pub mod CMPMODE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield select when the comparator shadow register will be loaded to its work register 00: after software set shlk bit of shlk register 01: immediately after the register being modified 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after SHSYNCI assert"]
    pub mod CMPSHDWUPT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield enable the comparator to compare xcmp with xcnt."]
    pub mod XCNTCMPEN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Comparator configure register"]
pub mod CMPCFG_18 {
    #[doc = "comparator mode 0- output compare mode 1- input capture mode"]
    pub mod CMPMODE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield select when the comparator shadow register will be loaded to its work register 00: after software set shlk bit of shlk register 01: immediately after the register being modified 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after SHSYNCI assert"]
    pub mod CMPSHDWUPT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield enable the comparator to compare xcmp with xcnt."]
    pub mod XCNTCMPEN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Comparator configure register"]
pub mod CMPCFG_19 {
    #[doc = "comparator mode 0- output compare mode 1- input capture mode"]
    pub mod CMPMODE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield select when the comparator shadow register will be loaded to its work register 00: after software set shlk bit of shlk register 01: immediately after the register being modified 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after SHSYNCI assert"]
    pub mod CMPSHDWUPT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield enable the comparator to compare xcmp with xcnt."]
    pub mod XCNTCMPEN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Comparator configure register"]
pub mod CMPCFG_20 {
    #[doc = "comparator mode 0- output compare mode 1- input capture mode"]
    pub mod CMPMODE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield select when the comparator shadow register will be loaded to its work register 00: after software set shlk bit of shlk register 01: immediately after the register being modified 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after SHSYNCI assert"]
    pub mod CMPSHDWUPT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield enable the comparator to compare xcmp with xcnt."]
    pub mod XCNTCMPEN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Comparator configure register"]
pub mod CMPCFG_21 {
    #[doc = "comparator mode 0- output compare mode 1- input capture mode"]
    pub mod CMPMODE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield select when the comparator shadow register will be loaded to its work register 00: after software set shlk bit of shlk register 01: immediately after the register being modified 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after SHSYNCI assert"]
    pub mod CMPSHDWUPT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield enable the comparator to compare xcmp with xcnt."]
    pub mod XCNTCMPEN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Comparator configure register"]
pub mod CMPCFG_22 {
    #[doc = "comparator mode 0- output compare mode 1- input capture mode"]
    pub mod CMPMODE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield select when the comparator shadow register will be loaded to its work register 00: after software set shlk bit of shlk register 01: immediately after the register being modified 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after SHSYNCI assert"]
    pub mod CMPSHDWUPT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield enable the comparator to compare xcmp with xcnt."]
    pub mod XCNTCMPEN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Comparator configure register"]
pub mod CMPCFG_23 {
    #[doc = "comparator mode 0- output compare mode 1- input capture mode"]
    pub mod CMPMODE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield select when the comparator shadow register will be loaded to its work register 00: after software set shlk bit of shlk register 01: immediately after the register being modified 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after SHSYNCI assert"]
    pub mod CMPSHDWUPT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield enable the comparator to compare xcmp with xcnt."]
    pub mod XCNTCMPEN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
