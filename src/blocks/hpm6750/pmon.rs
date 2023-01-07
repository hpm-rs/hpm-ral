#[doc = "PMON"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Glitch and clock monitor control"]
    pub MONITOR_GLITCH0_CONTROL: crate::RWRegister<u32>,
    #[doc = "Glitch and clock monitor status"]
    pub MONITOR_GLITCH0_STATUS: crate::RWRegister<u32>,
    #[doc = "Glitch and clock monitor control"]
    pub MONITOR_GLITCH1_CONTROL: crate::RWRegister<u32>,
    #[doc = "Glitch and clock monitor status"]
    pub MONITOR_GLITCH1_STATUS: crate::RWRegister<u32>,
    #[doc = "Glitch and clock monitor control"]
    pub MONITOR_CLOCK0_CONTROL: crate::RWRegister<u32>,
    #[doc = "Glitch and clock monitor status"]
    pub MONITOR_CLOCK0_STATUS: crate::RWRegister<u32>,
    #[doc = "Glitch and clock monitor control"]
    pub MONITOR_CLOCK1_CONTROL: crate::RWRegister<u32>,
    #[doc = "Glitch and clock monitor status"]
    pub MONITOR_CLOCK1_STATUS: crate::RWRegister<u32>,
    _reserved0: [u8; 0x20],
    #[doc = "No description avaiable"]
    pub IRQ_FLAG: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub IRQ_ENABLE: crate::RWRegister<u32>,
}
#[doc = "Glitch and clock monitor control"]
pub mod MONITOR_GLITCH0_CONTROL {
    #[doc = "enable glitch detector 0: detector disabled 1: detector enabled"]
    pub mod ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "select glitch works in active mode or passve mode. 0: passive mode, depends on power glitch destory DFF value 1: active mode, check glitch by DFF chain"]
    pub mod ACTIVE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Glitch and clock monitor status"]
pub mod MONITOR_GLITCH0_STATUS {
    #[doc = "flag for glitch detected, write 1 to clear this flag 0: glitch not detected 1: glitch detected"]
    pub mod FLAG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Glitch and clock monitor control"]
pub mod MONITOR_GLITCH1_CONTROL {
    #[doc = "enable glitch detector 0: detector disabled 1: detector enabled"]
    pub mod ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "select glitch works in active mode or passve mode. 0: passive mode, depends on power glitch destory DFF value 1: active mode, check glitch by DFF chain"]
    pub mod ACTIVE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Glitch and clock monitor status"]
pub mod MONITOR_GLITCH1_STATUS {
    #[doc = "flag for glitch detected, write 1 to clear this flag 0: glitch not detected 1: glitch detected"]
    pub mod FLAG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Glitch and clock monitor control"]
pub mod MONITOR_CLOCK0_CONTROL {
    #[doc = "enable glitch detector 0: detector disabled 1: detector enabled"]
    pub mod ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "select glitch works in active mode or passve mode. 0: passive mode, depends on power glitch destory DFF value 1: active mode, check glitch by DFF chain"]
    pub mod ACTIVE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Glitch and clock monitor status"]
pub mod MONITOR_CLOCK0_STATUS {
    #[doc = "flag for glitch detected, write 1 to clear this flag 0: glitch not detected 1: glitch detected"]
    pub mod FLAG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Glitch and clock monitor control"]
pub mod MONITOR_CLOCK1_CONTROL {
    #[doc = "enable glitch detector 0: detector disabled 1: detector enabled"]
    pub mod ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "select glitch works in active mode or passve mode. 0: passive mode, depends on power glitch destory DFF value 1: active mode, check glitch by DFF chain"]
    pub mod ACTIVE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Glitch and clock monitor status"]
pub mod MONITOR_CLOCK1_STATUS {
    #[doc = "flag for glitch detected, write 1 to clear this flag 0: glitch not detected 1: glitch detected"]
    pub mod FLAG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod IRQ_FLAG {
    #[doc = "interrupt flag, each bit represents for one monitor, write 1 to clear interrupt flag 0: no monitor interrupt 1: monitor interrupt happened"]
    pub mod FLAG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod IRQ_ENABLE {
    #[doc = "interrupt enable, each bit represents for one monitor 0: monitor interrupt disabled 1: monitor interrupt enabled"]
    pub mod ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
