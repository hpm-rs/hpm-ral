#[doc = "PLLCTL"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Crystal control and status"]
    pub XTAL: crate::RWRegister<u32>,
    _reserved0: [u8; 0x7c],
    #[doc = "PLLx config0"]
    pub PLL_PLL0_CFG0: crate::RWRegister<u32>,
    #[doc = "PLLx config1"]
    pub PLL_PLL0_CFG1: crate::RWRegister<u32>,
    #[doc = "PLLx config2"]
    pub PLL_PLL0_CFG2: crate::RWRegister<u32>,
    #[doc = "PLLx frac mode frequency adjust"]
    pub PLL_PLL0_FREQ: crate::RWRegister<u32>,
    #[doc = "PLLx lock control"]
    pub PLL_PLL0_LOCK: crate::RWRegister<u32>,
    _reserved1: [u8; 0x0c],
    #[doc = "PLLx status"]
    pub PLL_PLL0_STATUS: crate::RWRegister<u32>,
    _reserved2: [u8; 0x1c],
    #[doc = "PLLx divider0 control"]
    pub PLL_PLL0_DIV0: crate::RWRegister<u32>,
    #[doc = "PLLx divider1 control"]
    pub PLL_PLL0_DIV1: crate::RWRegister<u32>,
    _reserved3: [u8; 0x38],
    #[doc = "PLLx config0"]
    pub PLL_PLL1_CFG0: crate::RWRegister<u32>,
    #[doc = "PLLx config1"]
    pub PLL_PLL1_CFG1: crate::RWRegister<u32>,
    #[doc = "PLLx config2"]
    pub PLL_PLL1_CFG2: crate::RWRegister<u32>,
    #[doc = "PLLx frac mode frequency adjust"]
    pub PLL_PLL1_FREQ: crate::RWRegister<u32>,
    #[doc = "PLLx lock control"]
    pub PLL_PLL1_LOCK: crate::RWRegister<u32>,
    _reserved4: [u8; 0x0c],
    #[doc = "PLLx status"]
    pub PLL_PLL1_STATUS: crate::RWRegister<u32>,
    _reserved5: [u8; 0x1c],
    #[doc = "PLLx divider0 control"]
    pub PLL_PLL1_DIV0: crate::RWRegister<u32>,
    #[doc = "PLLx divider1 control"]
    pub PLL_PLL1_DIV1: crate::RWRegister<u32>,
    _reserved6: [u8; 0x38],
    #[doc = "PLLx config0"]
    pub PLL_PLL2_CFG0: crate::RWRegister<u32>,
    #[doc = "PLLx config1"]
    pub PLL_PLL2_CFG1: crate::RWRegister<u32>,
    #[doc = "PLLx config2"]
    pub PLL_PLL2_CFG2: crate::RWRegister<u32>,
    #[doc = "PLLx frac mode frequency adjust"]
    pub PLL_PLL2_FREQ: crate::RWRegister<u32>,
    #[doc = "PLLx lock control"]
    pub PLL_PLL2_LOCK: crate::RWRegister<u32>,
    _reserved7: [u8; 0x0c],
    #[doc = "PLLx status"]
    pub PLL_PLL2_STATUS: crate::RWRegister<u32>,
    _reserved8: [u8; 0x1c],
    #[doc = "PLLx divider0 control"]
    pub PLL_PLL2_DIV0: crate::RWRegister<u32>,
    #[doc = "PLLx divider1 control"]
    pub PLL_PLL2_DIV1: crate::RWRegister<u32>,
    _reserved9: [u8; 0x38],
    #[doc = "PLLx config0"]
    pub PLL_PLL3_CFG0: crate::RWRegister<u32>,
    #[doc = "PLLx config1"]
    pub PLL_PLL3_CFG1: crate::RWRegister<u32>,
    #[doc = "PLLx config2"]
    pub PLL_PLL3_CFG2: crate::RWRegister<u32>,
    #[doc = "PLLx frac mode frequency adjust"]
    pub PLL_PLL3_FREQ: crate::RWRegister<u32>,
    #[doc = "PLLx lock control"]
    pub PLL_PLL3_LOCK: crate::RWRegister<u32>,
    _reserved10: [u8; 0x0c],
    #[doc = "PLLx status"]
    pub PLL_PLL3_STATUS: crate::RWRegister<u32>,
    _reserved11: [u8; 0x1c],
    #[doc = "PLLx divider0 control"]
    pub PLL_PLL3_DIV0: crate::RWRegister<u32>,
    #[doc = "PLLx divider1 control"]
    pub PLL_PLL3_DIV1: crate::RWRegister<u32>,
    _reserved12: [u8; 0x38],
    #[doc = "PLLx config0"]
    pub PLL_PLL4_CFG0: crate::RWRegister<u32>,
    #[doc = "PLLx config1"]
    pub PLL_PLL4_CFG1: crate::RWRegister<u32>,
    #[doc = "PLLx config2"]
    pub PLL_PLL4_CFG2: crate::RWRegister<u32>,
    #[doc = "PLLx frac mode frequency adjust"]
    pub PLL_PLL4_FREQ: crate::RWRegister<u32>,
    #[doc = "PLLx lock control"]
    pub PLL_PLL4_LOCK: crate::RWRegister<u32>,
    _reserved13: [u8; 0x0c],
    #[doc = "PLLx status"]
    pub PLL_PLL4_STATUS: crate::RWRegister<u32>,
    _reserved14: [u8; 0x1c],
    #[doc = "PLLx divider0 control"]
    pub PLL_PLL4_DIV0: crate::RWRegister<u32>,
    #[doc = "PLLx divider1 control"]
    pub PLL_PLL4_DIV1: crate::RWRegister<u32>,
}
#[doc = "Crystal control and status"]
pub mod XTAL {
    #[doc = "Rampup time of XTAL oscillator in cycles of IRC24M clock 0: 0 cycle 1: 1 cycle 2: 2 cycle 1048575: 1048575 cycles"]
    pub mod RAMP_TIME {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Crystal oscillator enable status 0: Oscillator is off 1: Oscillator is on"]
    pub mod ENABLE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Crystal oscillator status 0: Oscillator is not stable 1: Oscillator is stable for use"]
    pub mod RESPONSE {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PLLx config0"]
pub mod PLL_PLL0_CFG0 {
    #[doc = "1: int mode; 0: frac mode"]
    pub mod DSMPD {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "No description avaiable"]
    pub mod SS_DISABLE_SSCG {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "No description avaiable"]
    pub mod SS_RESET {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Downspread control 1’b0 –> Center-Spread 1’b1 –> Downspread"]
    pub mod SS_DOWNSPREAD {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "sscg divval, lock when lock_en\\[8\\]&~pll_ana_pd"]
    pub mod SS_DIVVAL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "lock when lock_en\\[14\\]&~pll_ana_pd"]
    pub mod SS_SPREAD {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "lock when lock_en\\[20\\]&~pll_ana_pd"]
    pub mod POSTDIV1 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "refclk diverder, lock when lock_en\\[24\\]&~pll_ana_pd"]
    pub mod REFDIV {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reset pointer, for sscg, lock when lock_en\\[31\\]&~pll_ana_pd&~pll_lock_comb"]
    pub mod SS_RSTPTR {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PLLx config1"]
pub mod PLL_PLL0_CFG1 {
    #[doc = "used to wait lock if set larger than lock time; default 1500 24M cycle if refdiv is 1, 4500 cycle if refdiv is 3"]
    pub mod LOCK_CNT_CFG {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "pll power down. pll_ana_pd = pllctrl_hw_en ? (pll_pd_soc|pll_pd_chg) : pllpd_sw; pll_pd_soc is just delay of soc enable, for soc to control pll on/off; pll_pd_chg is used to power down pll when div_chg_mode is 1, if software update pll parameter(fbdiv or frac), pll_ctrl will power down pll, update parameter, then power up pll. response to soc will not de-asserted at this sequence"]
    pub mod PLLPD_SW {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "the clock enable used to gate pll output, should be set after lock, and clear before power down pll. pll_clock_enable = pllctrl_hw_en ? (pll_lock_comb & enable & pll_clk_enable_chg) : clken_sw;"]
    pub mod CLKEN_SW {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: hardware controll PLL settings, software can update register, but result unknown; suggested only update fbdiv and frac value 0: full software control PLL settings"]
    pub mod PLLCTRL_HW_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PLLx config2"]
pub mod PLL_PLL0_CFG2 {
    #[doc = "fbdiv used in int mode"]
    pub mod FBDIV_INT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PLLx frac mode frequency adjust"]
pub mod PLL_PLL0_FREQ {
    #[doc = "fbdiv used in frac mode"]
    pub mod FBDIV_FRAC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PLL output frequency is : Fout = Fref/refdiv*(fbdiv + frac/2^24)/postdiv1 for default refdiv=1 and postdiv1=1, 24MHz refclk Fout is 24*fbdiv in int mode if frac is set to 0x800000, Fout is 24*(fbdiv+0.5) Fout is 24*fbdiv in int mode if frac is set to 0x200000, Fout is 24*(fbdiv+0.125)"]
    pub mod FRAC {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PLLx lock control"]
pub mod PLL_PLL0_LOCK {
    #[doc = "lock bit of field ss_divval 0: field is open foe software to change 1: field is locked, not changeable"]
    pub mod LOCK_SS_DIVVAL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "lock bit of field ss_spead 0: field is open foe software to change 1: field is locked, not changeable"]
    pub mod LOCK_SS_SPEAD {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "lock bit of field postdiv1 0: field is open foe software to change 1: field is locked, not changeable"]
    pub mod LOCK_POSTDIV1 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "lock bit of field refdiv 0: field is open foe software to change 1: field is locked, not changeable"]
    pub mod LOCK_REFDIV {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "lock bit of field ss_rstptr 0: field is open foe software to change 1: field is locked, not changeable"]
    pub mod LOCK_SS_RSTPTR {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PLLx status"]
pub mod PLL_PLL0_STATUS {
    #[doc = "No description avaiable"]
    pub mod PLL_LOCK_SYNC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "No description avaiable"]
    pub mod PLL_LOCK_COMB {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "response to SYSCTL, PLL is power down when both enable and response are 0."]
    pub mod RESPONSE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "enable from SYSCTL block"]
    pub mod ENABLE {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PLLx divider0 control"]
pub mod PLL_PLL0_DIV0 {
    #[doc = "Divider 0: divide by 1 1: divide by2 . . . 255: divide by 256"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Crystal oscillator enable status 0: Oscillator is off 1: Oscillator is on"]
    pub mod ENABLE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Crystal oscillator status 0: Oscillator is not stable 1: Oscillator is stable for use"]
    pub mod RESPONSE {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Busy flag 0: divider is working 1: divider is changing status"]
    pub mod BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PLLx divider1 control"]
pub mod PLL_PLL0_DIV1 {
    #[doc = "Divider 0: divide by 1 1: divide by2 . . . 255: divide by 256"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Crystal oscillator enable status 0: Oscillator is off 1: Oscillator is on"]
    pub mod ENABLE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Crystal oscillator status 0: Oscillator is not stable 1: Oscillator is stable for use"]
    pub mod RESPONSE {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Busy flag 0: divider is working 1: divider is changing status"]
    pub mod BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PLLx config0"]
pub mod PLL_PLL1_CFG0 {
    #[doc = "1: int mode; 0: frac mode"]
    pub mod DSMPD {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "No description avaiable"]
    pub mod SS_DISABLE_SSCG {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "No description avaiable"]
    pub mod SS_RESET {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Downspread control 1’b0 –> Center-Spread 1’b1 –> Downspread"]
    pub mod SS_DOWNSPREAD {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "sscg divval, lock when lock_en\\[8\\]&~pll_ana_pd"]
    pub mod SS_DIVVAL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "lock when lock_en\\[14\\]&~pll_ana_pd"]
    pub mod SS_SPREAD {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "lock when lock_en\\[20\\]&~pll_ana_pd"]
    pub mod POSTDIV1 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "refclk diverder, lock when lock_en\\[24\\]&~pll_ana_pd"]
    pub mod REFDIV {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reset pointer, for sscg, lock when lock_en\\[31\\]&~pll_ana_pd&~pll_lock_comb"]
    pub mod SS_RSTPTR {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PLLx config1"]
pub mod PLL_PLL1_CFG1 {
    #[doc = "used to wait lock if set larger than lock time; default 1500 24M cycle if refdiv is 1, 4500 cycle if refdiv is 3"]
    pub mod LOCK_CNT_CFG {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "pll power down. pll_ana_pd = pllctrl_hw_en ? (pll_pd_soc|pll_pd_chg) : pllpd_sw; pll_pd_soc is just delay of soc enable, for soc to control pll on/off; pll_pd_chg is used to power down pll when div_chg_mode is 1, if software update pll parameter(fbdiv or frac), pll_ctrl will power down pll, update parameter, then power up pll. response to soc will not de-asserted at this sequence"]
    pub mod PLLPD_SW {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "the clock enable used to gate pll output, should be set after lock, and clear before power down pll. pll_clock_enable = pllctrl_hw_en ? (pll_lock_comb & enable & pll_clk_enable_chg) : clken_sw;"]
    pub mod CLKEN_SW {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: hardware controll PLL settings, software can update register, but result unknown; suggested only update fbdiv and frac value 0: full software control PLL settings"]
    pub mod PLLCTRL_HW_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PLLx config2"]
pub mod PLL_PLL1_CFG2 {
    #[doc = "fbdiv used in int mode"]
    pub mod FBDIV_INT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PLLx frac mode frequency adjust"]
pub mod PLL_PLL1_FREQ {
    #[doc = "fbdiv used in frac mode"]
    pub mod FBDIV_FRAC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PLL output frequency is : Fout = Fref/refdiv*(fbdiv + frac/2^24)/postdiv1 for default refdiv=1 and postdiv1=1, 24MHz refclk Fout is 24*fbdiv in int mode if frac is set to 0x800000, Fout is 24*(fbdiv+0.5) Fout is 24*fbdiv in int mode if frac is set to 0x200000, Fout is 24*(fbdiv+0.125)"]
    pub mod FRAC {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PLLx lock control"]
pub mod PLL_PLL1_LOCK {
    #[doc = "lock bit of field ss_divval 0: field is open foe software to change 1: field is locked, not changeable"]
    pub mod LOCK_SS_DIVVAL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "lock bit of field ss_spead 0: field is open foe software to change 1: field is locked, not changeable"]
    pub mod LOCK_SS_SPEAD {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "lock bit of field postdiv1 0: field is open foe software to change 1: field is locked, not changeable"]
    pub mod LOCK_POSTDIV1 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "lock bit of field refdiv 0: field is open foe software to change 1: field is locked, not changeable"]
    pub mod LOCK_REFDIV {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "lock bit of field ss_rstptr 0: field is open foe software to change 1: field is locked, not changeable"]
    pub mod LOCK_SS_RSTPTR {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PLLx status"]
pub mod PLL_PLL1_STATUS {
    #[doc = "No description avaiable"]
    pub mod PLL_LOCK_SYNC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "No description avaiable"]
    pub mod PLL_LOCK_COMB {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "response to SYSCTL, PLL is power down when both enable and response are 0."]
    pub mod RESPONSE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "enable from SYSCTL block"]
    pub mod ENABLE {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PLLx divider0 control"]
pub mod PLL_PLL1_DIV0 {
    #[doc = "Divider 0: divide by 1 1: divide by2 . . . 255: divide by 256"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Crystal oscillator enable status 0: Oscillator is off 1: Oscillator is on"]
    pub mod ENABLE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Crystal oscillator status 0: Oscillator is not stable 1: Oscillator is stable for use"]
    pub mod RESPONSE {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Busy flag 0: divider is working 1: divider is changing status"]
    pub mod BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PLLx divider1 control"]
pub mod PLL_PLL1_DIV1 {
    #[doc = "Divider 0: divide by 1 1: divide by2 . . . 255: divide by 256"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Crystal oscillator enable status 0: Oscillator is off 1: Oscillator is on"]
    pub mod ENABLE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Crystal oscillator status 0: Oscillator is not stable 1: Oscillator is stable for use"]
    pub mod RESPONSE {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Busy flag 0: divider is working 1: divider is changing status"]
    pub mod BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PLLx config0"]
pub mod PLL_PLL2_CFG0 {
    #[doc = "1: int mode; 0: frac mode"]
    pub mod DSMPD {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "No description avaiable"]
    pub mod SS_DISABLE_SSCG {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "No description avaiable"]
    pub mod SS_RESET {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Downspread control 1’b0 –> Center-Spread 1’b1 –> Downspread"]
    pub mod SS_DOWNSPREAD {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "sscg divval, lock when lock_en\\[8\\]&~pll_ana_pd"]
    pub mod SS_DIVVAL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "lock when lock_en\\[14\\]&~pll_ana_pd"]
    pub mod SS_SPREAD {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "lock when lock_en\\[20\\]&~pll_ana_pd"]
    pub mod POSTDIV1 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "refclk diverder, lock when lock_en\\[24\\]&~pll_ana_pd"]
    pub mod REFDIV {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reset pointer, for sscg, lock when lock_en\\[31\\]&~pll_ana_pd&~pll_lock_comb"]
    pub mod SS_RSTPTR {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PLLx config1"]
pub mod PLL_PLL2_CFG1 {
    #[doc = "used to wait lock if set larger than lock time; default 1500 24M cycle if refdiv is 1, 4500 cycle if refdiv is 3"]
    pub mod LOCK_CNT_CFG {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "pll power down. pll_ana_pd = pllctrl_hw_en ? (pll_pd_soc|pll_pd_chg) : pllpd_sw; pll_pd_soc is just delay of soc enable, for soc to control pll on/off; pll_pd_chg is used to power down pll when div_chg_mode is 1, if software update pll parameter(fbdiv or frac), pll_ctrl will power down pll, update parameter, then power up pll. response to soc will not de-asserted at this sequence"]
    pub mod PLLPD_SW {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "the clock enable used to gate pll output, should be set after lock, and clear before power down pll. pll_clock_enable = pllctrl_hw_en ? (pll_lock_comb & enable & pll_clk_enable_chg) : clken_sw;"]
    pub mod CLKEN_SW {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: hardware controll PLL settings, software can update register, but result unknown; suggested only update fbdiv and frac value 0: full software control PLL settings"]
    pub mod PLLCTRL_HW_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PLLx config2"]
pub mod PLL_PLL2_CFG2 {
    #[doc = "fbdiv used in int mode"]
    pub mod FBDIV_INT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PLLx frac mode frequency adjust"]
pub mod PLL_PLL2_FREQ {
    #[doc = "fbdiv used in frac mode"]
    pub mod FBDIV_FRAC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PLL output frequency is : Fout = Fref/refdiv*(fbdiv + frac/2^24)/postdiv1 for default refdiv=1 and postdiv1=1, 24MHz refclk Fout is 24*fbdiv in int mode if frac is set to 0x800000, Fout is 24*(fbdiv+0.5) Fout is 24*fbdiv in int mode if frac is set to 0x200000, Fout is 24*(fbdiv+0.125)"]
    pub mod FRAC {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PLLx lock control"]
pub mod PLL_PLL2_LOCK {
    #[doc = "lock bit of field ss_divval 0: field is open foe software to change 1: field is locked, not changeable"]
    pub mod LOCK_SS_DIVVAL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "lock bit of field ss_spead 0: field is open foe software to change 1: field is locked, not changeable"]
    pub mod LOCK_SS_SPEAD {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "lock bit of field postdiv1 0: field is open foe software to change 1: field is locked, not changeable"]
    pub mod LOCK_POSTDIV1 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "lock bit of field refdiv 0: field is open foe software to change 1: field is locked, not changeable"]
    pub mod LOCK_REFDIV {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "lock bit of field ss_rstptr 0: field is open foe software to change 1: field is locked, not changeable"]
    pub mod LOCK_SS_RSTPTR {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PLLx status"]
pub mod PLL_PLL2_STATUS {
    #[doc = "No description avaiable"]
    pub mod PLL_LOCK_SYNC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "No description avaiable"]
    pub mod PLL_LOCK_COMB {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "response to SYSCTL, PLL is power down when both enable and response are 0."]
    pub mod RESPONSE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "enable from SYSCTL block"]
    pub mod ENABLE {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PLLx divider0 control"]
pub mod PLL_PLL2_DIV0 {
    #[doc = "Divider 0: divide by 1 1: divide by2 . . . 255: divide by 256"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Crystal oscillator enable status 0: Oscillator is off 1: Oscillator is on"]
    pub mod ENABLE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Crystal oscillator status 0: Oscillator is not stable 1: Oscillator is stable for use"]
    pub mod RESPONSE {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Busy flag 0: divider is working 1: divider is changing status"]
    pub mod BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PLLx divider1 control"]
pub mod PLL_PLL2_DIV1 {
    #[doc = "Divider 0: divide by 1 1: divide by2 . . . 255: divide by 256"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Crystal oscillator enable status 0: Oscillator is off 1: Oscillator is on"]
    pub mod ENABLE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Crystal oscillator status 0: Oscillator is not stable 1: Oscillator is stable for use"]
    pub mod RESPONSE {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Busy flag 0: divider is working 1: divider is changing status"]
    pub mod BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PLLx config0"]
pub mod PLL_PLL3_CFG0 {
    #[doc = "1: int mode; 0: frac mode"]
    pub mod DSMPD {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "No description avaiable"]
    pub mod SS_DISABLE_SSCG {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "No description avaiable"]
    pub mod SS_RESET {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Downspread control 1’b0 –> Center-Spread 1’b1 –> Downspread"]
    pub mod SS_DOWNSPREAD {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "sscg divval, lock when lock_en\\[8\\]&~pll_ana_pd"]
    pub mod SS_DIVVAL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "lock when lock_en\\[14\\]&~pll_ana_pd"]
    pub mod SS_SPREAD {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "lock when lock_en\\[20\\]&~pll_ana_pd"]
    pub mod POSTDIV1 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "refclk diverder, lock when lock_en\\[24\\]&~pll_ana_pd"]
    pub mod REFDIV {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reset pointer, for sscg, lock when lock_en\\[31\\]&~pll_ana_pd&~pll_lock_comb"]
    pub mod SS_RSTPTR {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PLLx config1"]
pub mod PLL_PLL3_CFG1 {
    #[doc = "used to wait lock if set larger than lock time; default 1500 24M cycle if refdiv is 1, 4500 cycle if refdiv is 3"]
    pub mod LOCK_CNT_CFG {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "pll power down. pll_ana_pd = pllctrl_hw_en ? (pll_pd_soc|pll_pd_chg) : pllpd_sw; pll_pd_soc is just delay of soc enable, for soc to control pll on/off; pll_pd_chg is used to power down pll when div_chg_mode is 1, if software update pll parameter(fbdiv or frac), pll_ctrl will power down pll, update parameter, then power up pll. response to soc will not de-asserted at this sequence"]
    pub mod PLLPD_SW {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "the clock enable used to gate pll output, should be set after lock, and clear before power down pll. pll_clock_enable = pllctrl_hw_en ? (pll_lock_comb & enable & pll_clk_enable_chg) : clken_sw;"]
    pub mod CLKEN_SW {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: hardware controll PLL settings, software can update register, but result unknown; suggested only update fbdiv and frac value 0: full software control PLL settings"]
    pub mod PLLCTRL_HW_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PLLx config2"]
pub mod PLL_PLL3_CFG2 {
    #[doc = "fbdiv used in int mode"]
    pub mod FBDIV_INT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PLLx frac mode frequency adjust"]
pub mod PLL_PLL3_FREQ {
    #[doc = "fbdiv used in frac mode"]
    pub mod FBDIV_FRAC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PLL output frequency is : Fout = Fref/refdiv*(fbdiv + frac/2^24)/postdiv1 for default refdiv=1 and postdiv1=1, 24MHz refclk Fout is 24*fbdiv in int mode if frac is set to 0x800000, Fout is 24*(fbdiv+0.5) Fout is 24*fbdiv in int mode if frac is set to 0x200000, Fout is 24*(fbdiv+0.125)"]
    pub mod FRAC {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PLLx lock control"]
pub mod PLL_PLL3_LOCK {
    #[doc = "lock bit of field ss_divval 0: field is open foe software to change 1: field is locked, not changeable"]
    pub mod LOCK_SS_DIVVAL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "lock bit of field ss_spead 0: field is open foe software to change 1: field is locked, not changeable"]
    pub mod LOCK_SS_SPEAD {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "lock bit of field postdiv1 0: field is open foe software to change 1: field is locked, not changeable"]
    pub mod LOCK_POSTDIV1 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "lock bit of field refdiv 0: field is open foe software to change 1: field is locked, not changeable"]
    pub mod LOCK_REFDIV {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "lock bit of field ss_rstptr 0: field is open foe software to change 1: field is locked, not changeable"]
    pub mod LOCK_SS_RSTPTR {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PLLx status"]
pub mod PLL_PLL3_STATUS {
    #[doc = "No description avaiable"]
    pub mod PLL_LOCK_SYNC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "No description avaiable"]
    pub mod PLL_LOCK_COMB {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "response to SYSCTL, PLL is power down when both enable and response are 0."]
    pub mod RESPONSE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "enable from SYSCTL block"]
    pub mod ENABLE {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PLLx divider0 control"]
pub mod PLL_PLL3_DIV0 {
    #[doc = "Divider 0: divide by 1 1: divide by2 . . . 255: divide by 256"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Crystal oscillator enable status 0: Oscillator is off 1: Oscillator is on"]
    pub mod ENABLE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Crystal oscillator status 0: Oscillator is not stable 1: Oscillator is stable for use"]
    pub mod RESPONSE {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Busy flag 0: divider is working 1: divider is changing status"]
    pub mod BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PLLx divider1 control"]
pub mod PLL_PLL3_DIV1 {
    #[doc = "Divider 0: divide by 1 1: divide by2 . . . 255: divide by 256"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Crystal oscillator enable status 0: Oscillator is off 1: Oscillator is on"]
    pub mod ENABLE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Crystal oscillator status 0: Oscillator is not stable 1: Oscillator is stable for use"]
    pub mod RESPONSE {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Busy flag 0: divider is working 1: divider is changing status"]
    pub mod BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PLLx config0"]
pub mod PLL_PLL4_CFG0 {
    #[doc = "1: int mode; 0: frac mode"]
    pub mod DSMPD {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "No description avaiable"]
    pub mod SS_DISABLE_SSCG {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "No description avaiable"]
    pub mod SS_RESET {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Downspread control 1’b0 –> Center-Spread 1’b1 –> Downspread"]
    pub mod SS_DOWNSPREAD {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "sscg divval, lock when lock_en\\[8\\]&~pll_ana_pd"]
    pub mod SS_DIVVAL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "lock when lock_en\\[14\\]&~pll_ana_pd"]
    pub mod SS_SPREAD {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "lock when lock_en\\[20\\]&~pll_ana_pd"]
    pub mod POSTDIV1 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "refclk diverder, lock when lock_en\\[24\\]&~pll_ana_pd"]
    pub mod REFDIV {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reset pointer, for sscg, lock when lock_en\\[31\\]&~pll_ana_pd&~pll_lock_comb"]
    pub mod SS_RSTPTR {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PLLx config1"]
pub mod PLL_PLL4_CFG1 {
    #[doc = "used to wait lock if set larger than lock time; default 1500 24M cycle if refdiv is 1, 4500 cycle if refdiv is 3"]
    pub mod LOCK_CNT_CFG {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "pll power down. pll_ana_pd = pllctrl_hw_en ? (pll_pd_soc|pll_pd_chg) : pllpd_sw; pll_pd_soc is just delay of soc enable, for soc to control pll on/off; pll_pd_chg is used to power down pll when div_chg_mode is 1, if software update pll parameter(fbdiv or frac), pll_ctrl will power down pll, update parameter, then power up pll. response to soc will not de-asserted at this sequence"]
    pub mod PLLPD_SW {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "the clock enable used to gate pll output, should be set after lock, and clear before power down pll. pll_clock_enable = pllctrl_hw_en ? (pll_lock_comb & enable & pll_clk_enable_chg) : clken_sw;"]
    pub mod CLKEN_SW {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: hardware controll PLL settings, software can update register, but result unknown; suggested only update fbdiv and frac value 0: full software control PLL settings"]
    pub mod PLLCTRL_HW_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PLLx config2"]
pub mod PLL_PLL4_CFG2 {
    #[doc = "fbdiv used in int mode"]
    pub mod FBDIV_INT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PLLx frac mode frequency adjust"]
pub mod PLL_PLL4_FREQ {
    #[doc = "fbdiv used in frac mode"]
    pub mod FBDIV_FRAC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PLL output frequency is : Fout = Fref/refdiv*(fbdiv + frac/2^24)/postdiv1 for default refdiv=1 and postdiv1=1, 24MHz refclk Fout is 24*fbdiv in int mode if frac is set to 0x800000, Fout is 24*(fbdiv+0.5) Fout is 24*fbdiv in int mode if frac is set to 0x200000, Fout is 24*(fbdiv+0.125)"]
    pub mod FRAC {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PLLx lock control"]
pub mod PLL_PLL4_LOCK {
    #[doc = "lock bit of field ss_divval 0: field is open foe software to change 1: field is locked, not changeable"]
    pub mod LOCK_SS_DIVVAL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "lock bit of field ss_spead 0: field is open foe software to change 1: field is locked, not changeable"]
    pub mod LOCK_SS_SPEAD {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "lock bit of field postdiv1 0: field is open foe software to change 1: field is locked, not changeable"]
    pub mod LOCK_POSTDIV1 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "lock bit of field refdiv 0: field is open foe software to change 1: field is locked, not changeable"]
    pub mod LOCK_REFDIV {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "lock bit of field ss_rstptr 0: field is open foe software to change 1: field is locked, not changeable"]
    pub mod LOCK_SS_RSTPTR {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PLLx status"]
pub mod PLL_PLL4_STATUS {
    #[doc = "No description avaiable"]
    pub mod PLL_LOCK_SYNC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "No description avaiable"]
    pub mod PLL_LOCK_COMB {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "response to SYSCTL, PLL is power down when both enable and response are 0."]
    pub mod RESPONSE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "enable from SYSCTL block"]
    pub mod ENABLE {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PLLx divider0 control"]
pub mod PLL_PLL4_DIV0 {
    #[doc = "Divider 0: divide by 1 1: divide by2 . . . 255: divide by 256"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Crystal oscillator enable status 0: Oscillator is off 1: Oscillator is on"]
    pub mod ENABLE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Crystal oscillator status 0: Oscillator is not stable 1: Oscillator is stable for use"]
    pub mod RESPONSE {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Busy flag 0: divider is working 1: divider is changing status"]
    pub mod BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PLLx divider1 control"]
pub mod PLL_PLL4_DIV1 {
    #[doc = "Divider 0: divide by 1 1: divide by2 . . . 255: divide by 256"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Crystal oscillator enable status 0: Oscillator is off 1: Oscillator is on"]
    pub mod ENABLE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Crystal oscillator status 0: Oscillator is not stable 1: Oscillator is stable for use"]
    pub mod RESPONSE {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Busy flag 0: divider is working 1: divider is changing status"]
    pub mod BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
