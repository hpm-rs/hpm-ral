#[doc = "PTPC"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Control Register 0"]
    pub PTPC_0_CTRL0: crate::RWRegister<u32>,
    #[doc = "Control Register 1"]
    pub PTPC_0_CTRL1: crate::RWRegister<u32>,
    #[doc = "timestamp high"]
    pub PTPC_0_TIMEH: crate::RWRegister<u32>,
    #[doc = "timestamp low"]
    pub PTPC_0_TIMEL: crate::RWRegister<u32>,
    #[doc = "timestamp update high"]
    pub PTPC_0_TS_UPDTH: crate::RWRegister<u32>,
    #[doc = "timestamp update low"]
    pub PTPC_0_TS_UPDTL: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub PTPC_0_ADDEND: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub PTPC_0_TARH: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub PTPC_0_TARL: crate::RWRegister<u32>,
    _reserved0: [u8; 0x08],
    #[doc = "No description avaiable"]
    pub PTPC_0_PPS_CTRL: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub PTPC_0_CAPT_SNAPH: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub PTPC_0_CAPT_SNAPL: crate::RWRegister<u32>,
    _reserved1: [u8; 0x0fc8],
    #[doc = "Control Register 0"]
    pub PTPC_1_CTRL0: crate::RWRegister<u32>,
    #[doc = "Control Register 1"]
    pub PTPC_1_CTRL1: crate::RWRegister<u32>,
    #[doc = "timestamp high"]
    pub PTPC_1_TIMEH: crate::RWRegister<u32>,
    #[doc = "timestamp low"]
    pub PTPC_1_TIMEL: crate::RWRegister<u32>,
    #[doc = "timestamp update high"]
    pub PTPC_1_TS_UPDTH: crate::RWRegister<u32>,
    #[doc = "timestamp update low"]
    pub PTPC_1_TS_UPDTL: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub PTPC_1_ADDEND: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub PTPC_1_TARH: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub PTPC_1_TARL: crate::RWRegister<u32>,
    _reserved2: [u8; 0x08],
    #[doc = "No description avaiable"]
    pub PTPC_1_PPS_CTRL: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub PTPC_1_CAPT_SNAPH: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub PTPC_1_CAPT_SNAPL: crate::RWRegister<u32>,
    _reserved3: [u8; 0x0fc8],
    #[doc = "No description avaiable"]
    pub TIME_SEL: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub INT_STS: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub INT_EN: crate::RWRegister<u32>,
}
#[doc = "Control Register 0"]
pub mod PTPC_0_CTRL0 {
    #[doc = "No description avaiable"]
    pub mod TIMER_ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "0: fine update, ns counter add ss_incr\\[7:0\\] each time addend counter overflow 1: coarse update, ns counter add ss_incr\\[7:0\\] each clk"]
    pub mod FINE_COARSE_SEL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "initial timer with ts_updt, pulse, clear after set"]
    pub mod INIT_TIMER {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "update timer with +/- ts_updt, pulse, clear after set"]
    pub mod UPDATE_TIMER {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set to enable compare, will be cleared by HW when compare event triggered"]
    pub mod COMP_EN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "No description avaiable"]
    pub mod CAPT_SNAP_NEG_EN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set will use posege of input capture signal to latch timestamp value"]
    pub mod CAPT_SNAP_POS_EN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set will keep capture snap till software read capt_snapl. If this bit is set, software should read capt_snaph first to avoid wrong result. If this bit is cleared, capture result will be updated at each capture event"]
    pub mod CAPT_SNAP_KEEP {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Format for ns counter rollover, 1-digital, overflow time 1000000000/0x3B9ACA00 0-binary, overflow time 0x7FFFFFFF"]
    pub mod SUBSEC_DIGITAL_ROLLOVER {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control Register 1"]
pub mod PTPC_0_CTRL1 {
    #[doc = "constant value used to add ns counter; such as for 50MHz timer clock, set it to 8'd20"]
    pub mod SS_INCR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "timestamp high"]
pub mod PTPC_0_TIMEH {
    #[doc = "No description avaiable"]
    pub mod TIMESTAMP_HIGH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "timestamp low"]
pub mod PTPC_0_TIMEL {
    #[doc = "No description avaiable"]
    pub mod TIMESTAMP_LOW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "timestamp update high"]
pub mod PTPC_0_TS_UPDTH {
    #[doc = "together with ts_updtl, used to initial or update timestamp"]
    pub mod SEC_UPDATE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "timestamp update low"]
pub mod PTPC_0_TS_UPDTL {
    #[doc = "No description avaiable"]
    pub mod NS_UPDATE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1 for sub; 0 for add, used only at update"]
    pub mod ADD_SUB {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PTPC_0_ADDEND {
    #[doc = "used in fine update mode only"]
    pub mod ADDEND {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PTPC_0_TARH {
    #[doc = "used for generate compare signal if enabled"]
    pub mod TARGET_TIME_HIGH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PTPC_0_TARL {
    #[doc = "No description avaiable"]
    pub mod TARGET_TIME_LOW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PTPC_0_PPS_CTRL {
    #[doc = "No description avaiable"]
    pub mod PPS_CTRL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PTPC_0_CAPT_SNAPH {
    #[doc = "take snapshot for input capture signal, at pos or neg or both; the result can be kept or updated at each event according to cfg0.bit8"]
    pub mod CAPT_SNAP_HIGH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PTPC_0_CAPT_SNAPL {
    #[doc = "No description avaiable"]
    pub mod CAPT_SNAP_LOW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control Register 0"]
pub mod PTPC_1_CTRL0 {
    #[doc = "No description avaiable"]
    pub mod TIMER_ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "0: fine update, ns counter add ss_incr\\[7:0\\] each time addend counter overflow 1: coarse update, ns counter add ss_incr\\[7:0\\] each clk"]
    pub mod FINE_COARSE_SEL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "initial timer with ts_updt, pulse, clear after set"]
    pub mod INIT_TIMER {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "update timer with +/- ts_updt, pulse, clear after set"]
    pub mod UPDATE_TIMER {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set to enable compare, will be cleared by HW when compare event triggered"]
    pub mod COMP_EN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "No description avaiable"]
    pub mod CAPT_SNAP_NEG_EN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set will use posege of input capture signal to latch timestamp value"]
    pub mod CAPT_SNAP_POS_EN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "set will keep capture snap till software read capt_snapl. If this bit is set, software should read capt_snaph first to avoid wrong result. If this bit is cleared, capture result will be updated at each capture event"]
    pub mod CAPT_SNAP_KEEP {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Format for ns counter rollover, 1-digital, overflow time 1000000000/0x3B9ACA00 0-binary, overflow time 0x7FFFFFFF"]
    pub mod SUBSEC_DIGITAL_ROLLOVER {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control Register 1"]
pub mod PTPC_1_CTRL1 {
    #[doc = "constant value used to add ns counter; such as for 50MHz timer clock, set it to 8'd20"]
    pub mod SS_INCR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "timestamp high"]
pub mod PTPC_1_TIMEH {
    #[doc = "No description avaiable"]
    pub mod TIMESTAMP_HIGH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "timestamp low"]
pub mod PTPC_1_TIMEL {
    #[doc = "No description avaiable"]
    pub mod TIMESTAMP_LOW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "timestamp update high"]
pub mod PTPC_1_TS_UPDTH {
    #[doc = "together with ts_updtl, used to initial or update timestamp"]
    pub mod SEC_UPDATE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "timestamp update low"]
pub mod PTPC_1_TS_UPDTL {
    #[doc = "No description avaiable"]
    pub mod NS_UPDATE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1 for sub; 0 for add, used only at update"]
    pub mod ADD_SUB {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PTPC_1_ADDEND {
    #[doc = "used in fine update mode only"]
    pub mod ADDEND {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PTPC_1_TARH {
    #[doc = "used for generate compare signal if enabled"]
    pub mod TARGET_TIME_HIGH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PTPC_1_TARL {
    #[doc = "No description avaiable"]
    pub mod TARGET_TIME_LOW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PTPC_1_PPS_CTRL {
    #[doc = "No description avaiable"]
    pub mod PPS_CTRL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PTPC_1_CAPT_SNAPH {
    #[doc = "take snapshot for input capture signal, at pos or neg or both; the result can be kept or updated at each event according to cfg0.bit8"]
    pub mod CAPT_SNAP_HIGH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PTPC_1_CAPT_SNAPL {
    #[doc = "No description avaiable"]
    pub mod CAPT_SNAP_LOW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod TIME_SEL {
    #[doc = "set to use ptpc1 for canx clr to use ptpc0 for canx"]
    pub mod CAN0_TIME_SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "No description avaiable"]
    pub mod CAN1_TIME_SEL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "No description avaiable"]
    pub mod CAN2_TIME_SEL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "No description avaiable"]
    pub mod CAN3_TIME_SEL {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod INT_STS {
    #[doc = "No description avaiable"]
    pub mod PPS_INT_STS0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "No description avaiable"]
    pub mod CAPTURE_INT_STS0 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "No description avaiable"]
    pub mod COMP_INT_STS0 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "No description avaiable"]
    pub mod PPS_INT_STS1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "No description avaiable"]
    pub mod CAPTURE_INT_STS1 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "No description avaiable"]
    pub mod COMP_INT_STS1 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod INT_EN {
    #[doc = "No description avaiable"]
    pub mod PPS_INT_STS0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "No description avaiable"]
    pub mod CAPTURE_INT_STS0 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "No description avaiable"]
    pub mod COMP_INT_STS0 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "No description avaiable"]
    pub mod PPS_INT_STS1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "No description avaiable"]
    pub mod CAPTURE_INT_STS1 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "No description avaiable"]
    pub mod COMP_INT_STS1 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
