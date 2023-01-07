#[doc = "BCFG"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Bandgap config"]
    pub VBG_CFG: crate::RWRegister<u32>,
    #[doc = "LDO config"]
    pub LDO_CFG: crate::RWRegister<u32>,
    #[doc = "On-chip 32k oscillator config"]
    pub IRC32K_CFG: crate::RWRegister<u32>,
    #[doc = "XTAL 32K config"]
    pub XTAL32K_CFG: crate::RWRegister<u32>,
    #[doc = "Clock config"]
    pub CLK_CFG: crate::RWRegister<u32>,
}
#[doc = "Bandgap config"]
pub mod VBG_CFG {
    #[doc = "Bandgap 0.50V output trim"]
    pub mod VBG_P50 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bandgap 0.65V output trim"]
    pub mod VBG_P65 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bandgap 1.0V output trim"]
    pub mod VBG_1P0 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bandgap works in power save mode 0: not in power save mode 1: bandgap work in power save mode"]
    pub mod POWER_SAVE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bandgap works in low power mode 0: not in low power mode 1: bandgap work in low power mode"]
    pub mod LP_MODE {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bandgap trim happened, this bit set by hardware after trim value loaded, and stop load, write 0 will clear this bit and reload trim value 0: bandgap is not trimmed 1: bandgap is trimmed"]
    pub mod VBG_TRIMMED {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LDO config"]
pub mod LDO_CFG {
    #[doc = "LDO voltage setting in mV, valid range through 600mV to 1100mV, step 20mV. Hardware select voltage no less than target if not on valid steps, with maximum 1100mV. 600: 600mV 620: 620mV . . . 1100:1100mV"]
    pub mod VOLT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LDO enable 0: LDO is disabled 1: LDO is enabled"]
    pub mod ENABLE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "disable pull down resistor, enable pull down may lead to more power but better response 0: pulldown resistor enabled 1: pulldown resistor disabled"]
    pub mod DIS_PD {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "enable selfload, this bit helps improve LDO performance when current less than 200nA 0: self load disabled 1: selfload enabled"]
    pub mod EN_SL {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Capacitor trim"]
    pub mod CP_TRIM {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Resistor trim"]
    pub mod RES_TRIM {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "On-chip 32k oscillator config"]
pub mod IRC32K_CFG {
    #[doc = "capacitor trim bits"]
    pub mod CAP_TRIM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IRC32K bit 6"]
    pub mod CAPEX6_TRIM {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IRC32K bit 7"]
    pub mod CAPEX7_TRIM {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IRC32K trim happened, this bit set by hardware after trim value loaded, and stop load, write 0 will clear this bit and reload trim value 0: irc is not trimmed 1: irc is trimmed"]
    pub mod IRC_TRIMMED {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "XTAL 32K config"]
pub mod XTAL32K_CFG {
    #[doc = "crystal 32k amplifier"]
    pub mod AMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "crystal 32k config"]
    pub mod CFG {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "crystal 32k gm selection"]
    pub mod GMSEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "crystal 32k hysteres enable"]
    pub mod HYST_EN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock config"]
pub mod CLK_CFG {
    #[doc = "force switch to crystal"]
    pub mod FORCE_XTAL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "force irc32k run"]
    pub mod KEEP_IRC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "crystal selected"]
    pub mod XTAL_SEL {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
