#[doc = "WDG0"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    #[doc = "Control Register"]
    pub CTRL: crate::RWRegister<u32>,
    #[doc = "Restart Register"]
    pub RESTART: crate::RWRegister<u32>,
    #[doc = "Write Protection Register"]
    pub WREN: crate::RWRegister<u32>,
    #[doc = "Status Register"]
    pub ST: crate::RWRegister<u32>,
}
#[doc = "Control Register"]
pub mod CTRL {
    #[doc = "Enable or disable the watchdog timer 0: Disable 1: Enable"]
    pub mod EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Clock source of timer: 0: EXTCLK 1: PCLK"]
    pub mod CLKSEL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable or disable the watchdog interrupt 0: Disable 1: Enable"]
    pub mod INTEN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable or disable the watchdog reset 0: Disable 1: Enable"]
    pub mod RSTEN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The timer interval of the interrupt stage: 0: Clock period x 2^6 1: Clock period x 2^8 2: Clock period x 2^10 3: Clock period x 2^11 4: Clock period x 2^12 5: Clock period x 2^13 6: Clock period x 2^14 7: Clock period x 2^15 8: Clock period x 2^17 9: Clock period x 2^19 10: Clock period x 2^21 11: Clock period x 2^23 12: Clock period x 2^25 13: Clock period x 2^27 14: Clock period x 2^29 15: Clock period x 2^31"]
    pub mod INTTIME {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The time interval of the reset stage: 0: Clock period x 2^7 1: Clock period x 2^8 2: Clock period x 2^9 3: Clock period x 2^10 4: Clock period x 2^11 5: Clock period x 2^12 6: Clock period x 2^13 7: Clock period x 2^14"]
    pub mod RSTTIME {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Restart Register"]
pub mod RESTART {
    #[doc = "Write the magic number ATCWDT200_RESTART_NUM to restart the watchdog timer."]
    pub mod RESTART {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Write Protection Register"]
pub mod WREN {
    #[doc = "Write the magic code to disable the write protection of the Control Register and the Restart Register."]
    pub mod WEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status Register"]
pub mod ST {
    #[doc = "The status of the watchdog interrupt timer 0: timer is not expired yet 1: timer is expired"]
    pub mod INTEXPIRED {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
