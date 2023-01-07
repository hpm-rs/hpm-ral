#[doc = "MCHTMR"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Machine Time"]
    pub MTIME: crate::RWRegister<u64>,
    #[doc = "Machine Time Compare"]
    pub MTIMECMP: crate::RWRegister<u64>,
}
#[doc = "Machine Time"]
pub mod MTIME {
    #[doc = "Machine time"]
    pub mod MTIME {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Machine Time Compare"]
pub mod MTIMECMP {
    #[doc = "Machine time compare"]
    pub mod MTIMECMP {
        pub const offset: u64 = 0;
        pub const mask: u64 = 0 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
