#[doc = "PCFG"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "BANGGAP control"]
    pub BANDGAP: crate::RWRegister<u32>,
    #[doc = "1V LDO config"]
    pub LDO1P1: crate::RWRegister<u32>,
    #[doc = "2.5V LDO config"]
    pub LDO2P5: crate::RWRegister<u32>,
    _reserved0: [u8; 0x04],
    #[doc = "DCDC mode select"]
    pub DCDC_MODE: crate::RWRegister<u32>,
    #[doc = "DCDC low power mode"]
    pub DCDC_LPMODE: crate::RWRegister<u32>,
    #[doc = "DCDC protection"]
    pub DCDC_PROT: crate::RWRegister<u32>,
    #[doc = "DCDC current estimation"]
    pub DCDC_CURRENT: crate::RWRegister<u32>,
    #[doc = "DCDC advance setting"]
    pub DCDC_ADVMODE: crate::RWRegister<u32>,
    #[doc = "DCDC advance parameter"]
    pub DCDC_ADVPARAM: crate::RWRegister<u32>,
    #[doc = "DCDC misc parameter"]
    pub DCDC_MISC: crate::RWRegister<u32>,
    #[doc = "DCDC Debug"]
    pub DCDC_DEBUG: crate::RWRegister<u32>,
    #[doc = "DCDC ramp time"]
    pub DCDC_START_TIME: crate::RWRegister<u32>,
    #[doc = "DCDC resume time"]
    pub DCDC_RESUME_TIME: crate::RWRegister<u32>,
    _reserved1: [u8; 0x08],
    #[doc = "SOC power trap"]
    pub POWER_TRAP: crate::RWRegister<u32>,
    #[doc = "Wake up source"]
    pub WAKE_CAUSE: crate::RWRegister<u32>,
    #[doc = "Wake up mask"]
    pub WAKE_MASK: crate::RWRegister<u32>,
    #[doc = "Clock gate control in PMIC"]
    pub SCG_CTRL: crate::RWRegister<u32>,
    #[doc = "Debug stop config"]
    pub DEBUG_STOP: crate::RWRegister<u32>,
    _reserved2: [u8; 0x0c],
    #[doc = "RC 24M config"]
    pub RC24M: crate::RWRegister<u32>,
    #[doc = "RC 24M track mode"]
    pub RC24M_TRACK: crate::RWRegister<u32>,
    #[doc = "RC 24M track target"]
    pub TRACK_TARGET: crate::RWRegister<u32>,
    #[doc = "RC 24M track status"]
    pub STATUS: crate::RWRegister<u32>,
}
#[doc = "BANGGAP control"]
pub mod BANDGAP {
    #[doc = "Banggap 1.0V output trim value"]
    pub mod VBG_P50_TRIM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Banggap 1.0V output trim value"]
    pub mod VBG_P65_TRIM {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Banggap 1.0V output trim value"]
    pub mod VBG_1P0_TRIM {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Banggap work in power save mode, banggap function normally 0: banggap works in high performance mode 1: banggap works in power saving mode"]
    pub mod POWER_SAVE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Banggap work in low power mode, banggap function limited 0: banggap works in normal mode 1: banggap works in low power mode"]
    pub mod LOWPOWER_MODE {
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
#[doc = "1V LDO config"]
pub mod LDO1P1 {
    #[doc = "LDO output voltage in mV, value valid through 700-1320, , step 20mV. Hardware select voltage no less than target if not on valid steps, with maximum 1320mV. 700: 700mV 720: 720mV . . . 1320:1320mV"]
    pub mod VOLT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LDO enable 0: turn off LDO 1: turn on LDO"]
    pub mod ENABLE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "2.5V LDO config"]
pub mod LDO2P5 {
    #[doc = "LDO output voltage in mV, value valid through 2125-2900, step 25mV. Hardware select voltage no less than target if not on valid steps, with maximum 2900mV. 2125: 2125mV 2150: 2150mV . . . 2900:2900mV"]
    pub mod VOLT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LDO enable 0: turn off LDO 1: turn on LDO"]
    pub mod ENABLE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ready flag, will set 1ms after enabled or voltage change 0: LDO is not ready for use 1: LDO is ready"]
    pub mod READY {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCDC mode select"]
pub mod DCDC_MODE {
    #[doc = "DCDC voltage in mV in normal mode, value valid through 600-1375, , step 25mV. Hardware select voltage no less than target if not on valid steps, with maximum 1375mV. 600: 600mV 625: 625mV . . . 1375:1375mV"]
    pub mod VOLT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DCDC work mode XX0: trun off 001: basic mode 011: generic mode 101: automatic mode 111: expert mode"]
    pub mod MODE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ready flag 0: DCDC is applying new change 1: DCDC is ready"]
    pub mod READY {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCDC low power mode"]
pub mod DCDC_LPMODE {
    #[doc = "DCDC voltage in mV in standby mode, , value valid through 600-1375, , step 25mV. Hardware select voltage no less than target if not on valid steps, with maximum 1375mV. 600: 600mV 625: 625mV . . . 1375:1375mV"]
    pub mod STBY_VOLT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCDC protection"]
pub mod DCDC_PROT {
    #[doc = "short circuit flag 0: current is within limit 1: short circuits detected"]
    pub mod SHORT_FLAG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "short circuit current setting 0: 2.0A, 1: 1.3A"]
    pub mod SHORT_CURRENT {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "disable output short circuit protection 0: short circuits protection enabled, DCDC shut down if short circuit on ouput detected 1: short circuit protection disabled"]
    pub mod DISABLE_SHORT {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "output over voltage flag 0: output is normal 1: output is unexpected high"]
    pub mod OVERVOLT_FLAG {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ouput over voltage protection 0: protection enabled, DCDC will shut down is output voltage is unexpected high 1: protection disabled, DCDC continue to adjust output voltage"]
    pub mod DISABLE_OVERVOLTAGE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "power loss 0: input power is good 1: input power is too low"]
    pub mod POWER_LOSS_FLAG {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "disable power loss protection 0: power loss protection enabled, DCDC shuts down when power loss 1: power loss protection disabled, DCDC try working after power voltage drop"]
    pub mod DISABLE_POWER_LOSS {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "over current in low power mode 0: current is below setting 1: overcurrent happened in low power mode"]
    pub mod OVERLOAD_LP {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "over current setting for low power mode 0:250mA 1:200mA"]
    pub mod ILIMIT_LP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCDC current estimation"]
pub mod DCDC_CURRENT {
    #[doc = "DCDC current level, current level is num * 50mA"]
    pub mod LEVEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Current level valid 0: data is invalid 1: data is valid"]
    pub mod VALID {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "enable current measure"]
    pub mod ESTI_EN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCDC advance setting"]
pub mod DCDC_ADVMODE {
    #[doc = "DCM mode 0: CCM mode 1: DCM mode"]
    pub mod EN_DCM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "enable skip when voltage is higher than threshold 0: do not skip 1: skip if voltage is excess"]
    pub mod EN_IDLE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "enable skip on narrow pulse 0: do not skip narrow pulse 1: skip narrow pulse"]
    pub mod EN_SKIP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "avoid over voltage 0: stay in DCM mode when voltage excess 1: change to CCM mode when voltage excess"]
    pub mod EN_DCM_EXIT {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "enable auto enter low power mode 0: do not enter low power mode 1: enter low power mode if current is detected low"]
    pub mod EN_AUTOLP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "enable feed forward loop 0: feed forward loop is disabled 1: feed forward loop is enabled"]
    pub mod EN_FF_LOOP {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "enable feed forward detect 0: feed forward detect is disabled 1: feed forward detect is enabled"]
    pub mod EN_FF_DET {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Loop R number"]
    pub mod DC_R {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Loop C number"]
    pub mod DC_C {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable RC scale"]
    pub mod EN_RCSCALE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCDC advance parameter"]
pub mod DCDC_ADVPARAM {
    #[doc = "maximum duty cycle"]
    pub mod MAX_DUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "minimum duty cycle"]
    pub mod MIN_DUT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCDC misc parameter"]
pub mod DCDC_MISC {
    #[doc = "enable stepping in voltage change 0: stepping disabled, 1: steping enabled"]
    pub mod EN_STEP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock selection 0: select DCDC internal oscillator 1: select RC24M oscillator"]
    pub mod CLK_SEL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "enable delay 0: delay disabled, 1: delay enabled"]
    pub mod DELAY {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "current hysteres range 0: 12.5mV 1: 25mV"]
    pub mod OL_HYST {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "overload for threshold for lod power mode"]
    pub mod OL_THRE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Loop feed forward number"]
    pub mod DC_FF {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Loop RC scale threshold"]
    pub mod RC_SCALE {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "hysteres threshold"]
    pub mod HYST_THRS {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "hysteres sign"]
    pub mod HYST_SIGN {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "hysteres enable"]
    pub mod EN_HYST {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCDC Debug"]
pub mod DCDC_DEBUG {
    #[doc = "DCDC voltage change time in 24M clock cycles, default value is 1mS"]
    pub mod UPDATE_TIME {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCDC ramp time"]
pub mod DCDC_START_TIME {
    #[doc = "Start delay for DCDC to turn on, in 24M clock cycles, default value is 3mS"]
    pub mod START_TIME {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DCDC resume time"]
pub mod DCDC_RESUME_TIME {
    #[doc = "Resume delay for DCDC to recover from low power mode, in 24M clock cycles, default value is 1.5mS"]
    pub mod RESUME_TIME {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SOC power trap"]
pub mod POWER_TRAP {
    #[doc = "Enable trap of SOC power supply, trap is used to hold SOC in low power mode for DCDC to enter further low power mode, this bit will self-clear when power related low pwer flow triggered 0: trap not enabled, pmic side low power function disabled 1: trap enabled, STOP operation leads to PMIC low power flow if SOC is not retentioned."]
    pub mod TRAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DCDC enter standby mode, which will reduce voltage for memory content retention 0: Shutdown DCDC 1: reduce DCDC voltage"]
    pub mod RETENTION {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Low power trap status, thit bit will set when power related low power flow triggered, write 1 to clear this flag. 0: low power trap is not triggered 1: low power trap triggered"]
    pub mod TRIGGERED {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Wake up source"]
pub mod WAKE_CAUSE {
    #[doc = "wake up cause, each bit represents one wake up source, write 1 to clear the register bit 0: wake up source is not active during last wakeup 1: wake up source is active furing last wakeup bit 0: pmic_enable bit 1: debug wakeup bit 4: fuse interrupt bit 7: UART interrupt bit 8: TMR interrupt bit 9: WDG interrupt bit10: GPIO in PMIC interrupt bit11: Security monitor interrupt bit12: Security in PMIC event bit16: Security violation in BATT bit17: GPIO in BATT interrupt bit18: BATT Button interrupt bit19: RTC alarm interrupt"]
    pub mod CAUSE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Wake up mask"]
pub mod WAKE_MASK {
    #[doc = "mask for wake up sources, each bit represents one wakeup source 0: allow source to wake up system 1: disallow source to wakeup system bit 0: pmic_enable bit 1: debug wakeup bit 4: fuse interrupt bit 7: UART interrupt bit 8: TMR interrupt bit 9: WDG interrupt bit10: GPIO in PMIC interrupt bit11: Security monitor interrupt bit12: Security in PMIC event bit16: Security violation in BATT bit17: GPIO in BATT interrupt bit18: BATT Button interrupt bit19: RTC alarm interrupt"]
    pub mod MASK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock gate control in PMIC"]
pub mod SCG_CTRL {
    #[doc = "control whether clock being gated during PMIC low power flow, 2 bits for each peripheral 00,01: clock gated according to low power flow 10: clock is always off 11: clock is always on bit0-1: fuse bit2-3: sram bit4-5: vad bit6-7:gpio bit8-9:ioc bit10-11: timer bit12-13:wdog bit14-15:uart bit16-17:debug"]
    pub mod SCG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Debug stop config"]
pub mod DEBUG_STOP {
    #[doc = "Stop peripheral when CPU0 enter debug mode 0: peripheral keep running when CPU0 in debug mode 1: peripheral enter debug mode when CPU0 enter debug"]
    pub mod CPU0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Stop peripheral when CPU1 enter debug mode 0: peripheral keep running when CPU1 in debug mode 1: peripheral enter debug mode when CPU1 enter debug"]
    pub mod CPU1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "RC 24M config"]
pub mod RC24M {
    #[doc = "Fine trim for RC24M, bigger value means faster"]
    pub mod TRIM_F {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Coarse trim for RC24M, bigger value means faster"]
    pub mod TRIM_C {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RC24M trim happened, this bit set by hardware after trim value loaded, and stop load, write 0 will clear this bit and reload trim value 0: RC is not trimmed 1: RC is trimmed"]
    pub mod RC_TRIMMED {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "RC 24M track mode"]
pub mod RC24M_TRACK {
    #[doc = "track mode 0: RC24M free running 1: track RC24M to external XTAL"]
    pub mod TRACK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Retrun default value when XTAL loss 0: remain last tracking value 1: switch to default value"]
    pub mod RETURN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Select track reference 0: select 32K as reference 1: select 24M XTAL as reference"]
    pub mod SEL24M {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "RC 24M track target"]
pub mod TRACK_TARGET {
    #[doc = "Target frequency multiplier of divided source"]
    pub mod TARGET {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Divider for reference source"]
    pub mod PRE_DIV {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "RC 24M track status"]
pub mod STATUS {
    #[doc = "default fine trim value"]
    pub mod TRIM_F {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "default coarse trim value"]
    pub mod TRIM_C {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "default value takes effect 0: default value is invalid 1: default value is valid"]
    pub mod EN_TRIM {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "track is using XTAL24M 0: track is not using XTAL24M 1: track is using XTAL24M"]
    pub mod SEL24M {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "track is using XTAL32K 0: track is not using XTAL32K 1: track is using XTAL32K"]
    pub mod SEL32K {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
