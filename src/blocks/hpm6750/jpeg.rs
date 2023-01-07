#[doc = "JPEG"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "In DMA Misc Control Register"]
    pub INDMA_MISC: crate::RWRegister<u32>,
    #[doc = "In DMA Buf Address"]
    pub INDMABASE: crate::RWRegister<u32>,
    _reserved0: [u8; 0x04],
    #[doc = "In DMA Buf Control 0 Register"]
    pub INDMA_CTRL0: crate::RWRegister<u32>,
    #[doc = "In DMA Buf Control 1 Register"]
    pub INDMA_CTRL1: crate::RWRegister<u32>,
    #[doc = "In DMA Next Command Register"]
    pub INXT_CMD: crate::RWRegister<u32>,
    _reserved1: [u8; 0x08],
    #[doc = "Out DMA Misc Control Register"]
    pub OUTDMA_MISC: crate::RWRegister<u32>,
    #[doc = "Out DMA Buf Address"]
    pub OUTDMABASE: crate::RWRegister<u32>,
    _reserved2: [u8; 0x04],
    #[doc = "Out DMA Buf Control 0 Register"]
    pub OUTDMA_CTRL0: crate::RWRegister<u32>,
    #[doc = "Out DMA Buf Control 1 Register"]
    pub OUTDMA_CTRL1: crate::RWRegister<u32>,
    #[doc = "Out DMA Next Command Register"]
    pub ONXT_CMD: crate::RWRegister<u32>,
    _reserved3: [u8; 0x08],
    #[doc = "Configuration Register"]
    pub CFG: crate::RWRegister<u32>,
    #[doc = "Status Register"]
    pub STAT: crate::RWRegister<u32>,
    #[doc = "Image width register"]
    pub WIDTH: crate::RWRegister<u32>,
    #[doc = "Image height register"]
    pub HEIGHT: crate::RWRegister<u32>,
    #[doc = "Buf Access Addr"]
    pub BUFADDR: crate::RWRegister<u32>,
    #[doc = "Buf Access Data"]
    pub BUFDATA: crate::RWRegister<u32>,
    #[doc = "Out DMA Bytes Counter"]
    pub OUTDMACNT: crate::RWRegister<u32>,
    #[doc = "YUV2RGB coefficients Register 0"]
    pub CSC_COEF0: crate::RWRegister<u32>,
    #[doc = "YUV2RGB coefficients Register 1"]
    pub CSC_COEF1: crate::RWRegister<u32>,
    #[doc = "YUV2RGB coefficients Register 2"]
    pub CSC_COEF2: crate::RWRegister<u32>,
    #[doc = "RGB2YUV coefficients Register 0"]
    pub RGB2YUV_COEF0: crate::RWRegister<u32>,
    #[doc = "RGB2YUV coefficients Register 1"]
    pub RGB2YUV_COEF1: crate::RWRegister<u32>,
    #[doc = "RGB2YUV coefficients Register 2"]
    pub RGB2YUV_COEF2: crate::RWRegister<u32>,
    #[doc = "RGB2YUV coefficients Register 3"]
    pub RGB2YUV_COEF3: crate::RWRegister<u32>,
    #[doc = "RGB2YUV coefficients Register 4"]
    pub RGB2YUV_COEF4: crate::RWRegister<u32>,
    _reserved4: [u8; 0x08],
    #[doc = "Image Control Register 1"]
    pub IMGREG1: crate::RWRegister<u32>,
    #[doc = "Image Control Register 2"]
    pub IMGREG2: crate::RWRegister<u32>,
    #[doc = "Image Control Register 3"]
    pub IMGREG3: crate::RWRegister<u32>,
    #[doc = "Image Control Register 40"]
    pub IMGREG_REG40: crate::RWRegister<u32>,
    #[doc = "Image Control Register 41"]
    pub IMGREG_REG41: crate::RWRegister<u32>,
    #[doc = "Image Control Register 42"]
    pub IMGREG_REG42: crate::RWRegister<u32>,
    #[doc = "Image Control Register 43"]
    pub IMGREG_REG43: crate::RWRegister<u32>,
}
#[doc = "In DMA Misc Control Register"]
pub mod INDMA_MISC {
    #[doc = "Asserted if In_DMA_ID=Pixel."]
    pub mod INDMA2D {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Asserted to request DMA. Automatically clear after DMA is done."]
    pub mod IN_DMA_REQ {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "0: Pixel (In) 1: ECS (In) 2: Qmem 3: HuffEnc 4: HuffMin 5: HuffBase 6: HuffSymb"]
    pub mod IN_DMA_ID {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "interrupt enable for all interrupt sources of In DMA module"]
    pub mod IRQ_EN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "In DMA axi bus error inetrrupt enable"]
    pub mod AXI_ERR_IRQ_EN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "In DMA Done enable"]
    pub mod IN_DMA_DONE_IRQ_EN {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "In DMA Next Interrupt Enable"]
    pub mod NXT_IRQ_EN {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Renew In DMA. Default is to continue the write address counter when a new DMA request comes. Asserted to reset the write address counter."]
    pub mod INDMA_RENEW {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Decide the byte sequence of the 32-bit word {A3, A2, A1, A0}. The bit sequence in a byte is not changed. Only work for pixel data. 2'b00: no change {A3, A2, A1, A0} 2'b01: {A2, A3, A0, A1} 2'b10: {A1, A0, A3, A2} 2'b11: {A0, A1, A2, A3}"]
    pub mod PACK_DIR {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Swap bit\\[31:24\\] and bit \\[15:8\\] before pack dir operation. Only work for pixel data."]
    pub mod INB13_SWAP {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "max_ot when input are RGB pixels. For 16 bits per pixel, it can be set as 4. For 32 bits per pixel, it will be set as 2."]
    pub mod MAX_OT {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "QoS for AXI read channel"]
    pub mod ARQOS {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "In DMA Buf Address"]
pub mod INDMABASE {
    #[doc = "Y plane (or Encoded Bit Plane)"]
    pub mod ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "In DMA Buf Control 0 Register"]
pub mod INDMA_CTRL0 {
    #[doc = "Pitch between the starting point of Rows. Only active when In_DMA_ID=Pixel.."]
    pub mod PITCH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Total length (Low 16 bits) in Bytes -1 for transfer when In_DMA_ID!=Pixel."]
    pub mod TTLEN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "In DMA Buf Control 1 Register"]
pub mod INDMA_CTRL1 {
    #[doc = "Total length (High 16 bits) in Bytes -1 for transfer. See reference in InDMA_Ctrl0\\[TTLEN\\]"]
    pub mod ROWLEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "In DMA Next Command Register"]
pub mod INXT_CMD {
    #[doc = "NXTCMD phase Enable Bit"]
    pub mod EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "asserted if there is either a DATA DMA phase or NXTCMD phase. Automatically cleared. Will trigger the InDMA transfer if CFG\\[JPEG_EN\\] is 1."]
    pub mod OP_VALID {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The address pointing to the next command"]
    pub mod ADDR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x3fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Out DMA Misc Control Register"]
pub mod OUTDMA_MISC {
    #[doc = "Asserted if Out_DMA_ID==Pixel"]
    pub mod OUTDMA2D {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Asserted to enable Out DMA request"]
    pub mod OUT_DMA_REQ {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "0: Pixel (Out) 1: ECS (Out)"]
    pub mod OUT_DMA_ID {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "interrupt enable for all interrupt sources of Out DMA module"]
    pub mod IRQ_EN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Out DMA axi bus error inetrrupt enable"]
    pub mod AXI_ERR_IRQ_EN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Out DMA Done interrupt Enable"]
    pub mod OUT_DMA_DONE_IRQ_EN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Out DMA Next Interrupt Enable"]
    pub mod NXT_IRQ_EN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Add 0xFFD9 to the ending of the odma stream when all original image pixels are processed by the encoder module."]
    pub mod ADD_ODMA_ENDINGS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Asserted to ini output counter"]
    pub mod INI_OUTCNT {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable output counter (unit as bytes)"]
    pub mod EN_OUTCNT {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Decide the byte sequence of the 32-bit word {A3, A2, A1, A0}. The bit sequence in a byte is not changed. All outdma data are impacted. 2'b00: no change {A3, A2, A1, A0} (This is used for ecs stream) 2'b01: {A2, A3, A0, A1} 2'b10: {A1, A0, A3, A2} 2'b11: {A0, A1, A2, A3}"]
    pub mod PACK_DIR {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "No description avaiable"]
    pub mod AWQOS {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Out DMA Buf Address"]
pub mod OUTDMABASE {
    #[doc = "Y plane (or Encoded Bit Plane)"]
    pub mod ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Out DMA Buf Control 0 Register"]
pub mod OUTDMA_CTRL0 {
    #[doc = "Pitch between the starting point of Rows when Out_DMA_ID==Pixel"]
    pub mod PITCH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Total length (Low 16 bits) in Bytes -1 for transfer when Out_DMA_ID!=Pixel. If Out_DMA_ID=ECS, it can be any value greater than the length of the ECS, for example, the number of encoded bytes."]
    pub mod TTLEN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Out DMA Buf Control 1 Register"]
pub mod OUTDMA_CTRL1 {
    #[doc = "Total length (High 16 bits) in Bytes -1 for transfer. See reference in OutDMA_Ctrl0\\[TTLEN\\]"]
    pub mod ROWLEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Out DMA Next Command Register"]
pub mod ONXT_CMD {
    #[doc = "NXTCMD phase Enable Bit"]
    pub mod EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "asserted if there is either a DATA DMA phase or NXTCMD phase. Automatically cleared. Will trigger the OutDMA and NXTCMD phase transfer if CFG\\[JPEG_EN\\] is 1."]
    pub mod OP_VALID {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The address pointing to the next command"]
    pub mod ADDR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x3fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Configuration Register"]
pub mod CFG {
    #[doc = "1b - Enabled"]
    pub mod JPEG_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1: decoder, 0:encoder"]
    pub mod MODE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Asserted if to start a new encoder/decoder conversion. It will at first stop the inner JPEG module, then reset it, and then re-run it. It is a different mode from DMA phase mode. It cannot be configured in the DMA chain descriptor. It should be configured by the core processor. Auto clear."]
    pub mod START {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Software Reset"]
    pub mod JPEG_SFTRST {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "3'b000: for 420, hy=2, vy=2, hc=1, vc=1 // 6 sub-blocks per MCU 3'b001: for 422h, hy=2, vy=1, hc=1, vc=1 // 4 sub-blocks per MCU 3'b010: for 422v, hy=1, vy=2, hc=1, vc=1 // 4 sub-blocks per MCU 3'b011: for 444, hy=1, vy=1, hc=1, vc=1 // 3 sub-blocks per MCU 3'b100: for 400, hy=2, vy=2, hc=0, vc=0 // 4 sub-blocks per MCU Others: Undefined"]
    pub mod JDATA_FORMAT {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "2'b0:2-plane (Y- and UV- plane) or 1-plane (Y-only) as determined by the original data, byte sequence as Y0,Y1, or U,V 2'b01:ARGB8888, byte sequence as B,G,R,A 2'b10:RGB565, byte sequence as R,B 2'b11: YUV422H1P, byte sequence as Y0,U0,Y1,V0"]
    pub mod CFG_OPATH_SEL {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x03 << offset;
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
    #[doc = "asserted to use APB clock, so that the memory contents could be read out through APB interface"]
    pub mod MEM_DEBUG_CLK_SEL {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The jpg endec restart error interrupt enable"]
    pub mod CODEC_RESTART_ERR_IRQ_EN {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The jpg endec process done interrupt enable"]
    pub mod CODEC_OVER_IRQ_EN {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "2'b0:2-plane (Y- and UV- plane) or 1-plane (Y-only) as determined by the original data, byte sequence as Y0,Y1, or U,V 2'b01:ARGB8888, byte sequence as B,G,R,A 2'b10:RGB565, byte sequence as B,R 2'b11: YUV422H, byte sequence as Y0,U0,Y1,V0"]
    pub mod CFG_IPATH_SEL {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Normally the default CbCr sequence is that Cb macro block coming before Cr macro blk. If Cr macro block is first, set this bit to 1'b1. This bit only impact the color space conversion from/to RGB."]
    pub mod JD_UVSWAP {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status Register"]
pub mod STAT {
    #[doc = "codec restart marker error interrupt"]
    pub mod RESTART_MARKER_ERROR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Coding or decoding process is over. DMA is not included. The module is completely not busy only when in_dma_transfer_done and out_dma_transfer_done, and codec_over are all asserted."]
    pub mod CODEC_OVER {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "InDMA process done"]
    pub mod IN_DMA_TRANSFER_DONE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "OutDMA process done"]
    pub mod OUT_DMA_TRANSFER_DONE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "InDMA next interrupt"]
    pub mod INXT_IRQ {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "OutDMA next interrupt"]
    pub mod ONXT_IRQ {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "axi bus error"]
    pub mod AXI_ERR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "out-dma axi bus error"]
    pub mod AXI_WRITE_ERR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "in-dma axi bus error"]
    pub mod AXI_READ_ERR {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "the axi err id"]
    pub mod AXI_ERR_ID {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When 1 means that the module is busy doing conversion and data transfer."]
    pub mod BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Image width register"]
pub mod WIDTH {
    #[doc = "Image Width (it is the max index of pixel counting from 0, assuming the top left pixel is indexed as \\[0,0\\])"]
    pub mod IMG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Image height register"]
pub mod HEIGHT {
    #[doc = "Image Height (it is the max index of pixel counting from 0, assuming the top left pixel is indexed as \\[0,0\\])"]
    pub mod IMG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Buf Access Addr"]
pub mod BUFADDR {
    #[doc = "ADDR\\[31:28\\] denotes the buffer type: 0x2: Qmem 0x3: HuffEnc 0x4: HuffMin 0x5: HuffBase 0x6: HuffSymb ADDR\\[27:0\\] is the address inside the buffer"]
    pub mod ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Buf Access Data"]
pub mod BUFDATA {
    #[doc = "The data write-to/read-from buffer. The n-th address read will be actually the data written for n-1 th address, and the actual stored location is n-1 th address."]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Out DMA Bytes Counter"]
pub mod OUTDMACNT {
    #[doc = "The out DMA counter"]
    pub mod VAL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "YUV2RGB coefficients Register 0"]
pub mod CSC_COEF0 {
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
    #[doc = "Enable the CSC unit. 0b - The CSC is bypassed 1b - The CSC is enabled"]
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
#[doc = "YUV2RGB coefficients Register 1"]
pub mod CSC_COEF1 {
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
#[doc = "YUV2RGB coefficients Register 2"]
pub mod CSC_COEF2 {
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
#[doc = "RGB2YUV coefficients Register 0"]
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
    #[doc = "Asserted to enable this RGB2YCbCr CSC stage"]
    pub mod ENABLE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Asserted to use YCrCb mode. Must be assigned as 1."]
    pub mod YCBCR_MODE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "RGB2YUV coefficients Register 1"]
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
#[doc = "RGB2YUV coefficients Register 2"]
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
#[doc = "RGB2YUV coefficients Register 3"]
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
#[doc = "RGB2YUV coefficients Register 4"]
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
#[doc = "Image Control Register 1"]
pub mod IMGREG1 {
    #[doc = "Ncol is the number of color components in the image data to process minus 1. For example, for a grayscale image Ncol=0, for an RGB image, Ncol=2"]
    pub mod NCOL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Encoder Use only. Asseted to enable the Restart Marker processing. A Restart Marker is inserted in the outputted ECS (Entropy Coded Segment) every NRST+1 MCUs"]
    pub mod RE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Image Control Register 2"]
pub mod IMGREG2 {
    #[doc = "Encoder Use only. The number of NMCU to be generated in encoder mode"]
    pub mod NMCU {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Image Control Register 3"]
pub mod IMGREG3 {
    #[doc = "Encoder use only. It is the number of MCUs between two Restart Markers (if enabled) minus 1. The content of this register is ignored if the Re bit inregister 1 is not set."]
    pub mod NRST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Image Control Register 40"]
pub mod IMGREG_REG40 {
    #[doc = "Encoder use only. The selection of the Huffman table for the encoding of the DC coefficients in the data units belonging to the color component."]
    pub mod HD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Encoder use only. The selection of the Huffman table for the encoding of the AC coefficients in the data units belonging to the color component."]
    pub mod HA {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Encoder use only. The selection of the quantization table."]
    pub mod QT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Encoder use only. The number of data units (8x8 blocks of data) of the color componet contained in the MCU minus 1."]
    pub mod NBLOCK {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Image Control Register 41"]
pub mod IMGREG_REG41 {
    #[doc = "Encoder use only. The selection of the Huffman table for the encoding of the DC coefficients in the data units belonging to the color component."]
    pub mod HD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Encoder use only. The selection of the Huffman table for the encoding of the AC coefficients in the data units belonging to the color component."]
    pub mod HA {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Encoder use only. The selection of the quantization table."]
    pub mod QT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Encoder use only. The number of data units (8x8 blocks of data) of the color componet contained in the MCU minus 1."]
    pub mod NBLOCK {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Image Control Register 42"]
pub mod IMGREG_REG42 {
    #[doc = "Encoder use only. The selection of the Huffman table for the encoding of the DC coefficients in the data units belonging to the color component."]
    pub mod HD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Encoder use only. The selection of the Huffman table for the encoding of the AC coefficients in the data units belonging to the color component."]
    pub mod HA {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Encoder use only. The selection of the quantization table."]
    pub mod QT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Encoder use only. The number of data units (8x8 blocks of data) of the color componet contained in the MCU minus 1."]
    pub mod NBLOCK {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Image Control Register 43"]
pub mod IMGREG_REG43 {
    #[doc = "Encoder use only. The selection of the Huffman table for the encoding of the DC coefficients in the data units belonging to the color component."]
    pub mod HD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Encoder use only. The selection of the Huffman table for the encoding of the AC coefficients in the data units belonging to the color component."]
    pub mod HA {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Encoder use only. The selection of the quantization table."]
    pub mod QT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Encoder use only. The number of data units (8x8 blocks of data) of the color componet contained in the MCU minus 1."]
    pub mod NBLOCK {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
