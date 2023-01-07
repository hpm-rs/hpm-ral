#[doc = "CONCTL"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "No description avaiable"]
    pub CTRL0: crate::RWRegister<u32>,
    _reserved0: [u8; 0x04],
    #[doc = "No description avaiable"]
    pub CTRL2: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CTRL3: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CTRL4: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CTRL5: crate::RWRegister<u32>,
}
#[doc = "No description avaiable"]
pub mod CTRL0 {
    #[doc = "No description avaiable"]
    pub mod ENET0_TXCLK_DLY_SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "No description avaiable"]
    pub mod ENET0_RXCLK_DLY_SEL {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "No description avaiable"]
    pub mod ENET1_TXCLK_DLY_SEL {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "No description avaiable"]
    pub mod ENET1_RXCLK_DLY_SEL {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CTRL2 {
    #[doc = "default to use internal clk. set from pad， two option here: internal 50MHz clock out to pad then in; use external clock;"]
    pub mod ENET0_RMII_TXCLK_SEL {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "No description avaiable"]
    pub mod ENET0_FLOWCTRL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "000:Reserved 001:RGMII 100:RMII 111:Reserved"]
    pub mod ENET0_PHY_INTF_SEL {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "No description avaiable"]
    pub mod ENET0_REFCLK_OE {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CTRL3 {
    #[doc = "No description avaiable"]
    pub mod ENET1_RMII_TXCLK_SEL {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "No description avaiable"]
    pub mod ENET1_FLOWCTRL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "No description avaiable"]
    pub mod ENET1_PHY_INTF_SEL {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "No description avaiable"]
    pub mod ENET1_REFCLK_OE {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CTRL4 {
    #[doc = "enable strobe clock, maybe used when update strobe DLL"]
    pub mod SDXC0_GPR_STROBE_IN_ENABLE {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "for strobe DLL, default 7taps(1ns)"]
    pub mod SDXC0_GPR_TUNING_STROBE_SEL {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "for card clock DLL, default 0"]
    pub mod SDXC0_GPR_TUNING_CARD_CLK_SEL {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "card clock inverter enable"]
    pub mod SDXC0_CARDCLK_INV_EN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "wakeup irq enable"]
    pub mod SDXC0_WKP_IRQ_EN {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "system irq enable"]
    pub mod SDXC0_SYS_IRQ_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CTRL5 {
    #[doc = "No description avaiable"]
    pub mod SDXC1_GPR_STROBE_IN_ENABLE {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "No description avaiable"]
    pub mod SDXC1_GPR_TUNING_STROBE_SEL {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "No description avaiable"]
    pub mod SDXC1_GPR_TUNING_CARD_CLK_SEL {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "card clock inverter enable"]
    pub mod SDXC1_CARDCLK_INV_EN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "wakeup irq enable"]
    pub mod SDXC1_WKP_IRQ_EN {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "system irq enable"]
    pub mod SDXC1_SYS_IRQ_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
