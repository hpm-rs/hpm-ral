#[doc = "QEI0"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Control register"]
    pub CR: crate::RWRegister<u32>,
    #[doc = "Phase configure register"]
    pub PHCFG: crate::RWRegister<u32>,
    #[doc = "Watchdog configure register"]
    pub WDGCFG: crate::RWRegister<u32>,
    #[doc = "Phase index register"]
    pub PHIDX: crate::RWRegister<u32>,
    #[doc = "Tigger output enable register"]
    pub TRGOEN: crate::RWRegister<u32>,
    #[doc = "Read event enable register"]
    pub READEN: crate::RWRegister<u32>,
    #[doc = "Z comparator"]
    pub ZCMP: crate::RWRegister<u32>,
    #[doc = "Phase comparator"]
    pub PHCMP: crate::RWRegister<u32>,
    #[doc = "Speed comparator"]
    pub SPDCMP: crate::RWRegister<u32>,
    #[doc = "DMA request enable register"]
    pub DMAEN: crate::RWRegister<u32>,
    #[doc = "Status register"]
    pub SR: crate::RWRegister<u32>,
    #[doc = "Interrupt request register"]
    pub IRQEN: crate::RWRegister<u32>,
    #[doc = "Z counter"]
    pub COUNT_CURRENT_Z: crate::RWRegister<u32>,
    #[doc = "Phase counter"]
    pub COUNT_CURRENT_PH: crate::RWRegister<u32>,
    #[doc = "Speed counter"]
    pub COUNT_CURRENT_SPD: crate::RWRegister<u32>,
    #[doc = "Timer counter"]
    pub COUNT_CURRENT_TMR: crate::RWRegister<u32>,
    #[doc = "Z counter"]
    pub COUNT_READ_Z: crate::RWRegister<u32>,
    #[doc = "Phase counter"]
    pub COUNT_READ_PH: crate::RWRegister<u32>,
    #[doc = "Speed counter"]
    pub COUNT_READ_SPD: crate::RWRegister<u32>,
    #[doc = "Timer counter"]
    pub COUNT_READ_TMR: crate::RWRegister<u32>,
    #[doc = "Z snap register"]
    pub COUNT_SNAP0_Z: crate::RWRegister<u32>,
    #[doc = "Phase snap register"]
    pub COUNT_SNAP0_PH: crate::RWRegister<u32>,
    #[doc = "Speed snap register"]
    pub COUNT_SNAP0_SPD: crate::RWRegister<u32>,
    #[doc = "Timer snap register"]
    pub COUNT_SNAP0_TMR: crate::RWRegister<u32>,
    #[doc = "Z snap register 1"]
    pub COUNT_SNAP1_Z: crate::RWRegister<u32>,
    #[doc = "Phase snap register 1"]
    pub COUNT_SNAP1_PH: crate::RWRegister<u32>,
    #[doc = "Speed snap register 1"]
    pub COUNT_SNAP1_SPD: crate::RWRegister<u32>,
    #[doc = "Timer snap register 1"]
    pub COUNT_SNAP1_TMR: crate::RWRegister<u32>,
    #[doc = "Speed history"]
    pub SPDHIS_SPDHIS0: crate::RWRegister<u32>,
    #[doc = "Speed history 1"]
    pub SPDHIS_SPDHIS1: crate::RWRegister<u32>,
    #[doc = "Speed history 2"]
    pub SPDHIS_SPDHIS2: crate::RWRegister<u32>,
    #[doc = "Speed history 3"]
    pub SPDHIS_SPDHIS3: crate::RWRegister<u32>,
}
#[doc = "Control register"]
pub mod CR {
    #[doc = "00-abz; 01-pd; 10-ud; 11-reserved"]
    pub mod ENCTYP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- reset zcnt, spdcnt and tmrcnt to 0. reset phcnt to phidx"]
    pub mod RSTCNT {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- load phcnt, zcnt, spdcnt and tmrcnt into their snap registers when snapi input assert"]
    pub mod SNAPEN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- HOMEF will set at H falling edge when dir == 1 (positive rotation direction)"]
    pub mod HFDIR0 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- HOMEF will set at H falling edge when dir == 1 (negative rotation direction)"]
    pub mod HFDIR1 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- HOMEF will set at H rising edge when dir == 0 (positive rotation direction)"]
    pub mod HRDIR0 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- HOMEF will set at H rising edge when dir == 1 (negative rotation direction)"]
    pub mod HRDIR1 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- pause zcnt when PAUSE assert"]
    pub mod PAUSEZ {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- pause phcnt when PAUSE assert"]
    pub mod PAUSEPH {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- pause spdcnt when PAUSE assert"]
    pub mod PAUSESPD {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- reset zcnt when H assert"]
    pub mod HRSTZ {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- reset phcnt when H assert"]
    pub mod HRSTPH {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- reset spdcnt when H assert"]
    pub mod HRSTSPD {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- load phcnt, zcnt, spdcnt and tmrcnt into their read registers. Hardware auto-clear; read as 0"]
    pub mod READ {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Phase configure register"]
pub mod PHCFG {
    #[doc = "maximum phcnt number, phcnt will rollover to 0 when it upcount to phmax"]
    pub mod PHMAX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x001f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- phcnt will set to phidx when Z input assert"]
    pub mod PHCALIZ {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- zcnt will increment when phcnt upcount to phmax, decrement when phcnt downcount to 0 0- zcnt will increment or decrement when Z input assert"]
    pub mod ZCNTCFG {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Watchdog configure register"]
pub mod WDGCFG {
    #[doc = "watch dog timeout value"]
    pub mod WDGTO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- enable wdog counter"]
    pub mod WDGEN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Phase index register"]
pub mod PHIDX {
    #[doc = "phcnt reset value, phcnt will reset to phidx when phcaliz set to 1"]
    pub mod PHIDX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x001f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Tigger output enable register"]
pub mod TRGOEN {
    #[doc = "1- enable trigger output when zphf flag set"]
    pub mod ZPHFEN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- enable trigger output when poscmpf flag set"]
    pub mod POSCMPFEN {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- enable trigger output when homef flag set"]
    pub mod HOMEFEN {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- enable trigger output when wdg flag set"]
    pub mod WDGFEN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Read event enable register"]
pub mod READEN {
    #[doc = "1- load counters to their read registers when zphf flag set"]
    pub mod ZPHFEN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- load counters to their read registers when poscmpf flag set"]
    pub mod POSCMPFEN {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- load counters to their read registers when homef flag set"]
    pub mod HOMEFEN {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- load counters to their read registers when wdg flag set"]
    pub mod WDGFEN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Z comparator"]
pub mod ZCMP {
    #[doc = "zcnt postion compare value"]
    pub mod ZCMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Phase comparator"]
pub mod PHCMP {
    #[doc = "phcnt position compare value"]
    pub mod PHCMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x001f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "0- position compare need positive rotation 1- position compare need negative rotation"]
    pub mod DIRCMP {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- postion compare not include rotation direction"]
    pub mod DIRCMPDIS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- postion compare not include zcnt"]
    pub mod ZCMPDIS {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Speed comparator"]
pub mod SPDCMP {
    #[doc = "spdcnt position compare value"]
    pub mod SPDCMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA request enable register"]
pub mod DMAEN {
    #[doc = "1- generate dma request when zphf flag set"]
    pub mod ZPHFEN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- generate dma request when poscmpf flag set"]
    pub mod POSCMPFEN {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- generate dma request when homef flag set"]
    pub mod HOMEFEN {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- generate dma request when wdg flag set"]
    pub mod WDGFEN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status register"]
pub mod SR {
    #[doc = "z input flag"]
    pub mod ZPHF {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "postion compare match flag"]
    pub mod POSCMPF {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "home flag"]
    pub mod HOMEF {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "watchdog flag"]
    pub mod WDGF {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt request register"]
pub mod IRQEN {
    #[doc = "1- generate interrupt when zphf flag set"]
    pub mod ZPHIE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- generate interrupt when poscmpf flag set"]
    pub mod POSCMPIE {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- generate interrupt when homef flag set"]
    pub mod HOMEIE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- generate interrupt when wdg flag set"]
    pub mod WDGIE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Z counter"]
pub mod COUNT_CURRENT_Z {
    #[doc = "zcnt value"]
    pub mod ZCNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Phase counter"]
pub mod COUNT_CURRENT_PH {
    #[doc = "phcnt value"]
    pub mod PHCNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x001f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- b input is high 0- b input is low"]
    pub mod BSTAT {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- a input is high 0- a input is low"]
    pub mod ASTAT {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- reverse rotation 0- forward rotation"]
    pub mod DIR {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Speed counter"]
pub mod COUNT_CURRENT_SPD {
    #[doc = "spdcnt value"]
    pub mod SPDCNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- b input is high 0- b input is low"]
    pub mod BSTAT {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- a input is high 0- a input is low"]
    pub mod ASTAT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- reverse rotation 0- forward rotation"]
    pub mod DIR {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer counter"]
pub mod COUNT_CURRENT_TMR {
    #[doc = "32 bit free run timer"]
    pub mod TMRCNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Z counter"]
pub mod COUNT_READ_Z {
    #[doc = "zcnt value"]
    pub mod ZCNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Phase counter"]
pub mod COUNT_READ_PH {
    #[doc = "phcnt value"]
    pub mod PHCNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x001f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- b input is high 0- b input is low"]
    pub mod BSTAT {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- a input is high 0- a input is low"]
    pub mod ASTAT {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- reverse rotation 0- forward rotation"]
    pub mod DIR {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Speed counter"]
pub mod COUNT_READ_SPD {
    #[doc = "spdcnt value"]
    pub mod SPDCNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- b input is high 0- b input is low"]
    pub mod BSTAT {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- a input is high 0- a input is low"]
    pub mod ASTAT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- reverse rotation 0- forward rotation"]
    pub mod DIR {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer counter"]
pub mod COUNT_READ_TMR {
    #[doc = "32 bit free run timer"]
    pub mod TMRCNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Z snap register"]
pub mod COUNT_SNAP0_Z {
    #[doc = "zcnt value"]
    pub mod ZCNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Phase snap register"]
pub mod COUNT_SNAP0_PH {
    #[doc = "phcnt value"]
    pub mod PHCNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x001f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- b input is high 0- b input is low"]
    pub mod BSTAT {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- a input is high 0- a input is low"]
    pub mod ASTAT {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- reverse rotation 0- forward rotation"]
    pub mod DIR {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Speed snap register"]
pub mod COUNT_SNAP0_SPD {
    #[doc = "spdcnt value"]
    pub mod SPDCNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- b input is high 0- b input is low"]
    pub mod BSTAT {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- a input is high 0- a input is low"]
    pub mod ASTAT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- reverse rotation 0- forward rotation"]
    pub mod DIR {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer snap register"]
pub mod COUNT_SNAP0_TMR {
    #[doc = "32 bit free run timer"]
    pub mod TMRCNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Z snap register 1"]
pub mod COUNT_SNAP1_Z {
    #[doc = "zcnt value"]
    pub mod ZCNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Phase snap register 1"]
pub mod COUNT_SNAP1_PH {
    #[doc = "phcnt value"]
    pub mod PHCNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x001f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- b input is high 0- b input is low"]
    pub mod BSTAT {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- a input is high 0- a input is low"]
    pub mod ASTAT {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- reverse rotation 0- forward rotation"]
    pub mod DIR {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Speed snap register 1"]
pub mod COUNT_SNAP1_SPD {
    #[doc = "spdcnt value"]
    pub mod SPDCNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- b input is high 0- b input is low"]
    pub mod BSTAT {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- a input is high 0- a input is low"]
    pub mod ASTAT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- reverse rotation 0- forward rotation"]
    pub mod DIR {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer snap register 1"]
pub mod COUNT_SNAP1_TMR {
    #[doc = "32 bit free run timer"]
    pub mod TMRCNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Speed history"]
pub mod SPDHIS_SPDHIS0 {
    #[doc = "copy of spdcnt, load from spdcnt after any transition from a = low, b = low"]
    pub mod SPDHIS0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Speed history 1"]
pub mod SPDHIS_SPDHIS1 {
    #[doc = "copy of spdcnt, load from spdcnt after any transition from a = low, b = low"]
    pub mod SPDHIS0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Speed history 2"]
pub mod SPDHIS_SPDHIS2 {
    #[doc = "copy of spdcnt, load from spdcnt after any transition from a = low, b = low"]
    pub mod SPDHIS0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Speed history 3"]
pub mod SPDHIS_SPDHIS3 {
    #[doc = "copy of spdcnt, load from spdcnt after any transition from a = low, b = low"]
    pub mod SPDHIS0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
