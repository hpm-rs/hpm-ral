#[doc = "GPTMR0"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Control Register"]
    pub CHANNEL_CH0_CR: crate::RWRegister<u32>,
    #[doc = "Comparator register 0"]
    pub CHANNEL_CH0_CMP0: crate::RWRegister<u32>,
    #[doc = "Comparator register 1"]
    pub CHANNEL_CH0_CMP1: crate::RWRegister<u32>,
    #[doc = "Reload register"]
    pub CHANNEL_CH0_RLD: crate::RWRegister<u32>,
    #[doc = "Counter update value register"]
    pub CHANNEL_CH0_CNTUPTVAL: crate::RWRegister<u32>,
    _reserved0: [u8; 0x0c],
    #[doc = "Capture rising edge register"]
    pub CHANNEL_CH0_CAPPOS: crate::RWRegister<u32>,
    #[doc = "Capture falling edge register"]
    pub CHANNEL_CH0_CAPNEG: crate::RWRegister<u32>,
    #[doc = "PWM period measure register"]
    pub CHANNEL_CH0_CAPPRD: crate::RWRegister<u32>,
    #[doc = "PWM duty cycle measure register"]
    pub CHANNEL_CH0_CAPDTY: crate::RWRegister<u32>,
    #[doc = "Counter"]
    pub CHANNEL_CH0_CNT: crate::RWRegister<u32>,
    _reserved1: [u8; 0x0c],
    #[doc = "Control Register"]
    pub CHANNEL_CH1_CR: crate::RWRegister<u32>,
    #[doc = "Comparator register 0"]
    pub CHANNEL_CH1_CMP0: crate::RWRegister<u32>,
    #[doc = "Comparator register 1"]
    pub CHANNEL_CH1_CMP1: crate::RWRegister<u32>,
    #[doc = "Reload register"]
    pub CHANNEL_CH1_RLD: crate::RWRegister<u32>,
    #[doc = "Counter update value register"]
    pub CHANNEL_CH1_CNTUPTVAL: crate::RWRegister<u32>,
    _reserved2: [u8; 0x0c],
    #[doc = "Capture rising edge register"]
    pub CHANNEL_CH1_CAPPOS: crate::RWRegister<u32>,
    #[doc = "Capture falling edge register"]
    pub CHANNEL_CH1_CAPNEG: crate::RWRegister<u32>,
    #[doc = "PWM period measure register"]
    pub CHANNEL_CH1_CAPPRD: crate::RWRegister<u32>,
    #[doc = "PWM duty cycle measure register"]
    pub CHANNEL_CH1_CAPDTY: crate::RWRegister<u32>,
    #[doc = "Counter"]
    pub CHANNEL_CH1_CNT: crate::RWRegister<u32>,
    _reserved3: [u8; 0x0c],
    #[doc = "Control Register"]
    pub CHANNEL_CH2_CR: crate::RWRegister<u32>,
    #[doc = "Comparator register 0"]
    pub CHANNEL_CH2_CMP0: crate::RWRegister<u32>,
    #[doc = "Comparator register 1"]
    pub CHANNEL_CH2_CMP1: crate::RWRegister<u32>,
    #[doc = "Reload register"]
    pub CHANNEL_CH2_RLD: crate::RWRegister<u32>,
    #[doc = "Counter update value register"]
    pub CHANNEL_CH2_CNTUPTVAL: crate::RWRegister<u32>,
    _reserved4: [u8; 0x0c],
    #[doc = "Capture rising edge register"]
    pub CHANNEL_CH2_CAPPOS: crate::RWRegister<u32>,
    #[doc = "Capture falling edge register"]
    pub CHANNEL_CH2_CAPNEG: crate::RWRegister<u32>,
    #[doc = "PWM period measure register"]
    pub CHANNEL_CH2_CAPPRD: crate::RWRegister<u32>,
    #[doc = "PWM duty cycle measure register"]
    pub CHANNEL_CH2_CAPDTY: crate::RWRegister<u32>,
    #[doc = "Counter"]
    pub CHANNEL_CH2_CNT: crate::RWRegister<u32>,
    _reserved5: [u8; 0x0c],
    #[doc = "Control Register"]
    pub CHANNEL_CH3_CR: crate::RWRegister<u32>,
    #[doc = "Comparator register 0"]
    pub CHANNEL_CH3_CMP0: crate::RWRegister<u32>,
    #[doc = "Comparator register 1"]
    pub CHANNEL_CH3_CMP1: crate::RWRegister<u32>,
    #[doc = "Reload register"]
    pub CHANNEL_CH3_RLD: crate::RWRegister<u32>,
    #[doc = "Counter update value register"]
    pub CHANNEL_CH3_CNTUPTVAL: crate::RWRegister<u32>,
    _reserved6: [u8; 0x0c],
    #[doc = "Capture rising edge register"]
    pub CHANNEL_CH3_CAPPOS: crate::RWRegister<u32>,
    #[doc = "Capture falling edge register"]
    pub CHANNEL_CH3_CAPNEG: crate::RWRegister<u32>,
    #[doc = "PWM period measure register"]
    pub CHANNEL_CH3_CAPPRD: crate::RWRegister<u32>,
    #[doc = "PWM duty cycle measure register"]
    pub CHANNEL_CH3_CAPDTY: crate::RWRegister<u32>,
    #[doc = "Counter"]
    pub CHANNEL_CH3_CNT: crate::RWRegister<u32>,
    _reserved7: [u8; 0x010c],
    #[doc = "Status register"]
    pub SR: crate::RWRegister<u32>,
    #[doc = "Interrupt request enable register"]
    pub IRQEN: crate::RWRegister<u32>,
    #[doc = "Global control register"]
    pub GCR: crate::RWRegister<u32>,
}
#[doc = "Control Register"]
pub mod CHANNEL_CH0_CR {
    #[doc = "This bitfield define the input capture mode 100: width measure mode, timer will calculate the input signal period and duty cycle 011: capture at both rising edge and falling edge 010: capture at falling edge 001: capture at rising edge 000: No capture"]
    pub mod CAPMODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- counter will pause if chip is in debug mode"]
    pub mod DBGPAUSE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- enable software sync. When this bit is set, counter will reset to RLD when swsynct bit is set"]
    pub mod SWSYNCIEN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- enable dma"]
    pub mod DMAEN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "select one of DMA request: 00- RLD flag, counter reload; 01- Input signal toggle captured 10- CMP0 flag 11- CMP1 flag"]
    pub mod DMASEL {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Enable the channel output compare function. The output signal can be generated per comparator (CMPx) settings."]
    pub mod CMPEN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Output compare initial poliarity 1- The channel output initial level is high 0- The channel output initial level is low User should set this bit before set CMPEN to 1."]
    pub mod CMPINIT {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- counter enable"]
    pub mod CEN {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- SYNCI is valid on its rising edge"]
    pub mod SYNCIREN {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- SYNCI is valid on its falling edge"]
    pub mod SYNCIFEN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- enable this channel to reset counter to reload(RLD) together with its previous channel. This bit is not valid for channel 0."]
    pub mod SYNCFLW {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- reset counter"]
    pub mod CNTRST {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- update counter to new value as CNTUPTVAL This bit will be auto cleared after 1 cycle"]
    pub mod CNTUPT {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Comparator register 0"]
pub mod CHANNEL_CH0_CMP0 {
    #[doc = "compare value 0"]
    pub mod CMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Comparator register 1"]
pub mod CHANNEL_CH0_CMP1 {
    #[doc = "compare value 0"]
    pub mod CMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Reload register"]
pub mod CHANNEL_CH0_RLD {
    #[doc = "reload value"]
    pub mod RLD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Counter update value register"]
pub mod CHANNEL_CH0_CNTUPTVAL {
    #[doc = "counter will be set to this value when software write cntupt bit in CR"]
    pub mod CNTUPTVAL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture rising edge register"]
pub mod CHANNEL_CH0_CAPPOS {
    #[doc = "This register contains the counter value captured at input signal rising edge"]
    pub mod CAPPOS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture falling edge register"]
pub mod CHANNEL_CH0_CAPNEG {
    #[doc = "This register contains the counter value captured at input signal falling edge"]
    pub mod CAPNEG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PWM period measure register"]
pub mod CHANNEL_CH0_CAPPRD {
    #[doc = "This register contains the input signal period when channel is configured to input capture measure mode."]
    pub mod CAPPRD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PWM duty cycle measure register"]
pub mod CHANNEL_CH0_CAPDTY {
    #[doc = "This register contains the input signal duty cycle when channel is configured to input capture measure mode."]
    pub mod MEAS_HIGH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Counter"]
pub mod CHANNEL_CH0_CNT {
    #[doc = "32 bit counter value"]
    pub mod COUNTER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control Register"]
pub mod CHANNEL_CH1_CR {
    #[doc = "This bitfield define the input capture mode 100: width measure mode, timer will calculate the input signal period and duty cycle 011: capture at both rising edge and falling edge 010: capture at falling edge 001: capture at rising edge 000: No capture"]
    pub mod CAPMODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- counter will pause if chip is in debug mode"]
    pub mod DBGPAUSE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- enable software sync. When this bit is set, counter will reset to RLD when swsynct bit is set"]
    pub mod SWSYNCIEN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- enable dma"]
    pub mod DMAEN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "select one of DMA request: 00- RLD flag, counter reload; 01- Input signal toggle captured 10- CMP0 flag 11- CMP1 flag"]
    pub mod DMASEL {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Enable the channel output compare function. The output signal can be generated per comparator (CMPx) settings."]
    pub mod CMPEN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Output compare initial poliarity 1- The channel output initial level is high 0- The channel output initial level is low User should set this bit before set CMPEN to 1."]
    pub mod CMPINIT {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- counter enable"]
    pub mod CEN {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- SYNCI is valid on its rising edge"]
    pub mod SYNCIREN {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- SYNCI is valid on its falling edge"]
    pub mod SYNCIFEN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- enable this channel to reset counter to reload(RLD) together with its previous channel. This bit is not valid for channel 0."]
    pub mod SYNCFLW {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- reset counter"]
    pub mod CNTRST {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- update counter to new value as CNTUPTVAL This bit will be auto cleared after 1 cycle"]
    pub mod CNTUPT {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Comparator register 0"]
pub mod CHANNEL_CH1_CMP0 {
    #[doc = "compare value 0"]
    pub mod CMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Comparator register 1"]
pub mod CHANNEL_CH1_CMP1 {
    #[doc = "compare value 0"]
    pub mod CMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Reload register"]
pub mod CHANNEL_CH1_RLD {
    #[doc = "reload value"]
    pub mod RLD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Counter update value register"]
pub mod CHANNEL_CH1_CNTUPTVAL {
    #[doc = "counter will be set to this value when software write cntupt bit in CR"]
    pub mod CNTUPTVAL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture rising edge register"]
pub mod CHANNEL_CH1_CAPPOS {
    #[doc = "This register contains the counter value captured at input signal rising edge"]
    pub mod CAPPOS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture falling edge register"]
pub mod CHANNEL_CH1_CAPNEG {
    #[doc = "This register contains the counter value captured at input signal falling edge"]
    pub mod CAPNEG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PWM period measure register"]
pub mod CHANNEL_CH1_CAPPRD {
    #[doc = "This register contains the input signal period when channel is configured to input capture measure mode."]
    pub mod CAPPRD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PWM duty cycle measure register"]
pub mod CHANNEL_CH1_CAPDTY {
    #[doc = "This register contains the input signal duty cycle when channel is configured to input capture measure mode."]
    pub mod MEAS_HIGH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Counter"]
pub mod CHANNEL_CH1_CNT {
    #[doc = "32 bit counter value"]
    pub mod COUNTER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control Register"]
pub mod CHANNEL_CH2_CR {
    #[doc = "This bitfield define the input capture mode 100: width measure mode, timer will calculate the input signal period and duty cycle 011: capture at both rising edge and falling edge 010: capture at falling edge 001: capture at rising edge 000: No capture"]
    pub mod CAPMODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- counter will pause if chip is in debug mode"]
    pub mod DBGPAUSE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- enable software sync. When this bit is set, counter will reset to RLD when swsynct bit is set"]
    pub mod SWSYNCIEN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- enable dma"]
    pub mod DMAEN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "select one of DMA request: 00- RLD flag, counter reload; 01- Input signal toggle captured 10- CMP0 flag 11- CMP1 flag"]
    pub mod DMASEL {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Enable the channel output compare function. The output signal can be generated per comparator (CMPx) settings."]
    pub mod CMPEN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Output compare initial poliarity 1- The channel output initial level is high 0- The channel output initial level is low User should set this bit before set CMPEN to 1."]
    pub mod CMPINIT {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- counter enable"]
    pub mod CEN {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- SYNCI is valid on its rising edge"]
    pub mod SYNCIREN {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- SYNCI is valid on its falling edge"]
    pub mod SYNCIFEN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- enable this channel to reset counter to reload(RLD) together with its previous channel. This bit is not valid for channel 0."]
    pub mod SYNCFLW {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- reset counter"]
    pub mod CNTRST {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- update counter to new value as CNTUPTVAL This bit will be auto cleared after 1 cycle"]
    pub mod CNTUPT {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Comparator register 0"]
pub mod CHANNEL_CH2_CMP0 {
    #[doc = "compare value 0"]
    pub mod CMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Comparator register 1"]
pub mod CHANNEL_CH2_CMP1 {
    #[doc = "compare value 0"]
    pub mod CMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Reload register"]
pub mod CHANNEL_CH2_RLD {
    #[doc = "reload value"]
    pub mod RLD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Counter update value register"]
pub mod CHANNEL_CH2_CNTUPTVAL {
    #[doc = "counter will be set to this value when software write cntupt bit in CR"]
    pub mod CNTUPTVAL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture rising edge register"]
pub mod CHANNEL_CH2_CAPPOS {
    #[doc = "This register contains the counter value captured at input signal rising edge"]
    pub mod CAPPOS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture falling edge register"]
pub mod CHANNEL_CH2_CAPNEG {
    #[doc = "This register contains the counter value captured at input signal falling edge"]
    pub mod CAPNEG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PWM period measure register"]
pub mod CHANNEL_CH2_CAPPRD {
    #[doc = "This register contains the input signal period when channel is configured to input capture measure mode."]
    pub mod CAPPRD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PWM duty cycle measure register"]
pub mod CHANNEL_CH2_CAPDTY {
    #[doc = "This register contains the input signal duty cycle when channel is configured to input capture measure mode."]
    pub mod MEAS_HIGH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Counter"]
pub mod CHANNEL_CH2_CNT {
    #[doc = "32 bit counter value"]
    pub mod COUNTER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control Register"]
pub mod CHANNEL_CH3_CR {
    #[doc = "This bitfield define the input capture mode 100: width measure mode, timer will calculate the input signal period and duty cycle 011: capture at both rising edge and falling edge 010: capture at falling edge 001: capture at rising edge 000: No capture"]
    pub mod CAPMODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- counter will pause if chip is in debug mode"]
    pub mod DBGPAUSE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- enable software sync. When this bit is set, counter will reset to RLD when swsynct bit is set"]
    pub mod SWSYNCIEN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- enable dma"]
    pub mod DMAEN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "select one of DMA request: 00- RLD flag, counter reload; 01- Input signal toggle captured 10- CMP0 flag 11- CMP1 flag"]
    pub mod DMASEL {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- Enable the channel output compare function. The output signal can be generated per comparator (CMPx) settings."]
    pub mod CMPEN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Output compare initial poliarity 1- The channel output initial level is high 0- The channel output initial level is low User should set this bit before set CMPEN to 1."]
    pub mod CMPINIT {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- counter enable"]
    pub mod CEN {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- SYNCI is valid on its rising edge"]
    pub mod SYNCIREN {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- SYNCI is valid on its falling edge"]
    pub mod SYNCIFEN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- enable this channel to reset counter to reload(RLD) together with its previous channel. This bit is not valid for channel 0."]
    pub mod SYNCFLW {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- reset counter"]
    pub mod CNTRST {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- update counter to new value as CNTUPTVAL This bit will be auto cleared after 1 cycle"]
    pub mod CNTUPT {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Comparator register 0"]
pub mod CHANNEL_CH3_CMP0 {
    #[doc = "compare value 0"]
    pub mod CMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Comparator register 1"]
pub mod CHANNEL_CH3_CMP1 {
    #[doc = "compare value 0"]
    pub mod CMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Reload register"]
pub mod CHANNEL_CH3_RLD {
    #[doc = "reload value"]
    pub mod RLD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Counter update value register"]
pub mod CHANNEL_CH3_CNTUPTVAL {
    #[doc = "counter will be set to this value when software write cntupt bit in CR"]
    pub mod CNTUPTVAL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture rising edge register"]
pub mod CHANNEL_CH3_CAPPOS {
    #[doc = "This register contains the counter value captured at input signal rising edge"]
    pub mod CAPPOS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture falling edge register"]
pub mod CHANNEL_CH3_CAPNEG {
    #[doc = "This register contains the counter value captured at input signal falling edge"]
    pub mod CAPNEG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PWM period measure register"]
pub mod CHANNEL_CH3_CAPPRD {
    #[doc = "This register contains the input signal period when channel is configured to input capture measure mode."]
    pub mod CAPPRD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PWM duty cycle measure register"]
pub mod CHANNEL_CH3_CAPDTY {
    #[doc = "This register contains the input signal duty cycle when channel is configured to input capture measure mode."]
    pub mod MEAS_HIGH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Counter"]
pub mod CHANNEL_CH3_CNT {
    #[doc = "32 bit counter value"]
    pub mod COUNTER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status register"]
pub mod SR {
    #[doc = "channel 1 counter reload flag"]
    pub mod CH0RLDF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "channel 1 capture flag, the flag will be set at the valid capture edge per CAPMODE setting. If the capture channel is set to measure mode, the flag will be set at rising edge."]
    pub mod CH0CAPF {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "channel 1 compare value 1 match flag"]
    pub mod CH0CMP0F {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "channel 1 compare value 1 match flag"]
    pub mod CH0CMP1F {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "channel 1 counter reload flag"]
    pub mod CH1RLDF {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "channel 1 capture flag, the flag will be set at the valid capture edge per CAPMODE setting. If the capture channel is set to measure mode, the flag will be set at rising edge."]
    pub mod CH1CAPF {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "channel 1 compare value 1 match flag"]
    pub mod CH1CMP0F {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "channel 1 compare value 1 match flag"]
    pub mod CH1CMP1F {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "channel 2 counter reload flag"]
    pub mod CH2RLDF {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "channel 2 capture flag, the flag will be set at the valid capture edge per CAPMODE setting. If the capture channel is set to measure mode, the flag will be set at rising edge."]
    pub mod CH2CAPF {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "channel 2 compare value 1 match flag"]
    pub mod CH2CMP0F {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "channel 2 compare value 1 match flag"]
    pub mod CH2CMP1F {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "channel 3 counter reload flag"]
    pub mod CH3RLDF {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "channel 3 capture flag, the flag will be set at the valid capture edge per CAPMODE setting. If the capture channel is set to measure mode, the flag will be set at rising edge."]
    pub mod CH3CAPF {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "channel 3 compare value 1 match flag"]
    pub mod CH3CMP0F {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "channel 3 compare value 1 match flag"]
    pub mod CH3CMP1F {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt request enable register"]
pub mod IRQEN {
    #[doc = "1- generate interrupt request when ch0rldf flag is set"]
    pub mod CH0RLDEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- generate interrupt request when ch0capf flag is set"]
    pub mod CH0CAPEN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- generate interrupt request when ch0cmp0f flag is set"]
    pub mod CH0CMP0EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- generate interrupt request when ch0cmp1f flag is set"]
    pub mod CH0CMP1EN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- generate interrupt request when ch1rldf flag is set"]
    pub mod CH1RLDEN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- generate interrupt request when ch1capf flag is set"]
    pub mod CH1CAPEN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- generate interrupt request when ch1cmp0f flag is set"]
    pub mod CH1CMP0EN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- generate interrupt request when ch1cmp1f flag is set"]
    pub mod CH1CMP1EN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- generate interrupt request when ch2rldf flag is set"]
    pub mod CH2RLDEN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- generate interrupt request when ch2capf flag is set"]
    pub mod CH2CAPEN {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- generate interrupt request when ch2cmp0f flag is set"]
    pub mod CH2CMP0EN {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- generate interrupt request when ch2cmp1f flag is set"]
    pub mod CH2CMP1EN {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- generate interrupt request when ch3rldf flag is set"]
    pub mod CH3RLDEN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- generate interrupt request when ch3capf flag is set"]
    pub mod CH3CAPEN {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- generate interrupt request when ch3cmp0f flag is set"]
    pub mod CH3CMP0EN {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1- generate interrupt request when ch3cmp1f flag is set"]
    pub mod CH3CMP1EN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Global control register"]
pub mod GCR {
    #[doc = "set this bitfield to trigger software coutner sync event"]
    pub mod SWSYNCT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
