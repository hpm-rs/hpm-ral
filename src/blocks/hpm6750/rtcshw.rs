#[doc = "RTCSHW"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Second counter"]
    pub SECOND: crate::RWRegister<u32>,
    #[doc = "Sub-second counter"]
    pub SUBSEC: crate::RWRegister<u32>,
    #[doc = "Second counter snap shot"]
    pub SEC_SNAP: crate::RWRegister<u32>,
    #[doc = "Sub-second counter snap shot"]
    pub SUB_SNAP: crate::RWRegister<u32>,
    #[doc = "RTC alarm0"]
    pub ALARM0: crate::RWRegister<u32>,
    #[doc = "Alarm0 incremental"]
    pub ALARM0_INC: crate::RWRegister<u32>,
    #[doc = "RTC alarm1"]
    pub ALARM1: crate::RWRegister<u32>,
    #[doc = "Alarm1 incremental"]
    pub ALARM1_INC: crate::RWRegister<u32>,
    #[doc = "RTC alarm flag"]
    pub ALARM_FLAG: crate::RWRegister<u32>,
    #[doc = "RTC alarm enable"]
    pub ALARM_EN: crate::RWRegister<u32>,
}
#[doc = "Second counter"]
pub mod SECOND {
    #[doc = "second counter"]
    pub mod SECOND {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Sub-second counter"]
pub mod SUBSEC {
    #[doc = "sub second counter"]
    pub mod SUBSEC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Second counter snap shot"]
pub mod SEC_SNAP {
    #[doc = "second snap shot, write to take snap shot"]
    pub mod SEC_SNAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Sub-second counter snap shot"]
pub mod SUB_SNAP {
    #[doc = "sub second snap shot, write to take snap shot"]
    pub mod SUB_SNAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "RTC alarm0"]
pub mod ALARM0 {
    #[doc = "Alarm time for second counter, on each alarm match, alarm increase ALARM0_INC"]
    pub mod ALARM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Alarm0 incremental"]
pub mod ALARM0_INC {
    #[doc = "adder when ARLAM0 happen, helps to create periodical alarm"]
    pub mod INCREASE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "RTC alarm1"]
pub mod ALARM1 {
    #[doc = "Alarm time for second counter, on each alarm match, alarm increase ALARM0_INC"]
    pub mod ALARM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Alarm1 incremental"]
pub mod ALARM1_INC {
    #[doc = "adder when ARLAM0 happen, helps to create periodical alarm"]
    pub mod INCREASE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "RTC alarm flag"]
pub mod ALARM_FLAG {
    #[doc = "alarm0 happen"]
    pub mod ALARM0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "alarm1 happen"]
    pub mod ALARM1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "RTC alarm enable"]
pub mod ALARM_EN {
    #[doc = "alarm0 mask 0: alarm0 disabled 1: alarm0 enabled"]
    pub mod ENABLE0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "alarm1 mask 0: alarm1 disabled 1: alarm1 enabled"]
    pub mod ENABLE1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
