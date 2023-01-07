#[doc = "PDMA"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Control Register"]
    pub CTRL: crate::RWRegister<u32>,
    #[doc = "Status Register"]
    pub STAT: crate::RWRegister<u32>,
    #[doc = "Out Layer Control Register"]
    pub OUT_CTRL: crate::RWRegister<u32>,
    #[doc = "Output buffer address"]
    pub OUT_BUF: crate::RWRegister<u32>,
    _reserved0: [u8; 0x04],
    #[doc = "Outlayer Pitch Register"]
    pub OUT_PITCH: crate::RWRegister<u32>,
    #[doc = "Output Lower Right Corner Register"]
    pub OUT_LRC: crate::RWRegister<u32>,
    #[doc = "Layer Upper Left Corner Register"]
    pub OUT_PS_0_ULC: crate::RWRegister<u32>,
    #[doc = "Layer Lower Right Corner Register"]
    pub OUT_PS_0_LRC: crate::RWRegister<u32>,
    #[doc = "Layer Upper Left Corner Register"]
    pub OUT_PS_1_ULC: crate::RWRegister<u32>,
    #[doc = "Layer Lower Right Corner Register"]
    pub OUT_PS_1_LRC: crate::RWRegister<u32>,
    _reserved1: [u8; 0x04],
    #[doc = "Layer Control Register"]
    pub PS_0_CTRL: crate::RWRegister<u32>,
    #[doc = "Layer data buffer address"]
    pub PS_0_BUF: crate::RWRegister<u32>,
    _reserved2: [u8; 0x08],
    #[doc = "Layer data pitch register"]
    pub PS_0_PITCH: crate::RWRegister<u32>,
    #[doc = "Layer background color register"]
    pub PS_0_BKGD: crate::RWRegister<u32>,
    #[doc = "Layer scale register"]
    pub PS_0_SCALE: crate::RWRegister<u32>,
    #[doc = "Layer offset register"]
    pub PS_0_OFFSET: crate::RWRegister<u32>,
    #[doc = "Layer low color key register"]
    pub PS_0_CLRKEY_LOW: crate::RWRegister<u32>,
    #[doc = "Layer high color key register"]
    pub PS_0_CLRKEY_HIGH: crate::RWRegister<u32>,
    #[doc = "Layer original size register"]
    pub PS_0_ORG: crate::RWRegister<u32>,
    _reserved3: [u8; 0x04],
    #[doc = "layer control register"]
    pub PS_1_CTRL: crate::RWRegister<u32>,
    #[doc = "Layer data buffer address"]
    pub PS_1_BUF: crate::RWRegister<u32>,
    _reserved4: [u8; 0x08],
    #[doc = "Layer data pitch register"]
    pub PS_1_PITCH: crate::RWRegister<u32>,
    #[doc = "Layer background color register"]
    pub PS_1_BKGD: crate::RWRegister<u32>,
    #[doc = "Layer scale register"]
    pub PS_1_SCALE: crate::RWRegister<u32>,
    #[doc = "Layer offset register"]
    pub PS_1_OFFSET: crate::RWRegister<u32>,
    #[doc = "Layer low color key register"]
    pub PS_1_CLRKEY_LOW: crate::RWRegister<u32>,
    #[doc = "Layer high color key register"]
    pub PS_1_CLRKEY_HIGH: crate::RWRegister<u32>,
    #[doc = "Layer original size register"]
    pub PS_1_ORG: crate::RWRegister<u32>,
    _reserved5: [u8; 0x14],
    #[doc = "YUV2RGB coefficients register 0"]
    pub YUV2RGB_COEF0: crate::RWRegister<u32>,
    #[doc = "YUV2RGB coefficients register 1"]
    pub YUV2RGB_COEF1: crate::RWRegister<u32>,
    #[doc = "YUV2RGB coefficients register 2"]
    pub YUV2RGB_COEF2: crate::RWRegister<u32>,
    #[doc = "RGB2YUV coefficients register 0"]
    pub RGB2YUV_COEF0: crate::RWRegister<u32>,
    #[doc = "RGB2YUV coefficients register 1"]
    pub RGB2YUV_COEF1: crate::RWRegister<u32>,
    #[doc = "RGB2YUV coefficients register 2"]
    pub RGB2YUV_COEF2: crate::RWRegister<u32>,
    #[doc = "RGB2YUV coefficients register 3"]
    pub RGB2YUV_COEF3: crate::RWRegister<u32>,
    #[doc = "RGB2YUV coefficients register 4"]
    pub RGB2YUV_COEF4: crate::RWRegister<u32>,
}
#[doc = "Control Register"]
pub mod CTRL {
    #[doc = "1b - Enabled"]
    pub mod PDMA_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Software Reset. Write 1 to clear PDMA internal logic. Write 0 to exit software reset mode."]
    pub mod PDMA_SFTRST {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Plane 0 Enable"]
    pub mod P0_EN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Plane 1 Enable"]
    pub mod P1_EN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Asserted when the Block Size is 16x16, else 8x8"]
    pub mod BS16 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable normal interrupt"]
    pub mod IRQ_EN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Assert this bit to gate off clock when the module is not working. If reset to zero, the internal clock is always on."]
    pub mod CLKGATE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable interrupt of PDMA_DONE"]
    pub mod PDMA_DONE_IRQ_EN {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable interrupt of AXI bus error"]
    pub mod AXIERR_IRQ_EN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Decide the byte sequence of the 32-bit output word {A3, A2, A1, A0}. The bit sequence ina byte is not changed. 2'b00: no change {A3, A2, A1, A0} 2'b01: {A2, A3, A0, A1} 2'b10: {A1, A0, A3, A2} 2'b11: {A0, A1, A2, A3}"]
    pub mod PACK_DIR {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "QoS for AXI write bus"]
    pub mod AWQOS {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "QoS for AXI read bus"]
    pub mod ARQOS {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status Register"]
pub mod STAT {
    #[doc = "Asserted to indicate a IRQ event"]
    pub mod IRQ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AXI0 read err"]
    pub mod AXI_0_READ_ERR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AXI1 read err"]
    pub mod AXI_1_READ_ERR {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AXI0 write err"]
    pub mod AXI_0_WRITE_ERR {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AXI error ID"]
    pub mod AXI_ERR_ID {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PDMA one image done"]
    pub mod PDMA_DONE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "X block that is processing"]
    pub mod BLOCKX {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Y block that is processing"]
    pub mod BLOCKY {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Out Layer Control Register"]
pub mod OUT_CTRL {
    #[doc = "Output buffer format. 0x0 ARGB8888 - 32-bit pixles, byte sequence as B,G,R,A 0xE RGB565 - 16-bit pixels, byte sequence as B,R 0x12 UYVY1P422 - 16-bit pixels (1-plane , byte sequence as U0,Y0,V0,Y1)"]
    pub mod FORMAT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Alpha Blending Mode 0: SKBlendMode_Clear (If PS1_CTRL\\[BKGNDCL4CLR\\] is asserted, use PS1_BKGRND color to fill the range determined by PS1, else fill the range determined by PS1 with zero); 1: SKBlendMode_Src ; 2: SKBlendMode_Dst 3: SKBlendMode_SrcOver 4: SKBlendMode_DstOver 5: SKBlendMode_SrcIn 6: SKBlendMode_DstIn 7: SKBlendMode_SrcOut 8: SKBlendMode_DstOut 9: SKBlendMode_SrcATop 10: SKBlendMode_DstATop 11: SKBlendMode_Xor 12: SKBlendMode_Plus (The conventional belding mode) 13: SKBlendMode_Modulate 14: SRC org 15: DST org Others: Reserved."]
    pub mod ABLEND_MODE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The usage of the SRCALPHA\\[7:0\\]: (The system alpha value is not the data valid mask, the non-zero alpha value per pixel embedded in the stream indicates a valid pixel. If no such per pixel alpha value, it means all the pixels are valid) 0: the SRCALPHA\\[7:0\\] is invalid, use the alpha value embedded in the stream 1: the SRCALPHA\\[7:0\\] is used to override the alpha value embedded in the stream . (useful when the corresponding data stream has no alpha info) 2: the SRCALPHA\\[7:0\\] is used to scale the alpha value embedded in the stream Others: Reserved"]
    pub mod SRCALPHA_OP {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The usage of the DSTALPHA\\[7:0\\]: (The system alpha value is not the data valid mask, the non-zero alpha value per pixel embedded in the stream indicates a valid pixel. If no such per pixel alpha value, it means all the pixels are valid) 0: the DSTALPHA\\[7:0\\] is invalid, use the alpha value embedded in the stream 1: the DSTALPHA\\[7:0\\] is used to override the alpha value embedded in the stream. (useful when the corresponding data stream has no alpha info) 2: the DSTALPHA\\[7:0\\] is used to scale the alpha value embedded in the stream Others: Reserved"]
    pub mod DSTALPHA_OP {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The source (P0) system ALPHA value."]
    pub mod SRCALPHA {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The destination (P1) system ALPHA value."]
    pub mod DSTALPHA {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Output buffer address"]
pub mod OUT_BUF {
    #[doc = "Current address pointer for the output frame buffer. The address can have any byte alignment. 64B alignment is recommended for optimal performance."]
    pub mod ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Outlayer Pitch Register"]
pub mod OUT_PITCH {
    #[doc = "Indicates the number of bytes in memory between two vertically adjacent pixels."]
    pub mod BYTELEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Output Lower Right Corner Register"]
pub mod OUT_LRC {
    #[doc = "This field indicates the lower right X-coordinate (in pixels) of the output frame buffer. Should be the width of the output image size."]
    pub mod X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This field indicates the lower right Y-coordinate (in pixels) of the output frame buffer. The value is the height of the output image size."]
    pub mod Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x3fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Upper Left Corner Register"]
pub mod OUT_PS_0_ULC {
    #[doc = "This field indicates the upper left X-coordinate (in pixels) of the processed surface in the output frame buffer."]
    pub mod X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This field indicates the upper left Y-coordinate (in pixels) of the processed surface in the output frame buffer."]
    pub mod Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x3fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Lower Right Corner Register"]
pub mod OUT_PS_0_LRC {
    #[doc = "This field indicates the lower right X-coordinate (in pixels) of the processed surface in the output frame buffer."]
    pub mod X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This field indicates the lower right Y-coordinate (in pixels) of the processed surface in the output frame buffer."]
    pub mod Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x3fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Upper Left Corner Register"]
pub mod OUT_PS_1_ULC {
    #[doc = "This field indicates the upper left X-coordinate (in pixels) of the processed surface in the output frame buffer."]
    pub mod X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This field indicates the upper left Y-coordinate (in pixels) of the processed surface in the output frame buffer."]
    pub mod Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x3fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Lower Right Corner Register"]
pub mod OUT_PS_1_LRC {
    #[doc = "This field indicates the lower right X-coordinate (in pixels) of the processed surface in the output frame buffer."]
    pub mod X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This field indicates the lower right Y-coordinate (in pixels) of the processed surface in the output frame buffer."]
    pub mod Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x3fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer Control Register"]
pub mod PS_0_CTRL {
    #[doc = "PS buffer format. To select between YUV and YCbCr formats, see bit 16 of this register. 0x0 ARGB888 - 32-bit pixels, byte sequence as B,G,R,A 0xE RGB565 - 16-bit pixels, byte sequence as B,R 0x13 YUYV1P422 - 16-bit pixels (1-plane byte sequence Y0,U0,Y1,V0 interleaved bytes)"]
    pub mod FORMAT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Swap bytes in half-words. For each 16 bit half-word, the two bytes will be swapped."]
    pub mod HW_BYTE_SWAP {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Horizontal pre decimation filter control. 0x0 DISABLE - Disable pre-decimation filter. 0x1 DECX2 - Decimate PS by 2. 0x2 DECX4 - Decimate PS by 4. 0x3 DECX8 - Decimate PS by 8."]
    pub mod DECX {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Verticle pre decimation filter control. 0x0 DISABLE - Disable pre-decimation filter. 0x1 DECY2 - Decimate PS by 2. 0x2 DECY4 - Decimate PS by 4. 0x3 DECY8 - Decimate PS by 8."]
    pub mod DECY {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates the clockwise rotation to be applied at the input buffer. The rotation effect is defined as occurring after the FLIP_X and FLIP_Y permutation. 0x0 ROT_0 0x1 ROT_90 0x2 ROT_180 0x3 ROT_270"]
    pub mod ROTATE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates that the input should be flipped horizontally (effect applied before rotation)."]
    pub mod HFLIP {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates that the input should be flipped vertically (effect applied before rotation)."]
    pub mod VFLIP {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Asserted to bypass the CSC stage"]
    pub mod BYPASS {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "YCbCr mode or YUV mode"]
    pub mod YCBCR_MODE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable to use background color for clear area"]
    pub mod BKGCL4CLR {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Decide the byte sequence of the 32-bit word {A3, A2, A1, A0}. The bit sequence ina byte is not changed. 2'b00: no change {A3, A2, A1, A0} 2'b01: {A2, A3, A0, A1} 2'b10: {A1, A0, A3, A2} 2'b11: {A0, A1, A2, A3}"]
    pub mod PACK_DIR {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Swap bit\\[31:24\\] and bit \\[15:8\\] before pack_dir operation."]
    pub mod INB13_SWAP {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer data buffer address"]
pub mod PS_0_BUF {
    #[doc = "Address pointer for the PS RGB or Y (luma) input buffer."]
    pub mod ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer data pitch register"]
pub mod PS_0_PITCH {
    #[doc = "Indicates the number of bytes in memory between two vertically adjacent pixels."]
    pub mod BYTELEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer background color register"]
pub mod PS_0_BKGD {
    #[doc = "Background color (in 32bpp format) for any pixels not within the scaled range of the picture, but within the buffer range specified by the PS ULC/LRC. The top 8-bit is the alpha channel."]
    pub mod COLOR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer scale register"]
pub mod PS_0_SCALE {
    #[doc = "This is a two bit integer and 12 bit fractional representation (##.####_####_####) of the Y scaling factor for the PS source buffer. The maximum value programmed should be 2 since scaling down by a factor greater than 2 is not supported with the bilinear filter. Decimation and the bilinear filter should be used together to achieve scaling by more than a factor of 2."]
    pub mod X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This is a two bit integer and 12 bit fractional representation (##.####_####_####) of the X scaling factor for the PS source buffer. The maximum value programmed should be 2 since scaling down by a factor greater than 2 is not supported with the bilinear filter. Decimation and the bilinear filter should be used together to achieve scaling by more than a factor of 2."]
    pub mod Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x7fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer offset register"]
pub mod PS_0_OFFSET {
    #[doc = "This is a 12 bit fractional representation (0.####_####_####) of the X scaling offset. This represents a fixed pixel offset which gets added to the scaled address to determine source data for the scaling engine. It is applied after the decimation filter stage, and before the bilinear filter stage."]
    pub mod X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This is a 12 bit fractional representation (0.####_####_####) of the Y scaling offset. This represents a fixed pixel offset which gets added to the scaled address to determine source data for the scaling engine. It is applied after the decimation filter stage, and before the bilinear filter stage."]
    pub mod Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer low color key register"]
pub mod PS_0_CLRKEY_LOW {
    #[doc = "Low range of color key applied to PS buffer. To disable PS colorkeying, set the low colorkey to 0xFFFFFF and the high colorkey to 0x000000."]
    pub mod LIMIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer high color key register"]
pub mod PS_0_CLRKEY_HIGH {
    #[doc = "High range of color key applied to PS buffer. To disable PS colorkeying, set the low colorkey to 0xFFFFFF and the high colorkey to 0x000000"]
    pub mod LIMIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer original size register"]
pub mod PS_0_ORG {
    #[doc = "The number of horizontal pixels of the original frame (not -1)"]
    pub mod WIDTH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The number of vertical pixels of the original frame (not -1)"]
    pub mod HIGHT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x3fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "layer control register"]
pub mod PS_1_CTRL {
    #[doc = "PS buffer format. To select between YUV and YCbCr formats, see bit 16 of this register. 0x0 ARGB888 - 32-bit pixels, byte sequence as B,G,R,A 0xE RGB565 - 16-bit pixels, byte sequence as B,R 0x13 YUYV1P422 - 16-bit pixels (1-plane byte sequence Y0,U0,Y1,V0 interleaved bytes)"]
    pub mod FORMAT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Swap bytes in half-words. For each 16 bit half-word, the two bytes will be swapped."]
    pub mod HW_BYTE_SWAP {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Horizontal pre decimation filter control. 0x0 DISABLE - Disable pre-decimation filter. 0x1 DECX2 - Decimate PS by 2. 0x2 DECX4 - Decimate PS by 4. 0x3 DECX8 - Decimate PS by 8."]
    pub mod DECX {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Verticle pre decimation filter control. 0x0 DISABLE - Disable pre-decimation filter. 0x1 DECY2 - Decimate PS by 2. 0x2 DECY4 - Decimate PS by 4. 0x3 DECY8 - Decimate PS by 8."]
    pub mod DECY {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates the clockwise rotation to be applied at the input buffer. The rotation effect is defined as occurring after the FLIP_X and FLIP_Y permutation. 0x0 ROT_0 0x1 ROT_90 0x2 ROT_180 0x3 ROT_270"]
    pub mod ROTATE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates that the input should be flipped horizontally (effect applied before rotation)."]
    pub mod HFLIP {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates that the input should be flipped vertically (effect applied before rotation)."]
    pub mod VFLIP {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Asserted to bypass the CSC stage"]
    pub mod BYPASS {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "YCbCr mode or YUV mode"]
    pub mod YCBCR_MODE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable to use background color for clear area"]
    pub mod BKGCL4CLR {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Decide the byte sequence of the 32-bit word {A3, A2, A1, A0}. The bit sequence ina byte is not changed. 2'b00: no change {A3, A2, A1, A0} 2'b01: {A2, A3, A0, A1} 2'b10: {A1, A0, A3, A2} 2'b11: {A0, A1, A2, A3}"]
    pub mod PACK_DIR {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Swap bit\\[31:24\\] and bit \\[15:8\\] before pack_dir operation."]
    pub mod INB13_SWAP {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer data buffer address"]
pub mod PS_1_BUF {
    #[doc = "Address pointer for the PS RGB or Y (luma) input buffer."]
    pub mod ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer data pitch register"]
pub mod PS_1_PITCH {
    #[doc = "Indicates the number of bytes in memory between two vertically adjacent pixels."]
    pub mod BYTELEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer background color register"]
pub mod PS_1_BKGD {
    #[doc = "Background color (in 32bpp format) for any pixels not within the scaled range of the picture, but within the buffer range specified by the PS ULC/LRC. The top 8-bit is the alpha channel."]
    pub mod COLOR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer scale register"]
pub mod PS_1_SCALE {
    #[doc = "This is a two bit integer and 12 bit fractional representation (##.####_####_####) of the Y scaling factor for the PS source buffer. The maximum value programmed should be 2 since scaling down by a factor greater than 2 is not supported with the bilinear filter. Decimation and the bilinear filter should be used together to achieve scaling by more than a factor of 2."]
    pub mod X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This is a two bit integer and 12 bit fractional representation (##.####_####_####) of the X scaling factor for the PS source buffer. The maximum value programmed should be 2 since scaling down by a factor greater than 2 is not supported with the bilinear filter. Decimation and the bilinear filter should be used together to achieve scaling by more than a factor of 2."]
    pub mod Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x7fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer offset register"]
pub mod PS_1_OFFSET {
    #[doc = "This is a 12 bit fractional representation (0.####_####_####) of the X scaling offset. This represents a fixed pixel offset which gets added to the scaled address to determine source data for the scaling engine. It is applied after the decimation filter stage, and before the bilinear filter stage."]
    pub mod X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This is a 12 bit fractional representation (0.####_####_####) of the Y scaling offset. This represents a fixed pixel offset which gets added to the scaled address to determine source data for the scaling engine. It is applied after the decimation filter stage, and before the bilinear filter stage."]
    pub mod Y {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer low color key register"]
pub mod PS_1_CLRKEY_LOW {
    #[doc = "Low range of color key applied to PS buffer. To disable PS colorkeying, set the low colorkey to 0xFFFFFF and the high colorkey to 0x000000."]
    pub mod LIMIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer high color key register"]
pub mod PS_1_CLRKEY_HIGH {
    #[doc = "High range of color key applied to PS buffer. To disable PS colorkeying, set the low colorkey to 0xFFFFFF and the high colorkey to 0x000000"]
    pub mod LIMIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Layer original size register"]
pub mod PS_1_ORG {
    #[doc = "The number of horizontal pixels of the original frame (not -1)"]
    pub mod WIDTH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The number of vertical pixels of the original frame (not -1)"]
    pub mod HIGHT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x3fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "YUV2RGB coefficients register 0"]
pub mod YUV2RGB_COEF0 {
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
}
#[doc = "YUV2RGB coefficients register 1"]
pub mod YUV2RGB_COEF1 {
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
#[doc = "YUV2RGB coefficients register 2"]
pub mod YUV2RGB_COEF2 {
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
#[doc = "RGB2YUV coefficients register 0"]
pub mod RGB2YUV_COEF0 {
    #[doc = "CSC parameters Y_OFFSET"]
    pub mod Y_OFFSET {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CSC parameters UV_OFFSET"]
    pub mod UV_OFFSET {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CSC parameters C0"]
    pub mod C0 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Asserted to enable this RGB2YUV CSC stage"]
    pub mod ENABLE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Asserted to use YCrCb mode"]
    pub mod YCBCR_MODE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "RGB2YUV coefficients register 1"]
pub mod RGB2YUV_COEF1 {
    #[doc = "CSC parameters C4"]
    pub mod C4 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CSC parameters C1"]
    pub mod C1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "RGB2YUV coefficients register 2"]
pub mod RGB2YUV_COEF2 {
    #[doc = "CSC parameters C3"]
    pub mod C3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CSC parameters C2"]
    pub mod C2 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "RGB2YUV coefficients register 3"]
pub mod RGB2YUV_COEF3 {
    #[doc = "CSC parameters C5"]
    pub mod C5 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CSC parameters C6"]
    pub mod C6 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "RGB2YUV coefficients register 4"]
pub mod RGB2YUV_COEF4 {
    #[doc = "CSC parameters C7"]
    pub mod C7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CSC parameters C8"]
    pub mod C8 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
