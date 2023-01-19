#[doc = "SYSCTL"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Resource control register for cpu0"]
    pub RESOURCE_CPU0_CORE: crate::RWRegister<u32>,
    #[doc = "Resource control register for cpx0"]
    pub RESOURCE_CPU0_SUBSYS: crate::RWRegister<u32>,
    _reserved0: [u8; 0x18],
    #[doc = "Resource control register for cpu1"]
    pub RESOURCE_CPU1_CORE: crate::RWRegister<u32>,
    #[doc = "Resource control register for cpx1"]
    pub RESOURCE_CPX1_SUBSYS: crate::RWRegister<u32>,
    _reserved1: [u8; 0x2c],
    #[doc = "Resource control register for pow_con"]
    pub RESOURCE_POW_CON: crate::RWRegister<u32>,
    #[doc = "Resource control register for pow_vis"]
    pub RESOURCE_POW_VIS: crate::RWRegister<u32>,
    #[doc = "Resource control register for pow_cpu0"]
    pub RESOURCE_POW_CPU0: crate::RWRegister<u32>,
    #[doc = "Resource control register for pow_cpu1"]
    pub RESOURCE_POW_CPU1: crate::RWRegister<u32>,
    #[doc = "Resource control register for rst_soc"]
    pub RESOURCE_RST_SOC: crate::RWRegister<u32>,
    #[doc = "Resource control register for rst_con"]
    pub RESOURCE_RST_CON: crate::RWRegister<u32>,
    #[doc = "Resource control register for rst_vis"]
    pub RESOURCE_RST_VIS: crate::RWRegister<u32>,
    #[doc = "Resource control register for rst_cpu0"]
    pub RESOURCE_RST_CPU0: crate::RWRegister<u32>,
    #[doc = "Resource control register for rst_cpu1"]
    pub RESOURCE_RST_CPU1: crate::RWRegister<u32>,
    _reserved2: [u8; 0x08],
    #[doc = "Resource control register for xtal"]
    pub RESOURCE_CLK_SRC_XTAL: crate::RWRegister<u32>,
    #[doc = "Resource control register for pll0"]
    pub RESOURCE_CLK_SRC_PLL0: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk0_pll0"]
    pub RESOURCE_CLK_SRC_PLL0CLK0: crate::RWRegister<u32>,
    #[doc = "Resource control register for pll1"]
    pub RESOURCE_CLK_SRC_PLL1: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk0_pll1"]
    pub RESOURCE_CLK_SRC_PLL1CLK0: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk1_pll1"]
    pub RESOURCE_CLK_SRC_PLL1CLK1: crate::RWRegister<u32>,
    #[doc = "Resource control register for pll2"]
    pub RESOURCE_CLK_SRC_PLL2: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk0_pll2"]
    pub RESOURCE_CLK_SRC_PLL2CLK0: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk1_pll2"]
    pub RESOURCE_CLK_SRC_PLL2CLK1: crate::RWRegister<u32>,
    #[doc = "Resource control register for pll3"]
    pub RESOURCE_CLK_SRC_PLL3: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk0_pll3"]
    pub RESOURCE_CLK_SRC_PLL3CLK0: crate::RWRegister<u32>,
    #[doc = "Resource control register for pll4"]
    pub RESOURCE_CLK_SRC_PLL4: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk0_pll4"]
    pub RESOURCE_CLK_SRC_PLL4CLK0: crate::RWRegister<u32>,
    _reserved3: [u8; 0x4c],
    #[doc = "Resource control register for clk_top_cpu0"]
    pub RESOURCE_CLK_TOP_CPU0: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk_top_mct0"]
    pub RESOURCE_CLK_TOP_MCHTMR0: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk_top_cpu1"]
    pub RESOURCE_CLK_TOP_CPU1: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk_top_mct1"]
    pub RESOURCE_CLK_TOP_MCHTMR1: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk_top_axi0"]
    pub RESOURCE_CLK_TOP_AXI: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk_top_axi1"]
    pub RESOURCE_CLK_TOP_CONN: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk_top_axi2"]
    pub RESOURCE_CLK_TOP_VIS: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk_top_ahb0"]
    pub RESOURCE_CLK_TOP_AHB: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk_top_dram"]
    pub RESOURCE_CLK_TOP_DRAM: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk_top_xpi0"]
    pub RESOURCE_CLK_TOP_XPI0: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk_top_xpi1"]
    pub RESOURCE_CLK_TOP_XPI1: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk_top_tmr0"]
    pub RESOURCE_CLK_TOP_GPTMR0: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk_top_tmr1"]
    pub RESOURCE_CLK_TOP_GPTMR1: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk_top_tmr2"]
    pub RESOURCE_CLK_TOP_GPTMR2: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk_top_tmr3"]
    pub RESOURCE_CLK_TOP_GPTMR3: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk_top_tmr4"]
    pub RESOURCE_CLK_TOP_GPTMR4: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk_top_tmr5"]
    pub RESOURCE_CLK_TOP_GPTMR5: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk_top_tmr6"]
    pub RESOURCE_CLK_TOP_GPTMR6: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk_top_tmr7"]
    pub RESOURCE_CLK_TOP_GPTMR7: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk_top_urt0"]
    pub RESOURCE_CLK_TOP_UART0: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk_top_urt1"]
    pub RESOURCE_CLK_TOP_UART1: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk_top_urt2"]
    pub RESOURCE_CLK_TOP_UART2: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk_top_urt3"]
    pub RESOURCE_CLK_TOP_UART3: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk_top_urt4"]
    pub RESOURCE_CLK_TOP_UART4: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk_top_urt5"]
    pub RESOURCE_CLK_TOP_UART5: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk_top_urt6"]
    pub RESOURCE_CLK_TOP_UART6: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk_top_urt7"]
    pub RESOURCE_CLK_TOP_UART7: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk_top_urt8"]
    pub RESOURCE_CLK_TOP_UART8: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk_top_urt9"]
    pub RESOURCE_CLK_TOP_UART9: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk_top_urta"]
    pub RESOURCE_CLK_TOP_UART10: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk_top_urtb"]
    pub RESOURCE_CLK_TOP_UART11: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk_top_urtc"]
    pub RESOURCE_CLK_TOP_UART12: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk_top_urtd"]
    pub RESOURCE_CLK_TOP_UART13: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk_top_urte"]
    pub RESOURCE_CLK_TOP_UART14: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk_top_urtf"]
    pub RESOURCE_CLK_TOP_UART15: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk_top_i2c0"]
    pub RESOURCE_CLK_TOP_I2C0: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk_top_i2c1"]
    pub RESOURCE_CLK_TOP_I2C1: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk_top_i2c2"]
    pub RESOURCE_CLK_TOP_I2C2: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk_top_i2c3"]
    pub RESOURCE_CLK_TOP_I2C3: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk_top_spi0"]
    pub RESOURCE_CLK_TOP_SPI0: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk_top_spi1"]
    pub RESOURCE_CLK_TOP_SPI1: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk_top_spi2"]
    pub RESOURCE_CLK_TOP_SPI2: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk_top_spi3"]
    pub RESOURCE_CLK_TOP_SPI3: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk_top_can0"]
    pub RESOURCE_CLK_TOP_CAN0: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk_top_can1"]
    pub RESOURCE_CLK_TOP_CAN1: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk_top_can2"]
    pub RESOURCE_CLK_TOP_CAN2: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk_top_can3"]
    pub RESOURCE_CLK_TOP_CAN3: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk_top_ptpc"]
    pub RESOURCE_CLK_TOP_PTPC: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk_top_ana0"]
    pub RESOURCE_CLK_TOP_ANA0: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk_top_ana1"]
    pub RESOURCE_CLK_TOP_ANA1: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk_top_ana2"]
    pub RESOURCE_CLK_TOP_ANA2: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk_top_aud0"]
    pub RESOURCE_CLK_TOP_AUD0: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk_top_aud1"]
    pub RESOURCE_CLK_TOP_AUD1: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk_top_aud2"]
    pub RESOURCE_CLK_TOP_AUD2: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk_top_dis0"]
    pub RESOURCE_CLK_TOP_LCDC: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk_top_cam0"]
    pub RESOURCE_CLK_TOP_CAM0: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk_top_cam1"]
    pub RESOURCE_CLK_TOP_CAM1: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk_top_eth0"]
    pub RESOURCE_CLK_TOP_ENET0: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk_top_eth1"]
    pub RESOURCE_CLK_TOP_ENET1: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk_top_ptp0"]
    pub RESOURCE_CLK_TOP_PTP0: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk_top_ptp1"]
    pub RESOURCE_CLK_TOP_PTP1: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk_top_ref0"]
    pub RESOURCE_CLK_TOP_REF0: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk_top_ref1"]
    pub RESOURCE_CLK_TOP_REF1: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk_top_ntm0"]
    pub RESOURCE_CLK_TOP_NTMR0: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk_top_ntm1"]
    pub RESOURCE_CLK_TOP_NTMR1: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk_top_sdc0"]
    pub RESOURCE_CLK_TOP_SDXC0: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk_top_sdc1"]
    pub RESOURCE_CLK_TOP_SDXC1: crate::RWRegister<u32>,
    _reserved4: [u8; 0xf4],
    #[doc = "Resource control register for clk_top_adc0"]
    pub RESOURCE_CLK_TOP_ADC0: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk_top_adc1"]
    pub RESOURCE_CLK_TOP_ADC1: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk_top_adc2"]
    pub RESOURCE_CLK_TOP_ADC2: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk_top_adc3"]
    pub RESOURCE_CLK_TOP_ADC3: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk_top_i2s0"]
    pub RESOURCE_CLK_TOP_I2S0: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk_top_i2s1"]
    pub RESOURCE_CLK_TOP_I2S1: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk_top_i2s2"]
    pub RESOURCE_CLK_TOP_I2S2: crate::RWRegister<u32>,
    #[doc = "Resource control register for clk_top_i2s3"]
    pub RESOURCE_CLK_TOP_I2S3: crate::RWRegister<u32>,
    _reserved5: [u8; 0xe0],
    #[doc = "Resource control register for ahbp"]
    pub RESOURCE_AHBAPB_BUS: crate::RWRegister<u32>,
    #[doc = "Resource control register for axis"]
    pub RESOURCE_AXI_BUS: crate::RWRegister<u32>,
    #[doc = "Resource control register for axic"]
    pub RESOURCE_CONN_BUS: crate::RWRegister<u32>,
    #[doc = "Resource control register for axiv"]
    pub RESOURCE_VIS_BUS: crate::RWRegister<u32>,
    #[doc = "Resource control register for dram"]
    pub RESOURCE_DRAM: crate::RWRegister<u32>,
    #[doc = "Resource control register for rom0"]
    pub RESOURCE_ROM: crate::RWRegister<u32>,
    #[doc = "Resource control register for lmm0"]
    pub RESOURCE_LMM0: crate::RWRegister<u32>,
    #[doc = "Resource control register for lmm1"]
    pub RESOURCE_LMM1: crate::RWRegister<u32>,
    #[doc = "Resource control register for mct0"]
    pub RESOURCE_MCHTMR0: crate::RWRegister<u32>,
    #[doc = "Resource control register for mct1"]
    pub RESOURCE_MCHTMR1: crate::RWRegister<u32>,
    #[doc = "Resource control register for ram0"]
    pub RESOURCE_AXI_SRAM0: crate::RWRegister<u32>,
    #[doc = "Resource control register for ram1"]
    pub RESOURCE_AXI_SRAM1: crate::RWRegister<u32>,
    #[doc = "Resource control register for xpi0"]
    pub RESOURCE_XPI0: crate::RWRegister<u32>,
    #[doc = "Resource control register for xpi1"]
    pub RESOURCE_XPI1: crate::RWRegister<u32>,
    #[doc = "Resource control register for sdp0"]
    pub RESOURCE_SDP: crate::RWRegister<u32>,
    #[doc = "Resource control register for rng0"]
    pub RESOURCE_RNG: crate::RWRegister<u32>,
    #[doc = "Resource control register for kman"]
    pub RESOURCE_KEYM: crate::RWRegister<u32>,
    #[doc = "Resource control register for dma0"]
    pub RESOURCE_HDMA: crate::RWRegister<u32>,
    #[doc = "Resource control register for dma1"]
    pub RESOURCE_XDMA: crate::RWRegister<u32>,
    #[doc = "Resource control register for gpio"]
    pub RESOURCE_GPIO: crate::RWRegister<u32>,
    #[doc = "Resource control register for mbx0"]
    pub RESOURCE_MBX0: crate::RWRegister<u32>,
    #[doc = "Resource control register for mbx1"]
    pub RESOURCE_MBX1: crate::RWRegister<u32>,
    #[doc = "Resource control register for wdg0"]
    pub RESOURCE_WDG0: crate::RWRegister<u32>,
    #[doc = "Resource control register for wdg1"]
    pub RESOURCE_WDG1: crate::RWRegister<u32>,
    #[doc = "Resource control register for wdg2"]
    pub RESOURCE_WDG2: crate::RWRegister<u32>,
    #[doc = "Resource control register for wdg3"]
    pub RESOURCE_WDG3: crate::RWRegister<u32>,
    #[doc = "Resource control register for tmr0"]
    pub RESOURCE_GPTMR0: crate::RWRegister<u32>,
    #[doc = "Resource control register for tmr1"]
    pub RESOURCE_GPTMR1: crate::RWRegister<u32>,
    #[doc = "Resource control register for tmr2"]
    pub RESOURCE_GPTMR2: crate::RWRegister<u32>,
    #[doc = "Resource control register for tmr3"]
    pub RESOURCE_GPTMR3: crate::RWRegister<u32>,
    #[doc = "Resource control register for tmr4"]
    pub RESOURCE_GPTMR4: crate::RWRegister<u32>,
    #[doc = "Resource control register for tmr5"]
    pub RESOURCE_GPTMR5: crate::RWRegister<u32>,
    #[doc = "Resource control register for tmr6"]
    pub RESOURCE_GPTMR6: crate::RWRegister<u32>,
    #[doc = "Resource control register for tmr7"]
    pub RESOURCE_GPTMR7: crate::RWRegister<u32>,
    #[doc = "Resource control register for urt0"]
    pub RESOURCE_UART0: crate::RWRegister<u32>,
    #[doc = "Resource control register for urt1"]
    pub RESOURCE_UART1: crate::RWRegister<u32>,
    #[doc = "Resource control register for urt2"]
    pub RESOURCE_UART2: crate::RWRegister<u32>,
    #[doc = "Resource control register for urt3"]
    pub RESOURCE_UART3: crate::RWRegister<u32>,
    #[doc = "Resource control register for urt4"]
    pub RESOURCE_UART4: crate::RWRegister<u32>,
    #[doc = "Resource control register for urt5"]
    pub RESOURCE_UART5: crate::RWRegister<u32>,
    #[doc = "Resource control register for urt6"]
    pub RESOURCE_UART6: crate::RWRegister<u32>,
    #[doc = "Resource control register for urt7"]
    pub RESOURCE_UART7: crate::RWRegister<u32>,
    #[doc = "Resource control register for urt8"]
    pub RESOURCE_UART8: crate::RWRegister<u32>,
    #[doc = "Resource control register for urt9"]
    pub RESOURCE_UART9: crate::RWRegister<u32>,
    #[doc = "Resource control register for urta"]
    pub RESOURCE_UART10: crate::RWRegister<u32>,
    #[doc = "Resource control register for urtb"]
    pub RESOURCE_UART11: crate::RWRegister<u32>,
    #[doc = "Resource control register for urtc"]
    pub RESOURCE_UART12: crate::RWRegister<u32>,
    #[doc = "Resource control register for urtd"]
    pub RESOURCE_UART13: crate::RWRegister<u32>,
    #[doc = "Resource control register for urte"]
    pub RESOURCE_UART14: crate::RWRegister<u32>,
    #[doc = "Resource control register for urtf"]
    pub RESOURCE_UART15: crate::RWRegister<u32>,
    #[doc = "Resource control register for i2c0"]
    pub RESOURCE_I2C0: crate::RWRegister<u32>,
    #[doc = "Resource control register for i2c1"]
    pub RESOURCE_I2C1: crate::RWRegister<u32>,
    #[doc = "Resource control register for i2c2"]
    pub RESOURCE_I2C2: crate::RWRegister<u32>,
    #[doc = "Resource control register for i2c3"]
    pub RESOURCE_I2C3: crate::RWRegister<u32>,
    #[doc = "Resource control register for spi0"]
    pub RESOURCE_SPI0: crate::RWRegister<u32>,
    #[doc = "Resource control register for spi1"]
    pub RESOURCE_SPI1: crate::RWRegister<u32>,
    #[doc = "Resource control register for spi2"]
    pub RESOURCE_SPI2: crate::RWRegister<u32>,
    #[doc = "Resource control register for spi3"]
    pub RESOURCE_SPI3: crate::RWRegister<u32>,
    #[doc = "Resource control register for can0"]
    pub RESOURCE_CAN0: crate::RWRegister<u32>,
    #[doc = "Resource control register for can1"]
    pub RESOURCE_CAN1: crate::RWRegister<u32>,
    #[doc = "Resource control register for can2"]
    pub RESOURCE_CAN2: crate::RWRegister<u32>,
    #[doc = "Resource control register for can3"]
    pub RESOURCE_CAN3: crate::RWRegister<u32>,
    #[doc = "Resource control register for ptpc"]
    pub RESOURCE_PTPC: crate::RWRegister<u32>,
    #[doc = "Resource control register for adc0"]
    pub RESOURCE_ADC0: crate::RWRegister<u32>,
    #[doc = "Resource control register for adc1"]
    pub RESOURCE_ADC1: crate::RWRegister<u32>,
    #[doc = "Resource control register for adc2"]
    pub RESOURCE_ADC2: crate::RWRegister<u32>,
    #[doc = "Resource control register for adc3"]
    pub RESOURCE_ADC3: crate::RWRegister<u32>,
    #[doc = "Resource control register for acmp"]
    pub RESOURCE_ACMP: crate::RWRegister<u32>,
    #[doc = "Resource control register for i2s0"]
    pub RESOURCE_I2S0: crate::RWRegister<u32>,
    #[doc = "Resource control register for i2s1"]
    pub RESOURCE_I2S1: crate::RWRegister<u32>,
    #[doc = "Resource control register for i2s2"]
    pub RESOURCE_I2S2: crate::RWRegister<u32>,
    #[doc = "Resource control register for i2s3"]
    pub RESOURCE_I2S3: crate::RWRegister<u32>,
    #[doc = "Resource control register for pdm0"]
    pub RESOURCE_PDM: crate::RWRegister<u32>,
    #[doc = "Resource control register for clsd"]
    pub RESOURCE_DAO: crate::RWRegister<u32>,
    #[doc = "Resource control register for msyn"]
    pub RESOURCE_SYNT: crate::RWRegister<u32>,
    #[doc = "Resource control register for mot0"]
    pub RESOURCE_MOT0: crate::RWRegister<u32>,
    #[doc = "Resource control register for mot1"]
    pub RESOURCE_MOT1: crate::RWRegister<u32>,
    #[doc = "Resource control register for mot2"]
    pub RESOURCE_MOT2: crate::RWRegister<u32>,
    #[doc = "Resource control register for mot3"]
    pub RESOURCE_MOT3: crate::RWRegister<u32>,
    #[doc = "Resource control register for dis0"]
    pub RESOURCE_LCDC: crate::RWRegister<u32>,
    #[doc = "Resource control register for cam0"]
    pub RESOURCE_CAM0: crate::RWRegister<u32>,
    #[doc = "Resource control register for cam1"]
    pub RESOURCE_CAM1: crate::RWRegister<u32>,
    #[doc = "Resource control register for jpeg"]
    pub RESOURCE_JPEG: crate::RWRegister<u32>,
    #[doc = "Resource control register for pdma"]
    pub RESOURCE_PDMA: crate::RWRegister<u32>,
    #[doc = "Resource control register for eth0"]
    pub RESOURCE_ENET0: crate::RWRegister<u32>,
    #[doc = "Resource control register for eth1"]
    pub RESOURCE_ENET1: crate::RWRegister<u32>,
    #[doc = "Resource control register for ntm0"]
    pub RESOURCE_NTMR0: crate::RWRegister<u32>,
    #[doc = "Resource control register for ntm1"]
    pub RESOURCE_NTMR1: crate::RWRegister<u32>,
    #[doc = "Resource control register for sdc0"]
    pub RESOURCE_SDXC0: crate::RWRegister<u32>,
    #[doc = "Resource control register for sdc1"]
    pub RESOURCE_SDXC1: crate::RWRegister<u32>,
    #[doc = "Resource control register for usb0"]
    pub RESOURCE_USB0: crate::RWRegister<u32>,
    #[doc = "Resource control register for usb1"]
    pub RESOURCE_USB1: crate::RWRegister<u32>,
    #[doc = "Resource control register for ref0"]
    pub RESOURCE_REF0: crate::RWRegister<u32>,
    #[doc = "Resource control register for ref1"]
    pub RESOURCE_REF1: crate::RWRegister<u32>,
    _reserved6: [u8; 0x0288],
    #[doc = "Goup setting"]
    pub GROUP0_0_VALUE: crate::RWRegister<u32>,
    #[doc = "Goup setting"]
    pub GROUP0_0_SET: crate::RWRegister<u32>,
    #[doc = "Goup setting"]
    pub GROUP0_0_CLEAR: crate::RWRegister<u32>,
    #[doc = "Goup setting"]
    pub GROUP0_0_TOGGLE: crate::RWRegister<u32>,
    #[doc = "Goup setting"]
    pub GROUP0_1_VALUE: crate::RWRegister<u32>,
    #[doc = "Goup setting"]
    pub GROUP0_1_SET: crate::RWRegister<u32>,
    #[doc = "Goup setting"]
    pub GROUP0_1_CLEAR: crate::RWRegister<u32>,
    #[doc = "Goup setting"]
    pub GROUP0_1_TOGGLE: crate::RWRegister<u32>,
    #[doc = "Goup setting"]
    pub GROUP0_2_VALUE: crate::RWRegister<u32>,
    #[doc = "Goup setting"]
    pub GROUP0_2_SET: crate::RWRegister<u32>,
    #[doc = "Goup setting"]
    pub GROUP0_2_CLEAR: crate::RWRegister<u32>,
    #[doc = "Goup setting"]
    pub GROUP0_2_TOGGLE: crate::RWRegister<u32>,
    _reserved7: [u8; 0x10],
    #[doc = "Goup setting"]
    pub GROUP1_0_VALUE: crate::RWRegister<u32>,
    #[doc = "Goup setting"]
    pub GROUP1_0_SET: crate::RWRegister<u32>,
    #[doc = "Goup setting"]
    pub GROUP1_0_CLEAR: crate::RWRegister<u32>,
    #[doc = "Goup setting"]
    pub GROUP1_0_TOGGLE: crate::RWRegister<u32>,
    #[doc = "Goup setting"]
    pub GROUP1_1_VALUE: crate::RWRegister<u32>,
    #[doc = "Goup setting"]
    pub GROUP1_1_SET: crate::RWRegister<u32>,
    #[doc = "Goup setting"]
    pub GROUP1_1_CLEAR: crate::RWRegister<u32>,
    #[doc = "Goup setting"]
    pub GROUP1_1_TOGGLE: crate::RWRegister<u32>,
    #[doc = "Goup setting"]
    pub GROUP1_2_VALUE: crate::RWRegister<u32>,
    #[doc = "Goup setting"]
    pub GROUP1_2_SET: crate::RWRegister<u32>,
    #[doc = "Goup setting"]
    pub GROUP1_2_CLEAR: crate::RWRegister<u32>,
    #[doc = "Goup setting"]
    pub GROUP1_2_TOGGLE: crate::RWRegister<u32>,
    _reserved8: [u8; 0x90],
    #[doc = "Affiliate of Group"]
    pub AFFILIATE_CPU0_VALUE: crate::RWRegister<u32>,
    #[doc = "Affiliate of Group"]
    pub AFFILIATE_CPU0_SET: crate::RWRegister<u32>,
    #[doc = "Affiliate of Group"]
    pub AFFILIATE_CPU0_CLEAR: crate::RWRegister<u32>,
    #[doc = "Affiliate of Group"]
    pub AFFILIATE_CPU0_TOGGLE: crate::RWRegister<u32>,
    #[doc = "Affiliate of Group"]
    pub AFFILIATE_CPU1_VALUE: crate::RWRegister<u32>,
    #[doc = "Affiliate of Group"]
    pub AFFILIATE_CPU1_SET: crate::RWRegister<u32>,
    #[doc = "Affiliate of Group"]
    pub AFFILIATE_CPU1_CLEAR: crate::RWRegister<u32>,
    #[doc = "Affiliate of Group"]
    pub AFFILIATE_CPU1_TOGGLE: crate::RWRegister<u32>,
    #[doc = "Retention Contol"]
    pub RETENTION_CPU0_VALUE: crate::RWRegister<u32>,
    #[doc = "Retention Contol"]
    pub RETENTION_CPU0_SET: crate::RWRegister<u32>,
    #[doc = "Retention Contol"]
    pub RETENTION_CPU0_CLEAR: crate::RWRegister<u32>,
    #[doc = "Retention Contol"]
    pub RETENTION_CPU0_TOGGLE: crate::RWRegister<u32>,
    #[doc = "Retention Contol"]
    pub RETENTION_CPU1_VALUE: crate::RWRegister<u32>,
    #[doc = "Retention Contol"]
    pub RETENTION_CPU1_SET: crate::RWRegister<u32>,
    #[doc = "Retention Contol"]
    pub RETENTION_CPU1_CLEAR: crate::RWRegister<u32>,
    #[doc = "Retention Contol"]
    pub RETENTION_CPU1_TOGGLE: crate::RWRegister<u32>,
    _reserved9: [u8; 0x06c0],
    #[doc = "Power Setting"]
    pub POWER_CPU0_STATUS: crate::RWRegister<u32>,
    #[doc = "Power Setting"]
    pub POWER_CPU0_LF_WAIT: crate::RWRegister<u32>,
    _reserved10: [u8; 0x04],
    #[doc = "Power Setting"]
    pub POWER_CPU0_OFF_WAIT: crate::RWRegister<u32>,
    #[doc = "Power Setting"]
    pub POWER_CPU1_STATUS: crate::RWRegister<u32>,
    #[doc = "Power Setting"]
    pub POWER_CPU1_LF_WAIT: crate::RWRegister<u32>,
    _reserved11: [u8; 0x04],
    #[doc = "Power Setting"]
    pub POWER_CPU1_OFF_WAIT: crate::RWRegister<u32>,
    #[doc = "Power Setting"]
    pub POWER_CON_STATUS: crate::RWRegister<u32>,
    #[doc = "Power Setting"]
    pub POWER_CON_LF_WAIT: crate::RWRegister<u32>,
    _reserved12: [u8; 0x04],
    #[doc = "Power Setting"]
    pub POWER_CON_OFF_WAIT: crate::RWRegister<u32>,
    #[doc = "Power Setting"]
    pub POWER_VIS_STATUS: crate::RWRegister<u32>,
    #[doc = "Power Setting"]
    pub POWER_VIS_LF_WAIT: crate::RWRegister<u32>,
    _reserved13: [u8; 0x04],
    #[doc = "Power Setting"]
    pub POWER_VIS_OFF_WAIT: crate::RWRegister<u32>,
    _reserved14: [u8; 0x03c0],
    #[doc = "Reset Setting"]
    pub RESET_SOC_CONTROL: crate::RWRegister<u32>,
    #[doc = "Reset Setting"]
    pub RESET_SOC_CONFIG: crate::RWRegister<u32>,
    _reserved15: [u8; 0x04],
    #[doc = "Reset Setting"]
    pub RESET_SOC_COUNTER: crate::RWRegister<u32>,
    #[doc = "Reset Setting"]
    pub RESET_CON_CONTROL: crate::RWRegister<u32>,
    #[doc = "Reset Setting"]
    pub RESET_CON_CONFIG: crate::RWRegister<u32>,
    _reserved16: [u8; 0x04],
    #[doc = "Reset Setting"]
    pub RESET_CON_COUNTER: crate::RWRegister<u32>,
    #[doc = "Reset Setting"]
    pub RESET_VIS_CONTROL: crate::RWRegister<u32>,
    #[doc = "Reset Setting"]
    pub RESET_VIS_CONFIG: crate::RWRegister<u32>,
    _reserved17: [u8; 0x04],
    #[doc = "Reset Setting"]
    pub RESET_VIS_COUNTER: crate::RWRegister<u32>,
    #[doc = "Reset Setting"]
    pub RESET_CPU0_CONTROL: crate::RWRegister<u32>,
    #[doc = "Reset Setting"]
    pub RESET_CPU0_CONFIG: crate::RWRegister<u32>,
    _reserved18: [u8; 0x04],
    #[doc = "Reset Setting"]
    pub RESET_CPU0_COUNTER: crate::RWRegister<u32>,
    #[doc = "Reset Setting"]
    pub RESET_CPU1_CONTROL: crate::RWRegister<u32>,
    #[doc = "Reset Setting"]
    pub RESET_CPU1_CONFIG: crate::RWRegister<u32>,
    _reserved19: [u8; 0x04],
    #[doc = "Reset Setting"]
    pub RESET_CPU1_COUNTER: crate::RWRegister<u32>,
    _reserved20: [u8; 0x03b0],
    #[doc = "Clock setting"]
    pub CLOCK_CLK_TOP_CPU0: crate::RWRegister<u32>,
    #[doc = "Clock setting"]
    pub CLOCK_CLK_TOP_MCHTMR0: crate::RWRegister<u32>,
    #[doc = "Clock setting"]
    pub CLOCK_CLK_TOP_CPU1: crate::RWRegister<u32>,
    #[doc = "Clock setting"]
    pub CLOCK_CLK_TOP_MCHTMR: crate::RWRegister<u32>,
    #[doc = "Clock setting"]
    pub CLOCK_CLK_TOP_AXI: crate::RWRegister<u32>,
    #[doc = "Clock setting"]
    pub CLOCK_CLK_TOP_CONN: crate::RWRegister<u32>,
    #[doc = "Clock setting"]
    pub CLOCK_CLK_TOP_VIS: crate::RWRegister<u32>,
    #[doc = "Clock setting"]
    pub CLOCK_CLK_TOP_AHB: crate::RWRegister<u32>,
    #[doc = "Clock setting"]
    pub CLOCK_CLK_TOP_DRAM: crate::RWRegister<u32>,
    #[doc = "Clock setting"]
    pub CLOCK_CLK_TOP_XPI0: crate::RWRegister<u32>,
    #[doc = "Clock setting"]
    pub CLOCK_CLK_TOP_XPI1: crate::RWRegister<u32>,
    #[doc = "Clock setting"]
    pub CLOCK_CLK_TOP_GPTMR0: crate::RWRegister<u32>,
    #[doc = "Clock setting"]
    pub CLOCK_CLK_TOP_GPTMR1: crate::RWRegister<u32>,
    #[doc = "Clock setting"]
    pub CLOCK_CLK_TOP_GPTMR2: crate::RWRegister<u32>,
    #[doc = "Clock setting"]
    pub CLOCK_CLK_TOP_GPTMR3: crate::RWRegister<u32>,
    #[doc = "Clock setting"]
    pub CLOCK_CLK_TOP_GPTMR4: crate::RWRegister<u32>,
    #[doc = "Clock setting"]
    pub CLOCK_CLK_TOP_GPTMR5: crate::RWRegister<u32>,
    #[doc = "Clock setting"]
    pub CLOCK_CLK_TOP_GPTMR6: crate::RWRegister<u32>,
    #[doc = "Clock setting"]
    pub CLOCK_CLK_TOP_GPTMR7: crate::RWRegister<u32>,
    #[doc = "Clock setting"]
    pub CLOCK_CLK_TOP_UART0: crate::RWRegister<u32>,
    #[doc = "Clock setting"]
    pub CLOCK_CLK_TOP_UART1: crate::RWRegister<u32>,
    #[doc = "Clock setting"]
    pub CLOCK_CLK_TOP_UART2: crate::RWRegister<u32>,
    #[doc = "Clock setting"]
    pub CLOCK_CLK_TOP_UART3: crate::RWRegister<u32>,
    #[doc = "Clock setting"]
    pub CLOCK_CLK_TOP_UART4: crate::RWRegister<u32>,
    #[doc = "Clock setting"]
    pub CLOCK_CLK_TOP_UART5: crate::RWRegister<u32>,
    #[doc = "Clock setting"]
    pub CLOCK_CLK_TOP_UART6: crate::RWRegister<u32>,
    #[doc = "Clock setting"]
    pub CLOCK_CLK_TOP_UART7: crate::RWRegister<u32>,
    #[doc = "Clock setting"]
    pub CLOCK_CLK_TOP_UART8: crate::RWRegister<u32>,
    #[doc = "Clock setting"]
    pub CLOCK_CLK_TOP_UART9: crate::RWRegister<u32>,
    #[doc = "Clock setting"]
    pub CLOCK_CLK_TOP_UART10: crate::RWRegister<u32>,
    #[doc = "Clock setting"]
    pub CLOCK_CLK_TOP_UART11: crate::RWRegister<u32>,
    #[doc = "Clock setting"]
    pub CLOCK_CLK_TOP_UART12: crate::RWRegister<u32>,
    #[doc = "Clock setting"]
    pub CLOCK_CLK_TOP_UART13: crate::RWRegister<u32>,
    #[doc = "Clock setting"]
    pub CLOCK_CLK_TOP_UART14: crate::RWRegister<u32>,
    #[doc = "Clock setting"]
    pub CLOCK_CLK_TOP_UART15: crate::RWRegister<u32>,
    #[doc = "Clock setting"]
    pub CLOCK_CLK_TOP_I2C0: crate::RWRegister<u32>,
    #[doc = "Clock setting"]
    pub CLOCK_CLK_TOP_I2C1: crate::RWRegister<u32>,
    #[doc = "Clock setting"]
    pub CLOCK_CLK_TOP_I2C2: crate::RWRegister<u32>,
    #[doc = "Clock setting"]
    pub CLOCK_CLK_TOP_I2C3: crate::RWRegister<u32>,
    #[doc = "Clock setting"]
    pub CLOCK_CLK_TOP_SPI0: crate::RWRegister<u32>,
    #[doc = "Clock setting"]
    pub CLOCK_CLK_TOP_SPI1: crate::RWRegister<u32>,
    #[doc = "Clock setting"]
    pub CLOCK_CLK_TOP_SPI2: crate::RWRegister<u32>,
    #[doc = "Clock setting"]
    pub CLOCK_CLK_TOP_SPI3: crate::RWRegister<u32>,
    #[doc = "Clock setting"]
    pub CLOCK_CLK_TOP_CAN0: crate::RWRegister<u32>,
    #[doc = "Clock setting"]
    pub CLOCK_CLK_TOP_CAN1: crate::RWRegister<u32>,
    #[doc = "Clock setting"]
    pub CLOCK_CLK_TOP_CAN2: crate::RWRegister<u32>,
    #[doc = "Clock setting"]
    pub CLOCK_CLK_TOP_CAN3: crate::RWRegister<u32>,
    #[doc = "Clock setting"]
    pub CLOCK_CLK_TOP_PTPC: crate::RWRegister<u32>,
    #[doc = "Clock setting"]
    pub CLOCK_CLK_TOP_ANA0: crate::RWRegister<u32>,
    #[doc = "Clock setting"]
    pub CLOCK_CLK_TOP_ANA1: crate::RWRegister<u32>,
    #[doc = "Clock setting"]
    pub CLOCK_CLK_TOP_ANA2: crate::RWRegister<u32>,
    #[doc = "Clock setting"]
    pub CLOCK_CLK_TOP_AUD0: crate::RWRegister<u32>,
    #[doc = "Clock setting"]
    pub CLOCK_CLK_TOP_AUD1: crate::RWRegister<u32>,
    #[doc = "Clock setting"]
    pub CLOCK_CLK_TOP_AUD2: crate::RWRegister<u32>,
    #[doc = "Clock setting"]
    pub CLOCK_CLK_TOP_LCDC: crate::RWRegister<u32>,
    #[doc = "Clock setting"]
    pub CLOCK_CLK_TOP_CAM0: crate::RWRegister<u32>,
    #[doc = "Clock setting"]
    pub CLOCK_CLK_TOP_CAM1: crate::RWRegister<u32>,
    #[doc = "Clock setting"]
    pub CLOCK_CLK_TOP_ENET0: crate::RWRegister<u32>,
    #[doc = "Clock setting"]
    pub CLOCK_CLK_TOP_ENET1: crate::RWRegister<u32>,
    #[doc = "Clock setting"]
    pub CLOCK_CLK_TOP_PTP0: crate::RWRegister<u32>,
    #[doc = "Clock setting"]
    pub CLOCK_CLK_TOP_PTP1: crate::RWRegister<u32>,
    #[doc = "Clock setting"]
    pub CLOCK_CLK_TOP_REF0: crate::RWRegister<u32>,
    #[doc = "Clock setting"]
    pub CLOCK_CLK_TOP_REF1: crate::RWRegister<u32>,
    #[doc = "Clock setting"]
    pub CLOCK_CLK_TOP_NTMR0: crate::RWRegister<u32>,
    #[doc = "Clock setting"]
    pub CLOCK_CLK_TOP_NTMR1: crate::RWRegister<u32>,
    #[doc = "Clock setting"]
    pub CLOCK_CLK_TOP_SDXC0: crate::RWRegister<u32>,
    #[doc = "Clock setting"]
    pub CLOCK_CLK_TOP_SDXC1: crate::RWRegister<u32>,
    _reserved21: [u8; 0x02f4],
    #[doc = "Clock setting"]
    pub ADCCLK_CLK_TOP_ADC0: crate::RWRegister<u32>,
    #[doc = "Clock setting"]
    pub ADCCLK_CLK_TOP_ADC1: crate::RWRegister<u32>,
    #[doc = "Clock setting"]
    pub ADCCLK_CLK_TOP_ADC2: crate::RWRegister<u32>,
    #[doc = "Clock setting"]
    pub ADCCLK_CLK_TOP_ADC3: crate::RWRegister<u32>,
    #[doc = "Clock setting"]
    pub I2SCLK_CLK_TOP_I2S0: crate::RWRegister<u32>,
    #[doc = "Clock setting"]
    pub I2SCLK_CLK_TOP_I2S1: crate::RWRegister<u32>,
    #[doc = "Clock setting"]
    pub I2SCLK_CLK_TOP_I2S2: crate::RWRegister<u32>,
    #[doc = "Clock setting"]
    pub I2SCLK_CLK_TOP_I2S3: crate::RWRegister<u32>,
    _reserved22: [u8; 0x03e0],
    #[doc = "Clock senario"]
    pub GLOBAL00: crate::RWRegister<u32>,
    _reserved23: [u8; 0x03fc],
    #[doc = "Clock measure and monitor control"]
    pub MONITOR_SLICE0_CONTROL: crate::RWRegister<u32>,
    #[doc = "Clock measure result"]
    pub MONITOR_SLICE0_CURRENT: crate::RWRegister<u32>,
    #[doc = "Clock lower limit"]
    pub MONITOR_SLICE0_LOW_LIMIT: crate::RWRegister<u32>,
    #[doc = "Clock upper limit"]
    pub MONITOR_SLICE0_HIGH_LIMIT: crate::RWRegister<u32>,
    _reserved24: [u8; 0x10],
    #[doc = "Clock measure and monitor control"]
    pub MONITOR_SLICE1_CONTROL: crate::RWRegister<u32>,
    #[doc = "Clock measure result"]
    pub MONITOR_SLICE1_CURRENT: crate::RWRegister<u32>,
    #[doc = "Clock lower limit"]
    pub MONITOR_SLICE1_LOW_LIMIT: crate::RWRegister<u32>,
    #[doc = "Clock upper limit"]
    pub MONITOR_SLICE1_HIGH_LIMIT: crate::RWRegister<u32>,
    _reserved25: [u8; 0x10],
    #[doc = "Clock measure and monitor control"]
    pub MONITOR_SLICE2_CONTROL: crate::RWRegister<u32>,
    #[doc = "Clock measure result"]
    pub MONITOR_SLICE2_CURRENT: crate::RWRegister<u32>,
    #[doc = "Clock lower limit"]
    pub MONITOR_SLICE2_LOW_LIMIT: crate::RWRegister<u32>,
    #[doc = "Clock upper limit"]
    pub MONITOR_SLICE2_HIGH_LIMIT: crate::RWRegister<u32>,
    _reserved26: [u8; 0x10],
    #[doc = "Clock measure and monitor control"]
    pub MONITOR_SLICE3_CONTROL: crate::RWRegister<u32>,
    #[doc = "Clock measure result"]
    pub MONITOR_SLICE3_CURRENT: crate::RWRegister<u32>,
    #[doc = "Clock lower limit"]
    pub MONITOR_SLICE3_LOW_LIMIT: crate::RWRegister<u32>,
    #[doc = "Clock upper limit"]
    pub MONITOR_SLICE3_HIGH_LIMIT: crate::RWRegister<u32>,
    _reserved27: [u8; 0x0390],
    #[doc = "No description avaiable"]
    pub CPU_CPU0_LP: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CPU_CPU0_LOCK: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CPU_CPU0_GPR0: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CPU_CPU0_GPR1: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CPU_CPU0_GPR2: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CPU_CPU0_GPR3: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CPU_CPU0_GPR4: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CPU_CPU0_GPR5: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CPU_CPU0_GPR6: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CPU_CPU0_GPR7: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CPU_CPU0_GPR8: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CPU_CPU0_GPR9: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CPU_CPU0_GPR10: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CPU_CPU0_GPR11: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CPU_CPU0_GPR12: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CPU_CPU0_GPR13: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CPU_CPU0_STATUS0: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CPU_CPU0_STATUS1: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CPU_CPU0_STATUS2: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CPU_CPU0_STATUS3: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CPU_CPU0_STATUS4: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CPU_CPU0_STATUS5: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CPU_CPU0_STATUS6: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CPU_CPU0_STATUS7: crate::RWRegister<u32>,
    _reserved28: [u8; 0x20],
    #[doc = "No description avaiable"]
    pub CPU_CPU0_ENABLE0: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CPU_CPU0_ENABLE1: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CPU_CPU0_ENABLE2: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CPU_CPU0_ENABLE3: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CPU_CPU0_ENABLE4: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CPU_CPU0_ENABLE5: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CPU_CPU0_ENABLE6: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CPU_CPU0_ENABLE7: crate::RWRegister<u32>,
    _reserved29: [u8; 0x0360],
    #[doc = "No description avaiable"]
    pub CPU_CPU1_LP: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CPU_CPU1_LOCK: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CPU_CPU1_GPR0: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CPU_CPU1_GPR1: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CPU_CPU1_GPR2: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CPU_CPU1_GPR3: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CPU_CPU1_GPR4: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CPU_CPU1_GPR5: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CPU_CPU1_GPR6: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CPU_CPU1_GPR7: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CPU_CPU1_GPR8: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CPU_CPU1_GPR9: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CPU_CPU1_GPR10: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CPU_CPU1_GPR11: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CPU_CPU1_GPR12: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CPU_CPU1_GPR13: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CPU_CPU1_STATUS0: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CPU_CPU1_STATUS1: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CPU_CPU1_STATUS2: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CPU_CPU1_STATUS3: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CPU_CPU1_STATUS4: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CPU_CPU1_STATUS5: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CPU_CPU1_STATUS6: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CPU_CPU1_STATUS7: crate::RWRegister<u32>,
    _reserved30: [u8; 0x20],
    #[doc = "No description avaiable"]
    pub CPU_CPU1_ENABLE0: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CPU_CPU1_ENABLE1: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CPU_CPU1_ENABLE2: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CPU_CPU1_ENABLE3: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CPU_CPU1_ENABLE4: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CPU_CPU1_ENABLE5: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CPU_CPU1_ENABLE6: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CPU_CPU1_ENABLE7: crate::RWRegister<u32>,
}
#[doc = "Resource control register for cpu0"]
pub mod RESOURCE_CPU0_CORE {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for cpx0"]
pub mod RESOURCE_CPU0_SUBSYS {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for cpu1"]
pub mod RESOURCE_CPU1_CORE {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for cpx1"]
pub mod RESOURCE_CPX1_SUBSYS {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for pow_con"]
pub mod RESOURCE_POW_CON {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for pow_vis"]
pub mod RESOURCE_POW_VIS {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for pow_cpu0"]
pub mod RESOURCE_POW_CPU0 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for pow_cpu1"]
pub mod RESOURCE_POW_CPU1 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for rst_soc"]
pub mod RESOURCE_RST_SOC {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for rst_con"]
pub mod RESOURCE_RST_CON {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for rst_vis"]
pub mod RESOURCE_RST_VIS {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for rst_cpu0"]
pub mod RESOURCE_RST_CPU0 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for rst_cpu1"]
pub mod RESOURCE_RST_CPU1 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for xtal"]
pub mod RESOURCE_CLK_SRC_XTAL {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for pll0"]
pub mod RESOURCE_CLK_SRC_PLL0 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk0_pll0"]
pub mod RESOURCE_CLK_SRC_PLL0CLK0 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for pll1"]
pub mod RESOURCE_CLK_SRC_PLL1 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk0_pll1"]
pub mod RESOURCE_CLK_SRC_PLL1CLK0 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk1_pll1"]
pub mod RESOURCE_CLK_SRC_PLL1CLK1 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for pll2"]
pub mod RESOURCE_CLK_SRC_PLL2 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk0_pll2"]
pub mod RESOURCE_CLK_SRC_PLL2CLK0 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk1_pll2"]
pub mod RESOURCE_CLK_SRC_PLL2CLK1 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for pll3"]
pub mod RESOURCE_CLK_SRC_PLL3 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk0_pll3"]
pub mod RESOURCE_CLK_SRC_PLL3CLK0 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for pll4"]
pub mod RESOURCE_CLK_SRC_PLL4 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk0_pll4"]
pub mod RESOURCE_CLK_SRC_PLL4CLK0 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk_top_cpu0"]
pub mod RESOURCE_CLK_TOP_CPU0 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk_top_mct0"]
pub mod RESOURCE_CLK_TOP_MCHTMR0 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk_top_cpu1"]
pub mod RESOURCE_CLK_TOP_CPU1 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk_top_mct1"]
pub mod RESOURCE_CLK_TOP_MCHTMR1 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk_top_axi0"]
pub mod RESOURCE_CLK_TOP_AXI {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk_top_axi1"]
pub mod RESOURCE_CLK_TOP_CONN {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk_top_axi2"]
pub mod RESOURCE_CLK_TOP_VIS {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk_top_ahb0"]
pub mod RESOURCE_CLK_TOP_AHB {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk_top_dram"]
pub mod RESOURCE_CLK_TOP_DRAM {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk_top_xpi0"]
pub mod RESOURCE_CLK_TOP_XPI0 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk_top_xpi1"]
pub mod RESOURCE_CLK_TOP_XPI1 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk_top_tmr0"]
pub mod RESOURCE_CLK_TOP_GPTMR0 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk_top_tmr1"]
pub mod RESOURCE_CLK_TOP_GPTMR1 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk_top_tmr2"]
pub mod RESOURCE_CLK_TOP_GPTMR2 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk_top_tmr3"]
pub mod RESOURCE_CLK_TOP_GPTMR3 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk_top_tmr4"]
pub mod RESOURCE_CLK_TOP_GPTMR4 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk_top_tmr5"]
pub mod RESOURCE_CLK_TOP_GPTMR5 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk_top_tmr6"]
pub mod RESOURCE_CLK_TOP_GPTMR6 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk_top_tmr7"]
pub mod RESOURCE_CLK_TOP_GPTMR7 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk_top_urt0"]
pub mod RESOURCE_CLK_TOP_UART0 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk_top_urt1"]
pub mod RESOURCE_CLK_TOP_UART1 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk_top_urt2"]
pub mod RESOURCE_CLK_TOP_UART2 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk_top_urt3"]
pub mod RESOURCE_CLK_TOP_UART3 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk_top_urt4"]
pub mod RESOURCE_CLK_TOP_UART4 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk_top_urt5"]
pub mod RESOURCE_CLK_TOP_UART5 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk_top_urt6"]
pub mod RESOURCE_CLK_TOP_UART6 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk_top_urt7"]
pub mod RESOURCE_CLK_TOP_UART7 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk_top_urt8"]
pub mod RESOURCE_CLK_TOP_UART8 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk_top_urt9"]
pub mod RESOURCE_CLK_TOP_UART9 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk_top_urta"]
pub mod RESOURCE_CLK_TOP_UART10 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk_top_urtb"]
pub mod RESOURCE_CLK_TOP_UART11 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk_top_urtc"]
pub mod RESOURCE_CLK_TOP_UART12 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk_top_urtd"]
pub mod RESOURCE_CLK_TOP_UART13 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk_top_urte"]
pub mod RESOURCE_CLK_TOP_UART14 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk_top_urtf"]
pub mod RESOURCE_CLK_TOP_UART15 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk_top_i2c0"]
pub mod RESOURCE_CLK_TOP_I2C0 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk_top_i2c1"]
pub mod RESOURCE_CLK_TOP_I2C1 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk_top_i2c2"]
pub mod RESOURCE_CLK_TOP_I2C2 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk_top_i2c3"]
pub mod RESOURCE_CLK_TOP_I2C3 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk_top_spi0"]
pub mod RESOURCE_CLK_TOP_SPI0 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk_top_spi1"]
pub mod RESOURCE_CLK_TOP_SPI1 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk_top_spi2"]
pub mod RESOURCE_CLK_TOP_SPI2 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk_top_spi3"]
pub mod RESOURCE_CLK_TOP_SPI3 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk_top_can0"]
pub mod RESOURCE_CLK_TOP_CAN0 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk_top_can1"]
pub mod RESOURCE_CLK_TOP_CAN1 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk_top_can2"]
pub mod RESOURCE_CLK_TOP_CAN2 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk_top_can3"]
pub mod RESOURCE_CLK_TOP_CAN3 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk_top_ptpc"]
pub mod RESOURCE_CLK_TOP_PTPC {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk_top_ana0"]
pub mod RESOURCE_CLK_TOP_ANA0 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk_top_ana1"]
pub mod RESOURCE_CLK_TOP_ANA1 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk_top_ana2"]
pub mod RESOURCE_CLK_TOP_ANA2 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk_top_aud0"]
pub mod RESOURCE_CLK_TOP_AUD0 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk_top_aud1"]
pub mod RESOURCE_CLK_TOP_AUD1 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk_top_aud2"]
pub mod RESOURCE_CLK_TOP_AUD2 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk_top_dis0"]
pub mod RESOURCE_CLK_TOP_LCDC {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk_top_cam0"]
pub mod RESOURCE_CLK_TOP_CAM0 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk_top_cam1"]
pub mod RESOURCE_CLK_TOP_CAM1 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk_top_eth0"]
pub mod RESOURCE_CLK_TOP_ENET0 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk_top_eth1"]
pub mod RESOURCE_CLK_TOP_ENET1 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk_top_ptp0"]
pub mod RESOURCE_CLK_TOP_PTP0 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk_top_ptp1"]
pub mod RESOURCE_CLK_TOP_PTP1 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk_top_ref0"]
pub mod RESOURCE_CLK_TOP_REF0 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk_top_ref1"]
pub mod RESOURCE_CLK_TOP_REF1 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk_top_ntm0"]
pub mod RESOURCE_CLK_TOP_NTMR0 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk_top_ntm1"]
pub mod RESOURCE_CLK_TOP_NTMR1 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk_top_sdc0"]
pub mod RESOURCE_CLK_TOP_SDXC0 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk_top_sdc1"]
pub mod RESOURCE_CLK_TOP_SDXC1 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk_top_adc0"]
pub mod RESOURCE_CLK_TOP_ADC0 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk_top_adc1"]
pub mod RESOURCE_CLK_TOP_ADC1 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk_top_adc2"]
pub mod RESOURCE_CLK_TOP_ADC2 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk_top_adc3"]
pub mod RESOURCE_CLK_TOP_ADC3 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk_top_i2s0"]
pub mod RESOURCE_CLK_TOP_I2S0 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk_top_i2s1"]
pub mod RESOURCE_CLK_TOP_I2S1 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk_top_i2s2"]
pub mod RESOURCE_CLK_TOP_I2S2 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clk_top_i2s3"]
pub mod RESOURCE_CLK_TOP_I2S3 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for ahbp"]
pub mod RESOURCE_AHBAPB_BUS {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for axis"]
pub mod RESOURCE_AXI_BUS {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for axic"]
pub mod RESOURCE_CONN_BUS {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for axiv"]
pub mod RESOURCE_VIS_BUS {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for dram"]
pub mod RESOURCE_DRAM {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for rom0"]
pub mod RESOURCE_ROM {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for lmm0"]
pub mod RESOURCE_LMM0 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for lmm1"]
pub mod RESOURCE_LMM1 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for mct0"]
pub mod RESOURCE_MCHTMR0 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for mct1"]
pub mod RESOURCE_MCHTMR1 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for ram0"]
pub mod RESOURCE_AXI_SRAM0 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for ram1"]
pub mod RESOURCE_AXI_SRAM1 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for xpi0"]
pub mod RESOURCE_XPI0 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for xpi1"]
pub mod RESOURCE_XPI1 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for sdp0"]
pub mod RESOURCE_SDP {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for rng0"]
pub mod RESOURCE_RNG {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for kman"]
pub mod RESOURCE_KEYM {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for dma0"]
pub mod RESOURCE_HDMA {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for dma1"]
pub mod RESOURCE_XDMA {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for gpio"]
pub mod RESOURCE_GPIO {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for mbx0"]
pub mod RESOURCE_MBX0 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for mbx1"]
pub mod RESOURCE_MBX1 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for wdg0"]
pub mod RESOURCE_WDG0 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for wdg1"]
pub mod RESOURCE_WDG1 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for wdg2"]
pub mod RESOURCE_WDG2 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for wdg3"]
pub mod RESOURCE_WDG3 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for tmr0"]
pub mod RESOURCE_GPTMR0 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for tmr1"]
pub mod RESOURCE_GPTMR1 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for tmr2"]
pub mod RESOURCE_GPTMR2 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for tmr3"]
pub mod RESOURCE_GPTMR3 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for tmr4"]
pub mod RESOURCE_GPTMR4 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for tmr5"]
pub mod RESOURCE_GPTMR5 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for tmr6"]
pub mod RESOURCE_GPTMR6 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for tmr7"]
pub mod RESOURCE_GPTMR7 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for urt0"]
pub mod RESOURCE_UART0 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for urt1"]
pub mod RESOURCE_UART1 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for urt2"]
pub mod RESOURCE_UART2 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for urt3"]
pub mod RESOURCE_UART3 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for urt4"]
pub mod RESOURCE_UART4 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for urt5"]
pub mod RESOURCE_UART5 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for urt6"]
pub mod RESOURCE_UART6 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for urt7"]
pub mod RESOURCE_UART7 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for urt8"]
pub mod RESOURCE_UART8 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for urt9"]
pub mod RESOURCE_UART9 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for urta"]
pub mod RESOURCE_UART10 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for urtb"]
pub mod RESOURCE_UART11 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for urtc"]
pub mod RESOURCE_UART12 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for urtd"]
pub mod RESOURCE_UART13 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for urte"]
pub mod RESOURCE_UART14 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for urtf"]
pub mod RESOURCE_UART15 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for i2c0"]
pub mod RESOURCE_I2C0 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for i2c1"]
pub mod RESOURCE_I2C1 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for i2c2"]
pub mod RESOURCE_I2C2 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for i2c3"]
pub mod RESOURCE_I2C3 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for spi0"]
pub mod RESOURCE_SPI0 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for spi1"]
pub mod RESOURCE_SPI1 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for spi2"]
pub mod RESOURCE_SPI2 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for spi3"]
pub mod RESOURCE_SPI3 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for can0"]
pub mod RESOURCE_CAN0 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for can1"]
pub mod RESOURCE_CAN1 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for can2"]
pub mod RESOURCE_CAN2 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for can3"]
pub mod RESOURCE_CAN3 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for ptpc"]
pub mod RESOURCE_PTPC {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for adc0"]
pub mod RESOURCE_ADC0 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for adc1"]
pub mod RESOURCE_ADC1 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for adc2"]
pub mod RESOURCE_ADC2 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for adc3"]
pub mod RESOURCE_ADC3 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for acmp"]
pub mod RESOURCE_ACMP {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for i2s0"]
pub mod RESOURCE_I2S0 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for i2s1"]
pub mod RESOURCE_I2S1 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for i2s2"]
pub mod RESOURCE_I2S2 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for i2s3"]
pub mod RESOURCE_I2S3 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for pdm0"]
pub mod RESOURCE_PDM {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for clsd"]
pub mod RESOURCE_DAO {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for msyn"]
pub mod RESOURCE_SYNT {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for mot0"]
pub mod RESOURCE_MOT0 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for mot1"]
pub mod RESOURCE_MOT1 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for mot2"]
pub mod RESOURCE_MOT2 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for mot3"]
pub mod RESOURCE_MOT3 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for dis0"]
pub mod RESOURCE_LCDC {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for cam0"]
pub mod RESOURCE_CAM0 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for cam1"]
pub mod RESOURCE_CAM1 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for jpeg"]
pub mod RESOURCE_JPEG {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for pdma"]
pub mod RESOURCE_PDMA {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for eth0"]
pub mod RESOURCE_ENET0 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for eth1"]
pub mod RESOURCE_ENET1 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for ntm0"]
pub mod RESOURCE_NTMR0 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for ntm1"]
pub mod RESOURCE_NTMR1 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for sdc0"]
pub mod RESOURCE_SDXC0 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for sdc1"]
pub mod RESOURCE_SDXC1 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for usb0"]
pub mod RESOURCE_USB0 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for usb1"]
pub mod RESOURCE_USB1 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for ref0"]
pub mod RESOURCE_REF0 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Resource control register for ref1"]
pub mod RESOURCE_REF1 {
    #[doc = "resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: no change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Goup setting"]
pub mod GROUP0_0_VALUE {
    pub mod AHB_APB {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod AXI {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod CONN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod VIS {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod DRAM {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod ROM {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod ILM_DLM0 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod ILM_DLM1 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod MCHTMRO {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod MCHTMR1 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod AXI_SRAMO {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod AXI_SRAM1 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod XPI0 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod XPI1 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod SDP {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod RNG {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod KEYM {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod HDMA {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod XDMA {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod GPIO0_1 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod MBX0 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod MBX1 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod WDG0 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod WDG1 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod WDG2 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod WDG3 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod GPTMRO {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod GPTMR1 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod GPTMR2 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod GPTMR3 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod GPTMR4 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod GPTMR5 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
}
#[doc = "Goup setting"]
pub mod GROUP0_0_SET {
    #[doc = "denpendency on peripherals, index count from resource ahbp(0x400),each bit represents a peripheral 0: peripheral is not needed 1: periphera is needed"]
    pub mod LINK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Goup setting"]
pub mod GROUP0_0_CLEAR {
    #[doc = "denpendency on peripherals, index count from resource ahbp(0x400),each bit represents a peripheral 0: peripheral is not needed 1: periphera is needed"]
    pub mod LINK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Goup setting"]
pub mod GROUP0_0_TOGGLE {
    #[doc = "denpendency on peripherals, index count from resource ahbp(0x400),each bit represents a peripheral 0: peripheral is not needed 1: periphera is needed"]
    pub mod LINK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Goup setting"]
pub mod GROUP0_1_VALUE {
    pub mod GPTMR6 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod GPTMR7 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod UARTO {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod UART1 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod UART2 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod UART3 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod UART4 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod UART5 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod UART6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod UART7 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod UART8 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod UART9 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod UART10 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod UART11 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod UART12 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod UART13 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod UART14 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod UART15 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod I2C0 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod I2C1 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod L2C2 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod I2C3 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod SPI0 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod SPI1 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod SPI2 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod SPI3 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod CANO {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod CAN1 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod CAN2 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod CAN3 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod PTPC {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod ADC0 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
}
#[doc = "Goup setting"]
pub mod GROUP0_1_SET {
    #[doc = "denpendency on peripherals, index count from resource ahbp(0x400),each bit represents a peripheral 0: peripheral is not needed 1: periphera is needed"]
    pub mod LINK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Goup setting"]
pub mod GROUP0_1_CLEAR {
    #[doc = "denpendency on peripherals, index count from resource ahbp(0x400),each bit represents a peripheral 0: peripheral is not needed 1: periphera is needed"]
    pub mod LINK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Goup setting"]
pub mod GROUP0_1_TOGGLE {
    #[doc = "denpendency on peripherals, index count from resource ahbp(0x400),each bit represents a peripheral 0: peripheral is not needed 1: periphera is needed"]
    pub mod LINK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Goup setting"]
pub mod GROUP0_2_VALUE {
    pub mod ADC1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod ADC2 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod ADC3 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod ACMP {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod I2S0 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod I2S1 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod I2S2 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod I2S3 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod PDM {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod DAO {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod SYNT {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod MOTO {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod MOT1 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod MOT2 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod MOT3 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod LCDC {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod CAMO {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod CAM1 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod JPEG {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod PDMA {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod ENETO {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod ENET1 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod NTMRO {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod NTMR1 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod SDXC0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod SDXC1 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod USBO {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
    pub mod USB1 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
}
#[doc = "Goup setting"]
pub mod GROUP0_2_SET {
    #[doc = "denpendency on peripherals, index count from resource ahbp(0x400),each bit represents a peripheral 0: peripheral is not needed 1: periphera is needed"]
    pub mod LINK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Goup setting"]
pub mod GROUP0_2_CLEAR {
    #[doc = "denpendency on peripherals, index count from resource ahbp(0x400),each bit represents a peripheral 0: peripheral is not needed 1: periphera is needed"]
    pub mod LINK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Goup setting"]
pub mod GROUP0_2_TOGGLE {
    #[doc = "denpendency on peripherals, index count from resource ahbp(0x400),each bit represents a peripheral 0: peripheral is not needed 1: periphera is needed"]
    pub mod LINK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Goup setting"]
pub mod GROUP1_0_VALUE {
    #[doc = "denpendency on peripherals, index count from resource ahbp(0x400),each bit represents a peripheral 0: peripheral is not needed 1: periphera is needed"]
    pub mod LINK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
}
#[doc = "Goup setting"]
pub mod GROUP1_0_SET {
    #[doc = "denpendency on peripherals, index count from resource ahbp(0x400),each bit represents a peripheral 0: peripheral is not needed 1: periphera is needed"]
    pub mod LINK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Goup setting"]
pub mod GROUP1_0_CLEAR {
    #[doc = "denpendency on peripherals, index count from resource ahbp(0x400),each bit represents a peripheral 0: peripheral is not needed 1: periphera is needed"]
    pub mod LINK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Goup setting"]
pub mod GROUP1_0_TOGGLE {
    #[doc = "denpendency on peripherals, index count from resource ahbp(0x400),each bit represents a peripheral 0: peripheral is not needed 1: periphera is needed"]
    pub mod LINK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Goup setting"]
pub mod GROUP1_1_VALUE {
    #[doc = "denpendency on peripherals, index count from resource ahbp(0x400),each bit represents a peripheral 0: peripheral is not needed 1: periphera is needed"]
    pub mod LINK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
}
#[doc = "Goup setting"]
pub mod GROUP1_1_SET {
    #[doc = "denpendency on peripherals, index count from resource ahbp(0x400),each bit represents a peripheral 0: peripheral is not needed 1: periphera is needed"]
    pub mod LINK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Goup setting"]
pub mod GROUP1_1_CLEAR {
    #[doc = "denpendency on peripherals, index count from resource ahbp(0x400),each bit represents a peripheral 0: peripheral is not needed 1: periphera is needed"]
    pub mod LINK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Goup setting"]
pub mod GROUP1_1_TOGGLE {
    #[doc = "denpendency on peripherals, index count from resource ahbp(0x400),each bit represents a peripheral 0: peripheral is not needed 1: periphera is needed"]
    pub mod LINK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Goup setting"]
pub mod GROUP1_2_VALUE {
    #[doc = "denpendency on peripherals, index count from resource ahbp(0x400),each bit represents a peripheral 0: peripheral is not needed 1: periphera is needed"]
    pub mod LINK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unlink from group"]
            pub const Unlinked: u32 = 0;
            #[doc = "Link to group"]
            pub const Linked: u32 = 0x01;
        }
    }
}
#[doc = "Goup setting"]
pub mod GROUP1_2_SET {
    #[doc = "denpendency on peripherals, index count from resource ahbp(0x400),each bit represents a peripheral 0: peripheral is not needed 1: periphera is needed"]
    pub mod LINK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Goup setting"]
pub mod GROUP1_2_CLEAR {
    #[doc = "denpendency on peripherals, index count from resource ahbp(0x400),each bit represents a peripheral 0: peripheral is not needed 1: periphera is needed"]
    pub mod LINK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Goup setting"]
pub mod GROUP1_2_TOGGLE {
    #[doc = "denpendency on peripherals, index count from resource ahbp(0x400),each bit represents a peripheral 0: peripheral is not needed 1: periphera is needed"]
    pub mod LINK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Affiliate of Group"]
pub mod AFFILIATE_CPU0_VALUE {
    #[doc = "Affiliate groups of cpu0 bit0: cpu0 depends on logic node0 bit1: cpu0 depends on logic node1 bit2: cpu0 depends on logic node2 bit3: cpu0 depends on logic node3"]
    pub mod LINK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Affiliate of Group"]
pub mod AFFILIATE_CPU0_SET {
    #[doc = "Affiliate groups of cpu0 bit0: cpu0 depends on logic node0 bit1: cpu0 depends on logic node1 bit2: cpu0 depends on logic node2 bit3: cpu0 depends on logic node3"]
    pub mod LINK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Affiliate of Group"]
pub mod AFFILIATE_CPU0_CLEAR {
    #[doc = "Affiliate groups of cpu0 bit0: cpu0 depends on logic node0 bit1: cpu0 depends on logic node1 bit2: cpu0 depends on logic node2 bit3: cpu0 depends on logic node3"]
    pub mod LINK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Affiliate of Group"]
pub mod AFFILIATE_CPU0_TOGGLE {
    #[doc = "Affiliate groups of cpu0 bit0: cpu0 depends on logic node0 bit1: cpu0 depends on logic node1 bit2: cpu0 depends on logic node2 bit3: cpu0 depends on logic node3"]
    pub mod LINK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Affiliate of Group"]
pub mod AFFILIATE_CPU1_VALUE {
    #[doc = "Affiliate groups of cpu0 bit0: cpu0 depends on logic node0 bit1: cpu0 depends on logic node1 bit2: cpu0 depends on logic node2 bit3: cpu0 depends on logic node3"]
    pub mod LINK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Affiliate of Group"]
pub mod AFFILIATE_CPU1_SET {
    #[doc = "Affiliate groups of cpu0 bit0: cpu0 depends on logic node0 bit1: cpu0 depends on logic node1 bit2: cpu0 depends on logic node2 bit3: cpu0 depends on logic node3"]
    pub mod LINK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Affiliate of Group"]
pub mod AFFILIATE_CPU1_CLEAR {
    #[doc = "Affiliate groups of cpu0 bit0: cpu0 depends on logic node0 bit1: cpu0 depends on logic node1 bit2: cpu0 depends on logic node2 bit3: cpu0 depends on logic node3"]
    pub mod LINK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Affiliate of Group"]
pub mod AFFILIATE_CPU1_TOGGLE {
    #[doc = "Affiliate groups of cpu0 bit0: cpu0 depends on logic node0 bit1: cpu0 depends on logic node1 bit2: cpu0 depends on logic node2 bit3: cpu0 depends on logic node3"]
    pub mod LINK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Retention Contol"]
pub mod RETENTION_CPU0_VALUE {
    #[doc = "retention setting while system sleep, each bit represents a resource bit0: soc_pow bit1: soc_rst bit2: cpu0_pow bit3: cpu0_rst bit4: cpu1_pow bit5: cpu1_rst bit6: con_pow bit7: con_rst bit8: vis_pow bit9: vis_rst bit10: xtal bit11: pll0 bit12: pll1 bit13: pll2 bit14: pll3 bit15: pll4"]
    pub mod LINK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Retention Contol"]
pub mod RETENTION_CPU0_SET {
    #[doc = "retention setting while system sleep"]
    pub mod LINK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Retention Contol"]
pub mod RETENTION_CPU0_CLEAR {
    #[doc = "retention setting while system sleep"]
    pub mod LINK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Retention Contol"]
pub mod RETENTION_CPU0_TOGGLE {
    #[doc = "retention setting while system sleep"]
    pub mod LINK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Retention Contol"]
pub mod RETENTION_CPU1_VALUE {
    #[doc = "retention setting while system sleep, each bit represents a resource bit0: soc_pow bit1: soc_rst bit2: cpu0_pow bit3: cpu0_rst bit4: cpu1_pow bit5: cpu1_rst bit6: con_pow bit7: con_rst bit8: vis_pow bit9: vis_rst bit10: xtal bit11: pll0 bit12: pll1 bit13: pll2 bit14: pll3 bit15: pll4"]
    pub mod LINK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Retention Contol"]
pub mod RETENTION_CPU1_SET {
    #[doc = "retention setting while system sleep"]
    pub mod LINK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Retention Contol"]
pub mod RETENTION_CPU1_CLEAR {
    #[doc = "retention setting while system sleep"]
    pub mod LINK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Retention Contol"]
pub mod RETENTION_CPU1_TOGGLE {
    #[doc = "retention setting while system sleep"]
    pub mod LINK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0003_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Power Setting"]
pub mod POWER_CPU0_STATUS {
    #[doc = "low fanout power switch feedback 0: low fanout power switches are turned on 1: low fanout power switches are truned off"]
    pub mod LF_ACK {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "low fanout power switch disable 0: low fanout power switches are turned on 1: low fanout power switches are truned off"]
    pub mod LF_DISABLE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "flag represents wakeup power cycle happened from last clear of this bit 0: power domain did not edurance wakeup power cycle since last clear of this bit 1: power domain enduranced wakeup power cycle since last clear of this bit"]
    pub mod FLAG_WAKE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "flag represents power cycle happened from last clear of this bit 0: power domain did not edurance power cycle since last clear of this bit 1: power domain enduranced power cycle since last clear of this bit"]
    pub mod FLAG {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Power Setting"]
pub mod POWER_CPU0_LF_WAIT {
    #[doc = "wait time for low fan out power switch turn on, default value is 255 0: 0 clock cycle 1: 1 clock cycles . . . clock cycles count on 24MHz"]
    pub mod WAIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Power Setting"]
pub mod POWER_CPU0_OFF_WAIT {
    #[doc = "wait time for power switch turn off, default value is 15 0: 0 clock cycle 1: 1 clock cycles . . . clock cycles count on 24MHz"]
    pub mod WAIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Power Setting"]
pub mod POWER_CPU1_STATUS {
    #[doc = "low fanout power switch feedback 0: low fanout power switches are turned on 1: low fanout power switches are truned off"]
    pub mod LF_ACK {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "low fanout power switch disable 0: low fanout power switches are turned on 1: low fanout power switches are truned off"]
    pub mod LF_DISABLE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "flag represents wakeup power cycle happened from last clear of this bit 0: power domain did not edurance wakeup power cycle since last clear of this bit 1: power domain enduranced wakeup power cycle since last clear of this bit"]
    pub mod FLAG_WAKE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "flag represents power cycle happened from last clear of this bit 0: power domain did not edurance power cycle since last clear of this bit 1: power domain enduranced power cycle since last clear of this bit"]
    pub mod FLAG {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Power Setting"]
pub mod POWER_CPU1_LF_WAIT {
    #[doc = "wait time for low fan out power switch turn on, default value is 255 0: 0 clock cycle 1: 1 clock cycles . . . clock cycles count on 24MHz"]
    pub mod WAIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Power Setting"]
pub mod POWER_CPU1_OFF_WAIT {
    #[doc = "wait time for power switch turn off, default value is 15 0: 0 clock cycle 1: 1 clock cycles . . . clock cycles count on 24MHz"]
    pub mod WAIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Power Setting"]
pub mod POWER_CON_STATUS {
    #[doc = "low fanout power switch feedback 0: low fanout power switches are turned on 1: low fanout power switches are truned off"]
    pub mod LF_ACK {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "low fanout power switch disable 0: low fanout power switches are turned on 1: low fanout power switches are truned off"]
    pub mod LF_DISABLE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "flag represents wakeup power cycle happened from last clear of this bit 0: power domain did not edurance wakeup power cycle since last clear of this bit 1: power domain enduranced wakeup power cycle since last clear of this bit"]
    pub mod FLAG_WAKE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "flag represents power cycle happened from last clear of this bit 0: power domain did not edurance power cycle since last clear of this bit 1: power domain enduranced power cycle since last clear of this bit"]
    pub mod FLAG {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Power Setting"]
pub mod POWER_CON_LF_WAIT {
    #[doc = "wait time for low fan out power switch turn on, default value is 255 0: 0 clock cycle 1: 1 clock cycles . . . clock cycles count on 24MHz"]
    pub mod WAIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Power Setting"]
pub mod POWER_CON_OFF_WAIT {
    #[doc = "wait time for power switch turn off, default value is 15 0: 0 clock cycle 1: 1 clock cycles . . . clock cycles count on 24MHz"]
    pub mod WAIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Power Setting"]
pub mod POWER_VIS_STATUS {
    #[doc = "low fanout power switch feedback 0: low fanout power switches are turned on 1: low fanout power switches are truned off"]
    pub mod LF_ACK {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "low fanout power switch disable 0: low fanout power switches are turned on 1: low fanout power switches are truned off"]
    pub mod LF_DISABLE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "flag represents wakeup power cycle happened from last clear of this bit 0: power domain did not edurance wakeup power cycle since last clear of this bit 1: power domain enduranced wakeup power cycle since last clear of this bit"]
    pub mod FLAG_WAKE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "flag represents power cycle happened from last clear of this bit 0: power domain did not edurance power cycle since last clear of this bit 1: power domain enduranced power cycle since last clear of this bit"]
    pub mod FLAG {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Power Setting"]
pub mod POWER_VIS_LF_WAIT {
    #[doc = "wait time for low fan out power switch turn on, default value is 255 0: 0 clock cycle 1: 1 clock cycles . . . clock cycles count on 24MHz"]
    pub mod WAIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Power Setting"]
pub mod POWER_VIS_OFF_WAIT {
    #[doc = "wait time for power switch turn off, default value is 15 0: 0 clock cycle 1: 1 clock cycles . . . clock cycles count on 24MHz"]
    pub mod WAIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Reset Setting"]
pub mod RESET_SOC_CONTROL {
    #[doc = "perform reset and release imediately 0: reset is released 1 reset is asserted and will release automaticly"]
    pub mod RESET {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "perform reset and hold in reset, until ths bit cleared by software 0: reset is released for function 1: reset is assert and hold"]
    pub mod HOLD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "flag represents wakeup reset happened from last clear of this bit 0: domain did not edurance wakeup reset cycle since last clear of this bit 1: domain enduranced wakeup reset cycle since last clear of this bit"]
    pub mod FLAG_WAKE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "flag represents reset happened from last clear of this bit 0: domain did not edurance reset cycle since last clear of this bit 1: domain enduranced reset cycle since last clear of this bit"]
    pub mod FLAG {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Reset Setting"]
pub mod RESET_SOC_CONFIG {
    #[doc = "time guard band for reset release 0: wait 0 cycle 1: wait 1 cycles . . . Note, clock cycle is base on 24M"]
    pub mod POST_WAIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reset clock number(must be even number) 0: 0 cycle 1: 0 cycles 2: 2 cycles 3: 2 cycles . . . Note, clock cycle is base on 24M"]
    pub mod RSTCLK_NUM {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "wait cycle numbers before assert reset 0: wait 0 cycle 1: wait 1 cycles . . . Note, clock cycle is base on 24M"]
    pub mod PRE_WAIT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Reset Setting"]
pub mod RESET_SOC_COUNTER {
    #[doc = "self clear trigger counter, reset triggered when counter value is 1, write 0 will cancel reset 0: wait 0 cycle 1: wait 1 cycles . . . Note, clock cycle is base on 24M"]
    pub mod COUNTER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Reset Setting"]
pub mod RESET_CON_CONTROL {
    #[doc = "perform reset and release imediately 0: reset is released 1 reset is asserted and will release automaticly"]
    pub mod RESET {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "perform reset and hold in reset, until ths bit cleared by software 0: reset is released for function 1: reset is assert and hold"]
    pub mod HOLD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "flag represents wakeup reset happened from last clear of this bit 0: domain did not edurance wakeup reset cycle since last clear of this bit 1: domain enduranced wakeup reset cycle since last clear of this bit"]
    pub mod FLAG_WAKE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "flag represents reset happened from last clear of this bit 0: domain did not edurance reset cycle since last clear of this bit 1: domain enduranced reset cycle since last clear of this bit"]
    pub mod FLAG {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Reset Setting"]
pub mod RESET_CON_CONFIG {
    #[doc = "time guard band for reset release 0: wait 0 cycle 1: wait 1 cycles . . . Note, clock cycle is base on 24M"]
    pub mod POST_WAIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reset clock number(must be even number) 0: 0 cycle 1: 0 cycles 2: 2 cycles 3: 2 cycles . . . Note, clock cycle is base on 24M"]
    pub mod RSTCLK_NUM {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "wait cycle numbers before assert reset 0: wait 0 cycle 1: wait 1 cycles . . . Note, clock cycle is base on 24M"]
    pub mod PRE_WAIT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Reset Setting"]
pub mod RESET_CON_COUNTER {
    #[doc = "self clear trigger counter, reset triggered when counter value is 1, write 0 will cancel reset 0: wait 0 cycle 1: wait 1 cycles . . . Note, clock cycle is base on 24M"]
    pub mod COUNTER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Reset Setting"]
pub mod RESET_VIS_CONTROL {
    #[doc = "perform reset and release imediately 0: reset is released 1 reset is asserted and will release automaticly"]
    pub mod RESET {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "perform reset and hold in reset, until ths bit cleared by software 0: reset is released for function 1: reset is assert and hold"]
    pub mod HOLD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "flag represents wakeup reset happened from last clear of this bit 0: domain did not edurance wakeup reset cycle since last clear of this bit 1: domain enduranced wakeup reset cycle since last clear of this bit"]
    pub mod FLAG_WAKE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "flag represents reset happened from last clear of this bit 0: domain did not edurance reset cycle since last clear of this bit 1: domain enduranced reset cycle since last clear of this bit"]
    pub mod FLAG {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Reset Setting"]
pub mod RESET_VIS_CONFIG {
    #[doc = "time guard band for reset release 0: wait 0 cycle 1: wait 1 cycles . . . Note, clock cycle is base on 24M"]
    pub mod POST_WAIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reset clock number(must be even number) 0: 0 cycle 1: 0 cycles 2: 2 cycles 3: 2 cycles . . . Note, clock cycle is base on 24M"]
    pub mod RSTCLK_NUM {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "wait cycle numbers before assert reset 0: wait 0 cycle 1: wait 1 cycles . . . Note, clock cycle is base on 24M"]
    pub mod PRE_WAIT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Reset Setting"]
pub mod RESET_VIS_COUNTER {
    #[doc = "self clear trigger counter, reset triggered when counter value is 1, write 0 will cancel reset 0: wait 0 cycle 1: wait 1 cycles . . . Note, clock cycle is base on 24M"]
    pub mod COUNTER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Reset Setting"]
pub mod RESET_CPU0_CONTROL {
    #[doc = "perform reset and release imediately 0: reset is released 1 reset is asserted and will release automaticly"]
    pub mod RESET {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "perform reset and hold in reset, until ths bit cleared by software 0: reset is released for function 1: reset is assert and hold"]
    pub mod HOLD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "flag represents wakeup reset happened from last clear of this bit 0: domain did not edurance wakeup reset cycle since last clear of this bit 1: domain enduranced wakeup reset cycle since last clear of this bit"]
    pub mod FLAG_WAKE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "flag represents reset happened from last clear of this bit 0: domain did not edurance reset cycle since last clear of this bit 1: domain enduranced reset cycle since last clear of this bit"]
    pub mod FLAG {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Reset Setting"]
pub mod RESET_CPU0_CONFIG {
    #[doc = "time guard band for reset release 0: wait 0 cycle 1: wait 1 cycles . . . Note, clock cycle is base on 24M"]
    pub mod POST_WAIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reset clock number(must be even number) 0: 0 cycle 1: 0 cycles 2: 2 cycles 3: 2 cycles . . . Note, clock cycle is base on 24M"]
    pub mod RSTCLK_NUM {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "wait cycle numbers before assert reset 0: wait 0 cycle 1: wait 1 cycles . . . Note, clock cycle is base on 24M"]
    pub mod PRE_WAIT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Reset Setting"]
pub mod RESET_CPU0_COUNTER {
    #[doc = "self clear trigger counter, reset triggered when counter value is 1, write 0 will cancel reset 0: wait 0 cycle 1: wait 1 cycles . . . Note, clock cycle is base on 24M"]
    pub mod COUNTER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Reset Setting"]
pub mod RESET_CPU1_CONTROL {
    #[doc = "perform reset and release imediately 0: reset is released 1 reset is asserted and will release automaticly"]
    pub mod RESET {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "perform reset and hold in reset, until ths bit cleared by software 0: reset is released for function 1: reset is assert and hold"]
    pub mod HOLD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "flag represents wakeup reset happened from last clear of this bit 0: domain did not edurance wakeup reset cycle since last clear of this bit 1: domain enduranced wakeup reset cycle since last clear of this bit"]
    pub mod FLAG_WAKE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "flag represents reset happened from last clear of this bit 0: domain did not edurance reset cycle since last clear of this bit 1: domain enduranced reset cycle since last clear of this bit"]
    pub mod FLAG {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Reset Setting"]
pub mod RESET_CPU1_CONFIG {
    #[doc = "time guard band for reset release 0: wait 0 cycle 1: wait 1 cycles . . . Note, clock cycle is base on 24M"]
    pub mod POST_WAIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reset clock number(must be even number) 0: 0 cycle 1: 0 cycles 2: 2 cycles 3: 2 cycles . . . Note, clock cycle is base on 24M"]
    pub mod RSTCLK_NUM {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "wait cycle numbers before assert reset 0: wait 0 cycle 1: wait 1 cycles . . . Note, clock cycle is base on 24M"]
    pub mod PRE_WAIT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Reset Setting"]
pub mod RESET_CPU1_COUNTER {
    #[doc = "self clear trigger counter, reset triggered when counter value is 1, write 0 will cancel reset 0: wait 0 cycle 1: wait 1 cycles . . . Note, clock cycle is base on 24M"]
    pub mod COUNTER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock setting"]
pub mod CLOCK_CLK_TOP_CPU0 {
    #[doc = "clock divider 0: divider by1 1: divider by 2 2 divider by 3 . . . 255: divider by 256"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock source selection 0:osc0_clk0 1:pll0_clk0 2:pll1_clk0 3:pll1_clk1 4:pll2_clk0 5:pll2_clk1 6:pll3_clk0 7:pll4_clk0"]
    pub mod MUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: a change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock setting"]
pub mod CLOCK_CLK_TOP_MCHTMR0 {
    #[doc = "clock divider 0: divider by1 1: divider by 2 2 divider by 3 . . . 255: divider by 256"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock source selection 0:osc0_clk0 1:pll0_clk0 2:pll1_clk0 3:pll1_clk1 4:pll2_clk0 5:pll2_clk1 6:pll3_clk0 7:pll4_clk0"]
    pub mod MUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: a change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock setting"]
pub mod CLOCK_CLK_TOP_CPU1 {
    #[doc = "clock divider 0: divider by1 1: divider by 2 2 divider by 3 . . . 255: divider by 256"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock source selection 0:osc0_clk0 1:pll0_clk0 2:pll1_clk0 3:pll1_clk1 4:pll2_clk0 5:pll2_clk1 6:pll3_clk0 7:pll4_clk0"]
    pub mod MUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: a change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock setting"]
pub mod CLOCK_CLK_TOP_MCHTMR {
    #[doc = "clock divider 0: divider by1 1: divider by 2 2 divider by 3 . . . 255: divider by 256"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock source selection 0:osc0_clk0 1:pll0_clk0 2:pll1_clk0 3:pll1_clk1 4:pll2_clk0 5:pll2_clk1 6:pll3_clk0 7:pll4_clk0"]
    pub mod MUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: a change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock setting"]
pub mod CLOCK_CLK_TOP_AXI {
    #[doc = "clock divider 0: divider by1 1: divider by 2 2 divider by 3 . . . 255: divider by 256"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock source selection 0:osc0_clk0 1:pll0_clk0 2:pll1_clk0 3:pll1_clk1 4:pll2_clk0 5:pll2_clk1 6:pll3_clk0 7:pll4_clk0"]
    pub mod MUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: a change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock setting"]
pub mod CLOCK_CLK_TOP_CONN {
    #[doc = "clock divider 0: divider by1 1: divider by 2 2 divider by 3 . . . 255: divider by 256"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock source selection 0:osc0_clk0 1:pll0_clk0 2:pll1_clk0 3:pll1_clk1 4:pll2_clk0 5:pll2_clk1 6:pll3_clk0 7:pll4_clk0"]
    pub mod MUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: a change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock setting"]
pub mod CLOCK_CLK_TOP_VIS {
    #[doc = "clock divider 0: divider by1 1: divider by 2 2 divider by 3 . . . 255: divider by 256"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock source selection 0:osc0_clk0 1:pll0_clk0 2:pll1_clk0 3:pll1_clk1 4:pll2_clk0 5:pll2_clk1 6:pll3_clk0 7:pll4_clk0"]
    pub mod MUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: a change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock setting"]
pub mod CLOCK_CLK_TOP_AHB {
    #[doc = "clock divider 0: divider by1 1: divider by 2 2 divider by 3 . . . 255: divider by 256"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock source selection 0:osc0_clk0 1:pll0_clk0 2:pll1_clk0 3:pll1_clk1 4:pll2_clk0 5:pll2_clk1 6:pll3_clk0 7:pll4_clk0"]
    pub mod MUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: a change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock setting"]
pub mod CLOCK_CLK_TOP_DRAM {
    #[doc = "clock divider 0: divider by1 1: divider by 2 2 divider by 3 . . . 255: divider by 256"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock source selection 0:osc0_clk0 1:pll0_clk0 2:pll1_clk0 3:pll1_clk1 4:pll2_clk0 5:pll2_clk1 6:pll3_clk0 7:pll4_clk0"]
    pub mod MUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: a change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock setting"]
pub mod CLOCK_CLK_TOP_XPI0 {
    #[doc = "clock divider 0: divider by1 1: divider by 2 2 divider by 3 . . . 255: divider by 256"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock source selection 0:osc0_clk0 1:pll0_clk0 2:pll1_clk0 3:pll1_clk1 4:pll2_clk0 5:pll2_clk1 6:pll3_clk0 7:pll4_clk0"]
    pub mod MUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: a change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock setting"]
pub mod CLOCK_CLK_TOP_XPI1 {
    #[doc = "clock divider 0: divider by1 1: divider by 2 2 divider by 3 . . . 255: divider by 256"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock source selection 0:osc0_clk0 1:pll0_clk0 2:pll1_clk0 3:pll1_clk1 4:pll2_clk0 5:pll2_clk1 6:pll3_clk0 7:pll4_clk0"]
    pub mod MUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: a change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock setting"]
pub mod CLOCK_CLK_TOP_GPTMR0 {
    #[doc = "clock divider 0: divider by1 1: divider by 2 2 divider by 3 . . . 255: divider by 256"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock source selection 0:osc0_clk0 1:pll0_clk0 2:pll1_clk0 3:pll1_clk1 4:pll2_clk0 5:pll2_clk1 6:pll3_clk0 7:pll4_clk0"]
    pub mod MUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: a change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock setting"]
pub mod CLOCK_CLK_TOP_GPTMR1 {
    #[doc = "clock divider 0: divider by1 1: divider by 2 2 divider by 3 . . . 255: divider by 256"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock source selection 0:osc0_clk0 1:pll0_clk0 2:pll1_clk0 3:pll1_clk1 4:pll2_clk0 5:pll2_clk1 6:pll3_clk0 7:pll4_clk0"]
    pub mod MUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: a change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock setting"]
pub mod CLOCK_CLK_TOP_GPTMR2 {
    #[doc = "clock divider 0: divider by1 1: divider by 2 2 divider by 3 . . . 255: divider by 256"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock source selection 0:osc0_clk0 1:pll0_clk0 2:pll1_clk0 3:pll1_clk1 4:pll2_clk0 5:pll2_clk1 6:pll3_clk0 7:pll4_clk0"]
    pub mod MUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: a change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock setting"]
pub mod CLOCK_CLK_TOP_GPTMR3 {
    #[doc = "clock divider 0: divider by1 1: divider by 2 2 divider by 3 . . . 255: divider by 256"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock source selection 0:osc0_clk0 1:pll0_clk0 2:pll1_clk0 3:pll1_clk1 4:pll2_clk0 5:pll2_clk1 6:pll3_clk0 7:pll4_clk0"]
    pub mod MUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: a change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock setting"]
pub mod CLOCK_CLK_TOP_GPTMR4 {
    #[doc = "clock divider 0: divider by1 1: divider by 2 2 divider by 3 . . . 255: divider by 256"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock source selection 0:osc0_clk0 1:pll0_clk0 2:pll1_clk0 3:pll1_clk1 4:pll2_clk0 5:pll2_clk1 6:pll3_clk0 7:pll4_clk0"]
    pub mod MUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: a change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock setting"]
pub mod CLOCK_CLK_TOP_GPTMR5 {
    #[doc = "clock divider 0: divider by1 1: divider by 2 2 divider by 3 . . . 255: divider by 256"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock source selection 0:osc0_clk0 1:pll0_clk0 2:pll1_clk0 3:pll1_clk1 4:pll2_clk0 5:pll2_clk1 6:pll3_clk0 7:pll4_clk0"]
    pub mod MUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: a change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock setting"]
pub mod CLOCK_CLK_TOP_GPTMR6 {
    #[doc = "clock divider 0: divider by1 1: divider by 2 2 divider by 3 . . . 255: divider by 256"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock source selection 0:osc0_clk0 1:pll0_clk0 2:pll1_clk0 3:pll1_clk1 4:pll2_clk0 5:pll2_clk1 6:pll3_clk0 7:pll4_clk0"]
    pub mod MUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: a change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock setting"]
pub mod CLOCK_CLK_TOP_GPTMR7 {
    #[doc = "clock divider 0: divider by1 1: divider by 2 2 divider by 3 . . . 255: divider by 256"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock source selection 0:osc0_clk0 1:pll0_clk0 2:pll1_clk0 3:pll1_clk1 4:pll2_clk0 5:pll2_clk1 6:pll3_clk0 7:pll4_clk0"]
    pub mod MUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: a change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock setting"]
pub mod CLOCK_CLK_TOP_UART0 {
    #[doc = "clock divider 0: divider by1 1: divider by 2 2 divider by 3 . . . 255: divider by 256"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock source selection 0:osc0_clk0 1:pll0_clk0 2:pll1_clk0 3:pll1_clk1 4:pll2_clk0 5:pll2_clk1 6:pll3_clk0 7:pll4_clk0"]
    pub mod MUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: a change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock setting"]
pub mod CLOCK_CLK_TOP_UART1 {
    #[doc = "clock divider 0: divider by1 1: divider by 2 2 divider by 3 . . . 255: divider by 256"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock source selection 0:osc0_clk0 1:pll0_clk0 2:pll1_clk0 3:pll1_clk1 4:pll2_clk0 5:pll2_clk1 6:pll3_clk0 7:pll4_clk0"]
    pub mod MUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: a change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock setting"]
pub mod CLOCK_CLK_TOP_UART2 {
    #[doc = "clock divider 0: divider by1 1: divider by 2 2 divider by 3 . . . 255: divider by 256"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock source selection 0:osc0_clk0 1:pll0_clk0 2:pll1_clk0 3:pll1_clk1 4:pll2_clk0 5:pll2_clk1 6:pll3_clk0 7:pll4_clk0"]
    pub mod MUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: a change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock setting"]
pub mod CLOCK_CLK_TOP_UART3 {
    #[doc = "clock divider 0: divider by1 1: divider by 2 2 divider by 3 . . . 255: divider by 256"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock source selection 0:osc0_clk0 1:pll0_clk0 2:pll1_clk0 3:pll1_clk1 4:pll2_clk0 5:pll2_clk1 6:pll3_clk0 7:pll4_clk0"]
    pub mod MUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: a change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock setting"]
pub mod CLOCK_CLK_TOP_UART4 {
    #[doc = "clock divider 0: divider by1 1: divider by 2 2 divider by 3 . . . 255: divider by 256"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock source selection 0:osc0_clk0 1:pll0_clk0 2:pll1_clk0 3:pll1_clk1 4:pll2_clk0 5:pll2_clk1 6:pll3_clk0 7:pll4_clk0"]
    pub mod MUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: a change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock setting"]
pub mod CLOCK_CLK_TOP_UART5 {
    #[doc = "clock divider 0: divider by1 1: divider by 2 2 divider by 3 . . . 255: divider by 256"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock source selection 0:osc0_clk0 1:pll0_clk0 2:pll1_clk0 3:pll1_clk1 4:pll2_clk0 5:pll2_clk1 6:pll3_clk0 7:pll4_clk0"]
    pub mod MUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: a change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock setting"]
pub mod CLOCK_CLK_TOP_UART6 {
    #[doc = "clock divider 0: divider by1 1: divider by 2 2 divider by 3 . . . 255: divider by 256"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock source selection 0:osc0_clk0 1:pll0_clk0 2:pll1_clk0 3:pll1_clk1 4:pll2_clk0 5:pll2_clk1 6:pll3_clk0 7:pll4_clk0"]
    pub mod MUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: a change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock setting"]
pub mod CLOCK_CLK_TOP_UART7 {
    #[doc = "clock divider 0: divider by1 1: divider by 2 2 divider by 3 . . . 255: divider by 256"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock source selection 0:osc0_clk0 1:pll0_clk0 2:pll1_clk0 3:pll1_clk1 4:pll2_clk0 5:pll2_clk1 6:pll3_clk0 7:pll4_clk0"]
    pub mod MUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: a change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock setting"]
pub mod CLOCK_CLK_TOP_UART8 {
    #[doc = "clock divider 0: divider by1 1: divider by 2 2 divider by 3 . . . 255: divider by 256"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock source selection 0:osc0_clk0 1:pll0_clk0 2:pll1_clk0 3:pll1_clk1 4:pll2_clk0 5:pll2_clk1 6:pll3_clk0 7:pll4_clk0"]
    pub mod MUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: a change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock setting"]
pub mod CLOCK_CLK_TOP_UART9 {
    #[doc = "clock divider 0: divider by1 1: divider by 2 2 divider by 3 . . . 255: divider by 256"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock source selection 0:osc0_clk0 1:pll0_clk0 2:pll1_clk0 3:pll1_clk1 4:pll2_clk0 5:pll2_clk1 6:pll3_clk0 7:pll4_clk0"]
    pub mod MUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: a change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock setting"]
pub mod CLOCK_CLK_TOP_UART10 {
    #[doc = "clock divider 0: divider by1 1: divider by 2 2 divider by 3 . . . 255: divider by 256"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock source selection 0:osc0_clk0 1:pll0_clk0 2:pll1_clk0 3:pll1_clk1 4:pll2_clk0 5:pll2_clk1 6:pll3_clk0 7:pll4_clk0"]
    pub mod MUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: a change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock setting"]
pub mod CLOCK_CLK_TOP_UART11 {
    #[doc = "clock divider 0: divider by1 1: divider by 2 2 divider by 3 . . . 255: divider by 256"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock source selection 0:osc0_clk0 1:pll0_clk0 2:pll1_clk0 3:pll1_clk1 4:pll2_clk0 5:pll2_clk1 6:pll3_clk0 7:pll4_clk0"]
    pub mod MUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: a change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock setting"]
pub mod CLOCK_CLK_TOP_UART12 {
    #[doc = "clock divider 0: divider by1 1: divider by 2 2 divider by 3 . . . 255: divider by 256"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock source selection 0:osc0_clk0 1:pll0_clk0 2:pll1_clk0 3:pll1_clk1 4:pll2_clk0 5:pll2_clk1 6:pll3_clk0 7:pll4_clk0"]
    pub mod MUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: a change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock setting"]
pub mod CLOCK_CLK_TOP_UART13 {
    #[doc = "clock divider 0: divider by1 1: divider by 2 2 divider by 3 . . . 255: divider by 256"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock source selection 0:osc0_clk0 1:pll0_clk0 2:pll1_clk0 3:pll1_clk1 4:pll2_clk0 5:pll2_clk1 6:pll3_clk0 7:pll4_clk0"]
    pub mod MUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: a change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock setting"]
pub mod CLOCK_CLK_TOP_UART14 {
    #[doc = "clock divider 0: divider by1 1: divider by 2 2 divider by 3 . . . 255: divider by 256"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock source selection 0:osc0_clk0 1:pll0_clk0 2:pll1_clk0 3:pll1_clk1 4:pll2_clk0 5:pll2_clk1 6:pll3_clk0 7:pll4_clk0"]
    pub mod MUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: a change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock setting"]
pub mod CLOCK_CLK_TOP_UART15 {
    #[doc = "clock divider 0: divider by1 1: divider by 2 2 divider by 3 . . . 255: divider by 256"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock source selection 0:osc0_clk0 1:pll0_clk0 2:pll1_clk0 3:pll1_clk1 4:pll2_clk0 5:pll2_clk1 6:pll3_clk0 7:pll4_clk0"]
    pub mod MUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: a change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock setting"]
pub mod CLOCK_CLK_TOP_I2C0 {
    #[doc = "clock divider 0: divider by1 1: divider by 2 2 divider by 3 . . . 255: divider by 256"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock source selection 0:osc0_clk0 1:pll0_clk0 2:pll1_clk0 3:pll1_clk1 4:pll2_clk0 5:pll2_clk1 6:pll3_clk0 7:pll4_clk0"]
    pub mod MUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: a change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock setting"]
pub mod CLOCK_CLK_TOP_I2C1 {
    #[doc = "clock divider 0: divider by1 1: divider by 2 2 divider by 3 . . . 255: divider by 256"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock source selection 0:osc0_clk0 1:pll0_clk0 2:pll1_clk0 3:pll1_clk1 4:pll2_clk0 5:pll2_clk1 6:pll3_clk0 7:pll4_clk0"]
    pub mod MUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: a change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock setting"]
pub mod CLOCK_CLK_TOP_I2C2 {
    #[doc = "clock divider 0: divider by1 1: divider by 2 2 divider by 3 . . . 255: divider by 256"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock source selection 0:osc0_clk0 1:pll0_clk0 2:pll1_clk0 3:pll1_clk1 4:pll2_clk0 5:pll2_clk1 6:pll3_clk0 7:pll4_clk0"]
    pub mod MUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: a change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock setting"]
pub mod CLOCK_CLK_TOP_I2C3 {
    #[doc = "clock divider 0: divider by1 1: divider by 2 2 divider by 3 . . . 255: divider by 256"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock source selection 0:osc0_clk0 1:pll0_clk0 2:pll1_clk0 3:pll1_clk1 4:pll2_clk0 5:pll2_clk1 6:pll3_clk0 7:pll4_clk0"]
    pub mod MUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: a change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock setting"]
pub mod CLOCK_CLK_TOP_SPI0 {
    #[doc = "clock divider 0: divider by1 1: divider by 2 2 divider by 3 . . . 255: divider by 256"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock source selection 0:osc0_clk0 1:pll0_clk0 2:pll1_clk0 3:pll1_clk1 4:pll2_clk0 5:pll2_clk1 6:pll3_clk0 7:pll4_clk0"]
    pub mod MUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: a change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock setting"]
pub mod CLOCK_CLK_TOP_SPI1 {
    #[doc = "clock divider 0: divider by1 1: divider by 2 2 divider by 3 . . . 255: divider by 256"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock source selection 0:osc0_clk0 1:pll0_clk0 2:pll1_clk0 3:pll1_clk1 4:pll2_clk0 5:pll2_clk1 6:pll3_clk0 7:pll4_clk0"]
    pub mod MUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: a change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock setting"]
pub mod CLOCK_CLK_TOP_SPI2 {
    #[doc = "clock divider 0: divider by1 1: divider by 2 2 divider by 3 . . . 255: divider by 256"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock source selection 0:osc0_clk0 1:pll0_clk0 2:pll1_clk0 3:pll1_clk1 4:pll2_clk0 5:pll2_clk1 6:pll3_clk0 7:pll4_clk0"]
    pub mod MUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: a change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock setting"]
pub mod CLOCK_CLK_TOP_SPI3 {
    #[doc = "clock divider 0: divider by1 1: divider by 2 2 divider by 3 . . . 255: divider by 256"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock source selection 0:osc0_clk0 1:pll0_clk0 2:pll1_clk0 3:pll1_clk1 4:pll2_clk0 5:pll2_clk1 6:pll3_clk0 7:pll4_clk0"]
    pub mod MUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: a change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock setting"]
pub mod CLOCK_CLK_TOP_CAN0 {
    #[doc = "clock divider 0: divider by1 1: divider by 2 2 divider by 3 . . . 255: divider by 256"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock source selection 0:osc0_clk0 1:pll0_clk0 2:pll1_clk0 3:pll1_clk1 4:pll2_clk0 5:pll2_clk1 6:pll3_clk0 7:pll4_clk0"]
    pub mod MUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: a change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock setting"]
pub mod CLOCK_CLK_TOP_CAN1 {
    #[doc = "clock divider 0: divider by1 1: divider by 2 2 divider by 3 . . . 255: divider by 256"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock source selection 0:osc0_clk0 1:pll0_clk0 2:pll1_clk0 3:pll1_clk1 4:pll2_clk0 5:pll2_clk1 6:pll3_clk0 7:pll4_clk0"]
    pub mod MUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: a change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock setting"]
pub mod CLOCK_CLK_TOP_CAN2 {
    #[doc = "clock divider 0: divider by1 1: divider by 2 2 divider by 3 . . . 255: divider by 256"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock source selection 0:osc0_clk0 1:pll0_clk0 2:pll1_clk0 3:pll1_clk1 4:pll2_clk0 5:pll2_clk1 6:pll3_clk0 7:pll4_clk0"]
    pub mod MUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: a change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock setting"]
pub mod CLOCK_CLK_TOP_CAN3 {
    #[doc = "clock divider 0: divider by1 1: divider by 2 2 divider by 3 . . . 255: divider by 256"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock source selection 0:osc0_clk0 1:pll0_clk0 2:pll1_clk0 3:pll1_clk1 4:pll2_clk0 5:pll2_clk1 6:pll3_clk0 7:pll4_clk0"]
    pub mod MUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: a change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock setting"]
pub mod CLOCK_CLK_TOP_PTPC {
    #[doc = "clock divider 0: divider by1 1: divider by 2 2 divider by 3 . . . 255: divider by 256"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock source selection 0:osc0_clk0 1:pll0_clk0 2:pll1_clk0 3:pll1_clk1 4:pll2_clk0 5:pll2_clk1 6:pll3_clk0 7:pll4_clk0"]
    pub mod MUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: a change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock setting"]
pub mod CLOCK_CLK_TOP_ANA0 {
    #[doc = "clock divider 0: divider by1 1: divider by 2 2 divider by 3 . . . 255: divider by 256"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock source selection 0:osc0_clk0 1:pll0_clk0 2:pll1_clk0 3:pll1_clk1 4:pll2_clk0 5:pll2_clk1 6:pll3_clk0 7:pll4_clk0"]
    pub mod MUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: a change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock setting"]
pub mod CLOCK_CLK_TOP_ANA1 {
    #[doc = "clock divider 0: divider by1 1: divider by 2 2 divider by 3 . . . 255: divider by 256"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock source selection 0:osc0_clk0 1:pll0_clk0 2:pll1_clk0 3:pll1_clk1 4:pll2_clk0 5:pll2_clk1 6:pll3_clk0 7:pll4_clk0"]
    pub mod MUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: a change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock setting"]
pub mod CLOCK_CLK_TOP_ANA2 {
    #[doc = "clock divider 0: divider by1 1: divider by 2 2 divider by 3 . . . 255: divider by 256"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock source selection 0:osc0_clk0 1:pll0_clk0 2:pll1_clk0 3:pll1_clk1 4:pll2_clk0 5:pll2_clk1 6:pll3_clk0 7:pll4_clk0"]
    pub mod MUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: a change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock setting"]
pub mod CLOCK_CLK_TOP_AUD0 {
    #[doc = "clock divider 0: divider by1 1: divider by 2 2 divider by 3 . . . 255: divider by 256"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock source selection 0:osc0_clk0 1:pll0_clk0 2:pll1_clk0 3:pll1_clk1 4:pll2_clk0 5:pll2_clk1 6:pll3_clk0 7:pll4_clk0"]
    pub mod MUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: a change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock setting"]
pub mod CLOCK_CLK_TOP_AUD1 {
    #[doc = "clock divider 0: divider by1 1: divider by 2 2 divider by 3 . . . 255: divider by 256"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock source selection 0:osc0_clk0 1:pll0_clk0 2:pll1_clk0 3:pll1_clk1 4:pll2_clk0 5:pll2_clk1 6:pll3_clk0 7:pll4_clk0"]
    pub mod MUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: a change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock setting"]
pub mod CLOCK_CLK_TOP_AUD2 {
    #[doc = "clock divider 0: divider by1 1: divider by 2 2 divider by 3 . . . 255: divider by 256"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock source selection 0:osc0_clk0 1:pll0_clk0 2:pll1_clk0 3:pll1_clk1 4:pll2_clk0 5:pll2_clk1 6:pll3_clk0 7:pll4_clk0"]
    pub mod MUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: a change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock setting"]
pub mod CLOCK_CLK_TOP_LCDC {
    #[doc = "clock divider 0: divider by1 1: divider by 2 2 divider by 3 . . . 255: divider by 256"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock source selection 0:osc0_clk0 1:pll0_clk0 2:pll1_clk0 3:pll1_clk1 4:pll2_clk0 5:pll2_clk1 6:pll3_clk0 7:pll4_clk0"]
    pub mod MUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: a change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock setting"]
pub mod CLOCK_CLK_TOP_CAM0 {
    #[doc = "clock divider 0: divider by1 1: divider by 2 2 divider by 3 . . . 255: divider by 256"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock source selection 0:osc0_clk0 1:pll0_clk0 2:pll1_clk0 3:pll1_clk1 4:pll2_clk0 5:pll2_clk1 6:pll3_clk0 7:pll4_clk0"]
    pub mod MUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: a change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock setting"]
pub mod CLOCK_CLK_TOP_CAM1 {
    #[doc = "clock divider 0: divider by1 1: divider by 2 2 divider by 3 . . . 255: divider by 256"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock source selection 0:osc0_clk0 1:pll0_clk0 2:pll1_clk0 3:pll1_clk1 4:pll2_clk0 5:pll2_clk1 6:pll3_clk0 7:pll4_clk0"]
    pub mod MUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: a change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock setting"]
pub mod CLOCK_CLK_TOP_ENET0 {
    #[doc = "clock divider 0: divider by1 1: divider by 2 2 divider by 3 . . . 255: divider by 256"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock source selection 0:osc0_clk0 1:pll0_clk0 2:pll1_clk0 3:pll1_clk1 4:pll2_clk0 5:pll2_clk1 6:pll3_clk0 7:pll4_clk0"]
    pub mod MUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: a change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock setting"]
pub mod CLOCK_CLK_TOP_ENET1 {
    #[doc = "clock divider 0: divider by1 1: divider by 2 2 divider by 3 . . . 255: divider by 256"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock source selection 0:osc0_clk0 1:pll0_clk0 2:pll1_clk0 3:pll1_clk1 4:pll2_clk0 5:pll2_clk1 6:pll3_clk0 7:pll4_clk0"]
    pub mod MUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: a change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock setting"]
pub mod CLOCK_CLK_TOP_PTP0 {
    #[doc = "clock divider 0: divider by1 1: divider by 2 2 divider by 3 . . . 255: divider by 256"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock source selection 0:osc0_clk0 1:pll0_clk0 2:pll1_clk0 3:pll1_clk1 4:pll2_clk0 5:pll2_clk1 6:pll3_clk0 7:pll4_clk0"]
    pub mod MUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: a change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock setting"]
pub mod CLOCK_CLK_TOP_PTP1 {
    #[doc = "clock divider 0: divider by1 1: divider by 2 2 divider by 3 . . . 255: divider by 256"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock source selection 0:osc0_clk0 1:pll0_clk0 2:pll1_clk0 3:pll1_clk1 4:pll2_clk0 5:pll2_clk1 6:pll3_clk0 7:pll4_clk0"]
    pub mod MUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: a change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock setting"]
pub mod CLOCK_CLK_TOP_REF0 {
    #[doc = "clock divider 0: divider by1 1: divider by 2 2 divider by 3 . . . 255: divider by 256"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock source selection 0:osc0_clk0 1:pll0_clk0 2:pll1_clk0 3:pll1_clk1 4:pll2_clk0 5:pll2_clk1 6:pll3_clk0 7:pll4_clk0"]
    pub mod MUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: a change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock setting"]
pub mod CLOCK_CLK_TOP_REF1 {
    #[doc = "clock divider 0: divider by1 1: divider by 2 2 divider by 3 . . . 255: divider by 256"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock source selection 0:osc0_clk0 1:pll0_clk0 2:pll1_clk0 3:pll1_clk1 4:pll2_clk0 5:pll2_clk1 6:pll3_clk0 7:pll4_clk0"]
    pub mod MUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: a change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock setting"]
pub mod CLOCK_CLK_TOP_NTMR0 {
    #[doc = "clock divider 0: divider by1 1: divider by 2 2 divider by 3 . . . 255: divider by 256"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock source selection 0:osc0_clk0 1:pll0_clk0 2:pll1_clk0 3:pll1_clk1 4:pll2_clk0 5:pll2_clk1 6:pll3_clk0 7:pll4_clk0"]
    pub mod MUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: a change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock setting"]
pub mod CLOCK_CLK_TOP_NTMR1 {
    #[doc = "clock divider 0: divider by1 1: divider by 2 2 divider by 3 . . . 255: divider by 256"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock source selection 0:osc0_clk0 1:pll0_clk0 2:pll1_clk0 3:pll1_clk1 4:pll2_clk0 5:pll2_clk1 6:pll3_clk0 7:pll4_clk0"]
    pub mod MUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: a change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock setting"]
pub mod CLOCK_CLK_TOP_SDXC0 {
    #[doc = "clock divider 0: divider by1 1: divider by 2 2 divider by 3 . . . 255: divider by 256"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock source selection 0:osc0_clk0 1:pll0_clk0 2:pll1_clk0 3:pll1_clk1 4:pll2_clk0 5:pll2_clk1 6:pll3_clk0 7:pll4_clk0"]
    pub mod MUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: a change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock setting"]
pub mod CLOCK_CLK_TOP_SDXC1 {
    #[doc = "clock divider 0: divider by1 1: divider by 2 2 divider by 3 . . . 255: divider by 256"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock source selection 0:osc0_clk0 1:pll0_clk0 2:pll1_clk0 3:pll1_clk1 4:pll2_clk0 5:pll2_clk1 6:pll3_clk0 7:pll4_clk0"]
    pub mod MUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: a change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock setting"]
pub mod ADCCLK_CLK_TOP_ADC0 {
    #[doc = "clock source selection 0: ahb clock 1: adc clock 0 2: adc clock 1 3: adc clock 2"]
    pub mod MUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: a change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock setting"]
pub mod ADCCLK_CLK_TOP_ADC1 {
    #[doc = "clock source selection 0: ahb clock 1: adc clock 0 2: adc clock 1 3: adc clock 2"]
    pub mod MUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: a change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock setting"]
pub mod ADCCLK_CLK_TOP_ADC2 {
    #[doc = "clock source selection 0: ahb clock 1: adc clock 0 2: adc clock 1 3: adc clock 2"]
    pub mod MUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: a change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock setting"]
pub mod ADCCLK_CLK_TOP_ADC3 {
    #[doc = "clock source selection 0: ahb clock 1: adc clock 0 2: adc clock 1 3: adc clock 2"]
    pub mod MUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: a change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock setting"]
pub mod I2SCLK_CLK_TOP_I2S0 {
    #[doc = "clock source selection 0: ahb clock 1: i2s clock 0 2: i2s clock 1 3: i2s clock 2"]
    pub mod MUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: a change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock setting"]
pub mod I2SCLK_CLK_TOP_I2S1 {
    #[doc = "clock source selection 0: ahb clock 1: i2s clock 0 2: i2s clock 1 3: i2s clock 2"]
    pub mod MUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: a change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock setting"]
pub mod I2SCLK_CLK_TOP_I2S2 {
    #[doc = "clock source selection 0: ahb clock 1: i2s clock 0 2: i2s clock 1 3: i2s clock 2"]
    pub mod MUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: a change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock setting"]
pub mod I2SCLK_CLK_TOP_I2S3 {
    #[doc = "clock source selection 0: ahb clock 1: i2s clock 0 2: i2s clock 1 3: i2s clock 2"]
    pub mod MUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local busy 0: a change is pending for current node 1: current node is changing status"]
    pub mod LOC_BUSY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "global busy 0: no changes pending to any clock 1: any of nodes is changing status"]
    pub mod GLB_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock senario"]
pub mod GLOBAL00 {
    #[doc = "global clock override request bit0: override to preset0 bit1: override to preset1 bit2: override to preset2 bit3: override to preset3"]
    pub mod PRESET {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock measure and monitor control"]
pub mod MONITOR_SLICE0_CONTROL {
    #[doc = "clock measurement selection"]
    pub mod SELECTION {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "refrence clock selection, 0: 32k 1: 24M"]
    pub mod REFERENCE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "measurement accuracy, 0: resolution is 1kHz 1: resolution is 1Hz"]
    pub mod ACCURACY {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "work mode, 0: register value will be compared to measurement 1: upper and lower value will be recordered in register"]
    pub mod MODE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "start measurement"]
    pub mod START {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock frequency lower than lower limit"]
    pub mod LOW {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock frequency higher than upper limit"]
    pub mod HIGH {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "output divider"]
    pub mod DIV {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "enable clock output"]
    pub mod OUTEN {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "divider is applying new setting"]
    pub mod DIV_BUSY {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "result is ready for read 0: not ready 1: result is ready"]
    pub mod VALID {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock measure result"]
pub mod MONITOR_SLICE0_CURRENT {
    #[doc = "self updating measure result"]
    pub mod FREQUENCY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock lower limit"]
pub mod MONITOR_SLICE0_LOW_LIMIT {
    #[doc = "lower frequency"]
    pub mod FREQUENCY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock upper limit"]
pub mod MONITOR_SLICE0_HIGH_LIMIT {
    #[doc = "upper frequency"]
    pub mod FREQUENCY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock measure and monitor control"]
pub mod MONITOR_SLICE1_CONTROL {
    #[doc = "clock measurement selection"]
    pub mod SELECTION {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "refrence clock selection, 0: 32k 1: 24M"]
    pub mod REFERENCE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "measurement accuracy, 0: resolution is 1kHz 1: resolution is 1Hz"]
    pub mod ACCURACY {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "work mode, 0: register value will be compared to measurement 1: upper and lower value will be recordered in register"]
    pub mod MODE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "start measurement"]
    pub mod START {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock frequency lower than lower limit"]
    pub mod LOW {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock frequency higher than upper limit"]
    pub mod HIGH {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "output divider"]
    pub mod DIV {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "enable clock output"]
    pub mod OUTEN {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "divider is applying new setting"]
    pub mod DIV_BUSY {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "result is ready for read 0: not ready 1: result is ready"]
    pub mod VALID {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock measure result"]
pub mod MONITOR_SLICE1_CURRENT {
    #[doc = "self updating measure result"]
    pub mod FREQUENCY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock lower limit"]
pub mod MONITOR_SLICE1_LOW_LIMIT {
    #[doc = "lower frequency"]
    pub mod FREQUENCY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock upper limit"]
pub mod MONITOR_SLICE1_HIGH_LIMIT {
    #[doc = "upper frequency"]
    pub mod FREQUENCY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock measure and monitor control"]
pub mod MONITOR_SLICE2_CONTROL {
    #[doc = "clock measurement selection"]
    pub mod SELECTION {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "refrence clock selection, 0: 32k 1: 24M"]
    pub mod REFERENCE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "measurement accuracy, 0: resolution is 1kHz 1: resolution is 1Hz"]
    pub mod ACCURACY {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "work mode, 0: register value will be compared to measurement 1: upper and lower value will be recordered in register"]
    pub mod MODE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "start measurement"]
    pub mod START {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock frequency lower than lower limit"]
    pub mod LOW {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock frequency higher than upper limit"]
    pub mod HIGH {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "output divider"]
    pub mod DIV {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "enable clock output"]
    pub mod OUTEN {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "divider is applying new setting"]
    pub mod DIV_BUSY {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "result is ready for read 0: not ready 1: result is ready"]
    pub mod VALID {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock measure result"]
pub mod MONITOR_SLICE2_CURRENT {
    #[doc = "self updating measure result"]
    pub mod FREQUENCY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock lower limit"]
pub mod MONITOR_SLICE2_LOW_LIMIT {
    #[doc = "lower frequency"]
    pub mod FREQUENCY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock upper limit"]
pub mod MONITOR_SLICE2_HIGH_LIMIT {
    #[doc = "upper frequency"]
    pub mod FREQUENCY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock measure and monitor control"]
pub mod MONITOR_SLICE3_CONTROL {
    #[doc = "clock measurement selection"]
    pub mod SELECTION {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "refrence clock selection, 0: 32k 1: 24M"]
    pub mod REFERENCE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "measurement accuracy, 0: resolution is 1kHz 1: resolution is 1Hz"]
    pub mod ACCURACY {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "work mode, 0: register value will be compared to measurement 1: upper and lower value will be recordered in register"]
    pub mod MODE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "start measurement"]
    pub mod START {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock frequency lower than lower limit"]
    pub mod LOW {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "clock frequency higher than upper limit"]
    pub mod HIGH {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "output divider"]
    pub mod DIV {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "enable clock output"]
    pub mod OUTEN {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "divider is applying new setting"]
    pub mod DIV_BUSY {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "result is ready for read 0: not ready 1: result is ready"]
    pub mod VALID {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock measure result"]
pub mod MONITOR_SLICE3_CURRENT {
    #[doc = "self updating measure result"]
    pub mod FREQUENCY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock lower limit"]
pub mod MONITOR_SLICE3_LOW_LIMIT {
    #[doc = "lower frequency"]
    pub mod FREQUENCY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock upper limit"]
pub mod MONITOR_SLICE3_HIGH_LIMIT {
    #[doc = "upper frequency"]
    pub mod FREQUENCY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CPU_CPU0_LP {
    #[doc = "Low power mode, system behavior after WFI 00: CPU clock stop after WFI 01: System enter low power mode after WFI 10: Keep running after WFI 11: reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CPU0 reset flag, indicate a reset event got active, write 1 to clear this bit 0: CPU0 sleep not happened 1: CPU0 sleep happened"]
    pub mod RESET_FLAG {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CPU0 sleep flag, indicate a sleep event got active, write 1 to clear this bit 0: CPU0 sleep not happened 1: CPU0 sleep happened"]
    pub mod SLEEP_FLAG {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CPU0 wakeup flag, indicate a wakeup event got active, write 1 to clear this bit 0: CPU0 wakeup not happened 1: CPU0 wakeup happened"]
    pub mod WAKE_FLAG {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CPU0 is executing 0: CPU0 is not executing 1: CPU0 is executing"]
    pub mod EXEC {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CPU0 is waking up 0: CPU0 wake up not asserted 1: CPU0 wake up asserted"]
    pub mod WAKE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "halt request for CPU0, 0: CPU0 will start to execute after reset or receive wakeup request 1: CPU0 will not start after reset, or wakeup after WFI"]
    pub mod HALT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CPU0 wake up counter, counter saturated at 255, write 0x00 to clear"]
    pub mod WAKE_CNT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CPU_CPU0_LOCK {
    #[doc = "Lock bit for CPU_LOCK"]
    pub mod LOCK {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Lock bit for CPU_DATA0 to CPU_DATA13, once set, this bit will not clear untile next reset"]
    pub mod GPR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x3fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CPU_CPU0_GPR0 {
    #[doc = "register for software to handle resume, can save resume address or status"]
    pub mod GPR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CPU_CPU0_GPR1 {
    #[doc = "register for software to handle resume, can save resume address or status"]
    pub mod GPR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CPU_CPU0_GPR2 {
    #[doc = "register for software to handle resume, can save resume address or status"]
    pub mod GPR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CPU_CPU0_GPR3 {
    #[doc = "register for software to handle resume, can save resume address or status"]
    pub mod GPR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CPU_CPU0_GPR4 {
    #[doc = "register for software to handle resume, can save resume address or status"]
    pub mod GPR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CPU_CPU0_GPR5 {
    #[doc = "register for software to handle resume, can save resume address or status"]
    pub mod GPR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CPU_CPU0_GPR6 {
    #[doc = "register for software to handle resume, can save resume address or status"]
    pub mod GPR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CPU_CPU0_GPR7 {
    #[doc = "register for software to handle resume, can save resume address or status"]
    pub mod GPR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CPU_CPU0_GPR8 {
    #[doc = "register for software to handle resume, can save resume address or status"]
    pub mod GPR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CPU_CPU0_GPR9 {
    #[doc = "register for software to handle resume, can save resume address or status"]
    pub mod GPR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CPU_CPU0_GPR10 {
    #[doc = "register for software to handle resume, can save resume address or status"]
    pub mod GPR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CPU_CPU0_GPR11 {
    #[doc = "register for software to handle resume, can save resume address or status"]
    pub mod GPR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CPU_CPU0_GPR12 {
    #[doc = "register for software to handle resume, can save resume address or status"]
    pub mod GPR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CPU_CPU0_GPR13 {
    #[doc = "register for software to handle resume, can save resume address or status"]
    pub mod GPR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CPU_CPU0_STATUS0 {
    #[doc = "IRQ values"]
    pub mod STATUS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CPU_CPU0_STATUS1 {
    #[doc = "IRQ values"]
    pub mod STATUS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CPU_CPU0_STATUS2 {
    #[doc = "IRQ values"]
    pub mod STATUS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CPU_CPU0_STATUS3 {
    #[doc = "IRQ values"]
    pub mod STATUS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CPU_CPU0_STATUS4 {
    #[doc = "IRQ values"]
    pub mod STATUS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CPU_CPU0_STATUS5 {
    #[doc = "IRQ values"]
    pub mod STATUS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CPU_CPU0_STATUS6 {
    #[doc = "IRQ values"]
    pub mod STATUS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CPU_CPU0_STATUS7 {
    #[doc = "IRQ values"]
    pub mod STATUS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CPU_CPU0_ENABLE0 {
    #[doc = "IRQ wakeup enable"]
    pub mod ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CPU_CPU0_ENABLE1 {
    #[doc = "IRQ wakeup enable"]
    pub mod ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CPU_CPU0_ENABLE2 {
    #[doc = "IRQ wakeup enable"]
    pub mod ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CPU_CPU0_ENABLE3 {
    #[doc = "IRQ wakeup enable"]
    pub mod ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CPU_CPU0_ENABLE4 {
    #[doc = "IRQ wakeup enable"]
    pub mod ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CPU_CPU0_ENABLE5 {
    #[doc = "IRQ wakeup enable"]
    pub mod ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CPU_CPU0_ENABLE6 {
    #[doc = "IRQ wakeup enable"]
    pub mod ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CPU_CPU0_ENABLE7 {
    #[doc = "IRQ wakeup enable"]
    pub mod ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CPU_CPU1_LP {
    #[doc = "Low power mode, system behavior after WFI 00: CPU clock stop after WFI 01: System enter low power mode after WFI 10: Keep running after WFI 11: reserved"]
    pub mod MODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CPU0 reset flag, indicate a reset event got active, write 1 to clear this bit 0: CPU0 sleep not happened 1: CPU0 sleep happened"]
    pub mod RESET_FLAG {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CPU0 sleep flag, indicate a sleep event got active, write 1 to clear this bit 0: CPU0 sleep not happened 1: CPU0 sleep happened"]
    pub mod SLEEP_FLAG {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CPU0 wakeup flag, indicate a wakeup event got active, write 1 to clear this bit 0: CPU0 wakeup not happened 1: CPU0 wakeup happened"]
    pub mod WAKE_FLAG {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CPU0 is executing 0: CPU0 is not executing 1: CPU0 is executing"]
    pub mod EXEC {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CPU0 is waking up 0: CPU0 wake up not asserted 1: CPU0 wake up asserted"]
    pub mod WAKE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "halt request for CPU0, 0: CPU0 will start to execute after reset or receive wakeup request 1: CPU0 will not start after reset, or wakeup after WFI"]
    pub mod HALT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CPU0 wake up counter, counter saturated at 255, write 0x00 to clear"]
    pub mod WAKE_CNT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CPU_CPU1_LOCK {
    #[doc = "Lock bit for CPU_LOCK"]
    pub mod LOCK {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Lock bit for CPU_DATA0 to CPU_DATA13, once set, this bit will not clear untile next reset"]
    pub mod GPR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x3fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CPU_CPU1_GPR0 {
    #[doc = "register for software to handle resume, can save resume address or status"]
    pub mod GPR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CPU_CPU1_GPR1 {
    #[doc = "register for software to handle resume, can save resume address or status"]
    pub mod GPR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CPU_CPU1_GPR2 {
    #[doc = "register for software to handle resume, can save resume address or status"]
    pub mod GPR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CPU_CPU1_GPR3 {
    #[doc = "register for software to handle resume, can save resume address or status"]
    pub mod GPR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CPU_CPU1_GPR4 {
    #[doc = "register for software to handle resume, can save resume address or status"]
    pub mod GPR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CPU_CPU1_GPR5 {
    #[doc = "register for software to handle resume, can save resume address or status"]
    pub mod GPR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CPU_CPU1_GPR6 {
    #[doc = "register for software to handle resume, can save resume address or status"]
    pub mod GPR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CPU_CPU1_GPR7 {
    #[doc = "register for software to handle resume, can save resume address or status"]
    pub mod GPR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CPU_CPU1_GPR8 {
    #[doc = "register for software to handle resume, can save resume address or status"]
    pub mod GPR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CPU_CPU1_GPR9 {
    #[doc = "register for software to handle resume, can save resume address or status"]
    pub mod GPR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CPU_CPU1_GPR10 {
    #[doc = "register for software to handle resume, can save resume address or status"]
    pub mod GPR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CPU_CPU1_GPR11 {
    #[doc = "register for software to handle resume, can save resume address or status"]
    pub mod GPR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CPU_CPU1_GPR12 {
    #[doc = "register for software to handle resume, can save resume address or status"]
    pub mod GPR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CPU_CPU1_GPR13 {
    #[doc = "register for software to handle resume, can save resume address or status"]
    pub mod GPR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CPU_CPU1_STATUS0 {
    #[doc = "IRQ values"]
    pub mod STATUS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CPU_CPU1_STATUS1 {
    #[doc = "IRQ values"]
    pub mod STATUS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CPU_CPU1_STATUS2 {
    #[doc = "IRQ values"]
    pub mod STATUS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CPU_CPU1_STATUS3 {
    #[doc = "IRQ values"]
    pub mod STATUS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CPU_CPU1_STATUS4 {
    #[doc = "IRQ values"]
    pub mod STATUS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CPU_CPU1_STATUS5 {
    #[doc = "IRQ values"]
    pub mod STATUS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CPU_CPU1_STATUS6 {
    #[doc = "IRQ values"]
    pub mod STATUS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CPU_CPU1_STATUS7 {
    #[doc = "IRQ values"]
    pub mod STATUS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CPU_CPU1_ENABLE0 {
    #[doc = "IRQ wakeup enable"]
    pub mod ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CPU_CPU1_ENABLE1 {
    #[doc = "IRQ wakeup enable"]
    pub mod ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CPU_CPU1_ENABLE2 {
    #[doc = "IRQ wakeup enable"]
    pub mod ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CPU_CPU1_ENABLE3 {
    #[doc = "IRQ wakeup enable"]
    pub mod ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CPU_CPU1_ENABLE4 {
    #[doc = "IRQ wakeup enable"]
    pub mod ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CPU_CPU1_ENABLE5 {
    #[doc = "IRQ wakeup enable"]
    pub mod ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CPU_CPU1_ENABLE6 {
    #[doc = "IRQ wakeup enable"]
    pub mod ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CPU_CPU1_ENABLE7 {
    #[doc = "IRQ wakeup enable"]
    pub mod ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
