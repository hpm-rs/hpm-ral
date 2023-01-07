#[doc = "MONO"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Low part of monotonic counter"]
    pub MONOL: crate::RWRegister<u32>,
    #[doc = "High part of monotonic counter"]
    pub MONOH: crate::RWRegister<u32>,
}
#[doc = "Low part of monotonic counter"]
pub mod MONOL {
    #[doc = "low part of monotonica counter, write to this counter will cause counter increase by 1"]
    pub mod COUNTER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "High part of monotonic counter"]
pub mod MONOH {
    #[doc = "high part of monotonica counter, write to this counter will cause counter increase by 1 if low part overflow"]
    pub mod COUNTER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Fuse value for high part of monotonica"]
    pub mod EPOCH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
