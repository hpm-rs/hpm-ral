#[doc = "CAN0"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "receive buffer registers and reception time stamp"]
    pub RBUF_BUF0: crate::RWRegister<u32>,
    #[doc = "receive buffer registers and reception time stamp"]
    pub RBUF_BUF1: crate::RWRegister<u32>,
    #[doc = "receive buffer registers and reception time stamp"]
    pub RBUF_BUF2: crate::RWRegister<u32>,
    #[doc = "receive buffer registers and reception time stamp"]
    pub RBUF_BUF3: crate::RWRegister<u32>,
    #[doc = "receive buffer registers and reception time stamp"]
    pub RBUF_BUF4: crate::RWRegister<u32>,
    #[doc = "receive buffer registers and reception time stamp"]
    pub RBUF_BUF5: crate::RWRegister<u32>,
    #[doc = "receive buffer registers and reception time stamp"]
    pub RBUF_BUF6: crate::RWRegister<u32>,
    #[doc = "receive buffer registers and reception time stamp"]
    pub RBUF_BUF7: crate::RWRegister<u32>,
    #[doc = "receive buffer registers and reception time stamp"]
    pub RBUF_BUF8: crate::RWRegister<u32>,
    #[doc = "receive buffer registers and reception time stamp"]
    pub RBUF_BUF9: crate::RWRegister<u32>,
    #[doc = "receive buffer registers and reception time stamp"]
    pub RBUF_BUF10: crate::RWRegister<u32>,
    #[doc = "receive buffer registers and reception time stamp"]
    pub RBUF_BUF11: crate::RWRegister<u32>,
    #[doc = "receive buffer registers and reception time stamp"]
    pub RBUF_BUF12: crate::RWRegister<u32>,
    #[doc = "receive buffer registers and reception time stamp"]
    pub RBUF_BUF13: crate::RWRegister<u32>,
    #[doc = "receive buffer registers and reception time stamp"]
    pub RBUF_BUF14: crate::RWRegister<u32>,
    #[doc = "receive buffer registers and reception time stamp"]
    pub RBUF_BUF15: crate::RWRegister<u32>,
    #[doc = "receive buffer registers and reception time stamp"]
    pub RBUF_BUF16: crate::RWRegister<u32>,
    #[doc = "receive buffer registers and reception time stamp"]
    pub RBUF_BUF17: crate::RWRegister<u32>,
    #[doc = "receive buffer registers and reception time stamp"]
    pub RBUF_BUF18: crate::RWRegister<u32>,
    #[doc = "receive buffer registers and reception time stamp"]
    pub RBUF_BUF19: crate::RWRegister<u32>,
    #[doc = "transmit buffer register"]
    pub TBUF_BUF0: crate::RWRegister<u32>,
    #[doc = "transmit buffer register"]
    pub TBUF_BUF1: crate::RWRegister<u32>,
    #[doc = "transmit buffer register"]
    pub TBUF_BUF2: crate::RWRegister<u32>,
    #[doc = "transmit buffer register"]
    pub TBUF_BUF3: crate::RWRegister<u32>,
    #[doc = "transmit buffer register"]
    pub TBUF_BUF4: crate::RWRegister<u32>,
    #[doc = "transmit buffer register"]
    pub TBUF_BUF5: crate::RWRegister<u32>,
    #[doc = "transmit buffer register"]
    pub TBUF_BUF6: crate::RWRegister<u32>,
    #[doc = "transmit buffer register"]
    pub TBUF_BUF7: crate::RWRegister<u32>,
    #[doc = "transmit buffer register"]
    pub TBUF_BUF8: crate::RWRegister<u32>,
    #[doc = "transmit buffer register"]
    pub TBUF_BUF9: crate::RWRegister<u32>,
    #[doc = "transmit buffer register"]
    pub TBUF_BUF10: crate::RWRegister<u32>,
    #[doc = "transmit buffer register"]
    pub TBUF_BUF11: crate::RWRegister<u32>,
    #[doc = "transmit buffer register"]
    pub TBUF_BUF12: crate::RWRegister<u32>,
    #[doc = "transmit buffer register"]
    pub TBUF_BUF13: crate::RWRegister<u32>,
    #[doc = "transmit buffer register"]
    pub TBUF_BUF14: crate::RWRegister<u32>,
    #[doc = "transmit buffer register"]
    pub TBUF_BUF15: crate::RWRegister<u32>,
    #[doc = "transmit buffer register"]
    pub TBUF_BUF16: crate::RWRegister<u32>,
    #[doc = "transmit buffer register"]
    pub TBUF_BUF17: crate::RWRegister<u32>,
    #[doc = "transmission time stamp, LSB 32bit"]
    pub TTS_WRD0: crate::RWRegister<u32>,
    #[doc = "transmission time stamp, MSB 32bit"]
    pub TTS_WRD1: crate::RWRegister<u32>,
    #[doc = "config, status, command and control bits"]
    pub CMD_STA_CMD_CTRL: crate::RWRegister<u32>,
    #[doc = "Receive and Transmit Interrupt Enable Register RTIE"]
    pub RTIE: crate::RWRegister<u8>,
    #[doc = "Receive and Transmit Interrupt Flag Register RTIF (0xa5)"]
    pub RTIF: crate::RWRegister<u8>,
    #[doc = "ERRor INTerrupt Enable and Flag Register ERRINT"]
    pub ERRINT: crate::RWRegister<u8>,
    #[doc = "Warning Limits Register LIMIT"]
    pub LIMIT: crate::RWRegister<u8>,
    #[doc = "Bit Timing Register(Slow Speed)"]
    pub S_PRESC: crate::RWRegister<u32>,
    #[doc = "Bit Timing Register(Fast Speed)"]
    pub F_PRESC: crate::RWRegister<u32>,
    #[doc = "Error and Arbitration Lost Capture Register EALCAP"]
    pub EALCAP: crate::RWRegister<u8>,
    #[doc = "Transmitter Delay Compensation Register TDC"]
    pub TDC: crate::RWRegister<u8>,
    #[doc = "Error Counter Registers RECNT"]
    pub RECNT: crate::RWRegister<u8>,
    #[doc = "Error Counter Registers TECNT"]
    pub TECNT: crate::RWRegister<u8>,
    #[doc = "Acceptance Filter Control Register ACFCTRL"]
    pub ACFCTRL: crate::RWRegister<u8>,
    #[doc = "CiA 603 Time-Stamping TIMECFG"]
    pub TIMECFG: crate::RWRegister<u8>,
    #[doc = "Acceptance Filter Enable ACF_EN"]
    pub ACF_EN: crate::RWRegister<u16>,
    #[doc = "Acceptance CODE ACODE or ACMASK"]
    pub ACF: crate::RWRegister<u32>,
    #[doc = "Version Information VER"]
    pub VER: crate::RWRegister<u16>,
    #[doc = "TTCAN: TB Slot Pointer TBSLOT"]
    pub TBSLOT: crate::RWRegister<u8>,
    #[doc = "TTCAN: Time Trigger Configuration TTCFG"]
    pub TTCFG: crate::RWRegister<u8>,
    #[doc = "TTCAN: Reference Message REF_MSG"]
    pub REF_MSG: crate::RWRegister<u32>,
    #[doc = "TTCAN: Trigger Configuration TRIG_CFG"]
    pub TRIG_CFG: crate::RWRegister<u16>,
    #[doc = "TTCAN: Trigger Time TT_TRIG"]
    pub TT_TRIG: crate::RWRegister<u16>,
    #[doc = "TTCAN: Watch Trigger Time TT_WTRIG"]
    pub TT_WTRIG: crate::RWRegister<u16>,
}
#[doc = "receive buffer registers and reception time stamp"]
pub mod RBUF_BUF0 {
    #[doc = "receive buffer"]
    pub mod RBUF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "receive buffer registers and reception time stamp"]
pub mod RBUF_BUF1 {
    #[doc = "receive buffer"]
    pub mod RBUF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "receive buffer registers and reception time stamp"]
pub mod RBUF_BUF2 {
    #[doc = "receive buffer"]
    pub mod RBUF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "receive buffer registers and reception time stamp"]
pub mod RBUF_BUF3 {
    #[doc = "receive buffer"]
    pub mod RBUF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "receive buffer registers and reception time stamp"]
pub mod RBUF_BUF4 {
    #[doc = "receive buffer"]
    pub mod RBUF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "receive buffer registers and reception time stamp"]
pub mod RBUF_BUF5 {
    #[doc = "receive buffer"]
    pub mod RBUF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "receive buffer registers and reception time stamp"]
pub mod RBUF_BUF6 {
    #[doc = "receive buffer"]
    pub mod RBUF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "receive buffer registers and reception time stamp"]
pub mod RBUF_BUF7 {
    #[doc = "receive buffer"]
    pub mod RBUF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "receive buffer registers and reception time stamp"]
pub mod RBUF_BUF8 {
    #[doc = "receive buffer"]
    pub mod RBUF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "receive buffer registers and reception time stamp"]
pub mod RBUF_BUF9 {
    #[doc = "receive buffer"]
    pub mod RBUF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "receive buffer registers and reception time stamp"]
pub mod RBUF_BUF10 {
    #[doc = "receive buffer"]
    pub mod RBUF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "receive buffer registers and reception time stamp"]
pub mod RBUF_BUF11 {
    #[doc = "receive buffer"]
    pub mod RBUF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "receive buffer registers and reception time stamp"]
pub mod RBUF_BUF12 {
    #[doc = "receive buffer"]
    pub mod RBUF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "receive buffer registers and reception time stamp"]
pub mod RBUF_BUF13 {
    #[doc = "receive buffer"]
    pub mod RBUF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "receive buffer registers and reception time stamp"]
pub mod RBUF_BUF14 {
    #[doc = "receive buffer"]
    pub mod RBUF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "receive buffer registers and reception time stamp"]
pub mod RBUF_BUF15 {
    #[doc = "receive buffer"]
    pub mod RBUF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "receive buffer registers and reception time stamp"]
pub mod RBUF_BUF16 {
    #[doc = "receive buffer"]
    pub mod RBUF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "receive buffer registers and reception time stamp"]
pub mod RBUF_BUF17 {
    #[doc = "receive buffer"]
    pub mod RBUF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "receive buffer registers and reception time stamp"]
pub mod RBUF_BUF18 {
    #[doc = "receive buffer"]
    pub mod RBUF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "receive buffer registers and reception time stamp"]
pub mod RBUF_BUF19 {
    #[doc = "receive buffer"]
    pub mod RBUF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "transmit buffer register"]
pub mod TBUF_BUF0 {
    #[doc = "transmit buffer"]
    pub mod TBUF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "transmit buffer register"]
pub mod TBUF_BUF1 {
    #[doc = "transmit buffer"]
    pub mod TBUF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "transmit buffer register"]
pub mod TBUF_BUF2 {
    #[doc = "transmit buffer"]
    pub mod TBUF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "transmit buffer register"]
pub mod TBUF_BUF3 {
    #[doc = "transmit buffer"]
    pub mod TBUF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "transmit buffer register"]
pub mod TBUF_BUF4 {
    #[doc = "transmit buffer"]
    pub mod TBUF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "transmit buffer register"]
pub mod TBUF_BUF5 {
    #[doc = "transmit buffer"]
    pub mod TBUF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "transmit buffer register"]
pub mod TBUF_BUF6 {
    #[doc = "transmit buffer"]
    pub mod TBUF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "transmit buffer register"]
pub mod TBUF_BUF7 {
    #[doc = "transmit buffer"]
    pub mod TBUF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "transmit buffer register"]
pub mod TBUF_BUF8 {
    #[doc = "transmit buffer"]
    pub mod TBUF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "transmit buffer register"]
pub mod TBUF_BUF9 {
    #[doc = "transmit buffer"]
    pub mod TBUF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "transmit buffer register"]
pub mod TBUF_BUF10 {
    #[doc = "transmit buffer"]
    pub mod TBUF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "transmit buffer register"]
pub mod TBUF_BUF11 {
    #[doc = "transmit buffer"]
    pub mod TBUF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "transmit buffer register"]
pub mod TBUF_BUF12 {
    #[doc = "transmit buffer"]
    pub mod TBUF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "transmit buffer register"]
pub mod TBUF_BUF13 {
    #[doc = "transmit buffer"]
    pub mod TBUF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "transmit buffer register"]
pub mod TBUF_BUF14 {
    #[doc = "transmit buffer"]
    pub mod TBUF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "transmit buffer register"]
pub mod TBUF_BUF15 {
    #[doc = "transmit buffer"]
    pub mod TBUF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "transmit buffer register"]
pub mod TBUF_BUF16 {
    #[doc = "transmit buffer"]
    pub mod TBUF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "transmit buffer register"]
pub mod TBUF_BUF17 {
    #[doc = "transmit buffer"]
    pub mod TBUF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "transmission time stamp, LSB 32bit"]
pub mod TTS_WRD0 {
    #[doc = "transmission time stamp, word 0, LSB 32bit"]
    pub mod TTS_WRD0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "transmission time stamp, MSB 32bit"]
pub mod TTS_WRD1 {
    #[doc = "transmission time stamp, word 0, LSB 32bit"]
    pub mod TTS_WRD0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "config, status, command and control bits"]
pub mod CMD_STA_CMD_CTRL {
    #[doc = "Bus Off (Bus Status bit) 1 - The controller status is “bus off”. 0 - The controller status is “bus on”. Writing a 1 to BUSOFF will reset TECNT and RECNT. This should be done only for debugging. See Chapter 3.9.10.6 for details."]
    pub mod BUSOFF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmission ACTIVE (Transmit Status bit) 1 - The controller is currently transmitting a frame. 0 - No transmit activity."]
    pub mod TACTIVE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reception ACTIVE (Receive Status bit) 1 - The controller is currently receiving a frame. 0 - No receive activity."]
    pub mod RACTIVE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmission Secondary Single Shot mode for STB 0 - Disabled 1 - Enabled"]
    pub mod TSSS {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmission Primary Single Shot mode for PTB 0 - Disabled 1 - Enabled"]
    pub mod TPSS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Loop Back Mode, Internal 0 - Disabled1 - EnabledLBMI should not be enabled while a transmission is active."]
    pub mod LBMI {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Loop Back Mode, External 0 - Disabled 1 - EnabledLBME should not be enabled while a transmission is active"]
    pub mod LBME {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RESET request bit 1 - The host controller performs a local reset of CAN-CTRL. 0 - no local reset of CAN-CTRLThe some register (e.g for node configuration) can only be modified if RESET=1. Bit RESET forces several components to a reset state. RESET is automatically set if the node enters “bus off” state. Note that a CAN node will participate in CAN communication after RESET is switched to 0after 11 CAN bit times. This delay is required by the CAN standard (bus idle time).If RESET is set to 1 and immediately set to 0, then it takes some time until RESET can beread as 0 and becomes inactive. The reason is clock domain crossing from host to CAN clockdomain. RESET is held active as long as needed depending on the relation between host andCAN clock."]
    pub mod RESET {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmit Secondary Abort 1 – Aborts a transmission from STB which has been requested but not started yet. For a TSONE transmission, only one frame is aborted while for a TSALL Transmission, all frames are aborted. One or all message slots will be released which updates TSSTAT. All aborted messages are lost because they are not accessible any more. If in priority mode a TSONE transmission is aborted, then it is not clear which frame will be aborted if new frames are written to the STB meanwhile. 0 – no abort The bit has to be set by the host controller and will be reset by CAN-CTRL. Setting TSA,automatically de-asserts TSONE or TSALL respectively. The host controller can set TSA to 1 but can not reset it to 0. The bit will be reset to the hardware reset value if RESET=1. TSA should not be set simultaneously with TSONE or TSALL."]
    pub mod TSA {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmit Secondary ALL frames 1 – Transmission enable of all messages in the STB. The controller starts the transmission as soon as the bus becomes vacant and no request of the PTB (bit TPE) is pending. 0 – No transmission for the STB. TSALL stays set until all messages have been transmitted successfully or they are aborted using TSA. The host controller can set TSALL to 1 but can not reset it to 0. This would only be possible using TSA and aborting the messages. The bit will be reset to the hardware reset value if RESET=1, STBY=1, (LOM=1 and LBME=0) or (TTEN=1 and TTTBM=1). If during a transmission the STB is loaded with a new frame then the new frame will be transmitted too. In other words: a transmission initiated by TSALL is finished when the STB becomes empty."]
    pub mod TSALL {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmit Secondary ONE frame 1 – Transmission enable of one in the STB. In FIFO mode this is the oldest message and in priority mode this is the one with the highest priority. TSONE in priority mode is difficult to handle, because it is not always clear which message will be transmitted if new messages are written to the STB meanwhile. The controller starts the transmission as soon as the bus becomes vacant and no request of the PTB (bit TPE) is pending. 0 – No transmission for the STB. TSONE stays set until the message has been transmitted successfully or it is aborted using TSA. The host controller can set TSONE to 1 but can not reset it to 0. This would only be possible using TSA and aborting the message. The bit will be reset to the hardware reset value if RESET=1, STBY=1, (LOM=1 and LBME=0) or (TTEN=1 and TTTBM=1)."]
    pub mod TSONE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmit Primary Abort 1 – Aborts a transmission from PTB which has been requested by TPE=1 but not started yet. (The data bytes of the message remains in the PTB.) 0 – no abort The bit has to be set by the host controller and will be reset by CAN-CTRL. Setting TPA automatically de-asserts TPE. The host controller can set TPA to 1 but can not reset it to 0. During the short time while the CAN-CTRL core resets the bit, it cannot be set by the host. The bit will be reset to the hardware reset value if RESET=1 or (TTEN=1 and TTTBM=1). TPA should not be set simultaneously with TPE."]
    pub mod TPA {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmit Primary Enable 1 - Transmission enable for the message in the high-priority PTB 0 - No transmission for the PTB If TPE is set, the message from the PTB will be transmitted at the next possible transmit position. A started transmission from the STB will be completed before, but pending new messages are delayed until the PTB message has been transmitted. TPE stays set until the message has been transmitted successfully or it is aborted using TPA. The host controller can set TPE to 1 but can not reset it to 0. This would only be possible using TPA and aborting the message. The bit will be reset to the hardware reset value if RESET=1, STBY=1, (LOM=1 and LBME=0) or (TTEN=1 and TTTBM=1)."]
    pub mod TPE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transceiver Standby Mode 0 - Disabled 1 - Enabled This register bit is connected to the output signal stby which can be used to control a standby mode of a transceiver. STBY cannot be set to 1 if TPE=1, TSONE=1 or TSALL=1. If the host sets STBY to 0 then the host needs to wait for the time required by the transceiver to start up before the host requests a new transmission."]
    pub mod STBY {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Listen Only Mode 0 - Disabled 1 - Enabled LOM cannot be set if TPE, TSONE or TSALL is set. No transmission can be started if LOM is enabled and LBME is disabled. LOM=1 and LBME=0 disables all transmissions. LOM=1 and LBME=1 disables the ACK for received frames and error frames, but enables the transmission of own frames."]
    pub mod LOM {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmit Buffer Select Selects the transmit buffer to be loaded with a message. Use the TBUF registers for access. TBSEL needs to be stable all the time the TBUF registers are written and when TSNEXT is set. 0 - PTB (high-priority buffer) 1 - STB The bit will be reset to the hardware reset value if (TTEN=1 and TTTBM=1)"]
    pub mod TBSEL {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmission Secondary STATus bits If TTEN=0 or TTTBM=0: 00 – STB is empty 01 – STB is less than or equal to half full 10 – STB is more than half full 11 – STB is full If the STB is disabled using STB_DISABLE, then TSSTAT=00. If TTEN=1 and TTTBM=1: 00 – PTB and STB are empty 01 – PTB and STB are not empty and not full 11 – PTB and STB are full"]
    pub mod TSSTAT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TTCAN Transmit Buffer Mode If TTEN=0 then TTTBM is ignored, otherwise the following is valid: 0 - separate PTB and STB, behavior defined by TSMODE 1 - full TTCAN support: buffer slots selectable by TBPTR and TTPTR For event-driven CAN communication (TTEN=0), the system provides PTB and STB and the behavior of the STB is defined by TSMODE. Then TTTBM is ignored. For time-triggered CAN communication (TTEN=1) with full support of all features including time-triggered transmissions, TTTBM=1 needs to be chosen. Then the all TB slots are addressable using TTPTR and TBPTR. For time-triggered CAN communication (TTEN=1) with only support of reception timestamps, TTTBM=0 can be chosen. Then the transmit buffer acts as in event-driven mode and the behavior can be selected by TSMODE. TTTBM shall be switched only if the TBUF is empty."]
    pub mod TTTBM {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmit buffer Secondary operation MODE 0 - FIFO mode 1 - priority decision mode In FIFO mode frames are transmitted in the order in that they are written into the STB. In priority decision mode the frame with the highest priority in the STB is automatically transmitted first. The ID of a frame is used for the priority decision. A lower ID means a higher priority of a frame. A frame in the PTB has always the highest priority regardless of the ID. TSMODE shall be switched only if the STB if empty"]
    pub mod TSMODE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmit buffer Secondary NEXT 0 - no action 1 - STB slot filled, select next slot. After all frame bytes are written to the TBUF registers, the host controller has to set TSNEXT to signal that this slot has been filled. Then the CAN-CTRL core connects the TBUF registers to the next slot. Once a slot is marked as filled a transmission can be started using TSONE or TSALL. It is possible to set TSNEXT and TSONE or TSALL together in one write access. TSNEXT has to be set by the host controller and is automatically reset by the CAN-CTRL core immediately after it was set. Setting TSNEXT is meaningless if TBSEL=0. In this case TSNEXT is ignored and automatically cleared. It does not do any harm. If all slots of the STB are filled, TSNEXT stays set until a slot becomes free. TSNEXT has no meaning in TTCAN mode and is fixed to 0."]
    pub mod TSNEXT {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CAN FD ISO mode 0 - Bosch CAN FD (non-ISO) mode 1 - ISO CAN FD mode (ISO 11898-1:2015) ISO CAN FD mode has a different CRC initialization value and an additional stuff bit count. Both modes are incompatible and must not be mixed in one CAN network. This bit has no impact to CAN 2.0B. This bit is only writeable if RESET=1."]
    pub mod FD_ISO {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive buffer STATus 00 - empty 01 - > empty and < almost full (AFWL) 10 - \u{f0b3} almost full (programmable threshold by AFWL) but not full and no overflow 11 - full (stays set in case of overflow – for overflow signaling see ROV)"]
    pub mod RSTAT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive Buffer stores ALL data frames 0 – normal operation 1 – RB stores correct data frames as well as data frames with error"]
    pub mod RBALL {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive buffer RELease The host controller has read the actual RB slot and releases it. Afterwards the CAN-CTRL core points to the next RB slot. RSTAT gets updated. 1 – Release: The host has read the RB. 0 – No release"]
    pub mod RREL {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive buffer OVerflow 1 – Overflow. At least one message is lost. 0 – No Overflow. ROV is cleared by setting RREL=1."]
    pub mod ROV {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive buffer Overflow Mode In case of a full RBUF when a new message is received, then ROM selects the following: 1 – The new message will not be stored. 0 – The oldest message will be overwritten."]
    pub mod ROM {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Self-ACKnowledge 0 – no self-ACK 1 – self-ACK when LBME=1"]
    pub mod SACK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Receive and Transmit Interrupt Enable Register RTIE"]
pub mod RTIE {
    #[doc = "If TTEN=0 or TTTBM=0: Transmit Secondary buffer Full Flag 1 - The STB is filled with the maximal number of messages. 0 - The STB is not filled with the maximal number of messages. If the STB is disabled using STB_DISABLE, then TSFF=0. If TTEN=1 and TTTBM=1: Transmit buffer Slot Full Flag 1 - The buffer slot selected by TBPTR is filled. 0 - The buffer slot selected by TBPTR is empty."]
    pub mod TSFF {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error Interrupt Enable 0 – Disabled, 1 – Enabled"]
    pub mod EIE {
        pub const offset: u8 = 1;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmission Secondary Interrupt Enable 0 – Disabled, 1 – Enabled"]
    pub mod TSIE {
        pub const offset: u8 = 2;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmission Primary Interrupt Enable 0 – Disabled, 1 – Enabled"]
    pub mod TPIE {
        pub const offset: u8 = 3;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RB Almost Full Interrupt Enable 0 – Disabled, 1 – Enabled"]
    pub mod RAFIE {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RB Full Interrupt Enable 0 – Disabled, 1 – Enabled"]
    pub mod RFIE {
        pub const offset: u8 = 5;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RB Overrun Interrupt Enable 0 – Disabled, 1 – Enabled"]
    pub mod ROIE {
        pub const offset: u8 = 6;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive Interrupt Enable 0 – Disabled, 1 – Enabled"]
    pub mod RIE {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Receive and Transmit Interrupt Flag Register RTIF (0xa5)"]
pub mod RTIF {
    #[doc = "Abort Interrupt Flag 1 - After setting TPA or TSA the appropriated message(s) have been aborted. It is recommended to not set both TPA and TSA simultaneously because both source AIF. 0 - No abort has been executed. The AIF does not have an associated enable register."]
    pub mod AIF {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error Interrupt Flag 1 - The border of the error warning limit has been crossed in either direction, or the BUSOFF bit has been changed in either direction. 0 - There has been no change."]
    pub mod EIF {
        pub const offset: u8 = 1;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmission Secondary Interrupt Flag 1 - The requested transmission of the STB has been successfully completed. 0 - No transmission of the STB has been completed successfully. In TTCAN mode TSIF will signal all successful transmissions, regardless of storage location of the message."]
    pub mod TSIF {
        pub const offset: u8 = 2;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmission Primary Interrupt Flag 1 - The requested transmission of the PTB has been successfully completed. 0 - No transmission of the PTB has been completed. In TTCAN mode, TPIF will never be set. Then only TSIF is valid."]
    pub mod TPIF {
        pub const offset: u8 = 3;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RB Almost Full Interrupt Flag 1 - number of filled RB slots >= AFWL_i 0 - number of filled RB slots < AFWL_i"]
    pub mod RAFIF {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RB Full Interrupt Flag 1 - All RBs are full. If no RB will be released until the next valid message is received, the oldest message will be lost. 0 - The RB FIFO is not full."]
    pub mod RFIF {
        pub const offset: u8 = 5;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RB Overrun Interrupt Flag 1 - At least one received message has been overwritten in the RB. 0 - No RB overwritten. In case of an overrun both ROIF and RFIF will be set."]
    pub mod ROIF {
        pub const offset: u8 = 6;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive Interrupt Flag 1 - Data or a remote frame has been received and is available in the receive buffer. 0 - No frame has been received."]
    pub mod RIF {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ERRor INTerrupt Enable and Flag Register ERRINT"]
pub mod ERRINT {
    #[doc = "Bus Error Interrupt Flag"]
    pub mod BEIF {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bus Error Interrupt Enable"]
    pub mod BEIE {
        pub const offset: u8 = 1;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Arbitration Lost Interrupt Flag"]
    pub mod ALIF {
        pub const offset: u8 = 2;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Arbitration Lost Interrupt Enable"]
    pub mod ALIE {
        pub const offset: u8 = 3;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error Passive Interrupt Flag. EPIF will be activated if the error status changes from error active to error passive or vice versa and if this interrupt is enabled."]
    pub mod EPIF {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error Passive Interrupt Enable"]
    pub mod EPIE {
        pub const offset: u8 = 5;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error Passive mode active 0 - not active (node is error active) 1 - active (node is error passive)"]
    pub mod EPASS {
        pub const offset: u8 = 6;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error WARNing limit reached 1 - One of the error counters RECNT or TECNT is equal or bigger than EWL0 - The values in both counters are less than EWL."]
    pub mod EWARN {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Warning Limits Register LIMIT"]
pub mod LIMIT {
    #[doc = "Programmable Error Warning Limit = (EWL+1)*8. Possible Limit values: 8, 16, … 128. The value of EWL controls EIF."]
    pub mod EWL {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "receive buffer Almost Full Warning Limit AFWL defines the internal warning limit AFWL_i with being the number of availableRB slots. AFWL_i is compared to the number of filled RB slots and triggers RAFIF if equal. Thevalid range of . AFWL = 0 is meaningless and automatically treated as 0x1. (Note that AFWL is meant in this rule and not AFWL_i.) AFWL_i > nRB is meaningless and automatically treated as nRB. AFWL_i = nRB is a valid value, but note that RFIF also exists."]
    pub mod AFWL {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Bit Timing Register(Slow Speed)"]
pub mod S_PRESC {
    #[doc = "Bit Timing Segment 1 (slow speed) The sample point will be set to after start of bit time."]
    pub mod S_SEG_1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Timing Segment 2 (slow speed) Time after the sample point."]
    pub mod S_SEG_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Synchronization Jump Width (slow speed) The Synchronization Jump Width is the maximum time forshortening or lengthening the Bit Time for resynchronization, where TQ is a timequanta."]
    pub mod S_SJW {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Prescaler (slow speed) The prescaler divides the system clock to get the time quanta clock tq_clk.Valid range PRESC=\\[0x00, 0xff\\] results in divider values 1 to 256."]
    pub mod S_PRESC {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Bit Timing Register(Fast Speed)"]
pub mod F_PRESC {
    #[doc = "Bit Timing Segment 1 (fast speed) The sample point will be set to after start of bit time."]
    pub mod F_SEG_1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Timing Segment 2 (fast speed) Time after the sample point"]
    pub mod F_SEG_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Synchronization Jump Width (fast speed) The Synchronization Jump Width is the maximum time forshortening or lengthening the Bit Time for resynchronization, where TQ is a timequanta."]
    pub mod F_SJW {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Prescaler (fast speed) The prescaler divides the system clock to get the time quanta clock tq_clk.Valid range PRESC=\\[0x00, 0xff\\] results in divider values 1 to 256."]
    pub mod F_PRESC {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Error and Arbitration Lost Capture Register EALCAP"]
pub mod EALCAP {
    #[doc = "Arbitration Lost Capture (bit position in the frame where the arbitration has been lost)"]
    pub mod ALC {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Kind Of ERror (Error code) 000 - no error 001 - BIT ERROR 010 - FORM ERROR 011 - STUFF ERROR 100 - ACKNOWLEDGEMENT ERROR 101 - CRC ERROR 110 - OTHER ERROR(dominant bits after own error flag, received active Error Flag too long,dominant bit during Passive-Error-Flag after ACK error) 111 - not used KOER is updated with each new error. Therefore it stays untouched when frames aresuccessfully transmitted or received."]
    pub mod KOER {
        pub const offset: u8 = 5;
        pub const mask: u8 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Transmitter Delay Compensation Register TDC"]
pub mod TDC {
    #[doc = "Secondary Sample Point OFFset The transmitter delay plus SSPOFF defines the time of the secondary sample point for TDC. SSPOFF is given as a number of TQ."]
    pub mod SSPOFF {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmitter Delay Compensation ENable TDC will be activated during the data phase of a CAN FD frame if BRS is active if TDCEN=1."]
    pub mod TDCEN {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Error Counter Registers RECNT"]
pub mod RECNT {
    #[doc = "Receive Error CouNT (number of errors during reception) RECNT is incremented and decremented as defined in the CAN specification. RECNT does not overflow. If TXB=1, then the error counters are frozen."]
    pub mod RECNT {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Error Counter Registers TECNT"]
pub mod TECNT {
    #[doc = "Transmit Error CouNT (number of errors during transmission) TECNT is incremented and decremented as defined in the CAN specification. In case of the “bus off state” TECNT may overflow. If TXB=1, then the error counters are frozen."]
    pub mod TECNT {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Acceptance Filter Control Register ACFCTRL"]
pub mod ACFCTRL {
    #[doc = "acceptance filter address ACFADR points to a specific acceptance filter. The selected filter is accessible using theregisters ACF_x. Bit SELMASK selects between acceptance code and mask for theselected acceptance filter. A value of ACFADR>ACF_NUMBER-1 is meaningless and automatically treated as value ACF_NUMBER-1. ACF_NUMBER = 16."]
    pub mod ACFADR {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SELect acceptance MASK 0 - Registers ACF_x point to acceptance code 1 - Registers ACF_x point to acceptance mask. ACFADR selects one specific acceptance filter."]
    pub mod SELMASK {
        pub const offset: u8 = 5;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CiA 603 Time-Stamping TIMECFG"]
pub mod TIMECFG {
    #[doc = "TIME-stamping ENable 0 – disabled 1 – enabled"]
    pub mod TIMEEN {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TIME-stamping POSition 0 – SOF1 – EOF (see Chapter 7)TIMEPOS can only be changed if TIMEEN=0, but it is possible to modify TIMPOS withthe same write access that sets TIMEEN=1."]
    pub mod TIMEPOS {
        pub const offset: u8 = 1;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Acceptance Filter Enable ACF_EN"]
pub mod ACF_EN {
    #[doc = "Acceptance filter Enable 1 - acceptance filter enabled 0 - acceptance filter disable Each acceptance filter (AMASK / ACODE) can be individually enabled or disabled. Disabled filters reject a message. Only enabled filters can accept a message if the appropriate AMASK / ACODE configuration matches."]
    pub mod ACF_EN {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Acceptance CODE ACODE or ACMASK"]
pub mod ACF {
    #[doc = "Acceptance CODE 1 - ACC bit value to compare with ID bit of the received message 0 - ACC bit value to compare with ID bit of the received message ACODE_x(10:0) will be used for extended frames. ACODE_x(28:0) will be used for extended frames. Only filter 0 is affected by the power-on reset. Acceptance MASK(if SELMASK ==1 ) 1 - acceptance check for these bits of receive identifier disabled 0 - acceptance check for these bits of receive identifier enable AMASK_x(10:0) will be used for extended frames. AMASK_x(28:0) will be used for extended frames. Disabled bits result in accepting the message. Therefore the default configuration after reset for filter 0 accepts all messages. Only filter 0 is affected by the power-on reset."]
    pub mod CODE_MASK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Acceptance mask IDE bit value If AIDEE=1 then: 1 - acceptance filter accepts only extended frames 0 - acceptance filter accepts only standard frames Only filter 0 is affected by the power-on reset. All other filters stay uninitialized."]
    pub mod AIDE {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Acceptance mask IDE bit check enable 1 - acceptance filter accepts either standard or extended as defined by AIDE 0 - acceptance filter accepts both standard or extended frames Only filter 0 is affected by the power-on reset. All other filters stay uninitialized."]
    pub mod AIDEE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Version Information VER"]
pub mod VER {
    #[doc = "Version of CAN-CTRL, given as decimal value. VER_1 holds the major version and VER_0 the minor version.Example: version 5x16N00S00 is represented by VER_1=5 and VER_0=16"]
    pub mod VERSION {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "TTCAN: TB Slot Pointer TBSLOT"]
pub mod TBSLOT {
    #[doc = "Pointer to a TB message slot. 0x00 - Pointer to the PTB others - Pointer to a slot in the STB The message slot pointed to by TBPTR is readable / writable using the TBUF registers. Write access is only possible if TSFF=0. Setting TBF to 1 marks the selected slot asfilled and setting TBE to 1 marks the selected slot as empty. TBSEL and TSNEXT are unused in TTCAN mode and have no meaning. TBPTR can only point to buffer slots, that exist in the hardware. Unusable bits ofTBPTR are fixed to 0. TBPTR is limited to the PTB and 63 STB slots. More slots cannot be used in TTCANmode.If TBPTR is too big and points to a slot that is not available, then TBF and TBE arereset automatically and no action takes place."]
    pub mod TBPTR {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set TB slot to “Filled” 1 - slot selected by TBPTR shall be marked as “filled” 0 - no actionTBF is automatically reset to 0 as soon as the slot is marked as filled and TSFF=1. If both TBF and TBE are set, then TBE wins."]
    pub mod TBF {
        pub const offset: u8 = 6;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set TB slot to “Empty” 1 - slot selected by TBPTR shall be marked as “empty” 0 - no actionTBE is automatically reset to 0 as soon as the slot is marked as empty and TSFF=0. If atransmission from this slot is active, then TBE stays set as long as either the transmission completes or after a transmission error or arbitration loss the transmissionis not active any more. If both TBF and TBE are set, then TBE wins"]
    pub mod TBE {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "TTCAN: Time Trigger Configuration TTCFG"]
pub mod TTCFG {
    #[doc = "Time Trigger Enable 1 - TTCAN enabled, timer is running0 - disabled"]
    pub mod TTEN {
        pub const offset: u8 = 0;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TTCAN Timer PRESCaler 00b - 1 01b - 2 10b - 4 11b - 8 The TTCAN time base is a CAN bittime defined by S_PRES, S_SEG_1 and S_SEG_2.With T_PRESC an additional prescaling factor of 1, 2, 4 or 8 is defined. T_PRESC can only be modified if TTEN=0, but it is possible to modify T_PRESC and setTTEN simultaneously with one write access."]
    pub mod T_PRESC {
        pub const offset: u8 = 1;
        pub const mask: u8 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Time Trigger Interrupt Flag TTIF will be set if TTIE is set and the cycle time is equal to the trigger time TT_TRIG. Writing an one to TTIF resets it. Writing a zero has no impact.TTIF will be set only once. If TT_TRIG gets not updated, then TTIF will be not setagain in the next basic cycle."]
    pub mod TTIF {
        pub const offset: u8 = 3;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Time Trigger Interrupt Enable If TTIE is set, then TTIF will be set if the cycle time is equal to the trigger timeTT_TRIG."]
    pub mod TTIE {
        pub const offset: u8 = 4;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Trigger Error Interrupt Flag The conditions when TEIF will be set, are defined in Chapter 6.4. There is no bit toenable or disable the handling of TEIF"]
    pub mod TEIF {
        pub const offset: u8 = 5;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Watch Trigger Interrupt Flag WTIF will be set if the cycle count reaches the limited defined by TT_WTRIG and WTIE is set."]
    pub mod WTIF {
        pub const offset: u8 = 6;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Watch Trigger Interrupt Enable"]
    pub mod WTIE {
        pub const offset: u8 = 7;
        pub const mask: u8 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "TTCAN: Reference Message REF_MSG"]
pub mod REF_MSG {
    #[doc = "REFerence message IDentifier. If REF_IDE is 1 - REF_ID(28:0) is valid (extended ID) 0 - REF_ID(10:0) is valid (standard ID) REF_ID is used in TTCAN mode to detect a reference message. This holds for time slaves (reception) as well as for the time master (transmission). If the reference message is detected and there are no errors, then the Sync_Mark of this frame will become the Ref_Mark. REF_ID(2:0) is not tested and therefore the appropriate register bits are forced to 0. These bits are used for up to 8 potential time masters. CAN-CTRL recognizes the reference message only by ID. The payload is not tested. Additional note: A time master will transmit a reference message in the same way as a normal frame. REF_ID is intended for detection of a successful transmission of a reference message."]
    pub mod REF_MSG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "REFerence message IDE bit."]
    pub mod REF_IDE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "TTCAN: Trigger Configuration TRIG_CFG"]
pub mod TRIG_CFG {
    #[doc = "Transmit Trigger TB slot Pointer If TTPTR is too big and points to a slot that is not available, then TEIF is set and no new trigger can be activated after a write access to TT_TRIG_1. If TTPTR points to an empty slot, then TEIF will be set at the moment, when the trigger time is reached."]
    pub mod TTPTR {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Trigger Type 000b - Immediate Trigger for immediate transmission 001b - Time Trigger for receive triggers 010b - Single Shot Transmit Trigger for exclusive time windows 011b - Transmit Start Trigger for merged arbitrating time windows 100b - Transmit Stop Trigger for merged arbitrating time windows others - no action The time of the trigger is defined by TT_TRIG. TTPTR selects the TB slot for the transmit triggers. See Chapter 6.4 for more details."]
    pub mod TTYPE {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmit Enable Window For a single shot transmit trigger there is a time of up to 16 ticks of the cycle time where the frame is allowed to start. TWE+1 defines the number of ticks. TEW=0 is a valid setting and shortens the transmit enable window to 1 tick"]
    pub mod TEW {
        pub const offset: u16 = 12;
        pub const mask: u16 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "TTCAN: Trigger Time TT_TRIG"]
pub mod TT_TRIG {
    #[doc = "Trigger Time TT_TRIG(15:0) defines the cycle time for a trigger. For a transmission trigger theearliest point of transmission of the SOF of the appropriate frame will be TT_TRIG+1."]
    pub mod TT_TRIG {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "TTCAN: Watch Trigger Time TT_WTRIG"]
pub mod TT_WTRIG {
    #[doc = "Watch Trigger Time TT_WTRIG(15:0) defines the cycle time for a watch trigger. The initial watch trigger isthe maximum cycle time 0xffff."]
    pub mod TT_WTRIG {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
