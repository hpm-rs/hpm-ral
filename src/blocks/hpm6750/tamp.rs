#[doc = "TAMP"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Tamper n control"]
    pub TAMP_TAMP0_CONTROL: crate::RWRegister<u32>,
    #[doc = "Tamper n Polynomial of LFSR"]
    pub TAMP_TAMP0_POLY: crate::RWRegister<u32>,
    #[doc = "Tamper n LFSR shift register"]
    pub TAMP_TAMP0_LFSR: crate::RWRegister<u32>,
    _reserved0: [u8; 0x04],
    #[doc = "Tamper1 control"]
    pub TAMP_TAMP1_CONTROL: crate::RWRegister<u32>,
    #[doc = "Tamper1 Polynomial of LFSR"]
    pub TAMP_TAMP1_POLY: crate::RWRegister<u32>,
    #[doc = "Tamper1 LFSR shift register"]
    pub TAMP_TAMP1_LFSR: crate::RWRegister<u32>,
    _reserved1: [u8; 0x04],
    #[doc = "Tamper2 control"]
    pub TAMP_TAMP2_CONTROL: crate::RWRegister<u32>,
    #[doc = "Tamper2 Polynomial of LFSR"]
    pub TAMP_TAMP2_POLY: crate::RWRegister<u32>,
    #[doc = "Tamper2 LFSR shift register"]
    pub TAMP_TAMP2_LFSR: crate::RWRegister<u32>,
    _reserved2: [u8; 0x04],
    #[doc = "Tamper3 control"]
    pub TAMP_TAMP3_CONTROL: crate::RWRegister<u32>,
    #[doc = "Tamper3 Polynomial of LFSR"]
    pub TAMP_TAMP3_POLY: crate::RWRegister<u32>,
    #[doc = "Tamper3 LFSR shift register"]
    pub TAMP_TAMP3_LFSR: crate::RWRegister<u32>,
    _reserved3: [u8; 0x04],
    #[doc = "Tamper4 control"]
    pub TAMP_TAMP4_CONTROL: crate::RWRegister<u32>,
    #[doc = "Tamper4 Polynomial of LFSR"]
    pub TAMP_TAMP4_POLY: crate::RWRegister<u32>,
    #[doc = "Tamper4 LFSR shift register"]
    pub TAMP_TAMP4_LFSR: crate::RWRegister<u32>,
    _reserved4: [u8; 0x04],
    #[doc = "Tamper5 control"]
    pub TAMP_TAMP5_CONTROL: crate::RWRegister<u32>,
    #[doc = "Tamper5 Polynomial of LFSR"]
    pub TAMP_TAMP5_POLY: crate::RWRegister<u32>,
    #[doc = "Tamper5 LFSR shift register"]
    pub TAMP_TAMP5_LFSR: crate::RWRegister<u32>,
    _reserved5: [u8; 0x24],
    #[doc = "Tamper flag"]
    pub TAMP_FLAG: crate::RWRegister<u32>,
    #[doc = "Tamper interrupt enable"]
    pub IRQ_EN: crate::RWRegister<u32>,
}
#[doc = "Tamper n control"]
pub mod TAMP_TAMP0_CONTROL {
    #[doc = "enable tamper 0: tamper disableed 1: tamper enabled"]
    pub mod ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "select active or passive tamper 0: passive tamper 1: active tamper"]
    pub mod ACTIVE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "tamper will recover itself if tamper LFSR goes wrong 0: tamper will not recover 1: tamper will recover"]
    pub mod RECOVER {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "tamper speed selection, (2^SPEED) changes per second 0: 1 shift per second 1: 2 shifts per second . . . 15: 32768 shifts per second"]
    pub mod SPEED {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "pin value for passive tamper"]
    pub mod VALUE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "filter length 0: 1 cycle 1: 2 cycle 15: 65526 cycle"]
    pub mod FILTER {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "bypass tamper violation filter 0: filter applied 1: filter not used"]
    pub mod BYPASS {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "lock tamper setting 0: tamper setting can be changed 1: tamper setting will last to next battery domain power cycle"]
    pub mod LOCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Tamper n Polynomial of LFSR"]
pub mod TAMP_TAMP0_POLY {
    #[doc = "tamper LFSR polyminal, this is a write once register, once write content is locked, and readout value is \"1\""]
    pub mod POLY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Tamper n LFSR shift register"]
pub mod TAMP_TAMP0_LFSR {
    #[doc = "LFSR for active tamper, write only register, always read 0"]
    pub mod LFSR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Tamper1 control"]
pub mod TAMP_TAMP1_CONTROL {
    #[doc = "enable tamper 0: tamper disableed 1: tamper enabled"]
    pub mod ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "select active or passive tamper 0: passive tamper 1: active tamper"]
    pub mod ACTIVE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "tamper will recover itself if tamper LFSR goes wrong 0: tamper will not recover 1: tamper will recover"]
    pub mod RECOVER {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "tamper speed selection, (2^SPEED) changes per second 0: 1 shift per second 1: 2 shifts per second . . . 15: 32768 shifts per second"]
    pub mod SPEED {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "pin value for passive tamper"]
    pub mod VALUE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "filter length 0: 1 cycle 1: 2 cycle 15: 65526 cycle"]
    pub mod FILTER {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "bypass tamper violation filter 0: filter applied 1: filter not used"]
    pub mod BYPASS {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "lock tamper setting 0: tamper setting can be changed 1: tamper setting will last to next battery domain power cycle"]
    pub mod LOCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Tamper1 Polynomial of LFSR"]
pub mod TAMP_TAMP1_POLY {
    #[doc = "tamper LFSR polyminal, this is a write once register, once write content is locked, and readout value is \"1\""]
    pub mod POLY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Tamper1 LFSR shift register"]
pub mod TAMP_TAMP1_LFSR {
    #[doc = "LFSR for active tamper, write only register, always read 0"]
    pub mod LFSR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Tamper2 control"]
pub mod TAMP_TAMP2_CONTROL {
    #[doc = "enable tamper 0: tamper disableed 1: tamper enabled"]
    pub mod ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "select active or passive tamper 0: passive tamper 1: active tamper"]
    pub mod ACTIVE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "tamper will recover itself if tamper LFSR goes wrong 0: tamper will not recover 1: tamper will recover"]
    pub mod RECOVER {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "tamper speed selection, (2^SPEED) changes per second 0: 1 shift per second 1: 2 shifts per second . . . 15: 32768 shifts per second"]
    pub mod SPEED {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "pin value for passive tamper"]
    pub mod VALUE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "filter length 0: 1 cycle 1: 2 cycle 15: 65526 cycle"]
    pub mod FILTER {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "bypass tamper violation filter 0: filter applied 1: filter not used"]
    pub mod BYPASS {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "lock tamper setting 0: tamper setting can be changed 1: tamper setting will last to next battery domain power cycle"]
    pub mod LOCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Tamper2 Polynomial of LFSR"]
pub mod TAMP_TAMP2_POLY {
    #[doc = "tamper LFSR polyminal, this is a write once register, once write content is locked, and readout value is \"1\""]
    pub mod POLY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Tamper2 LFSR shift register"]
pub mod TAMP_TAMP2_LFSR {
    #[doc = "LFSR for active tamper, write only register, always read 0"]
    pub mod LFSR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Tamper3 control"]
pub mod TAMP_TAMP3_CONTROL {
    #[doc = "enable tamper 0: tamper disableed 1: tamper enabled"]
    pub mod ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "select active or passive tamper 0: passive tamper 1: active tamper"]
    pub mod ACTIVE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "tamper will recover itself if tamper LFSR goes wrong 0: tamper will not recover 1: tamper will recover"]
    pub mod RECOVER {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "tamper speed selection, (2^SPEED) changes per second 0: 1 shift per second 1: 2 shifts per second . . . 15: 32768 shifts per second"]
    pub mod SPEED {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "pin value for passive tamper"]
    pub mod VALUE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "filter length 0: 1 cycle 1: 2 cycle 15: 65526 cycle"]
    pub mod FILTER {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "bypass tamper violation filter 0: filter applied 1: filter not used"]
    pub mod BYPASS {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "lock tamper setting 0: tamper setting can be changed 1: tamper setting will last to next battery domain power cycle"]
    pub mod LOCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Tamper3 Polynomial of LFSR"]
pub mod TAMP_TAMP3_POLY {
    #[doc = "tamper LFSR polyminal, this is a write once register, once write content is locked, and readout value is \"1\""]
    pub mod POLY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Tamper3 LFSR shift register"]
pub mod TAMP_TAMP3_LFSR {
    #[doc = "LFSR for active tamper, write only register, always read 0"]
    pub mod LFSR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Tamper4 control"]
pub mod TAMP_TAMP4_CONTROL {
    #[doc = "enable tamper 0: tamper disableed 1: tamper enabled"]
    pub mod ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "select active or passive tamper 0: passive tamper 1: active tamper"]
    pub mod ACTIVE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "tamper will recover itself if tamper LFSR goes wrong 0: tamper will not recover 1: tamper will recover"]
    pub mod RECOVER {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "tamper speed selection, (2^SPEED) changes per second 0: 1 shift per second 1: 2 shifts per second . . . 15: 32768 shifts per second"]
    pub mod SPEED {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "pin value for passive tamper"]
    pub mod VALUE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "filter length 0: 1 cycle 1: 2 cycle 15: 65526 cycle"]
    pub mod FILTER {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "bypass tamper violation filter 0: filter applied 1: filter not used"]
    pub mod BYPASS {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "lock tamper setting 0: tamper setting can be changed 1: tamper setting will last to next battery domain power cycle"]
    pub mod LOCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Tamper4 Polynomial of LFSR"]
pub mod TAMP_TAMP4_POLY {
    #[doc = "tamper LFSR polyminal, this is a write once register, once write content is locked, and readout value is \"1\""]
    pub mod POLY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Tamper4 LFSR shift register"]
pub mod TAMP_TAMP4_LFSR {
    #[doc = "LFSR for active tamper, write only register, always read 0"]
    pub mod LFSR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Tamper5 control"]
pub mod TAMP_TAMP5_CONTROL {
    #[doc = "enable tamper 0: tamper disableed 1: tamper enabled"]
    pub mod ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "select active or passive tamper 0: passive tamper 1: active tamper"]
    pub mod ACTIVE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "tamper will recover itself if tamper LFSR goes wrong 0: tamper will not recover 1: tamper will recover"]
    pub mod RECOVER {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "tamper speed selection, (2^SPEED) changes per second 0: 1 shift per second 1: 2 shifts per second . . . 15: 32768 shifts per second"]
    pub mod SPEED {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "pin value for passive tamper"]
    pub mod VALUE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "filter length 0: 1 cycle 1: 2 cycle 15: 65526 cycle"]
    pub mod FILTER {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "bypass tamper violation filter 0: filter applied 1: filter not used"]
    pub mod BYPASS {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "lock tamper setting 0: tamper setting can be changed 1: tamper setting will last to next battery domain power cycle"]
    pub mod LOCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Tamper5 Polynomial of LFSR"]
pub mod TAMP_TAMP5_POLY {
    #[doc = "tamper LFSR polyminal, this is a write once register, once write content is locked, and readout value is \"1\""]
    pub mod POLY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Tamper5 LFSR shift register"]
pub mod TAMP_TAMP5_LFSR {
    #[doc = "LFSR for active tamper, write only register, always read 0"]
    pub mod LFSR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Tamper flag"]
pub mod TAMP_FLAG {
    #[doc = "tamper flag, each bit represents one tamper pin, write 1 to clear the flag Note, clear can only be cleared when tamper disapeared"]
    pub mod FLAG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Tamper interrupt enable"]
pub mod IRQ_EN {
    #[doc = "interrupt enable, each bit represents one tamper pin 0: interrupt disabled 1: interrupt enabled"]
    pub mod IRQ_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "lock bit for IRQ enable 0: enable bits can be changed 1: enable bits hold until next battery domain power cycle"]
    pub mod LOCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
