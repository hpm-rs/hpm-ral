#[doc = "PPOR"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "flag indicate reset source"]
    pub RESET_FLAG: crate::RWRegister<u32>,
    #[doc = "reset source status"]
    pub RESET_STATUS: crate::RWRegister<u32>,
    #[doc = "reset hold attribute"]
    pub RESET_HOLD: crate::RWRegister<u32>,
    #[doc = "reset source enable"]
    pub RESET_ENABLE: crate::RWRegister<u32>,
    #[doc = "reset type triggered by reset"]
    pub RESET_HOT: crate::RWRegister<u32>,
    #[doc = "reset type attribute"]
    pub RESET_COLD: crate::RWRegister<u32>,
    _reserved0: [u8; 0x04],
    #[doc = "Software reset counter"]
    pub SOFTWARE_RESET: crate::RWRegister<u32>,
}
#[doc = "flag indicate reset source"]
pub mod RESET_FLAG {
    #[doc = "reset reason of last hard reset, write 1 to clear each bit 0: brownout 1: temperature(not available) 2: resetpin(not available) 4: debug reset 5: jtag reset 8: cpu0 lockup(not available) 9: cpu1 lockup(not available) 10: cpu0 request(not available) 11: cpu1 request(not available) 16: watch dog 0 17: watch dog 1 18: watch dog 2 19: watch dog 3 20: pmic watch dog 31: software"]
    pub mod FLAG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "reset source status"]
pub mod RESET_STATUS {
    #[doc = "current status of reset sources 0: brownout 1: temperature(not available) 2: resetpin(not available) 4: debug reset 5: jtag reset 8: cpu0 lockup(not available) 9: cpu1 lockup(not available) 10: cpu0 request(not available) 11: cpu1 request(not available) 16: watch dog 0 17: watch dog 1 18: watch dog 2 19: watch dog 3 20: pmic watch dog 31: software"]
    pub mod STATUS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "reset hold attribute"]
pub mod RESET_HOLD {
    #[doc = "hold arrtibute, when set, SOC keep in reset status until reset source release, or, reset will be released after SOC enter reset status 0: brownout 1: temperature(not available) 2: resetpin(not available) 4: debug reset 5: jtag reset 8: cpu0 lockup(not available) 9: cpu1 lockup(not available) 10: cpu0 request(not available) 11: cpu1 request(not available) 16: watch dog 0 17: watch dog 1 18: watch dog 2 19: watch dog 3 20: pmic watch dog 31: software"]
    pub mod STATUS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "reset source enable"]
pub mod RESET_ENABLE {
    #[doc = "enable of reset sources 0: brownout 1: temperature(not available) 2: resetpin(not available) 4: debug reset 5: jtag reset 8: cpu0 lockup(not available) 9: cpu1 lockup(not available) 10: cpu0 request(not available) 11: cpu1 request(not available) 16: watch dog 0 17: watch dog 1 18: watch dog 2 19: watch dog 3 20: pmic watch dog 31: software"]
    pub mod ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "reset type triggered by reset"]
pub mod RESET_HOT {
    #[doc = "reset type of reset sources, 0 for cold/warm reset, all system control setting cleared including clock, ioc; 1 for hot reset, system control, ioc setting kept, peripheral setting cleared. 0: brownout 1: temperature(not available) 2: resetpin(not available) 4: debug reset 5: jtag reset 8: cpu0 lockup(not available) 9: cpu1 lockup(not available) 10: cpu0 request(not available) 11: cpu1 request(not available) 16: watch dog 0 17: watch dog 1 18: watch dog 2 19: watch dog 3 20: pmic watch dog 31: software"]
    pub mod TYPE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "reset type attribute"]
pub mod RESET_COLD {
    #[doc = "perform cold or warm reset of chip, 0 for warm reset, fuse value and debug connection preserved; 1 for cold reset, fuse value reloaded and debug connection corrupted. This bit is ignored when hot reset selected 0: brownout 1: temperature(not available) 2: resetpin(not available) 4: debug reset 5: jtag reset 8: cpu0 lockup(not available) 9: cpu1 lockup(not available) 10: cpu0 request(not available) 11: cpu1 request(not available) 16: watch dog 0 17: watch dog 1 18: watch dog 2 19: watch dog 3 20: pmic watch dog 31: software"]
    pub mod FLAG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Software reset counter"]
pub mod SOFTWARE_RESET {
    #[doc = "counter decrease in 24MHz and stop at 0, trigger reset when value reach 2, software can write 0 to cancel reset"]
    pub mod COUNTER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
