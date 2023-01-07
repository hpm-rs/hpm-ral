#[doc = "TRGM0"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Filter configure register"]
    pub FILTCFG_PWM_IN0: crate::RWRegister<u32>,
    #[doc = "Filter configure register"]
    pub FILTCFG_PWM_IN1: crate::RWRegister<u32>,
    #[doc = "Filter configure register"]
    pub FILTCFG_PWM_IN2: crate::RWRegister<u32>,
    #[doc = "Filter configure register"]
    pub FILTCFG_PWM_IN3: crate::RWRegister<u32>,
    #[doc = "Filter configure register"]
    pub FILTCFG_PWM_IN4: crate::RWRegister<u32>,
    #[doc = "Filter configure register"]
    pub FILTCFG_PWM_IN5: crate::RWRegister<u32>,
    #[doc = "Filter configure register"]
    pub FILTCFG_PWM_IN6: crate::RWRegister<u32>,
    #[doc = "Filter configure register"]
    pub FILTCFG_PWM_IN7: crate::RWRegister<u32>,
    #[doc = "Filter configure register"]
    pub FILTCFG_TRGM_IN0: crate::RWRegister<u32>,
    #[doc = "Filter configure register"]
    pub FILTCFG_TRGM_IN1: crate::RWRegister<u32>,
    #[doc = "Filter configure register"]
    pub FILTCFG_TRGM_IN2: crate::RWRegister<u32>,
    #[doc = "Filter configure register"]
    pub FILTCFG_TRGM_IN3: crate::RWRegister<u32>,
    #[doc = "Filter configure register"]
    pub FILTCFG_TRGM_IN4: crate::RWRegister<u32>,
    #[doc = "Filter configure register"]
    pub FILTCFG_TRGM_IN5: crate::RWRegister<u32>,
    #[doc = "Filter configure register"]
    pub FILTCFG_TRGM_IN6: crate::RWRegister<u32>,
    #[doc = "Filter configure register"]
    pub FILTCFG_TRGM_IN7: crate::RWRegister<u32>,
    #[doc = "Filter configure register"]
    pub FILTCFG_TRGM_IN8: crate::RWRegister<u32>,
    #[doc = "Filter configure register"]
    pub FILTCFG_TRGM_IN9: crate::RWRegister<u32>,
    #[doc = "Filter configure register"]
    pub FILTCFG_TRGM_IN10: crate::RWRegister<u32>,
    #[doc = "Filter configure register"]
    pub FILTCFG_TRGM_IN11: crate::RWRegister<u32>,
    _reserved0: [u8; 0xb0],
    #[doc = "Trigger manager output configure register"]
    pub TRGOCFG_TRGM_OUT0: crate::RWRegister<u32>,
    #[doc = "Trigger manager output configure register"]
    pub TRGOCFG_TRGM_OUT1: crate::RWRegister<u32>,
    #[doc = "Trigger manager output configure register"]
    pub TRGOCFG_TRGM_OUT2: crate::RWRegister<u32>,
    #[doc = "Trigger manager output configure register"]
    pub TRGOCFG_TRGM_OUT3: crate::RWRegister<u32>,
    #[doc = "Trigger manager output configure register"]
    pub TRGOCFG_TRGM_OUT4: crate::RWRegister<u32>,
    #[doc = "Trigger manager output configure register"]
    pub TRGOCFG_TRGM_OUT5: crate::RWRegister<u32>,
    #[doc = "Trigger manager output configure register"]
    pub TRGOCFG_TRGM_OUT6: crate::RWRegister<u32>,
    #[doc = "Trigger manager output configure register"]
    pub TRGOCFG_TRGM_OUT7: crate::RWRegister<u32>,
    #[doc = "Trigger manager output configure register"]
    pub TRGOCFG_TRGM_OUT8: crate::RWRegister<u32>,
    #[doc = "Trigger manager output configure register"]
    pub TRGOCFG_TRGM_OUT9: crate::RWRegister<u32>,
    #[doc = "Trigger manager output configure register"]
    pub TRGOCFG_TRGM_OUT10: crate::RWRegister<u32>,
    #[doc = "Trigger manager output configure register"]
    pub TRGOCFG_TRGM_OUT11: crate::RWRegister<u32>,
    #[doc = "Trigger manager output configure register"]
    pub TRGOCFG_TRGM_OUTX0: crate::RWRegister<u32>,
    #[doc = "Trigger manager output configure register"]
    pub TRGOCFG_TRGM_OUTX1: crate::RWRegister<u32>,
    #[doc = "Trigger manager output configure register"]
    pub TRGOCFG_PWM_SYNCI: crate::RWRegister<u32>,
    #[doc = "Trigger manager output configure register"]
    pub TRGOCFG_PWM_FRCI: crate::RWRegister<u32>,
    #[doc = "Trigger manager output configure register"]
    pub TRGOCFG_PWM_FRCSYNCI: crate::RWRegister<u32>,
    #[doc = "Trigger manager output configure register"]
    pub TRGOCFG_PWM_SHRLDSYNCI: crate::RWRegister<u32>,
    #[doc = "Trigger manager output configure register"]
    pub TRGOCFG_PWM_FAULTI0: crate::RWRegister<u32>,
    #[doc = "Trigger manager output configure register"]
    pub TRGOCFG_PWM_FAULTI1: crate::RWRegister<u32>,
    #[doc = "Trigger manager output configure register"]
    pub TRGOCFG_PWM_FAULTI2: crate::RWRegister<u32>,
    #[doc = "Trigger manager output configure register"]
    pub TRGOCFG_PWM_FAULTI3: crate::RWRegister<u32>,
    #[doc = "Trigger manager output configure register"]
    pub TRGOCFG_PWM_IN8: crate::RWRegister<u32>,
    #[doc = "Trigger manager output configure register"]
    pub TRGOCFG_PWM_IN9: crate::RWRegister<u32>,
    #[doc = "Trigger manager output configure register"]
    pub TRGOCFG_PWM_IN10: crate::RWRegister<u32>,
    #[doc = "Trigger manager output configure register"]
    pub TRGOCFG_PWM_IN11: crate::RWRegister<u32>,
    #[doc = "Trigger manager output configure register"]
    pub TRGOCFG_PWM_IN12: crate::RWRegister<u32>,
    #[doc = "Trigger manager output configure register"]
    pub TRGOCFG_PWM_IN13: crate::RWRegister<u32>,
    #[doc = "Trigger manager output configure register"]
    pub TRGOCFG_PWM_IN14: crate::RWRegister<u32>,
    #[doc = "Trigger manager output configure register"]
    pub TRGOCFG_PWM_IN15: crate::RWRegister<u32>,
    #[doc = "Trigger manager output configure register"]
    pub TRGOCFG_PWM_IN16: crate::RWRegister<u32>,
    #[doc = "Trigger manager output configure register"]
    pub TRGOCFG_PWM_IN17: crate::RWRegister<u32>,
    #[doc = "Trigger manager output configure register"]
    pub TRGOCFG_PWM_IN18: crate::RWRegister<u32>,
    #[doc = "Trigger manager output configure register"]
    pub TRGOCFG_PWM_IN19: crate::RWRegister<u32>,
    #[doc = "Trigger manager output configure register"]
    pub TRGOCFG_PWM_IN20: crate::RWRegister<u32>,
    #[doc = "Trigger manager output configure register"]
    pub TRGOCFG_PWM_IN21: crate::RWRegister<u32>,
    #[doc = "Trigger manager output configure register"]
    pub TRGOCFG_PWM_IN22: crate::RWRegister<u32>,
    #[doc = "Trigger manager output configure register"]
    pub TRGOCFG_PWM_IN23: crate::RWRegister<u32>,
    #[doc = "Trigger manager output configure register"]
    pub TRGOCFG_QEI_A: crate::RWRegister<u32>,
    #[doc = "Trigger manager output configure register"]
    pub TRGOCFG_QEI_B: crate::RWRegister<u32>,
    #[doc = "Trigger manager output configure register"]
    pub TRGOCFG_QEI_Z: crate::RWRegister<u32>,
    #[doc = "Trigger manager output configure register"]
    pub TRGOCFG_QEI_H: crate::RWRegister<u32>,
    #[doc = "Trigger manager output configure register"]
    pub TRGOCFG_QEI_PAUSE: crate::RWRegister<u32>,
    #[doc = "Trigger manager output configure register"]
    pub TRGOCFG_QEI_SNAPI: crate::RWRegister<u32>,
    #[doc = "Trigger manager output configure register"]
    pub TRGOCFG_HALL_U: crate::RWRegister<u32>,
    #[doc = "Trigger manager output configure register"]
    pub TRGOCFG_HALL_V: crate::RWRegister<u32>,
    #[doc = "Trigger manager output configure register"]
    pub TRGOCFG_HALL_W: crate::RWRegister<u32>,
    #[doc = "Trigger manager output configure register"]
    pub TRGOCFG_HALL_SNAPI: crate::RWRegister<u32>,
    #[doc = "Trigger manager output configure register"]
    pub TRGOCFG_ADC0_STRGI: crate::RWRegister<u32>,
    #[doc = "Trigger manager output configure register"]
    pub TRGOCFG_ADC1_STRGI: crate::RWRegister<u32>,
    #[doc = "Trigger manager output configure register"]
    pub TRGOCFG_ADC2_STRGI: crate::RWRegister<u32>,
    #[doc = "Trigger manager output configure register"]
    pub TRGOCFG_ADC3_STRGI: crate::RWRegister<u32>,
    #[doc = "Trigger manager output configure register"]
    pub TRGOCFG_ADCX_PTRGI0A: crate::RWRegister<u32>,
    #[doc = "Trigger manager output configure register"]
    pub TRGOCFG_ADCX_PTRGI0B: crate::RWRegister<u32>,
    #[doc = "Trigger manager output configure register"]
    pub TRGOCFG_ADCX_PTRGI0C: crate::RWRegister<u32>,
    #[doc = "Trigger manager output configure register"]
    pub TRGOCFG_GPTMRA_SYNCI: crate::RWRegister<u32>,
    #[doc = "Trigger manager output configure register"]
    pub TRGOCFG_GPTMRA_IN2: crate::RWRegister<u32>,
    #[doc = "Trigger manager output configure register"]
    pub TRGOCFG_GPTMRA_IN3: crate::RWRegister<u32>,
    #[doc = "Trigger manager output configure register"]
    pub TRGOCFG_GPTMRB_SYNCI: crate::RWRegister<u32>,
    #[doc = "Trigger manager output configure register"]
    pub TRGOCFG_GPTMRB_IN2: crate::RWRegister<u32>,
    #[doc = "Trigger manager output configure register"]
    pub TRGOCFG_GPTMRB_IN3: crate::RWRegister<u32>,
    #[doc = "Trigger manager output configure register"]
    pub TRGOCFG_CMPX_WIN: crate::RWRegister<u32>,
    #[doc = "Trigger manager output configure register"]
    pub TRGOCFG_CAN_PTPC0_CAP: crate::RWRegister<u32>,
    #[doc = "Trigger manager output configure register"]
    pub TRGOCFG_CAN_PTPC1_CAP: crate::RWRegister<u32>,
    #[doc = "DMA request configure register"]
    pub DMACFG_0: crate::RWRegister<u32>,
    #[doc = "DMA request configure register"]
    pub DMACFG_1: crate::RWRegister<u32>,
    #[doc = "DMA request configure register"]
    pub DMACFG_2: crate::RWRegister<u32>,
    #[doc = "DMA request configure register"]
    pub DMACFG_3: crate::RWRegister<u32>,
    _reserved1: [u8; 0x01f0],
    #[doc = "General Control Register"]
    pub GCR: crate::RWRegister<u32>,
}
#[doc = "Filter configure register"]
pub mod FILTCFG_PWM_IN0 {
    #[doc = "This bitfields defines the filter counter length."]
    pub mod FILTLEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set to enable sychronization input signal with TRGM clock"]
    pub mod SYNCEN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfields defines the filter mode 000-bypass; 100-rapid change mode; 101-delay filter mode; 110-stalbe low mode; 111-stable high mode"]
    pub mod MODE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Filter will invert the output 0- Filter will not invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Filter configure register"]
pub mod FILTCFG_PWM_IN1 {
    #[doc = "This bitfields defines the filter counter length."]
    pub mod FILTLEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set to enable sychronization input signal with TRGM clock"]
    pub mod SYNCEN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfields defines the filter mode 000-bypass; 100-rapid change mode; 101-delay filter mode; 110-stalbe low mode; 111-stable high mode"]
    pub mod MODE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Filter will invert the output 0- Filter will not invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Filter configure register"]
pub mod FILTCFG_PWM_IN2 {
    #[doc = "This bitfields defines the filter counter length."]
    pub mod FILTLEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set to enable sychronization input signal with TRGM clock"]
    pub mod SYNCEN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfields defines the filter mode 000-bypass; 100-rapid change mode; 101-delay filter mode; 110-stalbe low mode; 111-stable high mode"]
    pub mod MODE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Filter will invert the output 0- Filter will not invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Filter configure register"]
pub mod FILTCFG_PWM_IN3 {
    #[doc = "This bitfields defines the filter counter length."]
    pub mod FILTLEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set to enable sychronization input signal with TRGM clock"]
    pub mod SYNCEN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfields defines the filter mode 000-bypass; 100-rapid change mode; 101-delay filter mode; 110-stalbe low mode; 111-stable high mode"]
    pub mod MODE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Filter will invert the output 0- Filter will not invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Filter configure register"]
pub mod FILTCFG_PWM_IN4 {
    #[doc = "This bitfields defines the filter counter length."]
    pub mod FILTLEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set to enable sychronization input signal with TRGM clock"]
    pub mod SYNCEN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfields defines the filter mode 000-bypass; 100-rapid change mode; 101-delay filter mode; 110-stalbe low mode; 111-stable high mode"]
    pub mod MODE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Filter will invert the output 0- Filter will not invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Filter configure register"]
pub mod FILTCFG_PWM_IN5 {
    #[doc = "This bitfields defines the filter counter length."]
    pub mod FILTLEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set to enable sychronization input signal with TRGM clock"]
    pub mod SYNCEN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfields defines the filter mode 000-bypass; 100-rapid change mode; 101-delay filter mode; 110-stalbe low mode; 111-stable high mode"]
    pub mod MODE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Filter will invert the output 0- Filter will not invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Filter configure register"]
pub mod FILTCFG_PWM_IN6 {
    #[doc = "This bitfields defines the filter counter length."]
    pub mod FILTLEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set to enable sychronization input signal with TRGM clock"]
    pub mod SYNCEN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfields defines the filter mode 000-bypass; 100-rapid change mode; 101-delay filter mode; 110-stalbe low mode; 111-stable high mode"]
    pub mod MODE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Filter will invert the output 0- Filter will not invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Filter configure register"]
pub mod FILTCFG_PWM_IN7 {
    #[doc = "This bitfields defines the filter counter length."]
    pub mod FILTLEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set to enable sychronization input signal with TRGM clock"]
    pub mod SYNCEN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfields defines the filter mode 000-bypass; 100-rapid change mode; 101-delay filter mode; 110-stalbe low mode; 111-stable high mode"]
    pub mod MODE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Filter will invert the output 0- Filter will not invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Filter configure register"]
pub mod FILTCFG_TRGM_IN0 {
    #[doc = "This bitfields defines the filter counter length."]
    pub mod FILTLEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set to enable sychronization input signal with TRGM clock"]
    pub mod SYNCEN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfields defines the filter mode 000-bypass; 100-rapid change mode; 101-delay filter mode; 110-stalbe low mode; 111-stable high mode"]
    pub mod MODE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Filter will invert the output 0- Filter will not invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Filter configure register"]
pub mod FILTCFG_TRGM_IN1 {
    #[doc = "This bitfields defines the filter counter length."]
    pub mod FILTLEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set to enable sychronization input signal with TRGM clock"]
    pub mod SYNCEN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfields defines the filter mode 000-bypass; 100-rapid change mode; 101-delay filter mode; 110-stalbe low mode; 111-stable high mode"]
    pub mod MODE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Filter will invert the output 0- Filter will not invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Filter configure register"]
pub mod FILTCFG_TRGM_IN2 {
    #[doc = "This bitfields defines the filter counter length."]
    pub mod FILTLEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set to enable sychronization input signal with TRGM clock"]
    pub mod SYNCEN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfields defines the filter mode 000-bypass; 100-rapid change mode; 101-delay filter mode; 110-stalbe low mode; 111-stable high mode"]
    pub mod MODE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Filter will invert the output 0- Filter will not invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Filter configure register"]
pub mod FILTCFG_TRGM_IN3 {
    #[doc = "This bitfields defines the filter counter length."]
    pub mod FILTLEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set to enable sychronization input signal with TRGM clock"]
    pub mod SYNCEN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfields defines the filter mode 000-bypass; 100-rapid change mode; 101-delay filter mode; 110-stalbe low mode; 111-stable high mode"]
    pub mod MODE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Filter will invert the output 0- Filter will not invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Filter configure register"]
pub mod FILTCFG_TRGM_IN4 {
    #[doc = "This bitfields defines the filter counter length."]
    pub mod FILTLEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set to enable sychronization input signal with TRGM clock"]
    pub mod SYNCEN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfields defines the filter mode 000-bypass; 100-rapid change mode; 101-delay filter mode; 110-stalbe low mode; 111-stable high mode"]
    pub mod MODE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Filter will invert the output 0- Filter will not invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Filter configure register"]
pub mod FILTCFG_TRGM_IN5 {
    #[doc = "This bitfields defines the filter counter length."]
    pub mod FILTLEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set to enable sychronization input signal with TRGM clock"]
    pub mod SYNCEN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfields defines the filter mode 000-bypass; 100-rapid change mode; 101-delay filter mode; 110-stalbe low mode; 111-stable high mode"]
    pub mod MODE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Filter will invert the output 0- Filter will not invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Filter configure register"]
pub mod FILTCFG_TRGM_IN6 {
    #[doc = "This bitfields defines the filter counter length."]
    pub mod FILTLEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set to enable sychronization input signal with TRGM clock"]
    pub mod SYNCEN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfields defines the filter mode 000-bypass; 100-rapid change mode; 101-delay filter mode; 110-stalbe low mode; 111-stable high mode"]
    pub mod MODE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Filter will invert the output 0- Filter will not invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Filter configure register"]
pub mod FILTCFG_TRGM_IN7 {
    #[doc = "This bitfields defines the filter counter length."]
    pub mod FILTLEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set to enable sychronization input signal with TRGM clock"]
    pub mod SYNCEN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfields defines the filter mode 000-bypass; 100-rapid change mode; 101-delay filter mode; 110-stalbe low mode; 111-stable high mode"]
    pub mod MODE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Filter will invert the output 0- Filter will not invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Filter configure register"]
pub mod FILTCFG_TRGM_IN8 {
    #[doc = "This bitfields defines the filter counter length."]
    pub mod FILTLEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set to enable sychronization input signal with TRGM clock"]
    pub mod SYNCEN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfields defines the filter mode 000-bypass; 100-rapid change mode; 101-delay filter mode; 110-stalbe low mode; 111-stable high mode"]
    pub mod MODE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Filter will invert the output 0- Filter will not invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Filter configure register"]
pub mod FILTCFG_TRGM_IN9 {
    #[doc = "This bitfields defines the filter counter length."]
    pub mod FILTLEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set to enable sychronization input signal with TRGM clock"]
    pub mod SYNCEN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfields defines the filter mode 000-bypass; 100-rapid change mode; 101-delay filter mode; 110-stalbe low mode; 111-stable high mode"]
    pub mod MODE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Filter will invert the output 0- Filter will not invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Filter configure register"]
pub mod FILTCFG_TRGM_IN10 {
    #[doc = "This bitfields defines the filter counter length."]
    pub mod FILTLEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set to enable sychronization input signal with TRGM clock"]
    pub mod SYNCEN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfields defines the filter mode 000-bypass; 100-rapid change mode; 101-delay filter mode; 110-stalbe low mode; 111-stable high mode"]
    pub mod MODE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Filter will invert the output 0- Filter will not invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Filter configure register"]
pub mod FILTCFG_TRGM_IN11 {
    #[doc = "This bitfields defines the filter counter length."]
    pub mod FILTLEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set to enable sychronization input signal with TRGM clock"]
    pub mod SYNCEN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bitfields defines the filter mode 000-bypass; 100-rapid change mode; 101-delay filter mode; 110-stalbe low mode; 111-stable high mode"]
    pub mod MODE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Filter will invert the output 0- Filter will not invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Trigger manager output configure register"]
pub mod TRGOCFG_TRGM_OUT0 {
    #[doc = "This bitfield selects one of the TRGM inputs as output."]
    pub mod TRIGOSEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal rising edge will be convert to an pulse on output."]
    pub mod REDG2PEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal falling edge will be convert to an pulse on output."]
    pub mod FEDG2PEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Trigger manager output configure register"]
pub mod TRGOCFG_TRGM_OUT1 {
    #[doc = "This bitfield selects one of the TRGM inputs as output."]
    pub mod TRIGOSEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal rising edge will be convert to an pulse on output."]
    pub mod REDG2PEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal falling edge will be convert to an pulse on output."]
    pub mod FEDG2PEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Trigger manager output configure register"]
pub mod TRGOCFG_TRGM_OUT2 {
    #[doc = "This bitfield selects one of the TRGM inputs as output."]
    pub mod TRIGOSEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal rising edge will be convert to an pulse on output."]
    pub mod REDG2PEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal falling edge will be convert to an pulse on output."]
    pub mod FEDG2PEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Trigger manager output configure register"]
pub mod TRGOCFG_TRGM_OUT3 {
    #[doc = "This bitfield selects one of the TRGM inputs as output."]
    pub mod TRIGOSEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal rising edge will be convert to an pulse on output."]
    pub mod REDG2PEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal falling edge will be convert to an pulse on output."]
    pub mod FEDG2PEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Trigger manager output configure register"]
pub mod TRGOCFG_TRGM_OUT4 {
    #[doc = "This bitfield selects one of the TRGM inputs as output."]
    pub mod TRIGOSEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal rising edge will be convert to an pulse on output."]
    pub mod REDG2PEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal falling edge will be convert to an pulse on output."]
    pub mod FEDG2PEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Trigger manager output configure register"]
pub mod TRGOCFG_TRGM_OUT5 {
    #[doc = "This bitfield selects one of the TRGM inputs as output."]
    pub mod TRIGOSEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal rising edge will be convert to an pulse on output."]
    pub mod REDG2PEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal falling edge will be convert to an pulse on output."]
    pub mod FEDG2PEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Trigger manager output configure register"]
pub mod TRGOCFG_TRGM_OUT6 {
    #[doc = "This bitfield selects one of the TRGM inputs as output."]
    pub mod TRIGOSEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal rising edge will be convert to an pulse on output."]
    pub mod REDG2PEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal falling edge will be convert to an pulse on output."]
    pub mod FEDG2PEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Trigger manager output configure register"]
pub mod TRGOCFG_TRGM_OUT7 {
    #[doc = "This bitfield selects one of the TRGM inputs as output."]
    pub mod TRIGOSEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal rising edge will be convert to an pulse on output."]
    pub mod REDG2PEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal falling edge will be convert to an pulse on output."]
    pub mod FEDG2PEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Trigger manager output configure register"]
pub mod TRGOCFG_TRGM_OUT8 {
    #[doc = "This bitfield selects one of the TRGM inputs as output."]
    pub mod TRIGOSEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal rising edge will be convert to an pulse on output."]
    pub mod REDG2PEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal falling edge will be convert to an pulse on output."]
    pub mod FEDG2PEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Trigger manager output configure register"]
pub mod TRGOCFG_TRGM_OUT9 {
    #[doc = "This bitfield selects one of the TRGM inputs as output."]
    pub mod TRIGOSEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal rising edge will be convert to an pulse on output."]
    pub mod REDG2PEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal falling edge will be convert to an pulse on output."]
    pub mod FEDG2PEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Trigger manager output configure register"]
pub mod TRGOCFG_TRGM_OUT10 {
    #[doc = "This bitfield selects one of the TRGM inputs as output."]
    pub mod TRIGOSEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal rising edge will be convert to an pulse on output."]
    pub mod REDG2PEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal falling edge will be convert to an pulse on output."]
    pub mod FEDG2PEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Trigger manager output configure register"]
pub mod TRGOCFG_TRGM_OUT11 {
    #[doc = "This bitfield selects one of the TRGM inputs as output."]
    pub mod TRIGOSEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal rising edge will be convert to an pulse on output."]
    pub mod REDG2PEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal falling edge will be convert to an pulse on output."]
    pub mod FEDG2PEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Trigger manager output configure register"]
pub mod TRGOCFG_TRGM_OUTX0 {
    #[doc = "This bitfield selects one of the TRGM inputs as output."]
    pub mod TRIGOSEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal rising edge will be convert to an pulse on output."]
    pub mod REDG2PEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal falling edge will be convert to an pulse on output."]
    pub mod FEDG2PEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Trigger manager output configure register"]
pub mod TRGOCFG_TRGM_OUTX1 {
    #[doc = "This bitfield selects one of the TRGM inputs as output."]
    pub mod TRIGOSEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal rising edge will be convert to an pulse on output."]
    pub mod REDG2PEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal falling edge will be convert to an pulse on output."]
    pub mod FEDG2PEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Trigger manager output configure register"]
pub mod TRGOCFG_PWM_SYNCI {
    #[doc = "This bitfield selects one of the TRGM inputs as output."]
    pub mod TRIGOSEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal rising edge will be convert to an pulse on output."]
    pub mod REDG2PEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal falling edge will be convert to an pulse on output."]
    pub mod FEDG2PEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Trigger manager output configure register"]
pub mod TRGOCFG_PWM_FRCI {
    #[doc = "This bitfield selects one of the TRGM inputs as output."]
    pub mod TRIGOSEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal rising edge will be convert to an pulse on output."]
    pub mod REDG2PEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal falling edge will be convert to an pulse on output."]
    pub mod FEDG2PEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Trigger manager output configure register"]
pub mod TRGOCFG_PWM_FRCSYNCI {
    #[doc = "This bitfield selects one of the TRGM inputs as output."]
    pub mod TRIGOSEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal rising edge will be convert to an pulse on output."]
    pub mod REDG2PEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal falling edge will be convert to an pulse on output."]
    pub mod FEDG2PEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Trigger manager output configure register"]
pub mod TRGOCFG_PWM_SHRLDSYNCI {
    #[doc = "This bitfield selects one of the TRGM inputs as output."]
    pub mod TRIGOSEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal rising edge will be convert to an pulse on output."]
    pub mod REDG2PEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal falling edge will be convert to an pulse on output."]
    pub mod FEDG2PEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Trigger manager output configure register"]
pub mod TRGOCFG_PWM_FAULTI0 {
    #[doc = "This bitfield selects one of the TRGM inputs as output."]
    pub mod TRIGOSEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal rising edge will be convert to an pulse on output."]
    pub mod REDG2PEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal falling edge will be convert to an pulse on output."]
    pub mod FEDG2PEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Trigger manager output configure register"]
pub mod TRGOCFG_PWM_FAULTI1 {
    #[doc = "This bitfield selects one of the TRGM inputs as output."]
    pub mod TRIGOSEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal rising edge will be convert to an pulse on output."]
    pub mod REDG2PEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal falling edge will be convert to an pulse on output."]
    pub mod FEDG2PEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Trigger manager output configure register"]
pub mod TRGOCFG_PWM_FAULTI2 {
    #[doc = "This bitfield selects one of the TRGM inputs as output."]
    pub mod TRIGOSEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal rising edge will be convert to an pulse on output."]
    pub mod REDG2PEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal falling edge will be convert to an pulse on output."]
    pub mod FEDG2PEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Trigger manager output configure register"]
pub mod TRGOCFG_PWM_FAULTI3 {
    #[doc = "This bitfield selects one of the TRGM inputs as output."]
    pub mod TRIGOSEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal rising edge will be convert to an pulse on output."]
    pub mod REDG2PEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal falling edge will be convert to an pulse on output."]
    pub mod FEDG2PEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Trigger manager output configure register"]
pub mod TRGOCFG_PWM_IN8 {
    #[doc = "This bitfield selects one of the TRGM inputs as output."]
    pub mod TRIGOSEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal rising edge will be convert to an pulse on output."]
    pub mod REDG2PEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal falling edge will be convert to an pulse on output."]
    pub mod FEDG2PEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Trigger manager output configure register"]
pub mod TRGOCFG_PWM_IN9 {
    #[doc = "This bitfield selects one of the TRGM inputs as output."]
    pub mod TRIGOSEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal rising edge will be convert to an pulse on output."]
    pub mod REDG2PEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal falling edge will be convert to an pulse on output."]
    pub mod FEDG2PEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Trigger manager output configure register"]
pub mod TRGOCFG_PWM_IN10 {
    #[doc = "This bitfield selects one of the TRGM inputs as output."]
    pub mod TRIGOSEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal rising edge will be convert to an pulse on output."]
    pub mod REDG2PEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal falling edge will be convert to an pulse on output."]
    pub mod FEDG2PEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Trigger manager output configure register"]
pub mod TRGOCFG_PWM_IN11 {
    #[doc = "This bitfield selects one of the TRGM inputs as output."]
    pub mod TRIGOSEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal rising edge will be convert to an pulse on output."]
    pub mod REDG2PEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal falling edge will be convert to an pulse on output."]
    pub mod FEDG2PEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Trigger manager output configure register"]
pub mod TRGOCFG_PWM_IN12 {
    #[doc = "This bitfield selects one of the TRGM inputs as output."]
    pub mod TRIGOSEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal rising edge will be convert to an pulse on output."]
    pub mod REDG2PEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal falling edge will be convert to an pulse on output."]
    pub mod FEDG2PEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Trigger manager output configure register"]
pub mod TRGOCFG_PWM_IN13 {
    #[doc = "This bitfield selects one of the TRGM inputs as output."]
    pub mod TRIGOSEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal rising edge will be convert to an pulse on output."]
    pub mod REDG2PEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal falling edge will be convert to an pulse on output."]
    pub mod FEDG2PEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Trigger manager output configure register"]
pub mod TRGOCFG_PWM_IN14 {
    #[doc = "This bitfield selects one of the TRGM inputs as output."]
    pub mod TRIGOSEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal rising edge will be convert to an pulse on output."]
    pub mod REDG2PEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal falling edge will be convert to an pulse on output."]
    pub mod FEDG2PEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Trigger manager output configure register"]
pub mod TRGOCFG_PWM_IN15 {
    #[doc = "This bitfield selects one of the TRGM inputs as output."]
    pub mod TRIGOSEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal rising edge will be convert to an pulse on output."]
    pub mod REDG2PEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal falling edge will be convert to an pulse on output."]
    pub mod FEDG2PEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Trigger manager output configure register"]
pub mod TRGOCFG_PWM_IN16 {
    #[doc = "This bitfield selects one of the TRGM inputs as output."]
    pub mod TRIGOSEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal rising edge will be convert to an pulse on output."]
    pub mod REDG2PEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal falling edge will be convert to an pulse on output."]
    pub mod FEDG2PEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Trigger manager output configure register"]
pub mod TRGOCFG_PWM_IN17 {
    #[doc = "This bitfield selects one of the TRGM inputs as output."]
    pub mod TRIGOSEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal rising edge will be convert to an pulse on output."]
    pub mod REDG2PEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal falling edge will be convert to an pulse on output."]
    pub mod FEDG2PEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Trigger manager output configure register"]
pub mod TRGOCFG_PWM_IN18 {
    #[doc = "This bitfield selects one of the TRGM inputs as output."]
    pub mod TRIGOSEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal rising edge will be convert to an pulse on output."]
    pub mod REDG2PEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal falling edge will be convert to an pulse on output."]
    pub mod FEDG2PEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Trigger manager output configure register"]
pub mod TRGOCFG_PWM_IN19 {
    #[doc = "This bitfield selects one of the TRGM inputs as output."]
    pub mod TRIGOSEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal rising edge will be convert to an pulse on output."]
    pub mod REDG2PEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal falling edge will be convert to an pulse on output."]
    pub mod FEDG2PEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Trigger manager output configure register"]
pub mod TRGOCFG_PWM_IN20 {
    #[doc = "This bitfield selects one of the TRGM inputs as output."]
    pub mod TRIGOSEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal rising edge will be convert to an pulse on output."]
    pub mod REDG2PEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal falling edge will be convert to an pulse on output."]
    pub mod FEDG2PEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Trigger manager output configure register"]
pub mod TRGOCFG_PWM_IN21 {
    #[doc = "This bitfield selects one of the TRGM inputs as output."]
    pub mod TRIGOSEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal rising edge will be convert to an pulse on output."]
    pub mod REDG2PEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal falling edge will be convert to an pulse on output."]
    pub mod FEDG2PEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Trigger manager output configure register"]
pub mod TRGOCFG_PWM_IN22 {
    #[doc = "This bitfield selects one of the TRGM inputs as output."]
    pub mod TRIGOSEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal rising edge will be convert to an pulse on output."]
    pub mod REDG2PEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal falling edge will be convert to an pulse on output."]
    pub mod FEDG2PEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Trigger manager output configure register"]
pub mod TRGOCFG_PWM_IN23 {
    #[doc = "This bitfield selects one of the TRGM inputs as output."]
    pub mod TRIGOSEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal rising edge will be convert to an pulse on output."]
    pub mod REDG2PEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal falling edge will be convert to an pulse on output."]
    pub mod FEDG2PEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Trigger manager output configure register"]
pub mod TRGOCFG_QEI_A {
    #[doc = "This bitfield selects one of the TRGM inputs as output."]
    pub mod TRIGOSEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal rising edge will be convert to an pulse on output."]
    pub mod REDG2PEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal falling edge will be convert to an pulse on output."]
    pub mod FEDG2PEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Trigger manager output configure register"]
pub mod TRGOCFG_QEI_B {
    #[doc = "This bitfield selects one of the TRGM inputs as output."]
    pub mod TRIGOSEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal rising edge will be convert to an pulse on output."]
    pub mod REDG2PEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal falling edge will be convert to an pulse on output."]
    pub mod FEDG2PEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Trigger manager output configure register"]
pub mod TRGOCFG_QEI_Z {
    #[doc = "This bitfield selects one of the TRGM inputs as output."]
    pub mod TRIGOSEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal rising edge will be convert to an pulse on output."]
    pub mod REDG2PEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal falling edge will be convert to an pulse on output."]
    pub mod FEDG2PEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Trigger manager output configure register"]
pub mod TRGOCFG_QEI_H {
    #[doc = "This bitfield selects one of the TRGM inputs as output."]
    pub mod TRIGOSEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal rising edge will be convert to an pulse on output."]
    pub mod REDG2PEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal falling edge will be convert to an pulse on output."]
    pub mod FEDG2PEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Trigger manager output configure register"]
pub mod TRGOCFG_QEI_PAUSE {
    #[doc = "This bitfield selects one of the TRGM inputs as output."]
    pub mod TRIGOSEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal rising edge will be convert to an pulse on output."]
    pub mod REDG2PEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal falling edge will be convert to an pulse on output."]
    pub mod FEDG2PEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Trigger manager output configure register"]
pub mod TRGOCFG_QEI_SNAPI {
    #[doc = "This bitfield selects one of the TRGM inputs as output."]
    pub mod TRIGOSEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal rising edge will be convert to an pulse on output."]
    pub mod REDG2PEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal falling edge will be convert to an pulse on output."]
    pub mod FEDG2PEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Trigger manager output configure register"]
pub mod TRGOCFG_HALL_U {
    #[doc = "This bitfield selects one of the TRGM inputs as output."]
    pub mod TRIGOSEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal rising edge will be convert to an pulse on output."]
    pub mod REDG2PEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal falling edge will be convert to an pulse on output."]
    pub mod FEDG2PEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Trigger manager output configure register"]
pub mod TRGOCFG_HALL_V {
    #[doc = "This bitfield selects one of the TRGM inputs as output."]
    pub mod TRIGOSEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal rising edge will be convert to an pulse on output."]
    pub mod REDG2PEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal falling edge will be convert to an pulse on output."]
    pub mod FEDG2PEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Trigger manager output configure register"]
pub mod TRGOCFG_HALL_W {
    #[doc = "This bitfield selects one of the TRGM inputs as output."]
    pub mod TRIGOSEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal rising edge will be convert to an pulse on output."]
    pub mod REDG2PEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal falling edge will be convert to an pulse on output."]
    pub mod FEDG2PEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Trigger manager output configure register"]
pub mod TRGOCFG_HALL_SNAPI {
    #[doc = "This bitfield selects one of the TRGM inputs as output."]
    pub mod TRIGOSEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal rising edge will be convert to an pulse on output."]
    pub mod REDG2PEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal falling edge will be convert to an pulse on output."]
    pub mod FEDG2PEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Trigger manager output configure register"]
pub mod TRGOCFG_ADC0_STRGI {
    #[doc = "This bitfield selects one of the TRGM inputs as output."]
    pub mod TRIGOSEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal rising edge will be convert to an pulse on output."]
    pub mod REDG2PEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal falling edge will be convert to an pulse on output."]
    pub mod FEDG2PEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Trigger manager output configure register"]
pub mod TRGOCFG_ADC1_STRGI {
    #[doc = "This bitfield selects one of the TRGM inputs as output."]
    pub mod TRIGOSEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal rising edge will be convert to an pulse on output."]
    pub mod REDG2PEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal falling edge will be convert to an pulse on output."]
    pub mod FEDG2PEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Trigger manager output configure register"]
pub mod TRGOCFG_ADC2_STRGI {
    #[doc = "This bitfield selects one of the TRGM inputs as output."]
    pub mod TRIGOSEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal rising edge will be convert to an pulse on output."]
    pub mod REDG2PEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal falling edge will be convert to an pulse on output."]
    pub mod FEDG2PEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Trigger manager output configure register"]
pub mod TRGOCFG_ADC3_STRGI {
    #[doc = "This bitfield selects one of the TRGM inputs as output."]
    pub mod TRIGOSEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal rising edge will be convert to an pulse on output."]
    pub mod REDG2PEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal falling edge will be convert to an pulse on output."]
    pub mod FEDG2PEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Trigger manager output configure register"]
pub mod TRGOCFG_ADCX_PTRGI0A {
    #[doc = "This bitfield selects one of the TRGM inputs as output."]
    pub mod TRIGOSEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal rising edge will be convert to an pulse on output."]
    pub mod REDG2PEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal falling edge will be convert to an pulse on output."]
    pub mod FEDG2PEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Trigger manager output configure register"]
pub mod TRGOCFG_ADCX_PTRGI0B {
    #[doc = "This bitfield selects one of the TRGM inputs as output."]
    pub mod TRIGOSEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal rising edge will be convert to an pulse on output."]
    pub mod REDG2PEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal falling edge will be convert to an pulse on output."]
    pub mod FEDG2PEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Trigger manager output configure register"]
pub mod TRGOCFG_ADCX_PTRGI0C {
    #[doc = "This bitfield selects one of the TRGM inputs as output."]
    pub mod TRIGOSEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal rising edge will be convert to an pulse on output."]
    pub mod REDG2PEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal falling edge will be convert to an pulse on output."]
    pub mod FEDG2PEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Trigger manager output configure register"]
pub mod TRGOCFG_GPTMRA_SYNCI {
    #[doc = "This bitfield selects one of the TRGM inputs as output."]
    pub mod TRIGOSEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal rising edge will be convert to an pulse on output."]
    pub mod REDG2PEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal falling edge will be convert to an pulse on output."]
    pub mod FEDG2PEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Trigger manager output configure register"]
pub mod TRGOCFG_GPTMRA_IN2 {
    #[doc = "This bitfield selects one of the TRGM inputs as output."]
    pub mod TRIGOSEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal rising edge will be convert to an pulse on output."]
    pub mod REDG2PEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal falling edge will be convert to an pulse on output."]
    pub mod FEDG2PEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Trigger manager output configure register"]
pub mod TRGOCFG_GPTMRA_IN3 {
    #[doc = "This bitfield selects one of the TRGM inputs as output."]
    pub mod TRIGOSEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal rising edge will be convert to an pulse on output."]
    pub mod REDG2PEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal falling edge will be convert to an pulse on output."]
    pub mod FEDG2PEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Trigger manager output configure register"]
pub mod TRGOCFG_GPTMRB_SYNCI {
    #[doc = "This bitfield selects one of the TRGM inputs as output."]
    pub mod TRIGOSEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal rising edge will be convert to an pulse on output."]
    pub mod REDG2PEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal falling edge will be convert to an pulse on output."]
    pub mod FEDG2PEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Trigger manager output configure register"]
pub mod TRGOCFG_GPTMRB_IN2 {
    #[doc = "This bitfield selects one of the TRGM inputs as output."]
    pub mod TRIGOSEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal rising edge will be convert to an pulse on output."]
    pub mod REDG2PEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal falling edge will be convert to an pulse on output."]
    pub mod FEDG2PEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Trigger manager output configure register"]
pub mod TRGOCFG_GPTMRB_IN3 {
    #[doc = "This bitfield selects one of the TRGM inputs as output."]
    pub mod TRIGOSEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal rising edge will be convert to an pulse on output."]
    pub mod REDG2PEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal falling edge will be convert to an pulse on output."]
    pub mod FEDG2PEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Trigger manager output configure register"]
pub mod TRGOCFG_CMPX_WIN {
    #[doc = "This bitfield selects one of the TRGM inputs as output."]
    pub mod TRIGOSEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal rising edge will be convert to an pulse on output."]
    pub mod REDG2PEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal falling edge will be convert to an pulse on output."]
    pub mod FEDG2PEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Trigger manager output configure register"]
pub mod TRGOCFG_CAN_PTPC0_CAP {
    #[doc = "This bitfield selects one of the TRGM inputs as output."]
    pub mod TRIGOSEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal rising edge will be convert to an pulse on output."]
    pub mod REDG2PEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal falling edge will be convert to an pulse on output."]
    pub mod FEDG2PEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Trigger manager output configure register"]
pub mod TRGOCFG_CAN_PTPC1_CAP {
    #[doc = "This bitfield selects one of the TRGM inputs as output."]
    pub mod TRIGOSEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal rising edge will be convert to an pulse on output."]
    pub mod REDG2PEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- The selected input signal falling edge will be convert to an pulse on output."]
    pub mod FEDG2PEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Invert the output"]
    pub mod OUTINV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA request configure register"]
pub mod DMACFG_0 {
    #[doc = "This field selects one of the DMA requests as the DMA request output."]
    pub mod DMASRCSEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA request configure register"]
pub mod DMACFG_1 {
    #[doc = "This field selects one of the DMA requests as the DMA request output."]
    pub mod DMASRCSEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA request configure register"]
pub mod DMACFG_2 {
    #[doc = "This field selects one of the DMA requests as the DMA request output."]
    pub mod DMASRCSEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA request configure register"]
pub mod DMACFG_3 {
    #[doc = "This field selects one of the DMA requests as the DMA request output."]
    pub mod DMASRCSEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "General Control Register"]
pub mod GCR {
    #[doc = "The bitfield enable the TRGM outputs."]
    pub mod TRGOPEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
