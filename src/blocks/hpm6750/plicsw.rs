#[doc = "PLICSW"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x1000],
    #[doc = "Pending status"]
    pub PENDING: crate::RWRegister<u32>,
    _reserved1: [u8; 0x0ffc],
    #[doc = "Interrupt enable"]
    pub INTEN: crate::RWRegister<u32>,
    _reserved2: [u8; 0x001f_e000],
    #[doc = "Claim and complete."]
    pub CLAIM: crate::RWRegister<u32>,
}
#[doc = "Pending status"]
pub mod PENDING {
    #[doc = "writing 1 to trigger software interrupt"]
    pub mod INTERRUPT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt enable"]
pub mod INTEN {
    #[doc = "enable software interrupt"]
    pub mod INTERRUPT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Claim and complete."]
pub mod CLAIM {
    #[doc = "On reads, indicating the interrupt source that has being claimed. On writes, indicating the interrupt source that has been handled (completed)."]
    pub mod INTERRUPT_ID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
