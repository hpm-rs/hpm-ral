#[doc = "BKEY"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Key data"]
    pub KEY_0_DATA_0: crate::RWRegister<u32>,
    #[doc = "Key data"]
    pub KEY_0_DATA_1: crate::RWRegister<u32>,
    #[doc = "Key data"]
    pub KEY_0_DATA_2: crate::RWRegister<u32>,
    #[doc = "Key data"]
    pub KEY_0_DATA_3: crate::RWRegister<u32>,
    #[doc = "Key data"]
    pub KEY_0_DATA_4: crate::RWRegister<u32>,
    #[doc = "Key data"]
    pub KEY_0_DATA_5: crate::RWRegister<u32>,
    #[doc = "Key data"]
    pub KEY_0_DATA_6: crate::RWRegister<u32>,
    #[doc = "Key data"]
    pub KEY_0_DATA_7: crate::RWRegister<u32>,
    #[doc = "Key data"]
    pub KEY_1_DATA_0: crate::RWRegister<u32>,
    #[doc = "Key data"]
    pub KEY_1_DATA_1: crate::RWRegister<u32>,
    #[doc = "Key data"]
    pub KEY_1_DATA_2: crate::RWRegister<u32>,
    #[doc = "Key data"]
    pub KEY_1_DATA_3: crate::RWRegister<u32>,
    #[doc = "Key data"]
    pub KEY_1_DATA_4: crate::RWRegister<u32>,
    #[doc = "Key data"]
    pub KEY_1_DATA_5: crate::RWRegister<u32>,
    #[doc = "Key data"]
    pub KEY_1_DATA_6: crate::RWRegister<u32>,
    #[doc = "Key data"]
    pub KEY_1_DATA_7: crate::RWRegister<u32>,
    #[doc = "Key ECC and access control"]
    pub ECC_KEY0: crate::RWRegister<u32>,
    #[doc = "Key 1 ECC and access control"]
    pub ECC_KEY1: crate::RWRegister<u32>,
    #[doc = "Key selection"]
    pub SELECT: crate::RWRegister<u32>,
}
#[doc = "Key data"]
pub mod KEY_0_DATA_0 {
    #[doc = "security key data"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Key data"]
pub mod KEY_0_DATA_1 {
    #[doc = "security key data"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Key data"]
pub mod KEY_0_DATA_2 {
    #[doc = "security key data"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Key data"]
pub mod KEY_0_DATA_3 {
    #[doc = "security key data"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Key data"]
pub mod KEY_0_DATA_4 {
    #[doc = "security key data"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Key data"]
pub mod KEY_0_DATA_5 {
    #[doc = "security key data"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Key data"]
pub mod KEY_0_DATA_6 {
    #[doc = "security key data"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Key data"]
pub mod KEY_0_DATA_7 {
    #[doc = "security key data"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Key data"]
pub mod KEY_1_DATA_0 {
    #[doc = "security key data"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Key data"]
pub mod KEY_1_DATA_1 {
    #[doc = "security key data"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Key data"]
pub mod KEY_1_DATA_2 {
    #[doc = "security key data"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Key data"]
pub mod KEY_1_DATA_3 {
    #[doc = "security key data"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Key data"]
pub mod KEY_1_DATA_4 {
    #[doc = "security key data"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Key data"]
pub mod KEY_1_DATA_5 {
    #[doc = "security key data"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Key data"]
pub mod KEY_1_DATA_6 {
    #[doc = "security key data"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Key data"]
pub mod KEY_1_DATA_7 {
    #[doc = "security key data"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Key ECC and access control"]
pub mod ECC_KEY0 {
    #[doc = "Parity check bits for key0"]
    pub mod ECC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "read lock to key0 0: key read enable 1: key always read as 0"]
    pub mod RLOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "write lock to key0 0: write enable 1: write ignored"]
    pub mod WLOCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Key 1 ECC and access control"]
pub mod ECC_KEY1 {
    #[doc = "Parity check bits for key0"]
    pub mod ECC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "read lock to key0 0: key read enable 1: key always read as 0"]
    pub mod RLOCK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "write lock to key0 0: write enable 1: write ignored"]
    pub mod WLOCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Key selection"]
pub mod SELECT {
    #[doc = "select key, key0 treated as secure key, in non-scure mode, only key1 can be selected 0: select key0 in secure mode, key1 in non-secure mode 1: select key1 in secure or nonsecure mode"]
    pub mod SELECT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
