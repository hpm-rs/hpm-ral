#[doc = "LCDC"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Control Register"]
    pub CTRL: crate::RWRegister<u32>,
    #[doc = "Background Color Register"]
    pub BGND_CL: crate::RWRegister<u32>,
    #[doc = "Display Window Size Register"]
    pub DISP_WN_SIZE: crate::RWRegister<u32>,
    #[doc = "HSYNC Config Register"]
    pub HSYNC_PARA: crate::RWRegister<u32>,
    #[doc = "VSYNC Config Register"]
    pub VSYNC_PARA: crate::RWRegister<u32>,
    #[doc = "DMA Status Register"]
    pub DMA_ST: crate::RWRegister<u32>,
    #[doc = "Status Register"]
    pub ST: crate::RWRegister<u32>,
    #[doc = "Interrupt Enable Register"]
    pub INT_EN: crate::RWRegister<u32>,
    #[doc = "TX FIFO Register"]
    pub TXFIFO: crate::RWRegister<u32>,
    _reserved0: [u8; 0x01dc],
    #[doc = "Layer Control Register"]
    pub LAYER_0_LAYCTRL: crate::RWRegister<u32>,
    #[doc = "Layer Alpha Register"]
    pub LAYER_0_ALPHAS: crate::RWRegister<u32>,
    #[doc = "Layer Size Register"]
    pub LAYER_0_LAYSIZE: crate::RWRegister<u32>,
    #[doc = "Layer Position Register"]
    pub LAYER_0_LAYPOS: crate::RWRegister<u32>,
    #[doc = "Layer Buffer Pointer Register"]
    pub LAYER_0_START0: crate::RWRegister<u32>,
    _reserved1: [u8; 0x04],
    #[doc = "Layer Bus Config Register"]
    pub LAYER_0_LINECFG: crate::RWRegister<u32>,
    #[doc = "Layer Background Color Register"]
    pub LAYER_0_BG_CL: crate::RWRegister<u32>,
    #[doc = "Layer Color Space Conversion Config Register 0"]
    pub LAYER_0_CSC_COEF0: crate::RWRegister<u32>,
    #[doc = "Layer Color Space Conversion Config Register 1"]
    pub LAYER_0_CSC_COEF1: crate::RWRegister<u32>,
    #[doc = "Layer Color Space Conversion Config Register 2"]
    pub LAYER_0_CSC_COEF2: crate::RWRegister<u32>,
    _reserved2: [u8; 0x14],
    #[doc = "Layer Control Register"]
    pub LAYER_1_LAYCTRL: crate::RWRegister<u32>,
    #[doc = "Layer Alpha Register"]
    pub LAYER_1_ALPHAS: crate::RWRegister<u32>,
    #[doc = "Layer Size Register"]
    pub LAYER_1_LAYSIZE: crate::RWRegister<u32>,
    #[doc = "Layer Position Register"]
    pub LAYER_1_LAYPOS: crate::RWRegister<u32>,
    #[doc = "Layer Buffer Pointer Register"]
    pub LAYER_1_START0: crate::RWRegister<u32>,
    _reserved3: [u8; 0x04],
    #[doc = "Layer Bus Config Register"]
    pub LAYER_1_LINECFG: crate::RWRegister<u32>,
    #[doc = "Layer Background Color Register"]
    pub LAYER_1_BG_CL: crate::RWRegister<u32>,
    #[doc = "Layer Color Space Conversion Config Register 0"]
    pub LAYER_1_CSC_COEF0: crate::RWRegister<u32>,
    #[doc = "Layer Color Space Conversion Config Register 1"]
    pub LAYER_1_CSC_COEF1: crate::RWRegister<u32>,
    #[doc = "Layer Color Space Conversion Config Register 2"]
    pub LAYER_1_CSC_COEF2: crate::RWRegister<u32>,
    _reserved4: [u8; 0x14],
    #[doc = "Layer Control Register"]
    pub LAYER_2_LAYCTRL: crate::RWRegister<u32>,
    #[doc = "Layer Alpha Register"]
    pub LAYER_2_ALPHAS: crate::RWRegister<u32>,
    #[doc = "Layer Size Register"]
    pub LAYER_2_LAYSIZE: crate::RWRegister<u32>,
    #[doc = "Layer Position Register"]
    pub LAYER_2_LAYPOS: crate::RWRegister<u32>,
    #[doc = "Layer Buffer Pointer Register"]
    pub LAYER_2_START0: crate::RWRegister<u32>,
    _reserved5: [u8; 0x04],
    #[doc = "Layer Bus Config Register"]
    pub LAYER_2_LINECFG: crate::RWRegister<u32>,
    #[doc = "Layer Background Color Register"]
    pub LAYER_2_BG_CL: crate::RWRegister<u32>,
    #[doc = "Layer Color Space Conversion Config Register 0"]
    pub LAYER_2_CSC_COEF0: crate::RWRegister<u32>,
    #[doc = "Layer Color Space Conversion Config Register 1"]
    pub LAYER_2_CSC_COEF1: crate::RWRegister<u32>,
    #[doc = "Layer Color Space Conversion Config Register 2"]
    pub LAYER_2_CSC_COEF2: crate::RWRegister<u32>,
    _reserved6: [u8; 0x14],
    #[doc = "Layer Control Register"]
    pub LAYER_3_LAYCTRL: crate::RWRegister<u32>,
    #[doc = "Layer Alpha Register"]
    pub LAYER_3_ALPHAS: crate::RWRegister<u32>,
    #[doc = "Layer Size Register"]
    pub LAYER_3_LAYSIZE: crate::RWRegister<u32>,
    #[doc = "Layer Position Register"]
    pub LAYER_3_LAYPOS: crate::RWRegister<u32>,
    #[doc = "Layer Buffer Pointer Register"]
    pub LAYER_3_START0: crate::RWRegister<u32>,
    _reserved7: [u8; 0x04],
    #[doc = "Layer Bus Config Register"]
    pub LAYER_3_LINECFG: crate::RWRegister<u32>,
    #[doc = "Layer Background Color Register"]
    pub LAYER_3_BG_CL: crate::RWRegister<u32>,
    #[doc = "Layer Color Space Conversion Config Register 0"]
    pub LAYER_3_CSC_COEF0: crate::RWRegister<u32>,
    #[doc = "Layer Color Space Conversion Config Register 1"]
    pub LAYER_3_CSC_COEF1: crate::RWRegister<u32>,
    #[doc = "Layer Color Space Conversion Config Register 2"]
    pub LAYER_3_CSC_COEF2: crate::RWRegister<u32>,
    _reserved8: [u8; 0x14],
    #[doc = "Layer Control Register"]
    pub LAYER_4_LAYCTRL: crate::RWRegister<u32>,
    #[doc = "Layer Alpha Register"]
    pub LAYER_4_ALPHAS: crate::RWRegister<u32>,
    #[doc = "Layer Size Register"]
    pub LAYER_4_LAYSIZE: crate::RWRegister<u32>,
    #[doc = "Layer Position Register"]
    pub LAYER_4_LAYPOS: crate::RWRegister<u32>,
    #[doc = "Layer Buffer Pointer Register"]
    pub LAYER_4_START0: crate::RWRegister<u32>,
    _reserved9: [u8; 0x04],
    #[doc = "Layer Bus Config Register"]
    pub LAYER_4_LINECFG: crate::RWRegister<u32>,
    #[doc = "Layer Background Color Register"]
    pub LAYER_4_BG_CL: crate::RWRegister<u32>,
    #[doc = "Layer Color Space Conversion Config Register 0"]
    pub LAYER_4_CSC_COEF0: crate::RWRegister<u32>,
    #[doc = "Layer Color Space Conversion Config Register 1"]
    pub LAYER_4_CSC_COEF1: crate::RWRegister<u32>,
    #[doc = "Layer Color Space Conversion Config Register 2"]
    pub LAYER_4_CSC_COEF2: crate::RWRegister<u32>,
    _reserved10: [u8; 0x14],
    #[doc = "Layer Control Register"]
    pub LAYER_5_LAYCTRL: crate::RWRegister<u32>,
    #[doc = "Layer Alpha Register"]
    pub LAYER_5_ALPHAS: crate::RWRegister<u32>,
    #[doc = "Layer Size Register"]
    pub LAYER_5_LAYSIZE: crate::RWRegister<u32>,
    #[doc = "Layer Position Register"]
    pub LAYER_5_LAYPOS: crate::RWRegister<u32>,
    #[doc = "Layer Buffer Pointer Register"]
    pub LAYER_5_START0: crate::RWRegister<u32>,
    _reserved11: [u8; 0x04],
    #[doc = "Layer Bus Config Register"]
    pub LAYER_5_LINECFG: crate::RWRegister<u32>,
    #[doc = "Layer Background Color Register"]
    pub LAYER_5_BG_CL: crate::RWRegister<u32>,
    #[doc = "Layer Color Space Conversion Config Register 0"]
    pub LAYER_5_CSC_COEF0: crate::RWRegister<u32>,
    #[doc = "Layer Color Space Conversion Config Register 1"]
    pub LAYER_5_CSC_COEF1: crate::RWRegister<u32>,
    #[doc = "Layer Color Space Conversion Config Register 2"]
    pub LAYER_5_CSC_COEF2: crate::RWRegister<u32>,
    _reserved12: [u8; 0x14],
    #[doc = "Layer Control Register"]
    pub LAYER_6_LAYCTRL: crate::RWRegister<u32>,
    #[doc = "Layer Alpha Register"]
    pub LAYER_6_ALPHAS: crate::RWRegister<u32>,
    #[doc = "Layer Size Register"]
    pub LAYER_6_LAYSIZE: crate::RWRegister<u32>,
    #[doc = "Layer Position Register"]
    pub LAYER_6_LAYPOS: crate::RWRegister<u32>,
    #[doc = "Layer Buffer Pointer Register"]
    pub LAYER_6_START0: crate::RWRegister<u32>,
    _reserved13: [u8; 0x04],
    #[doc = "Layer Bus Config Register"]
    pub LAYER_6_LINECFG: crate::RWRegister<u32>,
    #[doc = "Layer Background Color Register"]
    pub LAYER_6_BG_CL: crate::RWRegister<u32>,
    #[doc = "Layer Color Space Conversion Config Register 0"]
    pub LAYER_6_CSC_COEF0: crate::RWRegister<u32>,
    #[doc = "Layer Color Space Conversion Config Register 1"]
    pub LAYER_6_CSC_COEF1: crate::RWRegister<u32>,
    #[doc = "Layer Color Space Conversion Config Register 2"]
    pub LAYER_6_CSC_COEF2: crate::RWRegister<u32>,
    _reserved14: [u8; 0x14],
    #[doc = "Layer Control Register"]
    pub LAYER_7_LAYCTRL: crate::RWRegister<u32>,
    #[doc = "Layer Alpha Register"]
    pub LAYER_7_ALPHAS: crate::RWRegister<u32>,
    #[doc = "Layer Size Register"]
    pub LAYER_7_LAYSIZE: crate::RWRegister<u32>,
    #[doc = "Layer Position Register"]
    pub LAYER_7_LAYPOS: crate::RWRegister<u32>,
    #[doc = "Layer Buffer Pointer Register"]
    pub LAYER_7_START0: crate::RWRegister<u32>,
    _reserved15: [u8; 0x04],
    #[doc = "Layer Bus Config Register"]
    pub LAYER_7_LINECFG: crate::RWRegister<u32>,
    #[doc = "Layer Background Color Register"]
    pub LAYER_7_BG_CL: crate::RWRegister<u32>,
    #[doc = "Layer Color Space Conversion Config Register 0"]
    pub LAYER_7_CSC_COEF0: crate::RWRegister<u32>,
    #[doc = "Layer Color Space Conversion Config Register 1"]
    pub LAYER_7_CSC_COEF1: crate::RWRegister<u32>,
    #[doc = "Layer Color Space Conversion Config Register 2"]
    pub LAYER_7_CSC_COEF2: crate::RWRegister<u32>,
    _reserved16: [u8; 0x14],
    #[doc = "Clut Load Control Register"]
    pub CLUT_LOAD: crate::RWRegister<u32>,
}
#[doc = "Control Register"]
pub mod CTRL {
    #[doc = "Polarity of HSYNC 0b - HSYNC signal active HIGH 1b - HSYNC signal active LOW"]
    pub mod INV_HSYNC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Polarity of VSYNC 0b - VSYNC signal active HIGH 1b - VSYNC signal active LOW"]
    pub mod INV_VSYNC {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Polarity of HREF 0b - HREF signal active HIGH, indicating active pixel data 1b - HREF signal active LOW"]
    pub mod INV_HREF {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Polarity change of Pixel Clock. 0b - LCDC outputs data on the rising edge, and Display samples data on the falling edge 1b - LCDC outputs data on the falling edge, Display samples data on the rising edge"]
    pub mod INV_PXCLK {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates if value at the output (pixel data output) needs to be negated. 0b - Output is to remain same as the data inside memory 1b - Output to be negated from the data inside memory"]
    pub mod INV_PXDATA {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ARQOS for bus fabric arbitration"]
    pub mod ARQOS {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "background color for clear mode when the alpha channel is 0"]
    pub mod BGDCL4CLR {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LCDIF operating mode. 00b - Normal mode. Panel content controlled by layer configuration. 01b - Test Mode1.(BGND Color Display) 10b - Test Mode2.(Column Color Bar) 11b - Test Mode3.(Row Color Bar)"]
    pub mod DISP_MODE {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LCDIF line output order. 000b - RGB. 001b - RBG. 010b - GBR. 011b - GRB. 100b - BRG. 101b - BGR."]
    pub mod LINE_PATTERN {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Display panel On/Off mode. 0b - Display Off. 1b - Display On. Display can be set off at any time, but it can only be set on after VS_BLANK status is asserted. So a good procedure to stop and turn on the display is: 1) clr VS_BLANK status 2) assert software reset 3) de-assert software reset 4) set display off 5) check VS_BLANK status until it is asserted, 6)reset the module, change settings 7) set display on"]
    pub mod DISP_ON {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Software reset, high active. When write 1 ,all internal logical will be reset. 0b - No action 1b - All LCDC internal registers are forced into their reset state. Interface registers are not affected."]
    pub mod SW_RST {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Background Color Register"]
pub mod BGND_CL {
    #[doc = "Blue component of the default color displayed in the sectors where no layer is active."]
    pub mod B {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Green component of the default color displayed in the sectors where no layer is active."]
    pub mod G {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Red component of the default color displayed in the sectors where no layer is active."]
    pub mod R {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Display Window Size Register"]
pub mod DISP_WN_SIZE {
    #[doc = "Sets the display size horizontal resolution in pixels."]
    pub mod X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Sets the display size vertical resolution in pixels."]
    pub mod Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "HSYNC Config Register"]
pub mod HSYNC_PARA {
    #[doc = "HSYNC active pulse width (in pixel clock cycles). Pulse width has a minimum value of 1."]
    pub mod PW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "HSYNC back-porch pulse width (in pixel clock cycles). If zero, indicates no back-porch for HSYNC"]
    pub mod BP {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "HSYNC front-porch pulse width (in pixel clock cycles). If zero, indicates no front-porch for HSYNC"]
    pub mod FP {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "VSYNC Config Register"]
pub mod VSYNC_PARA {
    #[doc = "VSYNC active pulse width (in horizontal line cycles). Pulse width has a minimum value of 1."]
    pub mod PW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "VSYNC back-porch pulse width (in horizontal line cycles). If zero, means no back-porch for VSYNC"]
    pub mod BP {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "VSYNC front-porch pulse width (in horizontal line cycles). If zero, means no front-porch for VSYNC"]
    pub mod FP {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA Status Register"]
pub mod DMA_ST {
    #[doc = "Plane n frame 0 dma done. W1C."]
    pub mod DMA0_DONE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Plane n frame 1 dma done. W1C."]
    pub mod DMA1_DONE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "plane n axi error. W1C."]
    pub mod DMA_ERR {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status Register"]
pub mod ST {
    #[doc = "Asserted when in vertical blanking period. At the end of VSYNC"]
    pub mod VSYNC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Asserted when the output buffer underrun condition encountered"]
    pub mod UNDERRUN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Asserted when in vertical blanking period. At the start of VSYNC"]
    pub mod VS_BLANK {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Asserted when the output buffer urgent underrun condition encountered"]
    pub mod URGENT_UNDERRUN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Enable Register"]
pub mod INT_EN {
    #[doc = "Interrupt enable for end of sof"]
    pub mod VSYNC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Interrupt enable for underrun"]
    pub mod UNDERRUN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Interrupt enable for start of sof"]
    pub mod VS_BLANK {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Asserted when the output buffer urgent underrun condition encountered"]
    pub mod URGENT_UNDERRUN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Interrupt enable for DMA done"]
    pub mod DMA_DONE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Interrupt enable for DMA error"]
    pub mod DMA_ERR {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "TX FIFO Register"]
pub mod TXFIFO {
    #[doc = "Threshold to start the lcd raster (0--0x7F)"]
    pub mod THRSH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Control Register"]
pub mod LAYER_0_LAYCTRL {
    #[doc = "Asserted when the layer is enabled. If this layer is not enabled, it means a bypassing plane."]
    pub mod EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Alpha Blending Mode 0: SKBlendMode_Clear; 1: SKBlendMode_Src ; 2: SKBlendMode_Dst 3: SKBlendMode_SrcOver 4: SKBlendMode_DstOver 5: SKBlendMode_SrcIn 6: SKBlendMode_DstIn 7: SKBlendMode_SrcOut 8: SKBlendMode_DstOut 9: SKBlendMode_SrcATop 10: SKBlendMode_DstATop 11: SKBlendMode_Xor 12: SKBlendMode_Plus (The conventional blending mode) 13: SKBlendMode_Modulate 14: SRC org 15: DST org Others: Reserved."]
    pub mod AB_MODE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The usage of the INALPHA\\[7:0\\]: (The system alpha value is not the data valid mask, the non-zero alpha value per pixel indicates a valid pixel. If no such per pixel alpha value, it means all the pixels are valid) 0: the INALPHA\\[7:0\\] is invalid, use the alpha value from previous pipeline 1: the INALPHA\\[7:0\\] is used to override the alpha value from previous pipeline. (useful when the corresponding data stream has no alpha info) 2: the INALPHA\\[7:0\\] is used to scale the alpha value from previous pipeline Others: Reserved"]
    pub mod INALPHA_OP {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The usage of the LOCALPHA\\[7:0\\]: (The system alpha value is not the data valid mask, the non-zero alpha value per pixel indicates a valid pixel. If no such per pixel alpha value, it means all the pixels are valid) 0: the LOCALPHA\\[7:0\\] is invalid, use the alpha value from the data stream 1: the LOCALPHA\\[7:0\\] is used to override the alpha value in the data stream (useful when the data stream has no alpha info) 2: the LOCALPHA\\[7:0\\] is used to scale the alpha value from the data stream Others: Reserved"]
    pub mod LOCALPHA_OP {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Layer encoding format (bit per pixel) 0000b - 1 bpp (pixel width must be multiples of 32), pixel sequence is from LSB to MSB in 32b word. 0001b - 2 bpp (pixel width must be multiples of 16), pixel sequence is from LSB to MSB in 32b word. 0010b - 4 bpp (pixel width must be multiples of 8), pixel sequence is from LSB to MSB in 32b word. 0011b - 8 bpp (pixel width must be multiples of 4), pixel sequence is from LSB to MSB in 32b word. 0100b - 16 bpp (RGB565), byte sequence as B,R 0111b - YCbCr422 (Only layer 0/1 can support this format), byte sequence determined by LAYCTRL\\[YUV_FORMAT\\] 1001b - 32 bpp (ARGB8888), byte sequence as B,G,R,A 1011b - Y8 (pixel width must be multiples of 4), byte sequence as Y1,Y2,Y3,Y4"]
    pub mod PIXFORMAT {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The YUV422 input format selection. 00b - The YVYU422 8bit sequence is U1,Y1,V1,Y2 01b - The YVYU422 8bit sequence is V1,Y1,U1,Y2 10b - The YVYU422 8bit sequence is Y1,U1,Y2,V1 11b - The YVYU422 8bit sequence is Y1,V1,Y2,U1 If not YUV422 mode, FORMAT\\[0\\]: asserted to exchange sequence inside the bytes. Org \\[15:8\\]-->New\\[8:15\\], Org \\[7:0\\]-->New\\[0:7\\]. (First exchange) FORMAT\\[1\\]: asserted to exchange the sequence of the odd and even 8 bits. Org Even \\[7:0\\]-->New\\[15:8\\], Org Odd \\[15:8\\]-->New\\[7:0\\]. (Second exchange)"]
    pub mod YUV_FORMAT {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Shadow Load Enable The SHADOW_LOAD_EN bit is written to 1 by software after all DMA control registers are written. If set to 1, shadowed control registers are updated to the active control registers on internal logical VSYNC of next frame. If set to 0, shadowed control registers are not loaded into the active control registers. The previous active control register settings will be used to process the next frame. Hardware will automatically clear this bit, when the shadow registers are loaded to the active control regsisters."]
    pub mod SHADOW_LOAD_EN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The byte sequence of the 4 bytes in a 32-bit word. 1: {A0, A1, A2, A3} byte re-ordered. 0: {A3, A2, A1, A0} the normal case with no byte re-order"]
    pub mod PACK_DIR {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Alpha Register"]
pub mod LAYER_0_ALPHAS {
    #[doc = "The system alpha value for the input stream from previous stage (DST)"]
    pub mod IND {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The system alpha value for the data stream of current layer stream (SRC)"]
    pub mod LOCD {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Size Register"]
pub mod LAYER_0_LAYSIZE {
    #[doc = "Width of the layer in pixels (Note: not actual width-1) The layer width must be in multiples of the number of pixels that can be stored in 32 bits, and therefore differs depending on color encoding. For example, if 2 bits per pixel format is used, then the layer width must be configured in multiples of 16."]
    pub mod WIDTH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Height of the layer in pixels"]
    pub mod HEIGHT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Position Register"]
pub mod LAYER_0_LAYPOS {
    #[doc = "The horizontal position of left-hand column of the layer, where 0 is the left-hand column of the panel, positive values are to the right the left-hand column of the panel."]
    pub mod X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The vertical position of top row of the layer, where 0 is the top row of the panel, positive values are below the top row of the panel."]
    pub mod Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Buffer Pointer Register"]
pub mod LAYER_0_START0 {
    #[doc = "Input buffer Start address 0"]
    pub mod ADDR0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Bus Config Register"]
pub mod LAYER_0_LINECFG {
    #[doc = "Number of bytes between 2 vertically adjacent pixels in system memory. Byte granularity is supported, but SW should align to 64B boundry."]
    pub mod PITCH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "the number of outstanding axi read transactions. If zero, it means max 8."]
    pub mod MAX_OT {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Maximal Per Transfer Data Size: 0: 64 bytes 1: 128 bytes 2: 256 bytes 3: 512 bytes 4: 1024 bytes"]
    pub mod MPT_SIZE {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Background Color Register"]
pub mod LAYER_0_BG_CL {
    #[doc = "ARGB8888. It is only useful in the last active stage in the pipeline."]
    pub mod ARGB {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Color Space Conversion Config Register 0"]
pub mod LAYER_0_CSC_COEF0 {
    #[doc = "Two's compliment amplitude offset implicit in the Y data Y_OFFSET. For YUV, this is typically 0 and for YCbCr, this is typically -16 (0x1F0)."]
    pub mod Y_OFFSET {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Two's compliment phase offset implicit for CbCr data UV_OFFSET. Generally used for YCbCr to RGB conversion. YCbCr=0x180, YUV=0x000 (typically -128 or 0x180 to indicate normalized -0.5 to 0.5 range)."]
    pub mod UV_OFFSET {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Two's compliment Y multiplier coefficient C0. YUV=0x100 (1.000) YCbCr=0x12A (1.164)"]
    pub mod C0 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable the CSC unit in the LCDC plane data path. 0b - The CSC is bypassed and the input pixels are RGB data already 1b - The CSC is enabled and the pixels will be converted to RGB data This bit will be shadowed."]
    pub mod ENABLE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit changes the behavior when performing U/V converting. 0b - Converting YUV to RGB data 1b - Converting YCbCr to RGB data"]
    pub mod YCBCR_MODE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Color Space Conversion Config Register 1"]
pub mod LAYER_0_CSC_COEF1 {
    #[doc = "Two's compliment Blue U/Cb multiplier coefficient C4. YUV=0x208 (2.032) YCbCr=0x204 (2.017)."]
    pub mod C4 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Two's compliment Red V/Cr multiplier coefficient C1. YUV=0x123 (1.140) YCbCr=0x198 (1.596)."]
    pub mod C1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Color Space Conversion Config Register 2"]
pub mod LAYER_0_CSC_COEF2 {
    #[doc = "Two's compliment Green U/Cb multiplier coefficient C3. YUV=0x79C (-0.394) YCbCr=0x79C (-0.392)."]
    pub mod C3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Two's compliment Green V/Cr multiplier coefficient C2. YUV=0x76B (-0.581) YCbCr=0x730 (-0.813)."]
    pub mod C2 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Control Register"]
pub mod LAYER_1_LAYCTRL {
    #[doc = "Asserted when the layer is enabled. If this layer is not enabled, it means a bypassing plane."]
    pub mod EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Alpha Blending Mode 0: SKBlendMode_Clear; 1: SKBlendMode_Src ; 2: SKBlendMode_Dst 3: SKBlendMode_SrcOver 4: SKBlendMode_DstOver 5: SKBlendMode_SrcIn 6: SKBlendMode_DstIn 7: SKBlendMode_SrcOut 8: SKBlendMode_DstOut 9: SKBlendMode_SrcATop 10: SKBlendMode_DstATop 11: SKBlendMode_Xor 12: SKBlendMode_Plus (The conventional blending mode) 13: SKBlendMode_Modulate 14: SRC org 15: DST org Others: Reserved."]
    pub mod AB_MODE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The usage of the INALPHA\\[7:0\\]: (The system alpha value is not the data valid mask, the non-zero alpha value per pixel indicates a valid pixel. If no such per pixel alpha value, it means all the pixels are valid) 0: the INALPHA\\[7:0\\] is invalid, use the alpha value from previous pipeline 1: the INALPHA\\[7:0\\] is used to override the alpha value from previous pipeline. (useful when the corresponding data stream has no alpha info) 2: the INALPHA\\[7:0\\] is used to scale the alpha value from previous pipeline Others: Reserved"]
    pub mod INALPHA_OP {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The usage of the LOCALPHA\\[7:0\\]: (The system alpha value is not the data valid mask, the non-zero alpha value per pixel indicates a valid pixel. If no such per pixel alpha value, it means all the pixels are valid) 0: the LOCALPHA\\[7:0\\] is invalid, use the alpha value from the data stream 1: the LOCALPHA\\[7:0\\] is used to override the alpha value in the data stream (useful when the data stream has no alpha info) 2: the LOCALPHA\\[7:0\\] is used to scale the alpha value from the data stream Others: Reserved"]
    pub mod LOCALPHA_OP {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Layer encoding format (bit per pixel) 0000b - 1 bpp (pixel width must be multiples of 32), pixel sequence is from LSB to MSB in 32b word. 0001b - 2 bpp (pixel width must be multiples of 16), pixel sequence is from LSB to MSB in 32b word. 0010b - 4 bpp (pixel width must be multiples of 8), pixel sequence is from LSB to MSB in 32b word. 0011b - 8 bpp (pixel width must be multiples of 4), pixel sequence is from LSB to MSB in 32b word. 0100b - 16 bpp (RGB565), byte sequence as B,R 0111b - YCbCr422 (Only layer 0/1 can support this format), byte sequence determined by LAYCTRL\\[YUV_FORMAT\\] 1001b - 32 bpp (ARGB8888), byte sequence as B,G,R,A 1011b - Y8 (pixel width must be multiples of 4), byte sequence as Y1,Y2,Y3,Y4"]
    pub mod PIXFORMAT {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The YUV422 input format selection. 00b - The YVYU422 8bit sequence is U1,Y1,V1,Y2 01b - The YVYU422 8bit sequence is V1,Y1,U1,Y2 10b - The YVYU422 8bit sequence is Y1,U1,Y2,V1 11b - The YVYU422 8bit sequence is Y1,V1,Y2,U1 If not YUV422 mode, FORMAT\\[0\\]: asserted to exchange sequence inside the bytes. Org \\[15:8\\]-->New\\[8:15\\], Org \\[7:0\\]-->New\\[0:7\\]. (First exchange) FORMAT\\[1\\]: asserted to exchange the sequence of the odd and even 8 bits. Org Even \\[7:0\\]-->New\\[15:8\\], Org Odd \\[15:8\\]-->New\\[7:0\\]. (Second exchange)"]
    pub mod YUV_FORMAT {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Shadow Load Enable The SHADOW_LOAD_EN bit is written to 1 by software after all DMA control registers are written. If set to 1, shadowed control registers are updated to the active control registers on internal logical VSYNC of next frame. If set to 0, shadowed control registers are not loaded into the active control registers. The previous active control register settings will be used to process the next frame. Hardware will automatically clear this bit, when the shadow registers are loaded to the active control regsisters."]
    pub mod SHADOW_LOAD_EN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The byte sequence of the 4 bytes in a 32-bit word. 1: {A0, A1, A2, A3} byte re-ordered. 0: {A3, A2, A1, A0} the normal case with no byte re-order"]
    pub mod PACK_DIR {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Alpha Register"]
pub mod LAYER_1_ALPHAS {
    #[doc = "The system alpha value for the input stream from previous stage (DST)"]
    pub mod IND {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The system alpha value for the data stream of current layer stream (SRC)"]
    pub mod LOCD {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Size Register"]
pub mod LAYER_1_LAYSIZE {
    #[doc = "Width of the layer in pixels (Note: not actual width-1) The layer width must be in multiples of the number of pixels that can be stored in 32 bits, and therefore differs depending on color encoding. For example, if 2 bits per pixel format is used, then the layer width must be configured in multiples of 16."]
    pub mod WIDTH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Height of the layer in pixels"]
    pub mod HEIGHT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Position Register"]
pub mod LAYER_1_LAYPOS {
    #[doc = "The horizontal position of left-hand column of the layer, where 0 is the left-hand column of the panel, positive values are to the right the left-hand column of the panel."]
    pub mod X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The vertical position of top row of the layer, where 0 is the top row of the panel, positive values are below the top row of the panel."]
    pub mod Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Buffer Pointer Register"]
pub mod LAYER_1_START0 {
    #[doc = "Input buffer Start address 0"]
    pub mod ADDR0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Bus Config Register"]
pub mod LAYER_1_LINECFG {
    #[doc = "Number of bytes between 2 vertically adjacent pixels in system memory. Byte granularity is supported, but SW should align to 64B boundry."]
    pub mod PITCH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "the number of outstanding axi read transactions. If zero, it means max 8."]
    pub mod MAX_OT {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Maximal Per Transfer Data Size: 0: 64 bytes 1: 128 bytes 2: 256 bytes 3: 512 bytes 4: 1024 bytes"]
    pub mod MPT_SIZE {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Background Color Register"]
pub mod LAYER_1_BG_CL {
    #[doc = "ARGB8888. It is only useful in the last active stage in the pipeline."]
    pub mod ARGB {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Color Space Conversion Config Register 0"]
pub mod LAYER_1_CSC_COEF0 {
    #[doc = "Two's compliment amplitude offset implicit in the Y data Y_OFFSET. For YUV, this is typically 0 and for YCbCr, this is typically -16 (0x1F0)."]
    pub mod Y_OFFSET {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Two's compliment phase offset implicit for CbCr data UV_OFFSET. Generally used for YCbCr to RGB conversion. YCbCr=0x180, YUV=0x000 (typically -128 or 0x180 to indicate normalized -0.5 to 0.5 range)."]
    pub mod UV_OFFSET {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Two's compliment Y multiplier coefficient C0. YUV=0x100 (1.000) YCbCr=0x12A (1.164)"]
    pub mod C0 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable the CSC unit in the LCDC plane data path. 0b - The CSC is bypassed and the input pixels are RGB data already 1b - The CSC is enabled and the pixels will be converted to RGB data This bit will be shadowed."]
    pub mod ENABLE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit changes the behavior when performing U/V converting. 0b - Converting YUV to RGB data 1b - Converting YCbCr to RGB data"]
    pub mod YCBCR_MODE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Color Space Conversion Config Register 1"]
pub mod LAYER_1_CSC_COEF1 {
    #[doc = "Two's compliment Blue U/Cb multiplier coefficient C4. YUV=0x208 (2.032) YCbCr=0x204 (2.017)."]
    pub mod C4 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Two's compliment Red V/Cr multiplier coefficient C1. YUV=0x123 (1.140) YCbCr=0x198 (1.596)."]
    pub mod C1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Color Space Conversion Config Register 2"]
pub mod LAYER_1_CSC_COEF2 {
    #[doc = "Two's compliment Green U/Cb multiplier coefficient C3. YUV=0x79C (-0.394) YCbCr=0x79C (-0.392)."]
    pub mod C3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Two's compliment Green V/Cr multiplier coefficient C2. YUV=0x76B (-0.581) YCbCr=0x730 (-0.813)."]
    pub mod C2 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Control Register"]
pub mod LAYER_2_LAYCTRL {
    #[doc = "Asserted when the layer is enabled. If this layer is not enabled, it means a bypassing plane."]
    pub mod EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Alpha Blending Mode 0: SKBlendMode_Clear; 1: SKBlendMode_Src ; 2: SKBlendMode_Dst 3: SKBlendMode_SrcOver 4: SKBlendMode_DstOver 5: SKBlendMode_SrcIn 6: SKBlendMode_DstIn 7: SKBlendMode_SrcOut 8: SKBlendMode_DstOut 9: SKBlendMode_SrcATop 10: SKBlendMode_DstATop 11: SKBlendMode_Xor 12: SKBlendMode_Plus (The conventional blending mode) 13: SKBlendMode_Modulate 14: SRC org 15: DST org Others: Reserved."]
    pub mod AB_MODE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The usage of the INALPHA\\[7:0\\]: (The system alpha value is not the data valid mask, the non-zero alpha value per pixel indicates a valid pixel. If no such per pixel alpha value, it means all the pixels are valid) 0: the INALPHA\\[7:0\\] is invalid, use the alpha value from previous pipeline 1: the INALPHA\\[7:0\\] is used to override the alpha value from previous pipeline. (useful when the corresponding data stream has no alpha info) 2: the INALPHA\\[7:0\\] is used to scale the alpha value from previous pipeline Others: Reserved"]
    pub mod INALPHA_OP {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The usage of the LOCALPHA\\[7:0\\]: (The system alpha value is not the data valid mask, the non-zero alpha value per pixel indicates a valid pixel. If no such per pixel alpha value, it means all the pixels are valid) 0: the LOCALPHA\\[7:0\\] is invalid, use the alpha value from the data stream 1: the LOCALPHA\\[7:0\\] is used to override the alpha value in the data stream (useful when the data stream has no alpha info) 2: the LOCALPHA\\[7:0\\] is used to scale the alpha value from the data stream Others: Reserved"]
    pub mod LOCALPHA_OP {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Layer encoding format (bit per pixel) 0000b - 1 bpp (pixel width must be multiples of 32), pixel sequence is from LSB to MSB in 32b word. 0001b - 2 bpp (pixel width must be multiples of 16), pixel sequence is from LSB to MSB in 32b word. 0010b - 4 bpp (pixel width must be multiples of 8), pixel sequence is from LSB to MSB in 32b word. 0011b - 8 bpp (pixel width must be multiples of 4), pixel sequence is from LSB to MSB in 32b word. 0100b - 16 bpp (RGB565), byte sequence as B,R 0111b - YCbCr422 (Only layer 0/1 can support this format), byte sequence determined by LAYCTRL\\[YUV_FORMAT\\] 1001b - 32 bpp (ARGB8888), byte sequence as B,G,R,A 1011b - Y8 (pixel width must be multiples of 4), byte sequence as Y1,Y2,Y3,Y4"]
    pub mod PIXFORMAT {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The YUV422 input format selection. 00b - The YVYU422 8bit sequence is U1,Y1,V1,Y2 01b - The YVYU422 8bit sequence is V1,Y1,U1,Y2 10b - The YVYU422 8bit sequence is Y1,U1,Y2,V1 11b - The YVYU422 8bit sequence is Y1,V1,Y2,U1 If not YUV422 mode, FORMAT\\[0\\]: asserted to exchange sequence inside the bytes. Org \\[15:8\\]-->New\\[8:15\\], Org \\[7:0\\]-->New\\[0:7\\]. (First exchange) FORMAT\\[1\\]: asserted to exchange the sequence of the odd and even 8 bits. Org Even \\[7:0\\]-->New\\[15:8\\], Org Odd \\[15:8\\]-->New\\[7:0\\]. (Second exchange)"]
    pub mod YUV_FORMAT {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Shadow Load Enable The SHADOW_LOAD_EN bit is written to 1 by software after all DMA control registers are written. If set to 1, shadowed control registers are updated to the active control registers on internal logical VSYNC of next frame. If set to 0, shadowed control registers are not loaded into the active control registers. The previous active control register settings will be used to process the next frame. Hardware will automatically clear this bit, when the shadow registers are loaded to the active control regsisters."]
    pub mod SHADOW_LOAD_EN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The byte sequence of the 4 bytes in a 32-bit word. 1: {A0, A1, A2, A3} byte re-ordered. 0: {A3, A2, A1, A0} the normal case with no byte re-order"]
    pub mod PACK_DIR {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Alpha Register"]
pub mod LAYER_2_ALPHAS {
    #[doc = "The system alpha value for the input stream from previous stage (DST)"]
    pub mod IND {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The system alpha value for the data stream of current layer stream (SRC)"]
    pub mod LOCD {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Size Register"]
pub mod LAYER_2_LAYSIZE {
    #[doc = "Width of the layer in pixels (Note: not actual width-1) The layer width must be in multiples of the number of pixels that can be stored in 32 bits, and therefore differs depending on color encoding. For example, if 2 bits per pixel format is used, then the layer width must be configured in multiples of 16."]
    pub mod WIDTH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Height of the layer in pixels"]
    pub mod HEIGHT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Position Register"]
pub mod LAYER_2_LAYPOS {
    #[doc = "The horizontal position of left-hand column of the layer, where 0 is the left-hand column of the panel, positive values are to the right the left-hand column of the panel."]
    pub mod X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The vertical position of top row of the layer, where 0 is the top row of the panel, positive values are below the top row of the panel."]
    pub mod Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Buffer Pointer Register"]
pub mod LAYER_2_START0 {
    #[doc = "Input buffer Start address 0"]
    pub mod ADDR0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Bus Config Register"]
pub mod LAYER_2_LINECFG {
    #[doc = "Number of bytes between 2 vertically adjacent pixels in system memory. Byte granularity is supported, but SW should align to 64B boundry."]
    pub mod PITCH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "the number of outstanding axi read transactions. If zero, it means max 8."]
    pub mod MAX_OT {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Maximal Per Transfer Data Size: 0: 64 bytes 1: 128 bytes 2: 256 bytes 3: 512 bytes 4: 1024 bytes"]
    pub mod MPT_SIZE {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Background Color Register"]
pub mod LAYER_2_BG_CL {
    #[doc = "ARGB8888. It is only useful in the last active stage in the pipeline."]
    pub mod ARGB {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Color Space Conversion Config Register 0"]
pub mod LAYER_2_CSC_COEF0 {
    #[doc = "Two's compliment amplitude offset implicit in the Y data Y_OFFSET. For YUV, this is typically 0 and for YCbCr, this is typically -16 (0x1F0)."]
    pub mod Y_OFFSET {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Two's compliment phase offset implicit for CbCr data UV_OFFSET. Generally used for YCbCr to RGB conversion. YCbCr=0x180, YUV=0x000 (typically -128 or 0x180 to indicate normalized -0.5 to 0.5 range)."]
    pub mod UV_OFFSET {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Two's compliment Y multiplier coefficient C0. YUV=0x100 (1.000) YCbCr=0x12A (1.164)"]
    pub mod C0 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable the CSC unit in the LCDC plane data path. 0b - The CSC is bypassed and the input pixels are RGB data already 1b - The CSC is enabled and the pixels will be converted to RGB data This bit will be shadowed."]
    pub mod ENABLE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit changes the behavior when performing U/V converting. 0b - Converting YUV to RGB data 1b - Converting YCbCr to RGB data"]
    pub mod YCBCR_MODE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Color Space Conversion Config Register 1"]
pub mod LAYER_2_CSC_COEF1 {
    #[doc = "Two's compliment Blue U/Cb multiplier coefficient C4. YUV=0x208 (2.032) YCbCr=0x204 (2.017)."]
    pub mod C4 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Two's compliment Red V/Cr multiplier coefficient C1. YUV=0x123 (1.140) YCbCr=0x198 (1.596)."]
    pub mod C1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Color Space Conversion Config Register 2"]
pub mod LAYER_2_CSC_COEF2 {
    #[doc = "Two's compliment Green U/Cb multiplier coefficient C3. YUV=0x79C (-0.394) YCbCr=0x79C (-0.392)."]
    pub mod C3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Two's compliment Green V/Cr multiplier coefficient C2. YUV=0x76B (-0.581) YCbCr=0x730 (-0.813)."]
    pub mod C2 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Control Register"]
pub mod LAYER_3_LAYCTRL {
    #[doc = "Asserted when the layer is enabled. If this layer is not enabled, it means a bypassing plane."]
    pub mod EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Alpha Blending Mode 0: SKBlendMode_Clear; 1: SKBlendMode_Src ; 2: SKBlendMode_Dst 3: SKBlendMode_SrcOver 4: SKBlendMode_DstOver 5: SKBlendMode_SrcIn 6: SKBlendMode_DstIn 7: SKBlendMode_SrcOut 8: SKBlendMode_DstOut 9: SKBlendMode_SrcATop 10: SKBlendMode_DstATop 11: SKBlendMode_Xor 12: SKBlendMode_Plus (The conventional blending mode) 13: SKBlendMode_Modulate 14: SRC org 15: DST org Others: Reserved."]
    pub mod AB_MODE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The usage of the INALPHA\\[7:0\\]: (The system alpha value is not the data valid mask, the non-zero alpha value per pixel indicates a valid pixel. If no such per pixel alpha value, it means all the pixels are valid) 0: the INALPHA\\[7:0\\] is invalid, use the alpha value from previous pipeline 1: the INALPHA\\[7:0\\] is used to override the alpha value from previous pipeline. (useful when the corresponding data stream has no alpha info) 2: the INALPHA\\[7:0\\] is used to scale the alpha value from previous pipeline Others: Reserved"]
    pub mod INALPHA_OP {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The usage of the LOCALPHA\\[7:0\\]: (The system alpha value is not the data valid mask, the non-zero alpha value per pixel indicates a valid pixel. If no such per pixel alpha value, it means all the pixels are valid) 0: the LOCALPHA\\[7:0\\] is invalid, use the alpha value from the data stream 1: the LOCALPHA\\[7:0\\] is used to override the alpha value in the data stream (useful when the data stream has no alpha info) 2: the LOCALPHA\\[7:0\\] is used to scale the alpha value from the data stream Others: Reserved"]
    pub mod LOCALPHA_OP {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Layer encoding format (bit per pixel) 0000b - 1 bpp (pixel width must be multiples of 32), pixel sequence is from LSB to MSB in 32b word. 0001b - 2 bpp (pixel width must be multiples of 16), pixel sequence is from LSB to MSB in 32b word. 0010b - 4 bpp (pixel width must be multiples of 8), pixel sequence is from LSB to MSB in 32b word. 0011b - 8 bpp (pixel width must be multiples of 4), pixel sequence is from LSB to MSB in 32b word. 0100b - 16 bpp (RGB565), byte sequence as B,R 0111b - YCbCr422 (Only layer 0/1 can support this format), byte sequence determined by LAYCTRL\\[YUV_FORMAT\\] 1001b - 32 bpp (ARGB8888), byte sequence as B,G,R,A 1011b - Y8 (pixel width must be multiples of 4), byte sequence as Y1,Y2,Y3,Y4"]
    pub mod PIXFORMAT {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The YUV422 input format selection. 00b - The YVYU422 8bit sequence is U1,Y1,V1,Y2 01b - The YVYU422 8bit sequence is V1,Y1,U1,Y2 10b - The YVYU422 8bit sequence is Y1,U1,Y2,V1 11b - The YVYU422 8bit sequence is Y1,V1,Y2,U1 If not YUV422 mode, FORMAT\\[0\\]: asserted to exchange sequence inside the bytes. Org \\[15:8\\]-->New\\[8:15\\], Org \\[7:0\\]-->New\\[0:7\\]. (First exchange) FORMAT\\[1\\]: asserted to exchange the sequence of the odd and even 8 bits. Org Even \\[7:0\\]-->New\\[15:8\\], Org Odd \\[15:8\\]-->New\\[7:0\\]. (Second exchange)"]
    pub mod YUV_FORMAT {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Shadow Load Enable The SHADOW_LOAD_EN bit is written to 1 by software after all DMA control registers are written. If set to 1, shadowed control registers are updated to the active control registers on internal logical VSYNC of next frame. If set to 0, shadowed control registers are not loaded into the active control registers. The previous active control register settings will be used to process the next frame. Hardware will automatically clear this bit, when the shadow registers are loaded to the active control regsisters."]
    pub mod SHADOW_LOAD_EN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The byte sequence of the 4 bytes in a 32-bit word. 1: {A0, A1, A2, A3} byte re-ordered. 0: {A3, A2, A1, A0} the normal case with no byte re-order"]
    pub mod PACK_DIR {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Alpha Register"]
pub mod LAYER_3_ALPHAS {
    #[doc = "The system alpha value for the input stream from previous stage (DST)"]
    pub mod IND {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The system alpha value for the data stream of current layer stream (SRC)"]
    pub mod LOCD {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Size Register"]
pub mod LAYER_3_LAYSIZE {
    #[doc = "Width of the layer in pixels (Note: not actual width-1) The layer width must be in multiples of the number of pixels that can be stored in 32 bits, and therefore differs depending on color encoding. For example, if 2 bits per pixel format is used, then the layer width must be configured in multiples of 16."]
    pub mod WIDTH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Height of the layer in pixels"]
    pub mod HEIGHT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Position Register"]
pub mod LAYER_3_LAYPOS {
    #[doc = "The horizontal position of left-hand column of the layer, where 0 is the left-hand column of the panel, positive values are to the right the left-hand column of the panel."]
    pub mod X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The vertical position of top row of the layer, where 0 is the top row of the panel, positive values are below the top row of the panel."]
    pub mod Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Buffer Pointer Register"]
pub mod LAYER_3_START0 {
    #[doc = "Input buffer Start address 0"]
    pub mod ADDR0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Bus Config Register"]
pub mod LAYER_3_LINECFG {
    #[doc = "Number of bytes between 2 vertically adjacent pixels in system memory. Byte granularity is supported, but SW should align to 64B boundry."]
    pub mod PITCH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "the number of outstanding axi read transactions. If zero, it means max 8."]
    pub mod MAX_OT {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Maximal Per Transfer Data Size: 0: 64 bytes 1: 128 bytes 2: 256 bytes 3: 512 bytes 4: 1024 bytes"]
    pub mod MPT_SIZE {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Background Color Register"]
pub mod LAYER_3_BG_CL {
    #[doc = "ARGB8888. It is only useful in the last active stage in the pipeline."]
    pub mod ARGB {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Color Space Conversion Config Register 0"]
pub mod LAYER_3_CSC_COEF0 {
    #[doc = "Two's compliment amplitude offset implicit in the Y data Y_OFFSET. For YUV, this is typically 0 and for YCbCr, this is typically -16 (0x1F0)."]
    pub mod Y_OFFSET {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Two's compliment phase offset implicit for CbCr data UV_OFFSET. Generally used for YCbCr to RGB conversion. YCbCr=0x180, YUV=0x000 (typically -128 or 0x180 to indicate normalized -0.5 to 0.5 range)."]
    pub mod UV_OFFSET {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Two's compliment Y multiplier coefficient C0. YUV=0x100 (1.000) YCbCr=0x12A (1.164)"]
    pub mod C0 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable the CSC unit in the LCDC plane data path. 0b - The CSC is bypassed and the input pixels are RGB data already 1b - The CSC is enabled and the pixels will be converted to RGB data This bit will be shadowed."]
    pub mod ENABLE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit changes the behavior when performing U/V converting. 0b - Converting YUV to RGB data 1b - Converting YCbCr to RGB data"]
    pub mod YCBCR_MODE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Color Space Conversion Config Register 1"]
pub mod LAYER_3_CSC_COEF1 {
    #[doc = "Two's compliment Blue U/Cb multiplier coefficient C4. YUV=0x208 (2.032) YCbCr=0x204 (2.017)."]
    pub mod C4 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Two's compliment Red V/Cr multiplier coefficient C1. YUV=0x123 (1.140) YCbCr=0x198 (1.596)."]
    pub mod C1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Color Space Conversion Config Register 2"]
pub mod LAYER_3_CSC_COEF2 {
    #[doc = "Two's compliment Green U/Cb multiplier coefficient C3. YUV=0x79C (-0.394) YCbCr=0x79C (-0.392)."]
    pub mod C3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Two's compliment Green V/Cr multiplier coefficient C2. YUV=0x76B (-0.581) YCbCr=0x730 (-0.813)."]
    pub mod C2 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Control Register"]
pub mod LAYER_4_LAYCTRL {
    #[doc = "Asserted when the layer is enabled. If this layer is not enabled, it means a bypassing plane."]
    pub mod EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Alpha Blending Mode 0: SKBlendMode_Clear; 1: SKBlendMode_Src ; 2: SKBlendMode_Dst 3: SKBlendMode_SrcOver 4: SKBlendMode_DstOver 5: SKBlendMode_SrcIn 6: SKBlendMode_DstIn 7: SKBlendMode_SrcOut 8: SKBlendMode_DstOut 9: SKBlendMode_SrcATop 10: SKBlendMode_DstATop 11: SKBlendMode_Xor 12: SKBlendMode_Plus (The conventional blending mode) 13: SKBlendMode_Modulate 14: SRC org 15: DST org Others: Reserved."]
    pub mod AB_MODE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The usage of the INALPHA\\[7:0\\]: (The system alpha value is not the data valid mask, the non-zero alpha value per pixel indicates a valid pixel. If no such per pixel alpha value, it means all the pixels are valid) 0: the INALPHA\\[7:0\\] is invalid, use the alpha value from previous pipeline 1: the INALPHA\\[7:0\\] is used to override the alpha value from previous pipeline. (useful when the corresponding data stream has no alpha info) 2: the INALPHA\\[7:0\\] is used to scale the alpha value from previous pipeline Others: Reserved"]
    pub mod INALPHA_OP {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The usage of the LOCALPHA\\[7:0\\]: (The system alpha value is not the data valid mask, the non-zero alpha value per pixel indicates a valid pixel. If no such per pixel alpha value, it means all the pixels are valid) 0: the LOCALPHA\\[7:0\\] is invalid, use the alpha value from the data stream 1: the LOCALPHA\\[7:0\\] is used to override the alpha value in the data stream (useful when the data stream has no alpha info) 2: the LOCALPHA\\[7:0\\] is used to scale the alpha value from the data stream Others: Reserved"]
    pub mod LOCALPHA_OP {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Layer encoding format (bit per pixel) 0000b - 1 bpp (pixel width must be multiples of 32), pixel sequence is from LSB to MSB in 32b word. 0001b - 2 bpp (pixel width must be multiples of 16), pixel sequence is from LSB to MSB in 32b word. 0010b - 4 bpp (pixel width must be multiples of 8), pixel sequence is from LSB to MSB in 32b word. 0011b - 8 bpp (pixel width must be multiples of 4), pixel sequence is from LSB to MSB in 32b word. 0100b - 16 bpp (RGB565), byte sequence as B,R 0111b - YCbCr422 (Only layer 0/1 can support this format), byte sequence determined by LAYCTRL\\[YUV_FORMAT\\] 1001b - 32 bpp (ARGB8888), byte sequence as B,G,R,A 1011b - Y8 (pixel width must be multiples of 4), byte sequence as Y1,Y2,Y3,Y4"]
    pub mod PIXFORMAT {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The YUV422 input format selection. 00b - The YVYU422 8bit sequence is U1,Y1,V1,Y2 01b - The YVYU422 8bit sequence is V1,Y1,U1,Y2 10b - The YVYU422 8bit sequence is Y1,U1,Y2,V1 11b - The YVYU422 8bit sequence is Y1,V1,Y2,U1 If not YUV422 mode, FORMAT\\[0\\]: asserted to exchange sequence inside the bytes. Org \\[15:8\\]-->New\\[8:15\\], Org \\[7:0\\]-->New\\[0:7\\]. (First exchange) FORMAT\\[1\\]: asserted to exchange the sequence of the odd and even 8 bits. Org Even \\[7:0\\]-->New\\[15:8\\], Org Odd \\[15:8\\]-->New\\[7:0\\]. (Second exchange)"]
    pub mod YUV_FORMAT {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Shadow Load Enable The SHADOW_LOAD_EN bit is written to 1 by software after all DMA control registers are written. If set to 1, shadowed control registers are updated to the active control registers on internal logical VSYNC of next frame. If set to 0, shadowed control registers are not loaded into the active control registers. The previous active control register settings will be used to process the next frame. Hardware will automatically clear this bit, when the shadow registers are loaded to the active control regsisters."]
    pub mod SHADOW_LOAD_EN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The byte sequence of the 4 bytes in a 32-bit word. 1: {A0, A1, A2, A3} byte re-ordered. 0: {A3, A2, A1, A0} the normal case with no byte re-order"]
    pub mod PACK_DIR {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Alpha Register"]
pub mod LAYER_4_ALPHAS {
    #[doc = "The system alpha value for the input stream from previous stage (DST)"]
    pub mod IND {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The system alpha value for the data stream of current layer stream (SRC)"]
    pub mod LOCD {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Size Register"]
pub mod LAYER_4_LAYSIZE {
    #[doc = "Width of the layer in pixels (Note: not actual width-1) The layer width must be in multiples of the number of pixels that can be stored in 32 bits, and therefore differs depending on color encoding. For example, if 2 bits per pixel format is used, then the layer width must be configured in multiples of 16."]
    pub mod WIDTH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Height of the layer in pixels"]
    pub mod HEIGHT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Position Register"]
pub mod LAYER_4_LAYPOS {
    #[doc = "The horizontal position of left-hand column of the layer, where 0 is the left-hand column of the panel, positive values are to the right the left-hand column of the panel."]
    pub mod X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The vertical position of top row of the layer, where 0 is the top row of the panel, positive values are below the top row of the panel."]
    pub mod Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Buffer Pointer Register"]
pub mod LAYER_4_START0 {
    #[doc = "Input buffer Start address 0"]
    pub mod ADDR0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Bus Config Register"]
pub mod LAYER_4_LINECFG {
    #[doc = "Number of bytes between 2 vertically adjacent pixels in system memory. Byte granularity is supported, but SW should align to 64B boundry."]
    pub mod PITCH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "the number of outstanding axi read transactions. If zero, it means max 8."]
    pub mod MAX_OT {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Maximal Per Transfer Data Size: 0: 64 bytes 1: 128 bytes 2: 256 bytes 3: 512 bytes 4: 1024 bytes"]
    pub mod MPT_SIZE {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Background Color Register"]
pub mod LAYER_4_BG_CL {
    #[doc = "ARGB8888. It is only useful in the last active stage in the pipeline."]
    pub mod ARGB {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Color Space Conversion Config Register 0"]
pub mod LAYER_4_CSC_COEF0 {
    #[doc = "Two's compliment amplitude offset implicit in the Y data Y_OFFSET. For YUV, this is typically 0 and for YCbCr, this is typically -16 (0x1F0)."]
    pub mod Y_OFFSET {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Two's compliment phase offset implicit for CbCr data UV_OFFSET. Generally used for YCbCr to RGB conversion. YCbCr=0x180, YUV=0x000 (typically -128 or 0x180 to indicate normalized -0.5 to 0.5 range)."]
    pub mod UV_OFFSET {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Two's compliment Y multiplier coefficient C0. YUV=0x100 (1.000) YCbCr=0x12A (1.164)"]
    pub mod C0 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable the CSC unit in the LCDC plane data path. 0b - The CSC is bypassed and the input pixels are RGB data already 1b - The CSC is enabled and the pixels will be converted to RGB data This bit will be shadowed."]
    pub mod ENABLE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit changes the behavior when performing U/V converting. 0b - Converting YUV to RGB data 1b - Converting YCbCr to RGB data"]
    pub mod YCBCR_MODE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Color Space Conversion Config Register 1"]
pub mod LAYER_4_CSC_COEF1 {
    #[doc = "Two's compliment Blue U/Cb multiplier coefficient C4. YUV=0x208 (2.032) YCbCr=0x204 (2.017)."]
    pub mod C4 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Two's compliment Red V/Cr multiplier coefficient C1. YUV=0x123 (1.140) YCbCr=0x198 (1.596)."]
    pub mod C1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Color Space Conversion Config Register 2"]
pub mod LAYER_4_CSC_COEF2 {
    #[doc = "Two's compliment Green U/Cb multiplier coefficient C3. YUV=0x79C (-0.394) YCbCr=0x79C (-0.392)."]
    pub mod C3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Two's compliment Green V/Cr multiplier coefficient C2. YUV=0x76B (-0.581) YCbCr=0x730 (-0.813)."]
    pub mod C2 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Control Register"]
pub mod LAYER_5_LAYCTRL {
    #[doc = "Asserted when the layer is enabled. If this layer is not enabled, it means a bypassing plane."]
    pub mod EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Alpha Blending Mode 0: SKBlendMode_Clear; 1: SKBlendMode_Src ; 2: SKBlendMode_Dst 3: SKBlendMode_SrcOver 4: SKBlendMode_DstOver 5: SKBlendMode_SrcIn 6: SKBlendMode_DstIn 7: SKBlendMode_SrcOut 8: SKBlendMode_DstOut 9: SKBlendMode_SrcATop 10: SKBlendMode_DstATop 11: SKBlendMode_Xor 12: SKBlendMode_Plus (The conventional blending mode) 13: SKBlendMode_Modulate 14: SRC org 15: DST org Others: Reserved."]
    pub mod AB_MODE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The usage of the INALPHA\\[7:0\\]: (The system alpha value is not the data valid mask, the non-zero alpha value per pixel indicates a valid pixel. If no such per pixel alpha value, it means all the pixels are valid) 0: the INALPHA\\[7:0\\] is invalid, use the alpha value from previous pipeline 1: the INALPHA\\[7:0\\] is used to override the alpha value from previous pipeline. (useful when the corresponding data stream has no alpha info) 2: the INALPHA\\[7:0\\] is used to scale the alpha value from previous pipeline Others: Reserved"]
    pub mod INALPHA_OP {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The usage of the LOCALPHA\\[7:0\\]: (The system alpha value is not the data valid mask, the non-zero alpha value per pixel indicates a valid pixel. If no such per pixel alpha value, it means all the pixels are valid) 0: the LOCALPHA\\[7:0\\] is invalid, use the alpha value from the data stream 1: the LOCALPHA\\[7:0\\] is used to override the alpha value in the data stream (useful when the data stream has no alpha info) 2: the LOCALPHA\\[7:0\\] is used to scale the alpha value from the data stream Others: Reserved"]
    pub mod LOCALPHA_OP {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Layer encoding format (bit per pixel) 0000b - 1 bpp (pixel width must be multiples of 32), pixel sequence is from LSB to MSB in 32b word. 0001b - 2 bpp (pixel width must be multiples of 16), pixel sequence is from LSB to MSB in 32b word. 0010b - 4 bpp (pixel width must be multiples of 8), pixel sequence is from LSB to MSB in 32b word. 0011b - 8 bpp (pixel width must be multiples of 4), pixel sequence is from LSB to MSB in 32b word. 0100b - 16 bpp (RGB565), byte sequence as B,R 0111b - YCbCr422 (Only layer 0/1 can support this format), byte sequence determined by LAYCTRL\\[YUV_FORMAT\\] 1001b - 32 bpp (ARGB8888), byte sequence as B,G,R,A 1011b - Y8 (pixel width must be multiples of 4), byte sequence as Y1,Y2,Y3,Y4"]
    pub mod PIXFORMAT {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The YUV422 input format selection. 00b - The YVYU422 8bit sequence is U1,Y1,V1,Y2 01b - The YVYU422 8bit sequence is V1,Y1,U1,Y2 10b - The YVYU422 8bit sequence is Y1,U1,Y2,V1 11b - The YVYU422 8bit sequence is Y1,V1,Y2,U1 If not YUV422 mode, FORMAT\\[0\\]: asserted to exchange sequence inside the bytes. Org \\[15:8\\]-->New\\[8:15\\], Org \\[7:0\\]-->New\\[0:7\\]. (First exchange) FORMAT\\[1\\]: asserted to exchange the sequence of the odd and even 8 bits. Org Even \\[7:0\\]-->New\\[15:8\\], Org Odd \\[15:8\\]-->New\\[7:0\\]. (Second exchange)"]
    pub mod YUV_FORMAT {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Shadow Load Enable The SHADOW_LOAD_EN bit is written to 1 by software after all DMA control registers are written. If set to 1, shadowed control registers are updated to the active control registers on internal logical VSYNC of next frame. If set to 0, shadowed control registers are not loaded into the active control registers. The previous active control register settings will be used to process the next frame. Hardware will automatically clear this bit, when the shadow registers are loaded to the active control regsisters."]
    pub mod SHADOW_LOAD_EN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The byte sequence of the 4 bytes in a 32-bit word. 1: {A0, A1, A2, A3} byte re-ordered. 0: {A3, A2, A1, A0} the normal case with no byte re-order"]
    pub mod PACK_DIR {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Alpha Register"]
pub mod LAYER_5_ALPHAS {
    #[doc = "The system alpha value for the input stream from previous stage (DST)"]
    pub mod IND {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The system alpha value for the data stream of current layer stream (SRC)"]
    pub mod LOCD {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Size Register"]
pub mod LAYER_5_LAYSIZE {
    #[doc = "Width of the layer in pixels (Note: not actual width-1) The layer width must be in multiples of the number of pixels that can be stored in 32 bits, and therefore differs depending on color encoding. For example, if 2 bits per pixel format is used, then the layer width must be configured in multiples of 16."]
    pub mod WIDTH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Height of the layer in pixels"]
    pub mod HEIGHT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Position Register"]
pub mod LAYER_5_LAYPOS {
    #[doc = "The horizontal position of left-hand column of the layer, where 0 is the left-hand column of the panel, positive values are to the right the left-hand column of the panel."]
    pub mod X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The vertical position of top row of the layer, where 0 is the top row of the panel, positive values are below the top row of the panel."]
    pub mod Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Buffer Pointer Register"]
pub mod LAYER_5_START0 {
    #[doc = "Input buffer Start address 0"]
    pub mod ADDR0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Bus Config Register"]
pub mod LAYER_5_LINECFG {
    #[doc = "Number of bytes between 2 vertically adjacent pixels in system memory. Byte granularity is supported, but SW should align to 64B boundry."]
    pub mod PITCH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "the number of outstanding axi read transactions. If zero, it means max 8."]
    pub mod MAX_OT {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Maximal Per Transfer Data Size: 0: 64 bytes 1: 128 bytes 2: 256 bytes 3: 512 bytes 4: 1024 bytes"]
    pub mod MPT_SIZE {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Background Color Register"]
pub mod LAYER_5_BG_CL {
    #[doc = "ARGB8888. It is only useful in the last active stage in the pipeline."]
    pub mod ARGB {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Color Space Conversion Config Register 0"]
pub mod LAYER_5_CSC_COEF0 {
    #[doc = "Two's compliment amplitude offset implicit in the Y data Y_OFFSET. For YUV, this is typically 0 and for YCbCr, this is typically -16 (0x1F0)."]
    pub mod Y_OFFSET {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Two's compliment phase offset implicit for CbCr data UV_OFFSET. Generally used for YCbCr to RGB conversion. YCbCr=0x180, YUV=0x000 (typically -128 or 0x180 to indicate normalized -0.5 to 0.5 range)."]
    pub mod UV_OFFSET {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Two's compliment Y multiplier coefficient C0. YUV=0x100 (1.000) YCbCr=0x12A (1.164)"]
    pub mod C0 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable the CSC unit in the LCDC plane data path. 0b - The CSC is bypassed and the input pixels are RGB data already 1b - The CSC is enabled and the pixels will be converted to RGB data This bit will be shadowed."]
    pub mod ENABLE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit changes the behavior when performing U/V converting. 0b - Converting YUV to RGB data 1b - Converting YCbCr to RGB data"]
    pub mod YCBCR_MODE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Color Space Conversion Config Register 1"]
pub mod LAYER_5_CSC_COEF1 {
    #[doc = "Two's compliment Blue U/Cb multiplier coefficient C4. YUV=0x208 (2.032) YCbCr=0x204 (2.017)."]
    pub mod C4 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Two's compliment Red V/Cr multiplier coefficient C1. YUV=0x123 (1.140) YCbCr=0x198 (1.596)."]
    pub mod C1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Color Space Conversion Config Register 2"]
pub mod LAYER_5_CSC_COEF2 {
    #[doc = "Two's compliment Green U/Cb multiplier coefficient C3. YUV=0x79C (-0.394) YCbCr=0x79C (-0.392)."]
    pub mod C3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Two's compliment Green V/Cr multiplier coefficient C2. YUV=0x76B (-0.581) YCbCr=0x730 (-0.813)."]
    pub mod C2 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Control Register"]
pub mod LAYER_6_LAYCTRL {
    #[doc = "Asserted when the layer is enabled. If this layer is not enabled, it means a bypassing plane."]
    pub mod EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Alpha Blending Mode 0: SKBlendMode_Clear; 1: SKBlendMode_Src ; 2: SKBlendMode_Dst 3: SKBlendMode_SrcOver 4: SKBlendMode_DstOver 5: SKBlendMode_SrcIn 6: SKBlendMode_DstIn 7: SKBlendMode_SrcOut 8: SKBlendMode_DstOut 9: SKBlendMode_SrcATop 10: SKBlendMode_DstATop 11: SKBlendMode_Xor 12: SKBlendMode_Plus (The conventional blending mode) 13: SKBlendMode_Modulate 14: SRC org 15: DST org Others: Reserved."]
    pub mod AB_MODE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The usage of the INALPHA\\[7:0\\]: (The system alpha value is not the data valid mask, the non-zero alpha value per pixel indicates a valid pixel. If no such per pixel alpha value, it means all the pixels are valid) 0: the INALPHA\\[7:0\\] is invalid, use the alpha value from previous pipeline 1: the INALPHA\\[7:0\\] is used to override the alpha value from previous pipeline. (useful when the corresponding data stream has no alpha info) 2: the INALPHA\\[7:0\\] is used to scale the alpha value from previous pipeline Others: Reserved"]
    pub mod INALPHA_OP {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The usage of the LOCALPHA\\[7:0\\]: (The system alpha value is not the data valid mask, the non-zero alpha value per pixel indicates a valid pixel. If no such per pixel alpha value, it means all the pixels are valid) 0: the LOCALPHA\\[7:0\\] is invalid, use the alpha value from the data stream 1: the LOCALPHA\\[7:0\\] is used to override the alpha value in the data stream (useful when the data stream has no alpha info) 2: the LOCALPHA\\[7:0\\] is used to scale the alpha value from the data stream Others: Reserved"]
    pub mod LOCALPHA_OP {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Layer encoding format (bit per pixel) 0000b - 1 bpp (pixel width must be multiples of 32), pixel sequence is from LSB to MSB in 32b word. 0001b - 2 bpp (pixel width must be multiples of 16), pixel sequence is from LSB to MSB in 32b word. 0010b - 4 bpp (pixel width must be multiples of 8), pixel sequence is from LSB to MSB in 32b word. 0011b - 8 bpp (pixel width must be multiples of 4), pixel sequence is from LSB to MSB in 32b word. 0100b - 16 bpp (RGB565), byte sequence as B,R 0111b - YCbCr422 (Only layer 0/1 can support this format), byte sequence determined by LAYCTRL\\[YUV_FORMAT\\] 1001b - 32 bpp (ARGB8888), byte sequence as B,G,R,A 1011b - Y8 (pixel width must be multiples of 4), byte sequence as Y1,Y2,Y3,Y4"]
    pub mod PIXFORMAT {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The YUV422 input format selection. 00b - The YVYU422 8bit sequence is U1,Y1,V1,Y2 01b - The YVYU422 8bit sequence is V1,Y1,U1,Y2 10b - The YVYU422 8bit sequence is Y1,U1,Y2,V1 11b - The YVYU422 8bit sequence is Y1,V1,Y2,U1 If not YUV422 mode, FORMAT\\[0\\]: asserted to exchange sequence inside the bytes. Org \\[15:8\\]-->New\\[8:15\\], Org \\[7:0\\]-->New\\[0:7\\]. (First exchange) FORMAT\\[1\\]: asserted to exchange the sequence of the odd and even 8 bits. Org Even \\[7:0\\]-->New\\[15:8\\], Org Odd \\[15:8\\]-->New\\[7:0\\]. (Second exchange)"]
    pub mod YUV_FORMAT {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Shadow Load Enable The SHADOW_LOAD_EN bit is written to 1 by software after all DMA control registers are written. If set to 1, shadowed control registers are updated to the active control registers on internal logical VSYNC of next frame. If set to 0, shadowed control registers are not loaded into the active control registers. The previous active control register settings will be used to process the next frame. Hardware will automatically clear this bit, when the shadow registers are loaded to the active control regsisters."]
    pub mod SHADOW_LOAD_EN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The byte sequence of the 4 bytes in a 32-bit word. 1: {A0, A1, A2, A3} byte re-ordered. 0: {A3, A2, A1, A0} the normal case with no byte re-order"]
    pub mod PACK_DIR {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Alpha Register"]
pub mod LAYER_6_ALPHAS {
    #[doc = "The system alpha value for the input stream from previous stage (DST)"]
    pub mod IND {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The system alpha value for the data stream of current layer stream (SRC)"]
    pub mod LOCD {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Size Register"]
pub mod LAYER_6_LAYSIZE {
    #[doc = "Width of the layer in pixels (Note: not actual width-1) The layer width must be in multiples of the number of pixels that can be stored in 32 bits, and therefore differs depending on color encoding. For example, if 2 bits per pixel format is used, then the layer width must be configured in multiples of 16."]
    pub mod WIDTH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Height of the layer in pixels"]
    pub mod HEIGHT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Position Register"]
pub mod LAYER_6_LAYPOS {
    #[doc = "The horizontal position of left-hand column of the layer, where 0 is the left-hand column of the panel, positive values are to the right the left-hand column of the panel."]
    pub mod X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The vertical position of top row of the layer, where 0 is the top row of the panel, positive values are below the top row of the panel."]
    pub mod Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Buffer Pointer Register"]
pub mod LAYER_6_START0 {
    #[doc = "Input buffer Start address 0"]
    pub mod ADDR0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Bus Config Register"]
pub mod LAYER_6_LINECFG {
    #[doc = "Number of bytes between 2 vertically adjacent pixels in system memory. Byte granularity is supported, but SW should align to 64B boundry."]
    pub mod PITCH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "the number of outstanding axi read transactions. If zero, it means max 8."]
    pub mod MAX_OT {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Maximal Per Transfer Data Size: 0: 64 bytes 1: 128 bytes 2: 256 bytes 3: 512 bytes 4: 1024 bytes"]
    pub mod MPT_SIZE {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Background Color Register"]
pub mod LAYER_6_BG_CL {
    #[doc = "ARGB8888. It is only useful in the last active stage in the pipeline."]
    pub mod ARGB {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Color Space Conversion Config Register 0"]
pub mod LAYER_6_CSC_COEF0 {
    #[doc = "Two's compliment amplitude offset implicit in the Y data Y_OFFSET. For YUV, this is typically 0 and for YCbCr, this is typically -16 (0x1F0)."]
    pub mod Y_OFFSET {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Two's compliment phase offset implicit for CbCr data UV_OFFSET. Generally used for YCbCr to RGB conversion. YCbCr=0x180, YUV=0x000 (typically -128 or 0x180 to indicate normalized -0.5 to 0.5 range)."]
    pub mod UV_OFFSET {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Two's compliment Y multiplier coefficient C0. YUV=0x100 (1.000) YCbCr=0x12A (1.164)"]
    pub mod C0 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable the CSC unit in the LCDC plane data path. 0b - The CSC is bypassed and the input pixels are RGB data already 1b - The CSC is enabled and the pixels will be converted to RGB data This bit will be shadowed."]
    pub mod ENABLE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit changes the behavior when performing U/V converting. 0b - Converting YUV to RGB data 1b - Converting YCbCr to RGB data"]
    pub mod YCBCR_MODE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Color Space Conversion Config Register 1"]
pub mod LAYER_6_CSC_COEF1 {
    #[doc = "Two's compliment Blue U/Cb multiplier coefficient C4. YUV=0x208 (2.032) YCbCr=0x204 (2.017)."]
    pub mod C4 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Two's compliment Red V/Cr multiplier coefficient C1. YUV=0x123 (1.140) YCbCr=0x198 (1.596)."]
    pub mod C1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Color Space Conversion Config Register 2"]
pub mod LAYER_6_CSC_COEF2 {
    #[doc = "Two's compliment Green U/Cb multiplier coefficient C3. YUV=0x79C (-0.394) YCbCr=0x79C (-0.392)."]
    pub mod C3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Two's compliment Green V/Cr multiplier coefficient C2. YUV=0x76B (-0.581) YCbCr=0x730 (-0.813)."]
    pub mod C2 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Control Register"]
pub mod LAYER_7_LAYCTRL {
    #[doc = "Asserted when the layer is enabled. If this layer is not enabled, it means a bypassing plane."]
    pub mod EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Alpha Blending Mode 0: SKBlendMode_Clear; 1: SKBlendMode_Src ; 2: SKBlendMode_Dst 3: SKBlendMode_SrcOver 4: SKBlendMode_DstOver 5: SKBlendMode_SrcIn 6: SKBlendMode_DstIn 7: SKBlendMode_SrcOut 8: SKBlendMode_DstOut 9: SKBlendMode_SrcATop 10: SKBlendMode_DstATop 11: SKBlendMode_Xor 12: SKBlendMode_Plus (The conventional blending mode) 13: SKBlendMode_Modulate 14: SRC org 15: DST org Others: Reserved."]
    pub mod AB_MODE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The usage of the INALPHA\\[7:0\\]: (The system alpha value is not the data valid mask, the non-zero alpha value per pixel indicates a valid pixel. If no such per pixel alpha value, it means all the pixels are valid) 0: the INALPHA\\[7:0\\] is invalid, use the alpha value from previous pipeline 1: the INALPHA\\[7:0\\] is used to override the alpha value from previous pipeline. (useful when the corresponding data stream has no alpha info) 2: the INALPHA\\[7:0\\] is used to scale the alpha value from previous pipeline Others: Reserved"]
    pub mod INALPHA_OP {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The usage of the LOCALPHA\\[7:0\\]: (The system alpha value is not the data valid mask, the non-zero alpha value per pixel indicates a valid pixel. If no such per pixel alpha value, it means all the pixels are valid) 0: the LOCALPHA\\[7:0\\] is invalid, use the alpha value from the data stream 1: the LOCALPHA\\[7:0\\] is used to override the alpha value in the data stream (useful when the data stream has no alpha info) 2: the LOCALPHA\\[7:0\\] is used to scale the alpha value from the data stream Others: Reserved"]
    pub mod LOCALPHA_OP {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Layer encoding format (bit per pixel) 0000b - 1 bpp (pixel width must be multiples of 32), pixel sequence is from LSB to MSB in 32b word. 0001b - 2 bpp (pixel width must be multiples of 16), pixel sequence is from LSB to MSB in 32b word. 0010b - 4 bpp (pixel width must be multiples of 8), pixel sequence is from LSB to MSB in 32b word. 0011b - 8 bpp (pixel width must be multiples of 4), pixel sequence is from LSB to MSB in 32b word. 0100b - 16 bpp (RGB565), byte sequence as B,R 0111b - YCbCr422 (Only layer 0/1 can support this format), byte sequence determined by LAYCTRL\\[YUV_FORMAT\\] 1001b - 32 bpp (ARGB8888), byte sequence as B,G,R,A 1011b - Y8 (pixel width must be multiples of 4), byte sequence as Y1,Y2,Y3,Y4"]
    pub mod PIXFORMAT {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The YUV422 input format selection. 00b - The YVYU422 8bit sequence is U1,Y1,V1,Y2 01b - The YVYU422 8bit sequence is V1,Y1,U1,Y2 10b - The YVYU422 8bit sequence is Y1,U1,Y2,V1 11b - The YVYU422 8bit sequence is Y1,V1,Y2,U1 If not YUV422 mode, FORMAT\\[0\\]: asserted to exchange sequence inside the bytes. Org \\[15:8\\]-->New\\[8:15\\], Org \\[7:0\\]-->New\\[0:7\\]. (First exchange) FORMAT\\[1\\]: asserted to exchange the sequence of the odd and even 8 bits. Org Even \\[7:0\\]-->New\\[15:8\\], Org Odd \\[15:8\\]-->New\\[7:0\\]. (Second exchange)"]
    pub mod YUV_FORMAT {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Shadow Load Enable The SHADOW_LOAD_EN bit is written to 1 by software after all DMA control registers are written. If set to 1, shadowed control registers are updated to the active control registers on internal logical VSYNC of next frame. If set to 0, shadowed control registers are not loaded into the active control registers. The previous active control register settings will be used to process the next frame. Hardware will automatically clear this bit, when the shadow registers are loaded to the active control regsisters."]
    pub mod SHADOW_LOAD_EN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The byte sequence of the 4 bytes in a 32-bit word. 1: {A0, A1, A2, A3} byte re-ordered. 0: {A3, A2, A1, A0} the normal case with no byte re-order"]
    pub mod PACK_DIR {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Alpha Register"]
pub mod LAYER_7_ALPHAS {
    #[doc = "The system alpha value for the input stream from previous stage (DST)"]
    pub mod IND {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The system alpha value for the data stream of current layer stream (SRC)"]
    pub mod LOCD {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Size Register"]
pub mod LAYER_7_LAYSIZE {
    #[doc = "Width of the layer in pixels (Note: not actual width-1) The layer width must be in multiples of the number of pixels that can be stored in 32 bits, and therefore differs depending on color encoding. For example, if 2 bits per pixel format is used, then the layer width must be configured in multiples of 16."]
    pub mod WIDTH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Height of the layer in pixels"]
    pub mod HEIGHT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Position Register"]
pub mod LAYER_7_LAYPOS {
    #[doc = "The horizontal position of left-hand column of the layer, where 0 is the left-hand column of the panel, positive values are to the right the left-hand column of the panel."]
    pub mod X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The vertical position of top row of the layer, where 0 is the top row of the panel, positive values are below the top row of the panel."]
    pub mod Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Buffer Pointer Register"]
pub mod LAYER_7_START0 {
    #[doc = "Input buffer Start address 0"]
    pub mod ADDR0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Bus Config Register"]
pub mod LAYER_7_LINECFG {
    #[doc = "Number of bytes between 2 vertically adjacent pixels in system memory. Byte granularity is supported, but SW should align to 64B boundry."]
    pub mod PITCH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "the number of outstanding axi read transactions. If zero, it means max 8."]
    pub mod MAX_OT {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Maximal Per Transfer Data Size: 0: 64 bytes 1: 128 bytes 2: 256 bytes 3: 512 bytes 4: 1024 bytes"]
    pub mod MPT_SIZE {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Background Color Register"]
pub mod LAYER_7_BG_CL {
    #[doc = "ARGB8888. It is only useful in the last active stage in the pipeline."]
    pub mod ARGB {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Color Space Conversion Config Register 0"]
pub mod LAYER_7_CSC_COEF0 {
    #[doc = "Two's compliment amplitude offset implicit in the Y data Y_OFFSET. For YUV, this is typically 0 and for YCbCr, this is typically -16 (0x1F0)."]
    pub mod Y_OFFSET {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Two's compliment phase offset implicit for CbCr data UV_OFFSET. Generally used for YCbCr to RGB conversion. YCbCr=0x180, YUV=0x000 (typically -128 or 0x180 to indicate normalized -0.5 to 0.5 range)."]
    pub mod UV_OFFSET {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Two's compliment Y multiplier coefficient C0. YUV=0x100 (1.000) YCbCr=0x12A (1.164)"]
    pub mod C0 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable the CSC unit in the LCDC plane data path. 0b - The CSC is bypassed and the input pixels are RGB data already 1b - The CSC is enabled and the pixels will be converted to RGB data This bit will be shadowed."]
    pub mod ENABLE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit changes the behavior when performing U/V converting. 0b - Converting YUV to RGB data 1b - Converting YCbCr to RGB data"]
    pub mod YCBCR_MODE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Color Space Conversion Config Register 1"]
pub mod LAYER_7_CSC_COEF1 {
    #[doc = "Two's compliment Blue U/Cb multiplier coefficient C4. YUV=0x208 (2.032) YCbCr=0x204 (2.017)."]
    pub mod C4 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Two's compliment Red V/Cr multiplier coefficient C1. YUV=0x123 (1.140) YCbCr=0x198 (1.596)."]
    pub mod C1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Color Space Conversion Config Register 2"]
pub mod LAYER_7_CSC_COEF2 {
    #[doc = "Two's compliment Green U/Cb multiplier coefficient C3. YUV=0x79C (-0.394) YCbCr=0x79C (-0.392)."]
    pub mod C3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Two's compliment Green V/Cr multiplier coefficient C2. YUV=0x76B (-0.581) YCbCr=0x730 (-0.813)."]
    pub mod C2 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clut Load Control Register"]
pub mod CLUT_LOAD {
    #[doc = "CLUT Update Enable The bit is written to 1 when software want to update the Color Look Up Tables during display. If set to 1, software update selected CLUT due to SEL_CLUT_NUM setting, the table will be copied from CLUT8 during vertical blanking period after SHADOW_LOAD_EN is set to 1. If set to 0, software can update CLUT8 directly according to the CLUT memory map. Hardware will automatically clear this bit when selected CLUT is updated according to SEL_CLUT_NUM."]
    pub mod UPDATE_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Selected CLUT Number The SEL_CLUT_NUM is used to select which plane's CLUT need to be updated. The hardware can only backup one CLUT setting and load, so the SEL_CLUT_NUM can't be changed when CLUT_LOAD\\[UPDATE_EN\\] is 1. . 3'h0 - PLANE 0 . 3'h1 - PLANE 1 . ------ . 3'h7 - PLANE 7 CLUT 8 can be modified via APB even when display is on. Currently CLUT for plane 0..7 cannot be modified via APB when display is on. Can only be updated via CLUT_LOAD\\[UPDATE_EN\\] bit."]
    pub mod SEL_NUM {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
