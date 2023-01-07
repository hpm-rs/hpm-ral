#[doc = "DRAM"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Control Register"]
    pub CTRL: crate::RWRegister<u32>,
    _reserved0: [u8; 0x04],
    #[doc = "Bus (AXI) Weight Control Register 0"]
    pub BMW0: crate::RWRegister<u32>,
    #[doc = "Bus (AXI) Weight Control Register 1"]
    pub BMW1: crate::RWRegister<u32>,
    #[doc = "Base Register 0 (for SDRAM CS0 device)"]
    pub BR_BASE0: crate::RWRegister<u32>,
    #[doc = "Base Register 1 (for SDRAM CS1 device)"]
    pub BR_BASE1: crate::RWRegister<u32>,
    _reserved1: [u8; 0x20],
    #[doc = "Interrupt Enable Register"]
    pub INTEN: crate::RWRegister<u32>,
    #[doc = "Interrupt Status Register"]
    pub INTR: crate::RWRegister<u32>,
    #[doc = "SDRAM Control Register 0"]
    pub SDRCTRL0: crate::RWRegister<u32>,
    #[doc = "SDRAM Control Register 1"]
    pub SDRCTRL1: crate::RWRegister<u32>,
    #[doc = "SDRAM Control Register 2"]
    pub SDRCTRL2: crate::RWRegister<u32>,
    #[doc = "SDRAM Control Register 3"]
    pub SDRCTRL3: crate::RWRegister<u32>,
    _reserved2: [u8; 0x40],
    #[doc = "IP Command Control Register 0"]
    pub SADDR: crate::RWRegister<u32>,
    #[doc = "IP Command Control Register 1"]
    pub DATSZ: crate::RWRegister<u32>,
    #[doc = "IP Command Control Register 2"]
    pub BYTEMSK: crate::RWRegister<u32>,
    #[doc = "IP Command Register"]
    pub IPCMD: crate::RWRegister<u32>,
    #[doc = "TX DATA Register"]
    pub IPTX: crate::RWRegister<u32>,
    _reserved3: [u8; 0x0c],
    #[doc = "RX DATA Register"]
    pub IPRX: crate::RWRegister<u32>,
    _reserved4: [u8; 0x0c],
    #[doc = "Status Register 0"]
    pub STAT0: crate::RWRegister<u32>,
    _reserved5: [u8; 0x8c],
    #[doc = "Delay Line Config Register"]
    pub DLYCFG: crate::RWRegister<u32>,
}
#[doc = "Control Register"]
pub mod CTRL {
    #[doc = "Software Reset Reset all internal logic in SEMC except configuration register"]
    pub mod RST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Module Disable 0b - Module enabled 1b - Module disabled"]
    pub mod DIS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DQS (read strobe) mode 0b - Dummy read strobe loopbacked internally 1b - Dummy read strobe loopbacked from DQS pad"]
    pub mod DQS {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Command Execution timeout cycles When Command Execution time exceed this timeout cycles, IPCMDERR or AXICMDERR interrupt is generated. When CTO is set to zero, timeout cycle is 256*1024 cycle. otherwisee timeout cycle is CTO*1024 cycle."]
    pub mod CTO {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bus timeout cycles AXI Bus timeout cycle is as following (255*(2^BTO)): 00000b - 255*1 00001-11110b - 255*2 - 255*2^30 11111b - 255*2^31"]
    pub mod BTO {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Bus (AXI) Weight Control Register 0"]
pub mod BMW0 {
    #[doc = "Weight of QOS calculation. AXI bus access has AxQOS signal set, which is used as a priority indicator for the associated write or read transaction. A higher value indicates a higher priority transaction. AxQOS is multiplied by WQOS to get weight score."]
    pub mod QOS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Weight of AGE calculation. Each command in queue has an age signal to indicate its wait period. It is multiplied by WAGE to get weight score."]
    pub mod AGE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Weight of Slave Hit without read/write switch. This weight score is valid when queue command's slave is same as current executing command without read/write operation switch."]
    pub mod SH {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Weight of slave hit with Read/Write Switch. This weight score is valid when queue command's slave is same as current executing command with read/write operation switch."]
    pub mod RWS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Bus (AXI) Weight Control Register 1"]
pub mod BMW1 {
    #[doc = "Weight of QOS calculation. AXI bus access has AxQOS signal set, which is used as a priority indicator for the associated write or read transaction. A higher value indicates a higher priority transaction. AxQOS is multiplied by WQOS to get weight score."]
    pub mod QOS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Weight of AGE calculation. Each command in queue has an age signal to indicate its wait period. It is multiplied by WAGE to get weight score."]
    pub mod AGE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Weight of Slave Hit without read/write switch. This weight score is valid when queue command's slave is same as current executing command without read/write operation switch."]
    pub mod PH {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Weight of slave hit with Read/Write Switch. This weight score is valid when queue command's slave is same as current executing command with read/write operation switch."]
    pub mod RWS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Weight of Bank Rotation. This weight score is valid when queue command's bank is not same as current executing command."]
    pub mod BR {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Base Register 0 (for SDRAM CS0 device)"]
pub mod BR_BASE0 {
    #[doc = "Valid"]
    pub mod VLD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Memory size 00000b - 4KB 00001b - 8KB 00010b - 16KB 00011b - 32KB 00100b - 64KB 00101b - 128KB 00110b - 256KB 00111b - 512KB 01000b - 1MB 01001b - 2MB 01010b - 4MB 01011b - 8MB 01100b - 16MB 01101b - 32MB 01110b - 64MB 01111b - 128MB 10000b - 256MB 10001b - 512MB 10010b - 1GB 10011b - 2GB 10100-11111b - 4GB"]
    pub mod SIZE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Base Address This field determines high position 20 bits of SoC level Base Address. SoC level Base Address low position 12 bits are all zero."]
    pub mod BASE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Base Register 1 (for SDRAM CS1 device)"]
pub mod BR_BASE1 {
    #[doc = "Valid"]
    pub mod VLD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Memory size 00000b - 4KB 00001b - 8KB 00010b - 16KB 00011b - 32KB 00100b - 64KB 00101b - 128KB 00110b - 256KB 00111b - 512KB 01000b - 1MB 01001b - 2MB 01010b - 4MB 01011b - 8MB 01100b - 16MB 01101b - 32MB 01110b - 64MB 01111b - 128MB 10000b - 256MB 10001b - 512MB 10010b - 1GB 10011b - 2GB 10100-11111b - 4GB"]
    pub mod SIZE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Base Address This field determines high position 20 bits of SoC level Base Address. SoC level Base Address low position 12 bits are all zero."]
    pub mod BASE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Enable Register"]
pub mod INTEN {
    #[doc = "IP command done interrupt enable 0b - Interrupt is disabled 1b - Interrupt is enabled"]
    pub mod IPCMDDONE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IP command error interrupt enable 0b - Interrupt is disabled 1b - Interrupt is enabled"]
    pub mod IPCMDERR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AXI command error interrupt enable 0b - Interrupt is disabled 1b - Interrupt is enabled"]
    pub mod AXICMDERR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AXI BUS error interrupt enable 0b - Interrupt is disabled 1b - Interrupt is enabled"]
    pub mod AXIBUSERR {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Status Register"]
pub mod INTR {
    #[doc = "IP command normal done interrupt"]
    pub mod IPCMDDONE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IP command error done interrupt IP command error interrupt is generated in following case: • IP Command Address target invalid device space • IP Command Code unsupported • IP Command triggered when previous command"]
    pub mod IPCMDERR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AXI command error interrupt AXI command error interrupt is generated when AXI command execution timeout."]
    pub mod AXICMDERR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AXI bus error interrupt AXI Bus error interrupt is generated in following cases: • AXI address is invalid • AXI 8-bit or 16-bit WRAP write/read"]
    pub mod AXIBUSERR {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SDRAM Control Register 0"]
pub mod SDRCTRL0 {
    #[doc = "Port Size 00b - 8bit 01b - 16bit 10b - 32bit"]
    pub mod PORTSZ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "high band select 0: use data\\[15:0\\] for 16bit SDRAM; 1: use data\\[31:16\\] for 16bit SDRAM; only used when Port Size is 16bit(PORTSZ=01b)"]
    pub mod HIGHBAND {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Burst Length 000b - 1 001b - 2 010b - 4 011b - 8 100b - 8 101b - 8 110b - 8 111b - 8"]
    pub mod BURSTLEN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Column 8 selection bit 0b - Column address bit number is decided by COL field. 1b - Column address bit number is 8. COL field is ignored."]
    pub mod COL8 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Column address bit number 00b - 12 bit 01b - 11 bit 10b - 10 bit 11b - 9 bit"]
    pub mod COL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CAS Latency 00b - 1 01b - 1 10b - 2 11b - 3"]
    pub mod CAS {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "2 Bank selection bit 0b - SDRAM device has 4 banks. 1b - SDRAM device has 2 banks."]
    pub mod BANK2 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SDRAM Control Register 1"]
pub mod SDRCTRL1 {
    #[doc = "PRECHARGE to ACT/Refresh wait time It is promised PRE2ACT+1 clock cycles delay between PRECHARGE/PRECHARGE_ALL commandto ACTIVE/REFRESH command. This could help to meet tRP timing requirement by SDRAM device."]
    pub mod PRE2ACT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ACT to Read/Write wait time It is promised ACT2RW+1 clock cycles delay between ACTIVE command to READ/WRITE command.This could help to meet tRCD timing requirement by SDRAM device."]
    pub mod ACT2RW {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Refresh recovery time It is promised RFRC+1 clock cycles delay between REFRESH command to ACTIVE command. Thiscould help to meet tRFC timing requirement by SDRAM device."]
    pub mod RFRC {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write recovery time It is promised WRC+1 clock cycles delay between WRITE command to PRECHARGE/PRECHARGE_ALL command. This could help to meet tWR timing requirement by SDRAM device."]
    pub mod WRC {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CKE OFF minimum time It is promised clock suspend last at leat CKEOFF+1 clock cycles."]
    pub mod CKEOFF {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ACT to Precharge minimum time It is promised ACT2PRE+1 clock cycles delay between ACTIVE command to PRECHARGE/PRECHARGE_ALL command."]
    pub mod ACT2PRE {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SDRAM Control Register 2"]
pub mod SDRCTRL2 {
    #[doc = "Self Refresh Recovery time It is promised SRRC+1 clock cycles delay between Self-REFRESH command to any command."]
    pub mod SRRC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Refresh to Refresh wait time It is promised REF2REF+1 clock cycles delay between REFRESH command to REFRESH command. This could help to meet tRFC timing requirement by SDRAM device."]
    pub mod REF2REF {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ACT to ACT wait time It is promised ACT2ACT+1 clock cycles delay between ACTIVE command to ACTIVE command. This could help to meet tRRD timing requirement by SDRAM device."]
    pub mod ACT2ACT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SDRAM Idle timeout It closes all opened pages if the SDRAM idle time lasts more than idle timeout period. SDRAM is considered idle when there is no AXI Bus transfer and no SDRAM command pending. 00000000b - IDLE timeout period is 256*Prescale period. 00000001-11111111b - IDLE timeout period is ITO*Prescale period."]
    pub mod ITO {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SDRAM Control Register 3"]
pub mod SDRCTRL3 {
    #[doc = "Refresh enable"]
    pub mod REN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Refresh burst length It could send multiple Auto-Refresh command in one burst when REBL is set to non-zero. The number of Auto-Refresh command cycle sent to all SDRAM device in one refresh period is as following. 000b - 1 001b - 2 010b - 3 011b - 4 100b - 5 101b - 6 110b - 7 111b - 8"]
    pub mod REBL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Prescaler timer period Prescaler timer period is as following: 00000000b - 256*16 clock cycles 00000001-11111111b - PRESCALE*16 clock cycles"]
    pub mod PRESCALE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Refresh timer period Refresh timer period is as following: 00000000b - 256*Prescaler period 00000001-11111111b - RT*Prescaler period"]
    pub mod RT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Refresh urgent threshold Internal refresh request is generated on every Refresh period. Before internal request timer count up to urgent request threshold, the refresh request is considered as normal refresh request. Normal refresh request is handled in lower priority than any pending AXI command or IP command to SDRAM device. When internal request timer count up to this urgent threshold, refresh request is considered as urgent refresh request. Urgent refresh request is handled in higher priority than any pending AXI command or IP command to SDRAM device. NOTE: When urgent threshold is no less than refresh period, refresh request is always considered as urgent refresh request. Refresh urgent threshold is as follwoing: 00000000b - 256*Prescaler period 00000001-11111111b - UT*Prescaler period"]
    pub mod UT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "IP Command Control Register 0"]
pub mod SADDR {
    #[doc = "Slave address"]
    pub mod SA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "IP Command Control Register 1"]
pub mod DATSZ {
    #[doc = "Data Size in Byte When IP command is not a write/read operation, DATSZ field would be ignored. 000b - 4 001b - 1 010b - 2 011b - 3 100b - 4 101b - 4 110b - 4 111b - 4"]
    pub mod DATSZ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "IP Command Control Register 2"]
pub mod BYTEMSK {
    #[doc = "Byte Mask for Byte 0 (IPTXD bit 7:0) 0b - Byte Unmasked 1b - Byte Masked"]
    pub mod BM0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Byte Mask for Byte 1 (IPTXD bit 15:8) 0b - Byte Unmasked 1b - Byte Masked"]
    pub mod BM1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Byte Mask for Byte 2 (IPTXD bit 23:16) 0b - Byte Unmasked 1b - Byte Masked"]
    pub mod BM2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Byte Mask for Byte 3 (IPTXD bit 31:24) 0b - Byte Unmasked 1b - Byte Masked"]
    pub mod BM3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "IP Command Register"]
pub mod IPCMD {
    #[doc = "SDRAM Commands: • 0x8: READ • 0x9: WRITE • 0xA: MODESET • 0xB: ACTIVE • 0xC: AUTO REFRESH • 0xD: SELF REFRESH • 0xE: PRECHARGE • 0xF: PRECHARGE ALL • Others: RSVD NOTE: SELF REFRESH is sent to all SDRAM devices because they shared same CLK pin."]
    pub mod CMD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This field should be written with 0x5AA5 when trigging an IP command for all device types. The memory device is selected by BRx settings and IPCR0 registers."]
    pub mod KEY {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "TX DATA Register"]
pub mod IPTX {
    #[doc = "Data"]
    pub mod DAT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "RX DATA Register"]
pub mod IPRX {
    #[doc = "Data"]
    pub mod DAT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status Register 0"]
pub mod STAT0 {
    #[doc = "Indicating whether it is in IDLE state. When IDLE=1, it is in IDLE state. There is no pending AXI command in internal queue and no pending device access."]
    pub mod IDLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Delay Line Config Register"]
pub mod DLYCFG {
    #[doc = "delay line enable"]
    pub mod DLYEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "delay line select, 0 for 1 cell, 31 for all 32 cells"]
    pub mod DLYSEL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "delay clock output enable, should be set after setting DLYEN and DLYSEL"]
    pub mod OE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
