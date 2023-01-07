#[doc = "BPOR"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Power on cause"]
    pub POR_CAUSE: crate::RWRegister<u32>,
    #[doc = "Power on select"]
    pub POR_SELECT: crate::RWRegister<u32>,
    #[doc = "Power on reset config"]
    pub POR_CONFIG: crate::RWRegister<u32>,
    #[doc = "Power down control"]
    pub POR_CONTROL: crate::RWRegister<u32>,
}
#[doc = "Power on cause"]
pub mod POR_CAUSE {
    #[doc = "Power on cause, each bit represnts one cause, write 1 to clear each bit bit0: wakeup button bit1: security violation bit2: RTC alarm 0 bit3: RTC alarm 1 bit4: GPIO"]
    pub mod CAUSE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Power on select"]
pub mod POR_SELECT {
    #[doc = "Power on cause select, each bit represnts one cause, value 1 enables corresponding cause bit0: wakeup button bit1: security violation bit2: RTC alarm 0 bit3: RTC alarm 1 bit4: GPIO"]
    pub mod SELECT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Power on reset config"]
pub mod POR_CONFIG {
    #[doc = "retention battery domain setting 0: battery reset on reset pin reset happen 1: battery domain retention when reset pin reset happen"]
    pub mod RETENTION {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Power down control"]
pub mod POR_CONTROL {
    #[doc = "Chip power down counter, counter decreasing if value is not 0, power down of chip happens on counter value is 1"]
    pub mod COUNTER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
