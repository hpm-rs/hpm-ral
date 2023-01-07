#[doc = "SYNT"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Global control register"]
    pub GCR: crate::RWRegister<u32>,
    #[doc = "Counter reload register"]
    pub RLD: crate::RWRegister<u32>,
    _reserved0: [u8; 0x04],
    #[doc = "Counter"]
    pub CNT: crate::RWRegister<u32>,
    _reserved1: [u8; 0x10],
    #[doc = "Comparator"]
    pub CMP_0: crate::RWRegister<u32>,
    #[doc = "Comparator"]
    pub CMP_1: crate::RWRegister<u32>,
    #[doc = "Comparator"]
    pub CMP_2: crate::RWRegister<u32>,
    #[doc = "Comparator"]
    pub CMP_3: crate::RWRegister<u32>,
}
#[doc = "Global control register"]
pub mod GCR {
    #[doc = "1- Enable counter"]
    pub mod CEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Reset counter"]
    pub mod CRST {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Counter reload register"]
pub mod RLD {
    #[doc = "counter reload value"]
    pub mod RLD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Counter"]
pub mod CNT {
    #[doc = "counter"]
    pub mod CNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Comparator"]
pub mod CMP_0 {
    #[doc = "comparator value, the output will assert when counter count to this value"]
    pub mod CMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Comparator"]
pub mod CMP_1 {
    #[doc = "comparator value, the output will assert when counter count to this value"]
    pub mod CMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Comparator"]
pub mod CMP_2 {
    #[doc = "comparator value, the output will assert when counter count to this value"]
    pub mod CMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Comparator"]
pub mod CMP_3 {
    #[doc = "comparator value, the output will assert when counter count to this value"]
    pub mod CMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
