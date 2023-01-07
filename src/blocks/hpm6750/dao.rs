#[doc = "DAO"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Control Register"]
    pub CTRL: crate::RWRegister<u32>,
    _reserved0: [u8; 0x04],
    #[doc = "Command Register"]
    pub CMD: crate::RWRegister<u32>,
    #[doc = "Configuration Register"]
    pub RX_CFGR: crate::RWRegister<u32>,
    #[doc = "RX Slot Control Register"]
    pub RXSLT: crate::RWRegister<u32>,
    #[doc = "HPF A Coef Register"]
    pub HPF_MA: crate::RWRegister<u32>,
    #[doc = "HPF B Coef Register"]
    pub HPF_B: crate::RWRegister<u32>,
}
#[doc = "Control Register"]
pub mod CTRL {
    #[doc = "the module continues to comsume data, but all the pads are constant, thus no audio out"]
    pub mod FALSE_RUN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "the pad output in False run mode, or when the module is disabled 0: all low 1: all high 2: P-high, N-low 3. output is not enabled"]
    pub mod FALSE_LEVEL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "all the outputs are inverted before sending to pad"]
    pub mod INVERT {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: Use remap pwm version. The remap version is a version that one pwm output is tied to zero when the input pcm signal is positive or negative 0: Don't use remap pwm version"]
    pub mod REMAP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Asserted to enable the left channel"]
    pub mod LEFT_EN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Asserted to enable the right channel"]
    pub mod RIGHT_EN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Asserted to let the left and right channel output the same value."]
    pub mod MONO {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error interrupt enable This bit controls the generation of an interrupt when an error condition (saturation) occurs. 0: Error interrupt is masked 1: Error interrupt is enabled"]
    pub mod SAT_ERR_IE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Whether HPF is enabled. This HPF is used to filter out the DC part."]
    pub mod HPF_EN {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Command Register"]
pub mod CMD {
    #[doc = "Enable this module to run."]
    pub mod RUN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Self-clear"]
    pub mod SFTRST {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Configuration Register"]
pub mod RX_CFGR {
    #[doc = "CH_MAX\\[3:0\\] is the number if channels supported in TDM mode. When not in TDM mode, it must be set as 2. It must be an even number, so CH_MAX\\[0\\] is always 0. 4'h2: 2 channels 4'h4: 4 channels etc"]
    pub mod CH_MAX {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "RX Slot Control Register"]
pub mod RXSLT {
    #[doc = "Slot enable for the channels."]
    pub mod EN {
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
