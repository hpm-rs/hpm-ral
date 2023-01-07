#[doc = "HALL0"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Control Register"]
    pub CR: crate::RWRegister<u32>,
    #[doc = "Phase configure register"]
    pub PHCFG: crate::RWRegister<u32>,
    #[doc = "Watchdog configure register"]
    pub WDGCFG: crate::RWRegister<u32>,
    #[doc = "U,V,W configure register"]
    pub UVWCFG: crate::RWRegister<u32>,
    #[doc = "Trigger output enable register"]
    pub TRGOEN: crate::RWRegister<u32>,
    #[doc = "Read event enable register"]
    pub READEN: crate::RWRegister<u32>,
    _reserved0: [u8; 0x0c],
    #[doc = "DMA enable register"]
    pub DMAEN: crate::RWRegister<u32>,
    #[doc = "Status register"]
    pub SR: crate::RWRegister<u32>,
    #[doc = "Interrupt request enable register"]
    pub IRQEN: crate::RWRegister<u32>,
    #[doc = "W counter"]
    pub COUNT_CURRENT_W: crate::RWRegister<u32>,
    #[doc = "V counter"]
    pub COUNT_CURRENT_V: crate::RWRegister<u32>,
    #[doc = "U counter"]
    pub COUNT_CURRENT_U: crate::RWRegister<u32>,
    #[doc = "Timer counter"]
    pub COUNT_CURRENT_TMR: crate::RWRegister<u32>,
    #[doc = "W read register"]
    pub COUNT_READ_W: crate::RWRegister<u32>,
    #[doc = "V read register"]
    pub COUNT_READ_V: crate::RWRegister<u32>,
    #[doc = "U read register"]
    pub COUNT_READ_U: crate::RWRegister<u32>,
    #[doc = "Timer read register"]
    pub COUNT_READ_TMR: crate::RWRegister<u32>,
    #[doc = "W snap register 0"]
    pub COUNT_SNAP0_W: crate::RWRegister<u32>,
    #[doc = "V snap register 0"]
    pub COUNT_SNAP0_V: crate::RWRegister<u32>,
    #[doc = "Usnap register 0"]
    pub COUNT_SNAP0_U: crate::RWRegister<u32>,
    #[doc = "Timer snap register 0"]
    pub COUNT_SNAP0_TMR: crate::RWRegister<u32>,
    #[doc = "W snap register 1"]
    pub COUNT_SNAP1_W: crate::RWRegister<u32>,
    #[doc = "V snap register 1"]
    pub COUNT_SNAP1_V: crate::RWRegister<u32>,
    #[doc = "U snap register 1"]
    pub COUNT_SNAP1_U: crate::RWRegister<u32>,
    #[doc = "Timer snap register 1"]
    pub COUNT_SNAP1_TMR: crate::RWRegister<u32>,
    #[doc = "history register 0"]
    pub HIS_U_HIS0: crate::RWRegister<u32>,
    #[doc = "history register 1"]
    pub HIS_U_HIS1: crate::RWRegister<u32>,
    #[doc = "V histroy register 0"]
    pub HIS_V_HIS0: crate::RWRegister<u32>,
    #[doc = "V histroy register 1"]
    pub HIS_V_HIS1: crate::RWRegister<u32>,
    #[doc = "W histroy register 0"]
    pub HIS_W_HIS0: crate::RWRegister<u32>,
    #[doc = "W histroy register 1"]
    pub HIS_W_HIS1: crate::RWRegister<u32>,
}
#[doc = "Control Register"]
pub mod CR {
    #[doc = "set to reset all counter and related snapshots"]
    pub mod RSTCNT {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- load ucnt, vcnt, wcnt and tmrcnt into their snap registers when snapi input assert"]
    pub mod SNAPEN {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- load ucnt, vcnt, wcnt and tmrcnt into their read registers. Hardware auto-clear; read as 0"]
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
    #[doc = "delay clock cycles number"]
    pub mod DLYCNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit select delay start time: 1- start counting delay after pre-trigger 0- start counting delay after u,v,w toggle"]
    pub mod DLYSEL {
        pub const offset: u32 = 31;
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
#[doc = "U,V,W configure register"]
pub mod UVWCFG {
    #[doc = "the clock cycle number which the pre flag will set before the next uvw transition"]
    pub mod PRECNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Trigger output enable register"]
pub mod TRGOEN {
    #[doc = "1- enable trigger output when w flag set"]
    pub mod WFEN {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- enable trigger output when v flag set"]
    pub mod VFEN {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- enable trigger output when u flag set"]
    pub mod UFEN {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- enable trigger output when phdly flag set"]
    pub mod PHDLYEN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- enable trigger output when phpre flag set"]
    pub mod PHPREEN {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- enable trigger output when phupt flag set"]
    pub mod PHUPTEN {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- enable trigger output when wdg flag set"]
    pub mod WDGEN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Read event enable register"]
pub mod READEN {
    #[doc = "1- load counters to their read registers when w flag set"]
    pub mod WFEN {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- load counters to their read registers when v flag set"]
    pub mod VFEN {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- load counters to their read registers when u flag set"]
    pub mod UFEN {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- load counters to their read registers when phdly flag set"]
    pub mod PHDLYEN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- load counters to their read registers when phpre flag set"]
    pub mod PHPREEN {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- load counters to their read registers when phupt flag set"]
    pub mod PHUPTEN {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- load counters to their read registers when wdg flag set"]
    pub mod WDGEN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA enable register"]
pub mod DMAEN {
    #[doc = "1- generate dma request when w flag set"]
    pub mod WFEN {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- generate dma request when v flag set"]
    pub mod VFEN {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- generate dma request when u flag set"]
    pub mod UFEN {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- generate dma request when phdly flag set"]
    pub mod PHDLYEN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- generate dma request when phpre flag set"]
    pub mod PHPREEN {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- generate dma request when phupt flag set"]
    pub mod PHUPTEN {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- generate dma request when wdg flag set"]
    pub mod WDGEN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status register"]
pub mod SR {
    #[doc = "w flag, will set when w signal toggle"]
    pub mod WF {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "v flag, will set when v signal toggle"]
    pub mod VF {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "u flag, will set when u signal toggle"]
    pub mod UF {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "phase update delay flag, will set DLYCNT cycles after any of u, v, w signal toggle or after the phpre flag depands on DLYSEL setting"]
    pub mod PHDLYF {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "phase update pre flag, will set PRECNT cycles before any of u, v, w signal toggle"]
    pub mod PHPREF {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "phase update flag, will set when any of u, v, w signal toggle"]
    pub mod PHUPTF {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "watchdog count timeout flag"]
    pub mod WDGF {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt request enable register"]
pub mod IRQEN {
    #[doc = "1- generate interrupt request when w flag set"]
    pub mod WFIE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- generate interrupt request when v flag set"]
    pub mod VFIE {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- generate interrupt request when u flag set"]
    pub mod UFIE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- generate interrupt request when phdly flag set"]
    pub mod PHDLYIE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- generate interrupt request when phpre flag set"]
    pub mod PHPREIE {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- generate interrupt request when phupt flag set"]
    pub mod PHUPTIE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- generate interrupt request when wdg flag set"]
    pub mod WDGIE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "W counter"]
pub mod COUNT_CURRENT_W {
    #[doc = "wcnt counter"]
    pub mod WCNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "V counter"]
pub mod COUNT_CURRENT_V {
    #[doc = "vcnt counter"]
    pub mod VCNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "U counter"]
pub mod COUNT_CURRENT_U {
    #[doc = "ucnt counter"]
    pub mod UCNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "this bit indicate W state"]
    pub mod WSTAT {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "this bit indicate V state"]
    pub mod VSTAT {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "this bit indicate U state"]
    pub mod USTAT {
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
    pub mod TIMER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "W read register"]
pub mod COUNT_READ_W {
    #[doc = "wcnt counter"]
    pub mod WCNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "V read register"]
pub mod COUNT_READ_V {
    #[doc = "vcnt counter"]
    pub mod VCNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "U read register"]
pub mod COUNT_READ_U {
    #[doc = "ucnt counter"]
    pub mod UCNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "this bit indicate W state"]
    pub mod WSTAT {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "this bit indicate V state"]
    pub mod VSTAT {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "this bit indicate U state"]
    pub mod USTAT {
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
#[doc = "Timer read register"]
pub mod COUNT_READ_TMR {
    #[doc = "32 bit free run timer"]
    pub mod TIMER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "W snap register 0"]
pub mod COUNT_SNAP0_W {
    #[doc = "wcnt counter"]
    pub mod WCNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "V snap register 0"]
pub mod COUNT_SNAP0_V {
    #[doc = "vcnt counter"]
    pub mod VCNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Usnap register 0"]
pub mod COUNT_SNAP0_U {
    #[doc = "ucnt counter"]
    pub mod UCNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "this bit indicate W state"]
    pub mod WSTAT {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "this bit indicate V state"]
    pub mod VSTAT {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "this bit indicate U state"]
    pub mod USTAT {
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
#[doc = "Timer snap register 0"]
pub mod COUNT_SNAP0_TMR {
    #[doc = "32 bit free run timer"]
    pub mod TIMER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "W snap register 1"]
pub mod COUNT_SNAP1_W {
    #[doc = "wcnt counter"]
    pub mod WCNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "V snap register 1"]
pub mod COUNT_SNAP1_V {
    #[doc = "vcnt counter"]
    pub mod VCNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "U snap register 1"]
pub mod COUNT_SNAP1_U {
    #[doc = "ucnt counter"]
    pub mod UCNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "this bit indicate W state"]
    pub mod WSTAT {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "this bit indicate V state"]
    pub mod VSTAT {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "this bit indicate U state"]
    pub mod USTAT {
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
    pub mod TIMER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "history register 0"]
pub mod HIS_U_HIS0 {
    #[doc = "copy of ucnt when u signal transition from 0 to 1"]
    pub mod UHIS0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "history register 1"]
pub mod HIS_U_HIS1 {
    #[doc = "copy of ucnt when u signal transition from 1 to 0"]
    pub mod UHIS1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "V histroy register 0"]
pub mod HIS_V_HIS0 {
    #[doc = "copy of ucnt when u signal transition from 0 to 1"]
    pub mod UHIS0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "V histroy register 1"]
pub mod HIS_V_HIS1 {
    #[doc = "copy of ucnt when u signal transition from 1 to 0"]
    pub mod UHIS1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "W histroy register 0"]
pub mod HIS_W_HIS0 {
    #[doc = "copy of ucnt when u signal transition from 0 to 1"]
    pub mod UHIS0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "W histroy register 1"]
pub mod HIS_W_HIS1 {
    #[doc = "copy of ucnt when u signal transition from 1 to 0"]
    pub mod UHIS1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
