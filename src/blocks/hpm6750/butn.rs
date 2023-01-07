#[doc = "BUTN"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Button status"]
    pub BTN_STATUS: crate::RWRegister<u32>,
    #[doc = "Button interrupt mask"]
    pub BTN_IRQ_MASK: crate::RWRegister<u32>,
    #[doc = "Debounce setting"]
    pub LED_INTENSE: crate::RWRegister<u32>,
}
#[doc = "Button status"]
pub mod BTN_STATUS {
    #[doc = "Power button press status, write 1 to clear flag bit0: button pressed bit1: button confirmd bit2: button long pressed bit3: button long long pressed"]
    pub mod PBTN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Wake button press status, write 1 to clear flag bit0: button pressed bit1: button confirmd bit2: button long pressed bit3: button long long pressed"]
    pub mod WBTN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Dual button press status, write 1 to clear flag bit0: button pressed bit1: button confirmd bit2: button long pressed bit3: button long long pressed"]
    pub mod DBTN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "power button click status, write 1 to clear flag bit0: clicked bit1: double clicked bit2: tripple clicked"]
    pub mod PCLICK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "power button click status when wake button held, write 1 to clear flag bit0: clicked bit1: double clicked bit2: tripple clicked"]
    pub mod XPCLICK {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "wake button click status, write 1 to clear flag bit0: clicked bit1: double clicked bit2: tripple clicked"]
    pub mod WCLICK {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "wake button click status when power button held, write 1 to clear flag bit0: clicked bit1: double clicked bit2: tripple clicked"]
    pub mod XWCLICK {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Button interrupt mask"]
pub mod BTN_IRQ_MASK {
    #[doc = "Power button press interrupt enable bit0: button pressed bit1: button confirmd bit2: button long pressed bit3: button long long pressed"]
    pub mod PBTN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Wake button press interrupt enable bit0: button pressed bit1: button confirmd bit2: button long pressed bit3: button long long pressed"]
    pub mod WBTN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Dual button press interrupt enable bit0: button pressed bit1: button confirmd bit2: button long pressed bit3: button long long pressed"]
    pub mod DBTN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "power button click interrupt enable bit0: clicked bit1: double clicked bit2: tripple clicked"]
    pub mod PCLICK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "power button click status when wake button held interrupt enable bit0: clicked bit1: double clicked bit2: tripple clicked"]
    pub mod XPCLICK {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "wake button click interrupt enable bit0: clicked bit1: double clicked bit2: tripple clicked"]
    pub mod WCLICK {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "wake button click status when power button held interrupt enable bit0: clicked bit1: double clicked bit2: tripple clicked"]
    pub mod XWCLICK {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Debounce setting"]
pub mod LED_INTENSE {
    #[doc = "Pbutton brightness 0"]
    pub mod PLED {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Rbutton brightness 0"]
    pub mod RLED {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
