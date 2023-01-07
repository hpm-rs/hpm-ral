#[doc = "RNG"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Command Register"]
    pub CMD: crate::RWRegister<u32>,
    #[doc = "Control Register"]
    pub CTRL: crate::RWRegister<u32>,
    #[doc = "Status Register"]
    pub STA: crate::RWRegister<u32>,
    #[doc = "Error Registers"]
    pub ERR: crate::RWRegister<u32>,
    #[doc = "FIFO out to bus/cpu"]
    pub FO2B: crate::RWRegister<u32>,
    _reserved0: [u8; 0x0c],
    #[doc = "FIFO out to SDP as AES engine key"]
    pub R2SK_FO2S0: crate::RWRegister<u32>,
    #[doc = "FIFO out to SDP as AES engine key"]
    pub R2SK_FO2S1: crate::RWRegister<u32>,
    #[doc = "FIFO out to SDP as AES engine key"]
    pub R2SK_FO2S2: crate::RWRegister<u32>,
    #[doc = "FIFO out to SDP as AES engine key"]
    pub R2SK_FO2S3: crate::RWRegister<u32>,
    #[doc = "FIFO out to SDP as AES engine key"]
    pub R2SK_FO2S4: crate::RWRegister<u32>,
    #[doc = "FIFO out to SDP as AES engine key"]
    pub R2SK_FO2S5: crate::RWRegister<u32>,
    #[doc = "FIFO out to SDP as AES engine key"]
    pub R2SK_FO2S6: crate::RWRegister<u32>,
    #[doc = "FIFO out to SDP as AES engine key"]
    pub R2SK_FO2S7: crate::RWRegister<u32>,
}
#[doc = "Command Register"]
pub mod CMD {
    #[doc = "Self Test, when both ST and GS triggered, ST first and GS next."]
    pub mod SLFCHK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Generate Seed, when both ST and GS triggered, ST first and GS next."]
    pub mod GENSD {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Clear the Interrupt, clear the RNG interrupt if an error is not present. This bit is self-clearing. 0 Do not clear the interrupt. 1 Clear the interrupt"]
    pub mod CLRINT {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Clear the Error, clear the errors in the ESR register and the RNG interrupt. This bit is self-clearing. 0 Do not clear the errors and the interrupt. 1 Clear the errors and the interrupt."]
    pub mod CLRERR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Soft Reset, Perform a software reset of the RNG This bit is self-clearing. 0 Do not perform a software reset. 1 Software reset"]
    pub mod SFTRST {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control Register"]
pub mod CTRL {
    #[doc = "FIFO underflow response mode 00 Return all zeros and set the ESR\\[FUFE\\]. 01 Return all zeros and set the ESR\\[FUFE\\]. 10 Generate the bus transfer error 11 Generate the interrupt and return all zeros (overrides the CTRL\\[MASKERR\\])."]
    pub mod FUFMOD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Auto Reseed"]
    pub mod AUTRSD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Mask Interrupt Request for Done Event, asks the interrupts generated upon the completion of the seed and self-test modes. The status of these jobs can be viewed by: • Reading the STA and viewing the seed done and the self-test done bits (STA\\[SDN, STDN\\]). • Viewing the RNG_CMD for the generate-seed or the self-test bits (CMD\\[GS,ST\\]) being set, indicating that the operation is still taking place."]
    pub mod MIRQDN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Mask Interrupt Request for Error"]
    pub mod MIRQERR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status Register"]
pub mod STA {
    #[doc = "when 1, means the RNG engine is busy for seeding or random number generation, self test and so on."]
    pub mod BUSY {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Idle, the RNG is in the idle mode, and internal clocks are disabled, in this mode, access to the FIFO is allowed. Once the FIFO is empty, the RNGB fills the FIFO and then enters idle mode again."]
    pub mod IDLE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reseed needed Indicates that the RNG needs to be reseeded. This is done by setting the CMD\\[GS\\], or automatically if the CTRL\\[ARS\\] is set."]
    pub mod RSDREQ {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Self Check Done Indicates whether Self Test is done or not. Can be cleared by the hardware reset or a new self test is initiated by setting the CMD\\[ST\\]. 0 Self test not completed 1 Completed a self test since the last reset."]
    pub mod SCDN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1st Seed done When \"1\", Indicates that the RNG generated the first seed."]
    pub mod FSDDN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "New seed done."]
    pub mod NSDDN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Fifo Level, Indicates the number of random words currently in the output FIFO"]
    pub mod FRNNU {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Fifo Size, it is 5 in this design."]
    pub mod FSIZE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error was detected, check ESR register for details"]
    pub mod FUNCERR {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Self Check Pass Fail"]
    pub mod SCPF {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Error Registers"]
pub mod ERR {
    #[doc = "Self-test error Indicates that the RNG failed the most recent self test. This bit is sticky and can only be reset by a hardware reset or by writing 1 to the CMD\\[CE\\]"]
    pub mod SCKERR {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FIFO access error(underflow)"]
    pub mod FUFE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "FIFO out to bus/cpu"]
pub mod FO2B {
    #[doc = "SW read the FIFO output."]
    pub mod FO2B {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "FIFO out to SDP as AES engine key"]
pub mod R2SK_FO2S0 {
    #[doc = "FIFO out to KMAN, will be SDP engine key."]
    pub mod FO2S0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "FIFO out to SDP as AES engine key"]
pub mod R2SK_FO2S1 {
    #[doc = "FIFO out to KMAN, will be SDP engine key."]
    pub mod FO2S0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "FIFO out to SDP as AES engine key"]
pub mod R2SK_FO2S2 {
    #[doc = "FIFO out to KMAN, will be SDP engine key."]
    pub mod FO2S0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "FIFO out to SDP as AES engine key"]
pub mod R2SK_FO2S3 {
    #[doc = "FIFO out to KMAN, will be SDP engine key."]
    pub mod FO2S0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "FIFO out to SDP as AES engine key"]
pub mod R2SK_FO2S4 {
    #[doc = "FIFO out to KMAN, will be SDP engine key."]
    pub mod FO2S0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "FIFO out to SDP as AES engine key"]
pub mod R2SK_FO2S5 {
    #[doc = "FIFO out to KMAN, will be SDP engine key."]
    pub mod FO2S0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "FIFO out to SDP as AES engine key"]
pub mod R2SK_FO2S6 {
    #[doc = "FIFO out to KMAN, will be SDP engine key."]
    pub mod FO2S0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "FIFO out to SDP as AES engine key"]
pub mod R2SK_FO2S7 {
    #[doc = "FIFO out to KMAN, will be SDP engine key."]
    pub mod FO2S0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
