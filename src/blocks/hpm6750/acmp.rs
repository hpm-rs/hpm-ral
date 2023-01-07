#[doc = "ACMP"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Configure Register"]
    pub CHANNEL_CHN0_CFG: crate::RWRegister<u32>,
    #[doc = "DAC configure register"]
    pub CHANNEL_CHN0_DACCFG: crate::RWRegister<u32>,
    _reserved0: [u8; 0x08],
    #[doc = "Status register"]
    pub CHANNEL_CHN0_SR: crate::RWRegister<u32>,
    #[doc = "Interrupt request enable register"]
    pub CHANNEL_CHN0_IRQEN: crate::RWRegister<u32>,
    #[doc = "DMA request enable register"]
    pub CHANNEL_CHN0_DMAEN: crate::RWRegister<u32>,
    _reserved1: [u8; 0x04],
    #[doc = "Configure Register"]
    pub CHANNEL_CHN1_CFG: crate::RWRegister<u32>,
    #[doc = "DAC configure register"]
    pub CHANNEL_CHN1_DACCFG: crate::RWRegister<u32>,
    _reserved2: [u8; 0x08],
    #[doc = "Status register"]
    pub CHANNEL_CHN1_SR: crate::RWRegister<u32>,
    #[doc = "Interrupt request enable register"]
    pub CHANNEL_CHN1_IRQEN: crate::RWRegister<u32>,
    #[doc = "DMA request enable register"]
    pub CHANNEL_CHN1_DMAEN: crate::RWRegister<u32>,
    _reserved3: [u8; 0x04],
    #[doc = "Configure Register"]
    pub CHANNEL_CHN2_CFG: crate::RWRegister<u32>,
    #[doc = "DAC configure register"]
    pub CHANNEL_CHN2_DACCFG: crate::RWRegister<u32>,
    _reserved4: [u8; 0x08],
    #[doc = "Status register"]
    pub CHANNEL_CHN2_SR: crate::RWRegister<u32>,
    #[doc = "Interrupt request enable register"]
    pub CHANNEL_CHN2_IRQEN: crate::RWRegister<u32>,
    #[doc = "DMA request enable register"]
    pub CHANNEL_CHN2_DMAEN: crate::RWRegister<u32>,
    _reserved5: [u8; 0x04],
    #[doc = "Configure Register"]
    pub CHANNEL_CHN3_CFG: crate::RWRegister<u32>,
    #[doc = "DAC configure register"]
    pub CHANNEL_CHN3_DACCFG: crate::RWRegister<u32>,
    _reserved6: [u8; 0x08],
    #[doc = "Status register"]
    pub CHANNEL_CHN3_SR: crate::RWRegister<u32>,
    #[doc = "Interrupt request enable register"]
    pub CHANNEL_CHN3_IRQEN: crate::RWRegister<u32>,
    #[doc = "DMA request enable register"]
    pub CHANNEL_CHN3_DMAEN: crate::RWRegister<u32>,
}
#[doc = "Configure Register"]
pub mod CHANNEL_CHN0_CFG {
    #[doc = "This bitfield define the ACMP output digital filter length. The unit is ACMP clock cycle."]
    pub mod FLTLEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit enable the comparator output synchronization. 0: ACMP output not synchronized with ACMP clock. 1: ACMP output synchronized with ACMP clock."]
    pub mod SYNCEN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield define the ACMP output digital filter mode: 000-bypass 100-change immediately; 101-change after filter; 110-stalbe low; 111-stable high"]
    pub mod FLTMODE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The output polarity control bit. 0: The ACMP output remain un-changed. 1: The ACMP output is inverted."]
    pub mod OPOL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit enable the comparator window mode. 0: Window mode is disabled 1: Window mode is enabled"]
    pub mod WINEN {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit bypass the comparator output digital filter. 0: The ACMP output need pass digital filter 1: The ACMP output digital filter is bypassed."]
    pub mod FLTBYPS {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit enable the comparator output on pad. 0: ACMP output disabled 1: ACMP output enabled"]
    pub mod CMPOEN {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MIN select, from pad_ai_acmp\\[7:1\\] and dac_out"]
    pub mod PINSEL {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PIN select, from pad_ai_acmp\\[7:1\\] and dac_out"]
    pub mod MINSEL {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit enable the comparator. 0: ACMP disabled 1: ACMP enabled"]
    pub mod CMPEN {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit enable the comparator high performance mode. 0: HP mode disabled 1: HP mode enabled"]
    pub mod HPMODE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit enable the comparator internal DAC 0: DAC disabled 1: DAC enabled"]
    pub mod DACEN {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield configure the comparator hysteresis. 00: Hysteresis level 0 01: Hysteresis level 1 10: Hysteresis level 2 11: Hysteresis level 3"]
    pub mod HYST {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DAC configure register"]
pub mod CHANNEL_CHN0_DACCFG {
    #[doc = "8bit DAC digital value"]
    pub mod DACCFG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status register"]
pub mod CHANNEL_CHN0_SR {
    #[doc = "Output rising edge flag. Write 1 to clear this flag."]
    pub mod REDGF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Output falling edge flag. Write 1 to clear this flag."]
    pub mod FEDGF {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt request enable register"]
pub mod CHANNEL_CHN0_IRQEN {
    #[doc = "Output rising edge flag interrupt enable bit."]
    pub mod REDGEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Output falling edge flag interrupt enable bit."]
    pub mod FEDGEN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA request enable register"]
pub mod CHANNEL_CHN0_DMAEN {
    #[doc = "Output rising edge flag DMA request enable bit."]
    pub mod REDGEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Output falling edge flag DMA request enable bit."]
    pub mod FEDGEN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Configure Register"]
pub mod CHANNEL_CHN1_CFG {
    #[doc = "This bitfield define the ACMP output digital filter length. The unit is ACMP clock cycle."]
    pub mod FLTLEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit enable the comparator output synchronization. 0: ACMP output not synchronized with ACMP clock. 1: ACMP output synchronized with ACMP clock."]
    pub mod SYNCEN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield define the ACMP output digital filter mode: 000-bypass 100-change immediately; 101-change after filter; 110-stalbe low; 111-stable high"]
    pub mod FLTMODE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The output polarity control bit. 0: The ACMP output remain un-changed. 1: The ACMP output is inverted."]
    pub mod OPOL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit enable the comparator window mode. 0: Window mode is disabled 1: Window mode is enabled"]
    pub mod WINEN {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit bypass the comparator output digital filter. 0: The ACMP output need pass digital filter 1: The ACMP output digital filter is bypassed."]
    pub mod FLTBYPS {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit enable the comparator output on pad. 0: ACMP output disabled 1: ACMP output enabled"]
    pub mod CMPOEN {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MIN select, from pad_ai_acmp\\[7:1\\] and dac_out"]
    pub mod PINSEL {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PIN select, from pad_ai_acmp\\[7:1\\] and dac_out"]
    pub mod MINSEL {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit enable the comparator. 0: ACMP disabled 1: ACMP enabled"]
    pub mod CMPEN {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit enable the comparator high performance mode. 0: HP mode disabled 1: HP mode enabled"]
    pub mod HPMODE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit enable the comparator internal DAC 0: DAC disabled 1: DAC enabled"]
    pub mod DACEN {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield configure the comparator hysteresis. 00: Hysteresis level 0 01: Hysteresis level 1 10: Hysteresis level 2 11: Hysteresis level 3"]
    pub mod HYST {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DAC configure register"]
pub mod CHANNEL_CHN1_DACCFG {
    #[doc = "8bit DAC digital value"]
    pub mod DACCFG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status register"]
pub mod CHANNEL_CHN1_SR {
    #[doc = "Output rising edge flag. Write 1 to clear this flag."]
    pub mod REDGF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Output falling edge flag. Write 1 to clear this flag."]
    pub mod FEDGF {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt request enable register"]
pub mod CHANNEL_CHN1_IRQEN {
    #[doc = "Output rising edge flag interrupt enable bit."]
    pub mod REDGEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Output falling edge flag interrupt enable bit."]
    pub mod FEDGEN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA request enable register"]
pub mod CHANNEL_CHN1_DMAEN {
    #[doc = "Output rising edge flag DMA request enable bit."]
    pub mod REDGEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Output falling edge flag DMA request enable bit."]
    pub mod FEDGEN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Configure Register"]
pub mod CHANNEL_CHN2_CFG {
    #[doc = "This bitfield define the ACMP output digital filter length. The unit is ACMP clock cycle."]
    pub mod FLTLEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit enable the comparator output synchronization. 0: ACMP output not synchronized with ACMP clock. 1: ACMP output synchronized with ACMP clock."]
    pub mod SYNCEN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield define the ACMP output digital filter mode: 000-bypass 100-change immediately; 101-change after filter; 110-stalbe low; 111-stable high"]
    pub mod FLTMODE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The output polarity control bit. 0: The ACMP output remain un-changed. 1: The ACMP output is inverted."]
    pub mod OPOL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit enable the comparator window mode. 0: Window mode is disabled 1: Window mode is enabled"]
    pub mod WINEN {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit bypass the comparator output digital filter. 0: The ACMP output need pass digital filter 1: The ACMP output digital filter is bypassed."]
    pub mod FLTBYPS {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit enable the comparator output on pad. 0: ACMP output disabled 1: ACMP output enabled"]
    pub mod CMPOEN {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MIN select, from pad_ai_acmp\\[7:1\\] and dac_out"]
    pub mod PINSEL {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PIN select, from pad_ai_acmp\\[7:1\\] and dac_out"]
    pub mod MINSEL {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit enable the comparator. 0: ACMP disabled 1: ACMP enabled"]
    pub mod CMPEN {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit enable the comparator high performance mode. 0: HP mode disabled 1: HP mode enabled"]
    pub mod HPMODE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit enable the comparator internal DAC 0: DAC disabled 1: DAC enabled"]
    pub mod DACEN {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield configure the comparator hysteresis. 00: Hysteresis level 0 01: Hysteresis level 1 10: Hysteresis level 2 11: Hysteresis level 3"]
    pub mod HYST {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DAC configure register"]
pub mod CHANNEL_CHN2_DACCFG {
    #[doc = "8bit DAC digital value"]
    pub mod DACCFG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status register"]
pub mod CHANNEL_CHN2_SR {
    #[doc = "Output rising edge flag. Write 1 to clear this flag."]
    pub mod REDGF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Output falling edge flag. Write 1 to clear this flag."]
    pub mod FEDGF {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt request enable register"]
pub mod CHANNEL_CHN2_IRQEN {
    #[doc = "Output rising edge flag interrupt enable bit."]
    pub mod REDGEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Output falling edge flag interrupt enable bit."]
    pub mod FEDGEN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA request enable register"]
pub mod CHANNEL_CHN2_DMAEN {
    #[doc = "Output rising edge flag DMA request enable bit."]
    pub mod REDGEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Output falling edge flag DMA request enable bit."]
    pub mod FEDGEN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Configure Register"]
pub mod CHANNEL_CHN3_CFG {
    #[doc = "This bitfield define the ACMP output digital filter length. The unit is ACMP clock cycle."]
    pub mod FLTLEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit enable the comparator output synchronization. 0: ACMP output not synchronized with ACMP clock. 1: ACMP output synchronized with ACMP clock."]
    pub mod SYNCEN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield define the ACMP output digital filter mode: 000-bypass 100-change immediately; 101-change after filter; 110-stalbe low; 111-stable high"]
    pub mod FLTMODE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The output polarity control bit. 0: The ACMP output remain un-changed. 1: The ACMP output is inverted."]
    pub mod OPOL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit enable the comparator window mode. 0: Window mode is disabled 1: Window mode is enabled"]
    pub mod WINEN {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit bypass the comparator output digital filter. 0: The ACMP output need pass digital filter 1: The ACMP output digital filter is bypassed."]
    pub mod FLTBYPS {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit enable the comparator output on pad. 0: ACMP output disabled 1: ACMP output enabled"]
    pub mod CMPOEN {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MIN select, from pad_ai_acmp\\[7:1\\] and dac_out"]
    pub mod PINSEL {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PIN select, from pad_ai_acmp\\[7:1\\] and dac_out"]
    pub mod MINSEL {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit enable the comparator. 0: ACMP disabled 1: ACMP enabled"]
    pub mod CMPEN {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit enable the comparator high performance mode. 0: HP mode disabled 1: HP mode enabled"]
    pub mod HPMODE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit enable the comparator internal DAC 0: DAC disabled 1: DAC enabled"]
    pub mod DACEN {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfield configure the comparator hysteresis. 00: Hysteresis level 0 01: Hysteresis level 1 10: Hysteresis level 2 11: Hysteresis level 3"]
    pub mod HYST {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DAC configure register"]
pub mod CHANNEL_CHN3_DACCFG {
    #[doc = "8bit DAC digital value"]
    pub mod DACCFG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status register"]
pub mod CHANNEL_CHN3_SR {
    #[doc = "Output rising edge flag. Write 1 to clear this flag."]
    pub mod REDGF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Output falling edge flag. Write 1 to clear this flag."]
    pub mod FEDGF {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt request enable register"]
pub mod CHANNEL_CHN3_IRQEN {
    #[doc = "Output rising edge flag interrupt enable bit."]
    pub mod REDGEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Output falling edge flag interrupt enable bit."]
    pub mod FEDGEN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA request enable register"]
pub mod CHANNEL_CHN3_DMAEN {
    #[doc = "Output rising edge flag DMA request enable bit."]
    pub mod REDGEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Output falling edge flag DMA request enable bit."]
    pub mod FEDGEN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
