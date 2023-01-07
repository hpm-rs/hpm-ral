#[doc = "KEYM"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "software set symmetric key"]
    pub SOFTMKEY_SFK0: crate::RWRegister<u32>,
    #[doc = "software set symmetric key"]
    pub SOFTMKEY_SFK1: crate::RWRegister<u32>,
    #[doc = "software set symmetric key"]
    pub SOFTMKEY_SFK2: crate::RWRegister<u32>,
    #[doc = "software set symmetric key"]
    pub SOFTMKEY_SFK3: crate::RWRegister<u32>,
    #[doc = "software set symmetric key"]
    pub SOFTMKEY_SFK4: crate::RWRegister<u32>,
    #[doc = "software set symmetric key"]
    pub SOFTMKEY_SFK5: crate::RWRegister<u32>,
    #[doc = "software set symmetric key"]
    pub SOFTMKEY_SFK6: crate::RWRegister<u32>,
    #[doc = "software set symmetric key"]
    pub SOFTMKEY_SFK7: crate::RWRegister<u32>,
    #[doc = "system asymmetric key"]
    pub SOFTPKEY_SPK0: crate::RWRegister<u32>,
    #[doc = "system asymmetric key"]
    pub SOFTPKEY_SPK1: crate::RWRegister<u32>,
    #[doc = "system asymmetric key"]
    pub SOFTPKEY_SPK2: crate::RWRegister<u32>,
    #[doc = "system asymmetric key"]
    pub SOFTPKEY_SPK3: crate::RWRegister<u32>,
    #[doc = "system asymmetric key"]
    pub SOFTPKEY_SPK4: crate::RWRegister<u32>,
    #[doc = "system asymmetric key"]
    pub SOFTPKEY_SPK5: crate::RWRegister<u32>,
    #[doc = "system asymmetric key"]
    pub SOFTPKEY_SPK6: crate::RWRegister<u32>,
    #[doc = "system asymmetric key"]
    pub SOFTPKEY_SPK7: crate::RWRegister<u32>,
    #[doc = "secure key generation"]
    pub SEC_KEY_CTL: crate::RWRegister<u32>,
    #[doc = "non-secure key generation"]
    pub NSC_KEY_CTL: crate::RWRegister<u32>,
    #[doc = "Random number interface behavior"]
    pub RNG: crate::RWRegister<u32>,
    #[doc = "key read out control"]
    pub READ_CONTROL: crate::RWRegister<u32>,
}
#[doc = "software set symmetric key"]
pub mod SOFTMKEY_SFK0 {
    #[doc = "software symmetric key key will be scambled to 4 variants for software to use, and replicable on same chip. scramble keys are chip different, and not replicable on different chip must be write sequencely from 0 - 7, otherwise key value will be treated as all 0"]
    pub mod KEY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "software set symmetric key"]
pub mod SOFTMKEY_SFK1 {
    #[doc = "software symmetric key key will be scambled to 4 variants for software to use, and replicable on same chip. scramble keys are chip different, and not replicable on different chip must be write sequencely from 0 - 7, otherwise key value will be treated as all 0"]
    pub mod KEY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "software set symmetric key"]
pub mod SOFTMKEY_SFK2 {
    #[doc = "software symmetric key key will be scambled to 4 variants for software to use, and replicable on same chip. scramble keys are chip different, and not replicable on different chip must be write sequencely from 0 - 7, otherwise key value will be treated as all 0"]
    pub mod KEY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "software set symmetric key"]
pub mod SOFTMKEY_SFK3 {
    #[doc = "software symmetric key key will be scambled to 4 variants for software to use, and replicable on same chip. scramble keys are chip different, and not replicable on different chip must be write sequencely from 0 - 7, otherwise key value will be treated as all 0"]
    pub mod KEY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "software set symmetric key"]
pub mod SOFTMKEY_SFK4 {
    #[doc = "software symmetric key key will be scambled to 4 variants for software to use, and replicable on same chip. scramble keys are chip different, and not replicable on different chip must be write sequencely from 0 - 7, otherwise key value will be treated as all 0"]
    pub mod KEY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "software set symmetric key"]
pub mod SOFTMKEY_SFK5 {
    #[doc = "software symmetric key key will be scambled to 4 variants for software to use, and replicable on same chip. scramble keys are chip different, and not replicable on different chip must be write sequencely from 0 - 7, otherwise key value will be treated as all 0"]
    pub mod KEY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "software set symmetric key"]
pub mod SOFTMKEY_SFK6 {
    #[doc = "software symmetric key key will be scambled to 4 variants for software to use, and replicable on same chip. scramble keys are chip different, and not replicable on different chip must be write sequencely from 0 - 7, otherwise key value will be treated as all 0"]
    pub mod KEY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "software set symmetric key"]
pub mod SOFTMKEY_SFK7 {
    #[doc = "software symmetric key key will be scambled to 4 variants for software to use, and replicable on same chip. scramble keys are chip different, and not replicable on different chip must be write sequencely from 0 - 7, otherwise key value will be treated as all 0"]
    pub mod KEY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "system asymmetric key"]
pub mod SOFTPKEY_SPK0 {
    #[doc = "software asymmetric key key is derived from scrambles of fuse private key, software input key, SRK, and system security status. This key os read once, sencondary read will read out 0"]
    pub mod KEY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "system asymmetric key"]
pub mod SOFTPKEY_SPK1 {
    #[doc = "software asymmetric key key is derived from scrambles of fuse private key, software input key, SRK, and system security status. This key os read once, sencondary read will read out 0"]
    pub mod KEY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "system asymmetric key"]
pub mod SOFTPKEY_SPK2 {
    #[doc = "software asymmetric key key is derived from scrambles of fuse private key, software input key, SRK, and system security status. This key os read once, sencondary read will read out 0"]
    pub mod KEY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "system asymmetric key"]
pub mod SOFTPKEY_SPK3 {
    #[doc = "software asymmetric key key is derived from scrambles of fuse private key, software input key, SRK, and system security status. This key os read once, sencondary read will read out 0"]
    pub mod KEY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "system asymmetric key"]
pub mod SOFTPKEY_SPK4 {
    #[doc = "software asymmetric key key is derived from scrambles of fuse private key, software input key, SRK, and system security status. This key os read once, sencondary read will read out 0"]
    pub mod KEY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "system asymmetric key"]
pub mod SOFTPKEY_SPK5 {
    #[doc = "software asymmetric key key is derived from scrambles of fuse private key, software input key, SRK, and system security status. This key os read once, sencondary read will read out 0"]
    pub mod KEY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "system asymmetric key"]
pub mod SOFTPKEY_SPK6 {
    #[doc = "software asymmetric key key is derived from scrambles of fuse private key, software input key, SRK, and system security status. This key os read once, sencondary read will read out 0"]
    pub mod KEY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "system asymmetric key"]
pub mod SOFTPKEY_SPK7 {
    #[doc = "software asymmetric key key is derived from scrambles of fuse private key, software input key, SRK, and system security status. This key os read once, sencondary read will read out 0"]
    pub mod KEY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "secure key generation"]
pub mod SEC_KEY_CTL {
    #[doc = "secure symmtric key synthesize setting, key is a XOR of followings bit0: fuse mk, 0: not selected, 1:selected bit1: zmk from batt, 0: not selected, 1:selected bit2: software key 0: not selected, 1:selected"]
    pub mod KEY_SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "fuse symmetric key selection 0: use scramble version of fuse symmetric key 1: use alnertave scramble of fuse symmetric key"]
    pub mod FMK_SEL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "batt symmetric key selection 0: use scramble version of software symmetric key 1: use origin value in software symmetric key"]
    pub mod ZMK_SEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "software symmetric key selection 0: use origin value in software symmetric key 1: use scramble version of software symmetric key"]
    pub mod SMK_SEL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "session key valid 0: session key is all 0's and not usable 1: session key is valid"]
    pub mod SK_VAL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "block secure state key setting being changed"]
    pub mod LOCK_SEC_CTL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "non-secure key generation"]
pub mod NSC_KEY_CTL {
    #[doc = "non-secure symmtric key synthesize setting, key is a XOR of followings bit0: fuse mk, 0: not selected, 1:selected bit1: zmk from batt, 0: not selected, 1:selected bit2: software key 0: not selected, 1:selected"]
    pub mod KEY_SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "fuse symmetric key selection 0: use scramble version of fuse symmetric key 1: use origin value in fuse symmetric key"]
    pub mod FMK_SEL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "batt symmetric key selection 0: use scramble version of software symmetric key 1: use origin value in software symmetric key"]
    pub mod ZMK_SEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "software symmetric key selection 0: use scramble version of software symmetric key 1: use origin value in software symmetric key"]
    pub mod SMK_SEL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "session key valid 0: session key is all 0's and not usable 1: session key is valid"]
    pub mod SK_VAL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "block non-secure state key setting being changed"]
    pub mod LOCK_NSC_CTL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Random number interface behavior"]
pub mod RNG {
    #[doc = "control how SFK is accepted from random number generator 0: SFK value replaced by random number input 1: SFK value exclusive or with random number input,this help generate random number using 2 rings inside RNG"]
    pub mod RNG_XOR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "block RNG_XOR bit from changing, if this bit is written to 1, it will hold 1 until next reset 0: RNG_XOR can be changed by software 1: RNG_XOR ignore software change from software"]
    pub mod BLOCK_RNG_XOR {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "key read out control"]
pub mod READ_CONTROL {
    #[doc = "symmetric key readout control, if this bit is written to 1, it will hold 1 until next reset 0: key can be read out 1: key cannot be read out"]
    pub mod BLOCK_SMK_READ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "asymmetric key readout control, if this bit is written to 1, it will hold 1 until next reset 0: key can be read out 1: key cannot be read out"]
    pub mod BLOCK_PK_READ {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
