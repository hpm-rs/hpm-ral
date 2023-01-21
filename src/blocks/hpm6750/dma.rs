#[doc = "HDMA"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    #[doc = "DMAC Configuration Register"]
    pub DMACFG: crate::RWRegister<u32>,
    _reserved1: [u8; 0x0c],
    #[doc = "DMAC Control Register"]
    pub DMACTRL: crate::RWRegister<u32>,
    #[doc = "Channel Abort Register"]
    pub CHABORT: crate::RWRegister<u32>,
    _reserved2: [u8; 0x08],
    #[doc = "Interrupt Status Register"]
    pub INTSTATUS: crate::RWRegister<u32>,
    #[doc = "Channel Enable Register"]
    pub CHEN: crate::RWRegister<u32>,
    _reserved3: [u8; 0x08],
    #[doc = "Channel n Control Register"]
    pub CHCTRL_CH0_CTRL: crate::RWRegister<u32>,
    #[doc = "Channel n Transfer Size Register"]
    pub CHCTRL_CH0_TRANSIZE: crate::RWRegister<u32>,
    #[doc = "Channel n Source Address Low Part Register"]
    pub CHCTRL_CH0_SRCADDR: crate::RWRegister<u32>,
    #[doc = "Channel n Source Address High Part Register"]
    pub CHCTRL_CH0_SRCADDRH: crate::RWRegister<u32>,
    #[doc = "Channel n Destination Address Low Part Register"]
    pub CHCTRL_CH0_DSTADDR: crate::RWRegister<u32>,
    #[doc = "Channel n Destination Address High Part Register"]
    pub CHCTRL_CH0_DSTADDRH: crate::RWRegister<u32>,
    #[doc = "Channel n Linked List Pointer Low Part Register"]
    pub CHCTRL_CH0_LLPOINTER: crate::RWRegister<u32>,
    #[doc = "Channel n Linked List Pointer High Part Register"]
    pub CHCTRL_CH0_LLPOINTERH: crate::RWRegister<u32>,
    #[doc = "Channel n Control Register"]
    pub CHCTRL_CH1_CTRL: crate::RWRegister<u32>,
    #[doc = "Channel n Transfer Size Register"]
    pub CHCTRL_CH1_TRANSIZE: crate::RWRegister<u32>,
    #[doc = "Channel n Source Address Low Part Register"]
    pub CHCTRL_CH1_SRCADDR: crate::RWRegister<u32>,
    #[doc = "Channel n Source Address High Part Register"]
    pub CHCTRL_CH1_SRCADDRH: crate::RWRegister<u32>,
    #[doc = "Channel n Destination Address Low Part Register"]
    pub CHCTRL_CH1_DSTADDR: crate::RWRegister<u32>,
    #[doc = "Channel n Destination Address High Part Register"]
    pub CHCTRL_CH1_DSTADDRH: crate::RWRegister<u32>,
    #[doc = "Channel n Linked List Pointer Low Part Register"]
    pub CHCTRL_CH1_LLPOINTER: crate::RWRegister<u32>,
    #[doc = "Channel n Linked List Pointer High Part Register"]
    pub CHCTRL_CH1_LLPOINTERH: crate::RWRegister<u32>,
    #[doc = "Channel n Control Register"]
    pub CHCTRL_CH2_CTRL: crate::RWRegister<u32>,
    #[doc = "Channel n Transfer Size Register"]
    pub CHCTRL_CH2_TRANSIZE: crate::RWRegister<u32>,
    #[doc = "Channel n Source Address Low Part Register"]
    pub CHCTRL_CH2_SRCADDR: crate::RWRegister<u32>,
    #[doc = "Channel n Source Address High Part Register"]
    pub CHCTRL_CH2_SRCADDRH: crate::RWRegister<u32>,
    #[doc = "Channel n Destination Address Low Part Register"]
    pub CHCTRL_CH2_DSTADDR: crate::RWRegister<u32>,
    #[doc = "Channel n Destination Address High Part Register"]
    pub CHCTRL_CH2_DSTADDRH: crate::RWRegister<u32>,
    #[doc = "Channel n Linked List Pointer Low Part Register"]
    pub CHCTRL_CH2_LLPOINTER: crate::RWRegister<u32>,
    #[doc = "Channel n Linked List Pointer High Part Register"]
    pub CHCTRL_CH2_LLPOINTERH: crate::RWRegister<u32>,
    #[doc = "Channel n Control Register"]
    pub CHCTRL_CH3_CTRL: crate::RWRegister<u32>,
    #[doc = "Channel n Transfer Size Register"]
    pub CHCTRL_CH3_TRANSIZE: crate::RWRegister<u32>,
    #[doc = "Channel n Source Address Low Part Register"]
    pub CHCTRL_CH3_SRCADDR: crate::RWRegister<u32>,
    #[doc = "Channel n Source Address High Part Register"]
    pub CHCTRL_CH3_SRCADDRH: crate::RWRegister<u32>,
    #[doc = "Channel n Destination Address Low Part Register"]
    pub CHCTRL_CH3_DSTADDR: crate::RWRegister<u32>,
    #[doc = "Channel n Destination Address High Part Register"]
    pub CHCTRL_CH3_DSTADDRH: crate::RWRegister<u32>,
    #[doc = "Channel n Linked List Pointer Low Part Register"]
    pub CHCTRL_CH3_LLPOINTER: crate::RWRegister<u32>,
    #[doc = "Channel n Linked List Pointer High Part Register"]
    pub CHCTRL_CH3_LLPOINTERH: crate::RWRegister<u32>,
    #[doc = "Channel n Control Register"]
    pub CHCTRL_CH4_CTRL: crate::RWRegister<u32>,
    #[doc = "Channel n Transfer Size Register"]
    pub CHCTRL_CH4_TRANSIZE: crate::RWRegister<u32>,
    #[doc = "Channel n Source Address Low Part Register"]
    pub CHCTRL_CH4_SRCADDR: crate::RWRegister<u32>,
    #[doc = "Channel n Source Address High Part Register"]
    pub CHCTRL_CH4_SRCADDRH: crate::RWRegister<u32>,
    #[doc = "Channel n Destination Address Low Part Register"]
    pub CHCTRL_CH4_DSTADDR: crate::RWRegister<u32>,
    #[doc = "Channel n Destination Address High Part Register"]
    pub CHCTRL_CH4_DSTADDRH: crate::RWRegister<u32>,
    #[doc = "Channel n Linked List Pointer Low Part Register"]
    pub CHCTRL_CH4_LLPOINTER: crate::RWRegister<u32>,
    #[doc = "Channel n Linked List Pointer High Part Register"]
    pub CHCTRL_CH4_LLPOINTERH: crate::RWRegister<u32>,
    #[doc = "Channel n Control Register"]
    pub CHCTRL_CH5_CTRL: crate::RWRegister<u32>,
    #[doc = "Channel n Transfer Size Register"]
    pub CHCTRL_CH5_TRANSIZE: crate::RWRegister<u32>,
    #[doc = "Channel n Source Address Low Part Register"]
    pub CHCTRL_CH5_SRCADDR: crate::RWRegister<u32>,
    #[doc = "Channel n Source Address High Part Register"]
    pub CHCTRL_CH5_SRCADDRH: crate::RWRegister<u32>,
    #[doc = "Channel n Destination Address Low Part Register"]
    pub CHCTRL_CH5_DSTADDR: crate::RWRegister<u32>,
    #[doc = "Channel n Destination Address High Part Register"]
    pub CHCTRL_CH5_DSTADDRH: crate::RWRegister<u32>,
    #[doc = "Channel n Linked List Pointer Low Part Register"]
    pub CHCTRL_CH5_LLPOINTER: crate::RWRegister<u32>,
    #[doc = "Channel n Linked List Pointer High Part Register"]
    pub CHCTRL_CH5_LLPOINTERH: crate::RWRegister<u32>,
    #[doc = "Channel n Control Register"]
    pub CHCTRL_CH6_CTRL: crate::RWRegister<u32>,
    #[doc = "Channel n Transfer Size Register"]
    pub CHCTRL_CH6_TRANSIZE: crate::RWRegister<u32>,
    #[doc = "Channel n Source Address Low Part Register"]
    pub CHCTRL_CH6_SRCADDR: crate::RWRegister<u32>,
    #[doc = "Channel n Source Address High Part Register"]
    pub CHCTRL_CH6_SRCADDRH: crate::RWRegister<u32>,
    #[doc = "Channel n Destination Address Low Part Register"]
    pub CHCTRL_CH6_DSTADDR: crate::RWRegister<u32>,
    #[doc = "Channel n Destination Address High Part Register"]
    pub CHCTRL_CH6_DSTADDRH: crate::RWRegister<u32>,
    #[doc = "Channel n Linked List Pointer Low Part Register"]
    pub CHCTRL_CH6_LLPOINTER: crate::RWRegister<u32>,
    #[doc = "Channel n Linked List Pointer High Part Register"]
    pub CHCTRL_CH6_LLPOINTERH: crate::RWRegister<u32>,
    #[doc = "Channel n Control Register"]
    pub CHCTRL_CH7_CTRL: crate::RWRegister<u32>,
    #[doc = "Channel n Transfer Size Register"]
    pub CHCTRL_CH7_TRANSIZE: crate::RWRegister<u32>,
    #[doc = "Channel n Source Address Low Part Register"]
    pub CHCTRL_CH7_SRCADDR: crate::RWRegister<u32>,
    #[doc = "Channel n Source Address High Part Register"]
    pub CHCTRL_CH7_SRCADDRH: crate::RWRegister<u32>,
    #[doc = "Channel n Destination Address Low Part Register"]
    pub CHCTRL_CH7_DSTADDR: crate::RWRegister<u32>,
    #[doc = "Channel n Destination Address High Part Register"]
    pub CHCTRL_CH7_DSTADDRH: crate::RWRegister<u32>,
    #[doc = "Channel n Linked List Pointer Low Part Register"]
    pub CHCTRL_CH7_LLPOINTER: crate::RWRegister<u32>,
    #[doc = "Channel n Linked List Pointer High Part Register"]
    pub CHCTRL_CH7_LLPOINTERH: crate::RWRegister<u32>,
}
#[doc = "DMAC Configuration Register"]
pub mod DMACFG {
    #[doc = "Channel number 0x1: 1 channel 0x2: 2 channels ... 0x8: 8 channels Others: Invalid"]
    pub mod CHANNELNUM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FIFO depth 0x4: 4 entries 0x8: 8 entries 0x10: 16 entries 0x20: 32 entries Others: Invalid"]
    pub mod FIFODEPTH {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Request/acknowledge pair number 0x0: 0 pair 0x1: 1 pair 0x2: 2 pairs ... 0x10: 16 pairs"]
    pub mod REQNUM {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AXI bus interface number 0x0: 1 AXI bus 0x1: 2 AXI busses"]
    pub mod BUSNUM {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA core number 0x0: 1 core 0x1: 2 cores"]
    pub mod CORENUM {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AXI bus address width 0x18: 24 bits 0x19: 25 bits ... 0x40: 64 bits Others: Invalid"]
    pub mod ADDRWIDTH {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AXI bus data width 0x0: 32 bits 0x1: 64 bits 0x2: 128 bits 0x3: 256 bits"]
    pub mod DATAWIDTH {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA request synchronization. The DMA request synchronization should be configured to avoid signal integrity problems when the request signal is not clocked by the system bus clock, which the DMA control logic operates in. If the request synchronization is not configured, the request signal is sampled directly without synchronization. 0x0: Request synchronization is not configured 0x1: Request synchronization is configured"]
    pub mod REQSYNC {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Chain transfer 0x0: Chain transfer is not configured 0x1: Chain transfer is configured"]
    pub mod CHAINXFR {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMAC Control Register"]
pub mod DMACTRL {
    #[doc = "Software reset control. Write 1 to this bit to reset the DMA core and disable all channels. Note: The software reset may cause the in-completion of AXI transaction."]
    pub mod RESET {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel Abort Register"]
pub mod CHABORT {
    #[doc = "Write 1 to bit n to abort channel n. The bits should only be set when the corresponding channels are enabled. Otherwise, the writes will be ignored for channels that are not enabled. (N: Number of channels)"]
    pub mod CHABORT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Status Register"]
pub mod INTSTATUS {
    #[doc = "The error status, one bit per channel. The error status is set when a channel transfer encounters the following error events: - Bus error - Unaligned address - Unaligned transfer width - Reserved configuration 0x0: Channel n has no error status 0x1: Channel n has error status"]
    pub mod ERROR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The abort status of channel, one bit per channel. The abort status is set when a channel transfer is aborted. 0x0: Channel n has no abort status 0x1: Channel n has abort status"]
    pub mod ABORT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The terminal count status, one bit per channel. The terminal count status is set when a channel transfer finishes without the abort or error event. 0x0: Channel n has no terminal count status 0x1: Channel n has terminal count status"]
    pub mod TC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel Enable Register"]
pub mod CHEN {
    #[doc = "Alias of the Enable field of all ChnCtrl registers"]
    pub mod CHEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel n Control Register"]
pub mod CHCTRL_CH0_CTRL {
    #[doc = "Channel enable bit 0x0: Disable 0x1: Enable"]
    pub mod ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const Disable: u32 = 0;
            #[doc = "Enable"]
            pub const Enable: u32 = 0x01;
        }
    }
    #[doc = "Channel terminal count interrupt mask 0x0: Allow the terminal count interrupt to be triggered 0x1: Disable the terminal count interrupt"]
    pub mod INTTCMASK {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Allow the terminal count interrupt to be triggered"]
            pub const Enable: u32 = 0;
            #[doc = "Disable the terminal count interrupt"]
            pub const Disable: u32 = 0x01;
        }
    }
    #[doc = "Channel error interrupt mask 0x0: Allow the error interrupt to be triggered 0x1: Disable the error interrupt"]
    pub mod INTERRMASK {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Allow the error interrupt to be triggered"]
            pub const Enable: u32 = 0;
            #[doc = "Disable the error interrupt interrupt"]
            pub const Disable: u32 = 0x01;
        }
    }
    #[doc = "Channel abort interrupt mask 0x0: Allow the abort interrupt to be triggered 0x1: Disable the abort interrupt"]
    pub mod INTABTMASK {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Allow the abort interrupt to be triggered"]
            pub const Enable: u32 = 0;
            #[doc = "Disable the abort interrupt interrupt"]
            pub const Disable: u32 = 0x01;
        }
    }
    #[doc = "Destination DMA request select. Select the request/ack handshake pair that the destination device is connected to."]
    pub mod DSTREQSEL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Source DMA request select. Select the request/ack handshake pair that the source device is connected to."]
    pub mod SRCREQSEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Destination address control 0x0: Increment address 0x1: Decrement address 0x2: Fixed address 0x3: Reserved, setting the field with this value triggers the error exception"]
    pub mod DSTADDRCTRL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Increment address"]
            pub const Increment: u32 = 0;
            #[doc = "Decrement address"]
            pub const Decrement: u32 = 0x01;
            #[doc = "Fixed address"]
            pub const Fixed: u32 = 0x02;
        }
    }
    #[doc = "Source address control 0x0: Increment address 0x1: Decrement address 0x2: Fixed address 0x3: Reserved, setting the field with this value triggers the error exception"]
    pub mod SRCADDRCTRL {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Increment address"]
            pub const Increment: u32 = 0;
            #[doc = "Decrement address"]
            pub const Decrement: u32 = 0x01;
            #[doc = "Fixed address"]
            pub const Fixed: u32 = 0x02;
        }
    }
    #[doc = "Destination DMA handshake mode 0x0: Normal mode 0x1: Handshake mode"]
    pub mod DSTMODE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal mode"]
            pub const Normal: u32 = 0;
            #[doc = "Handshake mode"]
            pub const Handshake: u32 = 0x01;
        }
    }
    #[doc = "Source DMA handshake mode 0x0: Normal mode 0x1: Handshake mode"]
    pub mod SRCMODE {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal mode"]
            pub const Normal: u32 = 0;
            #[doc = "Handshake mode"]
            pub const Handshake: u32 = 0x01;
        }
    }
    #[doc = "Destination transfer width. Both the total transfer byte number and the burst transfer byte number should be aligned to the destination transfer width; otherwise the error event will be triggered. For example, destination transfer width should be set as byte transfer if total transfer byte is not aligned to half-word. See field SrcBurstSize above for the definition of burst transfer byte number and section 3.2.8 for the definition of the total transfer byte number. 0x0: Byte transfer 0x1: Half-word transfer 0x2: Word transfer 0x3: Double word transfer 0x4: Quad word transfer 0x5: Eight word transfer 0x6�?x7: Reserved, setting this field with a reserved value triggers the error exception"]
    pub mod DSTWIDTH {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Byte transfer"]
            pub const Byte: u32 = 0;
            #[doc = "Half-word transfer"]
            pub const HalfWord: u32 = 0x01;
            #[doc = "Word transfer"]
            pub const Word: u32 = 0x02;
            #[doc = "Double word transfer"]
            pub const DoubleWord: u32 = 0x03;
            #[doc = "Quad word transfer"]
            pub const QuadWord: u32 = 0x04;
            #[doc = "Eight word transfer"]
            pub const EightWord: u32 = 0x05;
        }
    }
    #[doc = "Source transfer width 0x0: Byte transfer 0x1: Half-word transfer 0x2: Word transfer 0x3: Double word transfer 0x4: Quad word transfer 0x5: Eight word transfer 0x6�?x7: Reserved, setting this field with a reserved value triggers the error exception"]
    pub mod SRCWIDTH {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Byte transfer"]
            pub const Byte: u32 = 0;
            #[doc = "Half-word transfer"]
            pub const HalfWord: u32 = 0x01;
            #[doc = "Word transfer"]
            pub const Word: u32 = 0x02;
            #[doc = "Double word transfer"]
            pub const DoubleWord: u32 = 0x03;
            #[doc = "Quad word transfer"]
            pub const QuadWord: u32 = 0x04;
            #[doc = "Eight word transfer"]
            pub const EightWord: u32 = 0x05;
        }
    }
    #[doc = "Source burst size. This field indicates the number of transfers before DMA channel re-arbitration. The burst transfer byte number is (SrcBurstSize * SrcWidth). 0x0: 1 transfer 0x1: 2 transfers 0x2: 4 transfers 0x3: 8 transfers 0x4: 16 transfers 0x5: 32 transfers 0x6: 64 transfers 0x7: 128 transfers 0x8: 256 transfers 0x9:512 transfers 0xa: 1024 transfers 0xb �?0xf: Reserved, setting this field with a reserved value triggers the error exception"]
    pub mod SRCBURSTSIZE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1 transfer"]
            pub const Transfer1: u32 = 0;
            #[doc = "2 transfers"]
            pub const Transfers2: u32 = 0x01;
            #[doc = "4 transfers"]
            pub const Transfers4: u32 = 0x02;
            #[doc = "8 transfers"]
            pub const Transfers8: u32 = 0x03;
            #[doc = "16 transfers"]
            pub const Transfers16: u32 = 0x04;
            #[doc = "32 transfers"]
            pub const Transfers32: u32 = 0x05;
            #[doc = "64 transfers"]
            pub const Transfers64: u32 = 0x06;
            #[doc = "128 transfers"]
            pub const Transfers128: u32 = 0x07;
            #[doc = "256 transfers"]
            pub const Transfers256: u32 = 0x08;
            #[doc = "512 transfers"]
            pub const Transfers512: u32 = 0x09;
            #[doc = "1024 transfers"]
            pub const Transfers1024: u32 = 0x0a;
        }
    }
    #[doc = "Channel priority level 0x0: Lower priority 0x1: Higher priority"]
    pub mod PRIORITY {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Lower priority"]
            pub const Lower: u32 = 0;
            #[doc = "Higher priority"]
            pub const Higher: u32 = 0x01;
        }
    }
    #[doc = "Bus interface index that destination data is written to 0x0: Data is written to bus interface 0 0x1: Data is written to bus interface 1"]
    pub mod DSTBUSINFIDX {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bus interface index that source data is read from 0x0: Data is read from bus interface 0 0x1: Data is read from bus interface"]
    pub mod SRCBUSINFIDX {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel n Transfer Size Register"]
pub mod CHCTRL_CH0_TRANSIZE {
    #[doc = "Total transfer size from source. The total number of transferred bytes is (TranSize * SrcWidth). This register is cleared when the DMA transfer is done. If a channel is enabled with zero total transfer size, the error event will be triggered and the transfer will be terminated."]
    pub mod TRANSIZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel n Source Address Low Part Register"]
pub mod CHCTRL_CH0_SRCADDR {
    #[doc = "Low part of the source starting address. When the transfer completes, the value of {SrcAddrH,SrcAddrL} is updated to the ending address. This address must be aligned to the source transfer size; otherwise, an error event will be triggered."]
    pub mod SRCADDRL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel n Source Address High Part Register"]
pub mod CHCTRL_CH0_SRCADDRH {
    #[doc = "High part of the source starting address. When the transfer completes, the value of {SrcAddrH,SrcAddrL} is updated to the ending address. This register exists only when the address bus width is wider than 32 bits."]
    pub mod SRCADDRH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel n Destination Address Low Part Register"]
pub mod CHCTRL_CH0_DSTADDR {
    #[doc = "Low part of the destination starting address. When the transfer completes, the value of {DstAddrH,DstAddrL} is updated to the ending address. This address must be aligned to the destination transfer size; otherwise the error event will be triggered."]
    pub mod DSTADDRL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel n Destination Address High Part Register"]
pub mod CHCTRL_CH0_DSTADDRH {
    #[doc = "High part of the destination starting address. When the transfer completes, the value of {DstAddrH,DstAddrL} is updated to the ending address. This address must be aligned to the destination transfer size; otherwise the error event will be triggered. This register exists only when the address bus width is wider than 32 bits."]
    pub mod DSTADDRH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel n Linked List Pointer Low Part Register"]
pub mod CHCTRL_CH0_LLPOINTER {
    #[doc = "Bus interface index that the next descriptor is read from 0x0: The next descriptor is read from bus interface 0"]
    pub mod LLDBUSINFIDX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Low part of the pointer to the next descriptor. The pointer must be double word aligned."]
    pub mod LLPOINTERL {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel n Linked List Pointer High Part Register"]
pub mod CHCTRL_CH0_LLPOINTERH {
    #[doc = "High part of the pointer to the next descriptor. This register exists only when the address bus width is wider than 32 bits."]
    pub mod LLPOINTERH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel n Control Register"]
pub mod CHCTRL_CH1_CTRL {
    #[doc = "Channel enable bit 0x0: Disable 0x1: Enable"]
    pub mod ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const Disable: u32 = 0;
            #[doc = "Enable"]
            pub const Enable: u32 = 0x01;
        }
    }
    #[doc = "Channel terminal count interrupt mask 0x0: Allow the terminal count interrupt to be triggered 0x1: Disable the terminal count interrupt"]
    pub mod INTTCMASK {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Allow the terminal count interrupt to be triggered"]
            pub const Enable: u32 = 0;
            #[doc = "Disable the terminal count interrupt"]
            pub const Disable: u32 = 0x01;
        }
    }
    #[doc = "Channel error interrupt mask 0x0: Allow the error interrupt to be triggered 0x1: Disable the error interrupt"]
    pub mod INTERRMASK {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Allow the error interrupt to be triggered"]
            pub const Enable: u32 = 0;
            #[doc = "Disable the error interrupt interrupt"]
            pub const Disable: u32 = 0x01;
        }
    }
    #[doc = "Channel abort interrupt mask 0x0: Allow the abort interrupt to be triggered 0x1: Disable the abort interrupt"]
    pub mod INTABTMASK {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Allow the abort interrupt to be triggered"]
            pub const Enable: u32 = 0;
            #[doc = "Disable the abort interrupt interrupt"]
            pub const Disable: u32 = 0x01;
        }
    }
    #[doc = "Destination DMA request select. Select the request/ack handshake pair that the destination device is connected to."]
    pub mod DSTREQSEL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Source DMA request select. Select the request/ack handshake pair that the source device is connected to."]
    pub mod SRCREQSEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Destination address control 0x0: Increment address 0x1: Decrement address 0x2: Fixed address 0x3: Reserved, setting the field with this value triggers the error exception"]
    pub mod DSTADDRCTRL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Increment address"]
            pub const Increment: u32 = 0;
            #[doc = "Decrement address"]
            pub const Decrement: u32 = 0x01;
            #[doc = "Fixed address"]
            pub const Fixed: u32 = 0x02;
        }
    }
    #[doc = "Source address control 0x0: Increment address 0x1: Decrement address 0x2: Fixed address 0x3: Reserved, setting the field with this value triggers the error exception"]
    pub mod SRCADDRCTRL {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Increment address"]
            pub const Increment: u32 = 0;
            #[doc = "Decrement address"]
            pub const Decrement: u32 = 0x01;
            #[doc = "Fixed address"]
            pub const Fixed: u32 = 0x02;
        }
    }
    #[doc = "Destination DMA handshake mode 0x0: Normal mode 0x1: Handshake mode"]
    pub mod DSTMODE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal mode"]
            pub const Normal: u32 = 0;
            #[doc = "Handshake mode"]
            pub const Handshake: u32 = 0x01;
        }
    }
    #[doc = "Source DMA handshake mode 0x0: Normal mode 0x1: Handshake mode"]
    pub mod SRCMODE {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal mode"]
            pub const Normal: u32 = 0;
            #[doc = "Handshake mode"]
            pub const Handshake: u32 = 0x01;
        }
    }
    #[doc = "Destination transfer width. Both the total transfer byte number and the burst transfer byte number should be aligned to the destination transfer width; otherwise the error event will be triggered. For example, destination transfer width should be set as byte transfer if total transfer byte is not aligned to half-word. See field SrcBurstSize above for the definition of burst transfer byte number and section 3.2.8 for the definition of the total transfer byte number. 0x0: Byte transfer 0x1: Half-word transfer 0x2: Word transfer 0x3: Double word transfer 0x4: Quad word transfer 0x5: Eight word transfer 0x6�?x7: Reserved, setting this field with a reserved value triggers the error exception"]
    pub mod DSTWIDTH {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Byte transfer"]
            pub const Byte: u32 = 0;
            #[doc = "Half-word transfer"]
            pub const HalfWord: u32 = 0x01;
            #[doc = "Word transfer"]
            pub const Word: u32 = 0x02;
            #[doc = "Double word transfer"]
            pub const DoubleWord: u32 = 0x03;
            #[doc = "Quad word transfer"]
            pub const QuadWord: u32 = 0x04;
            #[doc = "Eight word transfer"]
            pub const EightWord: u32 = 0x05;
        }
    }
    #[doc = "Source transfer width 0x0: Byte transfer 0x1: Half-word transfer 0x2: Word transfer 0x3: Double word transfer 0x4: Quad word transfer 0x5: Eight word transfer 0x6�?x7: Reserved, setting this field with a reserved value triggers the error exception"]
    pub mod SRCWIDTH {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Byte transfer"]
            pub const Byte: u32 = 0;
            #[doc = "Half-word transfer"]
            pub const HalfWord: u32 = 0x01;
            #[doc = "Word transfer"]
            pub const Word: u32 = 0x02;
            #[doc = "Double word transfer"]
            pub const DoubleWord: u32 = 0x03;
            #[doc = "Quad word transfer"]
            pub const QuadWord: u32 = 0x04;
            #[doc = "Eight word transfer"]
            pub const EightWord: u32 = 0x05;
        }
    }
    #[doc = "Source burst size. This field indicates the number of transfers before DMA channel re-arbitration. The burst transfer byte number is (SrcBurstSize * SrcWidth). 0x0: 1 transfer 0x1: 2 transfers 0x2: 4 transfers 0x3: 8 transfers 0x4: 16 transfers 0x5: 32 transfers 0x6: 64 transfers 0x7: 128 transfers 0x8: 256 transfers 0x9:512 transfers 0xa: 1024 transfers 0xb �?0xf: Reserved, setting this field with a reserved value triggers the error exception"]
    pub mod SRCBURSTSIZE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1 transfer"]
            pub const Transfer1: u32 = 0;
            #[doc = "2 transfers"]
            pub const Transfers2: u32 = 0x01;
            #[doc = "4 transfers"]
            pub const Transfers4: u32 = 0x02;
            #[doc = "8 transfers"]
            pub const Transfers8: u32 = 0x03;
            #[doc = "16 transfers"]
            pub const Transfers16: u32 = 0x04;
            #[doc = "32 transfers"]
            pub const Transfers32: u32 = 0x05;
            #[doc = "64 transfers"]
            pub const Transfers64: u32 = 0x06;
            #[doc = "128 transfers"]
            pub const Transfers128: u32 = 0x07;
            #[doc = "256 transfers"]
            pub const Transfers256: u32 = 0x08;
            #[doc = "512 transfers"]
            pub const Transfers512: u32 = 0x09;
            #[doc = "1024 transfers"]
            pub const Transfers1024: u32 = 0x0a;
        }
    }
    #[doc = "Channel priority level 0x0: Lower priority 0x1: Higher priority"]
    pub mod PRIORITY {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Lower priority"]
            pub const Lower: u32 = 0;
            #[doc = "Higher priority"]
            pub const Higher: u32 = 0x01;
        }
    }
    #[doc = "Bus interface index that destination data is written to 0x0: Data is written to bus interface 0 0x1: Data is written to bus interface 1"]
    pub mod DSTBUSINFIDX {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bus interface index that source data is read from 0x0: Data is read from bus interface 0 0x1: Data is read from bus interface"]
    pub mod SRCBUSINFIDX {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel n Transfer Size Register"]
pub mod CHCTRL_CH1_TRANSIZE {
    #[doc = "Total transfer size from source. The total number of transferred bytes is (TranSize * SrcWidth). This register is cleared when the DMA transfer is done. If a channel is enabled with zero total transfer size, the error event will be triggered and the transfer will be terminated."]
    pub mod TRANSIZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel n Source Address Low Part Register"]
pub mod CHCTRL_CH1_SRCADDR {
    #[doc = "Low part of the source starting address. When the transfer completes, the value of {SrcAddrH,SrcAddrL} is updated to the ending address. This address must be aligned to the source transfer size; otherwise, an error event will be triggered."]
    pub mod SRCADDRL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel n Source Address High Part Register"]
pub mod CHCTRL_CH1_SRCADDRH {
    #[doc = "High part of the source starting address. When the transfer completes, the value of {SrcAddrH,SrcAddrL} is updated to the ending address. This register exists only when the address bus width is wider than 32 bits."]
    pub mod SRCADDRH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel n Destination Address Low Part Register"]
pub mod CHCTRL_CH1_DSTADDR {
    #[doc = "Low part of the destination starting address. When the transfer completes, the value of {DstAddrH,DstAddrL} is updated to the ending address. This address must be aligned to the destination transfer size; otherwise the error event will be triggered."]
    pub mod DSTADDRL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel n Destination Address High Part Register"]
pub mod CHCTRL_CH1_DSTADDRH {
    #[doc = "High part of the destination starting address. When the transfer completes, the value of {DstAddrH,DstAddrL} is updated to the ending address. This address must be aligned to the destination transfer size; otherwise the error event will be triggered. This register exists only when the address bus width is wider than 32 bits."]
    pub mod DSTADDRH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel n Linked List Pointer Low Part Register"]
pub mod CHCTRL_CH1_LLPOINTER {
    #[doc = "Bus interface index that the next descriptor is read from 0x0: The next descriptor is read from bus interface 0"]
    pub mod LLDBUSINFIDX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Low part of the pointer to the next descriptor. The pointer must be double word aligned."]
    pub mod LLPOINTERL {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel n Linked List Pointer High Part Register"]
pub mod CHCTRL_CH1_LLPOINTERH {
    #[doc = "High part of the pointer to the next descriptor. This register exists only when the address bus width is wider than 32 bits."]
    pub mod LLPOINTERH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel n Control Register"]
pub mod CHCTRL_CH2_CTRL {
    #[doc = "Channel enable bit 0x0: Disable 0x1: Enable"]
    pub mod ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const Disable: u32 = 0;
            #[doc = "Enable"]
            pub const Enable: u32 = 0x01;
        }
    }
    #[doc = "Channel terminal count interrupt mask 0x0: Allow the terminal count interrupt to be triggered 0x1: Disable the terminal count interrupt"]
    pub mod INTTCMASK {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Allow the terminal count interrupt to be triggered"]
            pub const Enable: u32 = 0;
            #[doc = "Disable the terminal count interrupt"]
            pub const Disable: u32 = 0x01;
        }
    }
    #[doc = "Channel error interrupt mask 0x0: Allow the error interrupt to be triggered 0x1: Disable the error interrupt"]
    pub mod INTERRMASK {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Allow the error interrupt to be triggered"]
            pub const Enable: u32 = 0;
            #[doc = "Disable the error interrupt interrupt"]
            pub const Disable: u32 = 0x01;
        }
    }
    #[doc = "Channel abort interrupt mask 0x0: Allow the abort interrupt to be triggered 0x1: Disable the abort interrupt"]
    pub mod INTABTMASK {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Allow the abort interrupt to be triggered"]
            pub const Enable: u32 = 0;
            #[doc = "Disable the abort interrupt interrupt"]
            pub const Disable: u32 = 0x01;
        }
    }
    #[doc = "Destination DMA request select. Select the request/ack handshake pair that the destination device is connected to."]
    pub mod DSTREQSEL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Source DMA request select. Select the request/ack handshake pair that the source device is connected to."]
    pub mod SRCREQSEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Destination address control 0x0: Increment address 0x1: Decrement address 0x2: Fixed address 0x3: Reserved, setting the field with this value triggers the error exception"]
    pub mod DSTADDRCTRL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Increment address"]
            pub const Increment: u32 = 0;
            #[doc = "Decrement address"]
            pub const Decrement: u32 = 0x01;
            #[doc = "Fixed address"]
            pub const Fixed: u32 = 0x02;
        }
    }
    #[doc = "Source address control 0x0: Increment address 0x1: Decrement address 0x2: Fixed address 0x3: Reserved, setting the field with this value triggers the error exception"]
    pub mod SRCADDRCTRL {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Increment address"]
            pub const Increment: u32 = 0;
            #[doc = "Decrement address"]
            pub const Decrement: u32 = 0x01;
            #[doc = "Fixed address"]
            pub const Fixed: u32 = 0x02;
        }
    }
    #[doc = "Destination DMA handshake mode 0x0: Normal mode 0x1: Handshake mode"]
    pub mod DSTMODE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal mode"]
            pub const Normal: u32 = 0;
            #[doc = "Handshake mode"]
            pub const Handshake: u32 = 0x01;
        }
    }
    #[doc = "Source DMA handshake mode 0x0: Normal mode 0x1: Handshake mode"]
    pub mod SRCMODE {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal mode"]
            pub const Normal: u32 = 0;
            #[doc = "Handshake mode"]
            pub const Handshake: u32 = 0x01;
        }
    }
    #[doc = "Destination transfer width. Both the total transfer byte number and the burst transfer byte number should be aligned to the destination transfer width; otherwise the error event will be triggered. For example, destination transfer width should be set as byte transfer if total transfer byte is not aligned to half-word. See field SrcBurstSize above for the definition of burst transfer byte number and section 3.2.8 for the definition of the total transfer byte number. 0x0: Byte transfer 0x1: Half-word transfer 0x2: Word transfer 0x3: Double word transfer 0x4: Quad word transfer 0x5: Eight word transfer 0x6�?x7: Reserved, setting this field with a reserved value triggers the error exception"]
    pub mod DSTWIDTH {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Byte transfer"]
            pub const Byte: u32 = 0;
            #[doc = "Half-word transfer"]
            pub const HalfWord: u32 = 0x01;
            #[doc = "Word transfer"]
            pub const Word: u32 = 0x02;
            #[doc = "Double word transfer"]
            pub const DoubleWord: u32 = 0x03;
            #[doc = "Quad word transfer"]
            pub const QuadWord: u32 = 0x04;
            #[doc = "Eight word transfer"]
            pub const EightWord: u32 = 0x05;
        }
    }
    #[doc = "Source transfer width 0x0: Byte transfer 0x1: Half-word transfer 0x2: Word transfer 0x3: Double word transfer 0x4: Quad word transfer 0x5: Eight word transfer 0x6�?x7: Reserved, setting this field with a reserved value triggers the error exception"]
    pub mod SRCWIDTH {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Byte transfer"]
            pub const Byte: u32 = 0;
            #[doc = "Half-word transfer"]
            pub const HalfWord: u32 = 0x01;
            #[doc = "Word transfer"]
            pub const Word: u32 = 0x02;
            #[doc = "Double word transfer"]
            pub const DoubleWord: u32 = 0x03;
            #[doc = "Quad word transfer"]
            pub const QuadWord: u32 = 0x04;
            #[doc = "Eight word transfer"]
            pub const EightWord: u32 = 0x05;
        }
    }
    #[doc = "Source burst size. This field indicates the number of transfers before DMA channel re-arbitration. The burst transfer byte number is (SrcBurstSize * SrcWidth). 0x0: 1 transfer 0x1: 2 transfers 0x2: 4 transfers 0x3: 8 transfers 0x4: 16 transfers 0x5: 32 transfers 0x6: 64 transfers 0x7: 128 transfers 0x8: 256 transfers 0x9:512 transfers 0xa: 1024 transfers 0xb �?0xf: Reserved, setting this field with a reserved value triggers the error exception"]
    pub mod SRCBURSTSIZE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1 transfer"]
            pub const Transfer1: u32 = 0;
            #[doc = "2 transfers"]
            pub const Transfers2: u32 = 0x01;
            #[doc = "4 transfers"]
            pub const Transfers4: u32 = 0x02;
            #[doc = "8 transfers"]
            pub const Transfers8: u32 = 0x03;
            #[doc = "16 transfers"]
            pub const Transfers16: u32 = 0x04;
            #[doc = "32 transfers"]
            pub const Transfers32: u32 = 0x05;
            #[doc = "64 transfers"]
            pub const Transfers64: u32 = 0x06;
            #[doc = "128 transfers"]
            pub const Transfers128: u32 = 0x07;
            #[doc = "256 transfers"]
            pub const Transfers256: u32 = 0x08;
            #[doc = "512 transfers"]
            pub const Transfers512: u32 = 0x09;
            #[doc = "1024 transfers"]
            pub const Transfers1024: u32 = 0x0a;
        }
    }
    #[doc = "Channel priority level 0x0: Lower priority 0x1: Higher priority"]
    pub mod PRIORITY {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Lower priority"]
            pub const Lower: u32 = 0;
            #[doc = "Higher priority"]
            pub const Higher: u32 = 0x01;
        }
    }
    #[doc = "Bus interface index that destination data is written to 0x0: Data is written to bus interface 0 0x1: Data is written to bus interface 1"]
    pub mod DSTBUSINFIDX {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bus interface index that source data is read from 0x0: Data is read from bus interface 0 0x1: Data is read from bus interface"]
    pub mod SRCBUSINFIDX {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel n Transfer Size Register"]
pub mod CHCTRL_CH2_TRANSIZE {
    #[doc = "Total transfer size from source. The total number of transferred bytes is (TranSize * SrcWidth). This register is cleared when the DMA transfer is done. If a channel is enabled with zero total transfer size, the error event will be triggered and the transfer will be terminated."]
    pub mod TRANSIZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel n Source Address Low Part Register"]
pub mod CHCTRL_CH2_SRCADDR {
    #[doc = "Low part of the source starting address. When the transfer completes, the value of {SrcAddrH,SrcAddrL} is updated to the ending address. This address must be aligned to the source transfer size; otherwise, an error event will be triggered."]
    pub mod SRCADDRL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel n Source Address High Part Register"]
pub mod CHCTRL_CH2_SRCADDRH {
    #[doc = "High part of the source starting address. When the transfer completes, the value of {SrcAddrH,SrcAddrL} is updated to the ending address. This register exists only when the address bus width is wider than 32 bits."]
    pub mod SRCADDRH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel n Destination Address Low Part Register"]
pub mod CHCTRL_CH2_DSTADDR {
    #[doc = "Low part of the destination starting address. When the transfer completes, the value of {DstAddrH,DstAddrL} is updated to the ending address. This address must be aligned to the destination transfer size; otherwise the error event will be triggered."]
    pub mod DSTADDRL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel n Destination Address High Part Register"]
pub mod CHCTRL_CH2_DSTADDRH {
    #[doc = "High part of the destination starting address. When the transfer completes, the value of {DstAddrH,DstAddrL} is updated to the ending address. This address must be aligned to the destination transfer size; otherwise the error event will be triggered. This register exists only when the address bus width is wider than 32 bits."]
    pub mod DSTADDRH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel n Linked List Pointer Low Part Register"]
pub mod CHCTRL_CH2_LLPOINTER {
    #[doc = "Bus interface index that the next descriptor is read from 0x0: The next descriptor is read from bus interface 0"]
    pub mod LLDBUSINFIDX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Low part of the pointer to the next descriptor. The pointer must be double word aligned."]
    pub mod LLPOINTERL {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel n Linked List Pointer High Part Register"]
pub mod CHCTRL_CH2_LLPOINTERH {
    #[doc = "High part of the pointer to the next descriptor. This register exists only when the address bus width is wider than 32 bits."]
    pub mod LLPOINTERH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel n Control Register"]
pub mod CHCTRL_CH3_CTRL {
    #[doc = "Channel enable bit 0x0: Disable 0x1: Enable"]
    pub mod ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const Disable: u32 = 0;
            #[doc = "Enable"]
            pub const Enable: u32 = 0x01;
        }
    }
    #[doc = "Channel terminal count interrupt mask 0x0: Allow the terminal count interrupt to be triggered 0x1: Disable the terminal count interrupt"]
    pub mod INTTCMASK {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Allow the terminal count interrupt to be triggered"]
            pub const Enable: u32 = 0;
            #[doc = "Disable the terminal count interrupt"]
            pub const Disable: u32 = 0x01;
        }
    }
    #[doc = "Channel error interrupt mask 0x0: Allow the error interrupt to be triggered 0x1: Disable the error interrupt"]
    pub mod INTERRMASK {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Allow the error interrupt to be triggered"]
            pub const Enable: u32 = 0;
            #[doc = "Disable the error interrupt interrupt"]
            pub const Disable: u32 = 0x01;
        }
    }
    #[doc = "Channel abort interrupt mask 0x0: Allow the abort interrupt to be triggered 0x1: Disable the abort interrupt"]
    pub mod INTABTMASK {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Allow the abort interrupt to be triggered"]
            pub const Enable: u32 = 0;
            #[doc = "Disable the abort interrupt interrupt"]
            pub const Disable: u32 = 0x01;
        }
    }
    #[doc = "Destination DMA request select. Select the request/ack handshake pair that the destination device is connected to."]
    pub mod DSTREQSEL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Source DMA request select. Select the request/ack handshake pair that the source device is connected to."]
    pub mod SRCREQSEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Destination address control 0x0: Increment address 0x1: Decrement address 0x2: Fixed address 0x3: Reserved, setting the field with this value triggers the error exception"]
    pub mod DSTADDRCTRL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Increment address"]
            pub const Increment: u32 = 0;
            #[doc = "Decrement address"]
            pub const Decrement: u32 = 0x01;
            #[doc = "Fixed address"]
            pub const Fixed: u32 = 0x02;
        }
    }
    #[doc = "Source address control 0x0: Increment address 0x1: Decrement address 0x2: Fixed address 0x3: Reserved, setting the field with this value triggers the error exception"]
    pub mod SRCADDRCTRL {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Increment address"]
            pub const Increment: u32 = 0;
            #[doc = "Decrement address"]
            pub const Decrement: u32 = 0x01;
            #[doc = "Fixed address"]
            pub const Fixed: u32 = 0x02;
        }
    }
    #[doc = "Destination DMA handshake mode 0x0: Normal mode 0x1: Handshake mode"]
    pub mod DSTMODE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal mode"]
            pub const Normal: u32 = 0;
            #[doc = "Handshake mode"]
            pub const Handshake: u32 = 0x01;
        }
    }
    #[doc = "Source DMA handshake mode 0x0: Normal mode 0x1: Handshake mode"]
    pub mod SRCMODE {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal mode"]
            pub const Normal: u32 = 0;
            #[doc = "Handshake mode"]
            pub const Handshake: u32 = 0x01;
        }
    }
    #[doc = "Destination transfer width. Both the total transfer byte number and the burst transfer byte number should be aligned to the destination transfer width; otherwise the error event will be triggered. For example, destination transfer width should be set as byte transfer if total transfer byte is not aligned to half-word. See field SrcBurstSize above for the definition of burst transfer byte number and section 3.2.8 for the definition of the total transfer byte number. 0x0: Byte transfer 0x1: Half-word transfer 0x2: Word transfer 0x3: Double word transfer 0x4: Quad word transfer 0x5: Eight word transfer 0x6�?x7: Reserved, setting this field with a reserved value triggers the error exception"]
    pub mod DSTWIDTH {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Byte transfer"]
            pub const Byte: u32 = 0;
            #[doc = "Half-word transfer"]
            pub const HalfWord: u32 = 0x01;
            #[doc = "Word transfer"]
            pub const Word: u32 = 0x02;
            #[doc = "Double word transfer"]
            pub const DoubleWord: u32 = 0x03;
            #[doc = "Quad word transfer"]
            pub const QuadWord: u32 = 0x04;
            #[doc = "Eight word transfer"]
            pub const EightWord: u32 = 0x05;
        }
    }
    #[doc = "Source transfer width 0x0: Byte transfer 0x1: Half-word transfer 0x2: Word transfer 0x3: Double word transfer 0x4: Quad word transfer 0x5: Eight word transfer 0x6�?x7: Reserved, setting this field with a reserved value triggers the error exception"]
    pub mod SRCWIDTH {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Byte transfer"]
            pub const Byte: u32 = 0;
            #[doc = "Half-word transfer"]
            pub const HalfWord: u32 = 0x01;
            #[doc = "Word transfer"]
            pub const Word: u32 = 0x02;
            #[doc = "Double word transfer"]
            pub const DoubleWord: u32 = 0x03;
            #[doc = "Quad word transfer"]
            pub const QuadWord: u32 = 0x04;
            #[doc = "Eight word transfer"]
            pub const EightWord: u32 = 0x05;
        }
    }
    #[doc = "Source burst size. This field indicates the number of transfers before DMA channel re-arbitration. The burst transfer byte number is (SrcBurstSize * SrcWidth). 0x0: 1 transfer 0x1: 2 transfers 0x2: 4 transfers 0x3: 8 transfers 0x4: 16 transfers 0x5: 32 transfers 0x6: 64 transfers 0x7: 128 transfers 0x8: 256 transfers 0x9:512 transfers 0xa: 1024 transfers 0xb �?0xf: Reserved, setting this field with a reserved value triggers the error exception"]
    pub mod SRCBURSTSIZE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1 transfer"]
            pub const Transfer1: u32 = 0;
            #[doc = "2 transfers"]
            pub const Transfers2: u32 = 0x01;
            #[doc = "4 transfers"]
            pub const Transfers4: u32 = 0x02;
            #[doc = "8 transfers"]
            pub const Transfers8: u32 = 0x03;
            #[doc = "16 transfers"]
            pub const Transfers16: u32 = 0x04;
            #[doc = "32 transfers"]
            pub const Transfers32: u32 = 0x05;
            #[doc = "64 transfers"]
            pub const Transfers64: u32 = 0x06;
            #[doc = "128 transfers"]
            pub const Transfers128: u32 = 0x07;
            #[doc = "256 transfers"]
            pub const Transfers256: u32 = 0x08;
            #[doc = "512 transfers"]
            pub const Transfers512: u32 = 0x09;
            #[doc = "1024 transfers"]
            pub const Transfers1024: u32 = 0x0a;
        }
    }
    #[doc = "Channel priority level 0x0: Lower priority 0x1: Higher priority"]
    pub mod PRIORITY {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Lower priority"]
            pub const Lower: u32 = 0;
            #[doc = "Higher priority"]
            pub const Higher: u32 = 0x01;
        }
    }
    #[doc = "Bus interface index that destination data is written to 0x0: Data is written to bus interface 0 0x1: Data is written to bus interface 1"]
    pub mod DSTBUSINFIDX {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bus interface index that source data is read from 0x0: Data is read from bus interface 0 0x1: Data is read from bus interface"]
    pub mod SRCBUSINFIDX {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel n Transfer Size Register"]
pub mod CHCTRL_CH3_TRANSIZE {
    #[doc = "Total transfer size from source. The total number of transferred bytes is (TranSize * SrcWidth). This register is cleared when the DMA transfer is done. If a channel is enabled with zero total transfer size, the error event will be triggered and the transfer will be terminated."]
    pub mod TRANSIZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel n Source Address Low Part Register"]
pub mod CHCTRL_CH3_SRCADDR {
    #[doc = "Low part of the source starting address. When the transfer completes, the value of {SrcAddrH,SrcAddrL} is updated to the ending address. This address must be aligned to the source transfer size; otherwise, an error event will be triggered."]
    pub mod SRCADDRL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel n Source Address High Part Register"]
pub mod CHCTRL_CH3_SRCADDRH {
    #[doc = "High part of the source starting address. When the transfer completes, the value of {SrcAddrH,SrcAddrL} is updated to the ending address. This register exists only when the address bus width is wider than 32 bits."]
    pub mod SRCADDRH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel n Destination Address Low Part Register"]
pub mod CHCTRL_CH3_DSTADDR {
    #[doc = "Low part of the destination starting address. When the transfer completes, the value of {DstAddrH,DstAddrL} is updated to the ending address. This address must be aligned to the destination transfer size; otherwise the error event will be triggered."]
    pub mod DSTADDRL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel n Destination Address High Part Register"]
pub mod CHCTRL_CH3_DSTADDRH {
    #[doc = "High part of the destination starting address. When the transfer completes, the value of {DstAddrH,DstAddrL} is updated to the ending address. This address must be aligned to the destination transfer size; otherwise the error event will be triggered. This register exists only when the address bus width is wider than 32 bits."]
    pub mod DSTADDRH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel n Linked List Pointer Low Part Register"]
pub mod CHCTRL_CH3_LLPOINTER {
    #[doc = "Bus interface index that the next descriptor is read from 0x0: The next descriptor is read from bus interface 0"]
    pub mod LLDBUSINFIDX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Low part of the pointer to the next descriptor. The pointer must be double word aligned."]
    pub mod LLPOINTERL {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel n Linked List Pointer High Part Register"]
pub mod CHCTRL_CH3_LLPOINTERH {
    #[doc = "High part of the pointer to the next descriptor. This register exists only when the address bus width is wider than 32 bits."]
    pub mod LLPOINTERH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel n Control Register"]
pub mod CHCTRL_CH4_CTRL {
    #[doc = "Channel enable bit 0x0: Disable 0x1: Enable"]
    pub mod ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const Disable: u32 = 0;
            #[doc = "Enable"]
            pub const Enable: u32 = 0x01;
        }
    }
    #[doc = "Channel terminal count interrupt mask 0x0: Allow the terminal count interrupt to be triggered 0x1: Disable the terminal count interrupt"]
    pub mod INTTCMASK {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Allow the terminal count interrupt to be triggered"]
            pub const Enable: u32 = 0;
            #[doc = "Disable the terminal count interrupt"]
            pub const Disable: u32 = 0x01;
        }
    }
    #[doc = "Channel error interrupt mask 0x0: Allow the error interrupt to be triggered 0x1: Disable the error interrupt"]
    pub mod INTERRMASK {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Allow the error interrupt to be triggered"]
            pub const Enable: u32 = 0;
            #[doc = "Disable the error interrupt interrupt"]
            pub const Disable: u32 = 0x01;
        }
    }
    #[doc = "Channel abort interrupt mask 0x0: Allow the abort interrupt to be triggered 0x1: Disable the abort interrupt"]
    pub mod INTABTMASK {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Allow the abort interrupt to be triggered"]
            pub const Enable: u32 = 0;
            #[doc = "Disable the abort interrupt interrupt"]
            pub const Disable: u32 = 0x01;
        }
    }
    #[doc = "Destination DMA request select. Select the request/ack handshake pair that the destination device is connected to."]
    pub mod DSTREQSEL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Source DMA request select. Select the request/ack handshake pair that the source device is connected to."]
    pub mod SRCREQSEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Destination address control 0x0: Increment address 0x1: Decrement address 0x2: Fixed address 0x3: Reserved, setting the field with this value triggers the error exception"]
    pub mod DSTADDRCTRL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Increment address"]
            pub const Increment: u32 = 0;
            #[doc = "Decrement address"]
            pub const Decrement: u32 = 0x01;
            #[doc = "Fixed address"]
            pub const Fixed: u32 = 0x02;
        }
    }
    #[doc = "Source address control 0x0: Increment address 0x1: Decrement address 0x2: Fixed address 0x3: Reserved, setting the field with this value triggers the error exception"]
    pub mod SRCADDRCTRL {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Increment address"]
            pub const Increment: u32 = 0;
            #[doc = "Decrement address"]
            pub const Decrement: u32 = 0x01;
            #[doc = "Fixed address"]
            pub const Fixed: u32 = 0x02;
        }
    }
    #[doc = "Destination DMA handshake mode 0x0: Normal mode 0x1: Handshake mode"]
    pub mod DSTMODE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal mode"]
            pub const Normal: u32 = 0;
            #[doc = "Handshake mode"]
            pub const Handshake: u32 = 0x01;
        }
    }
    #[doc = "Source DMA handshake mode 0x0: Normal mode 0x1: Handshake mode"]
    pub mod SRCMODE {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal mode"]
            pub const Normal: u32 = 0;
            #[doc = "Handshake mode"]
            pub const Handshake: u32 = 0x01;
        }
    }
    #[doc = "Destination transfer width. Both the total transfer byte number and the burst transfer byte number should be aligned to the destination transfer width; otherwise the error event will be triggered. For example, destination transfer width should be set as byte transfer if total transfer byte is not aligned to half-word. See field SrcBurstSize above for the definition of burst transfer byte number and section 3.2.8 for the definition of the total transfer byte number. 0x0: Byte transfer 0x1: Half-word transfer 0x2: Word transfer 0x3: Double word transfer 0x4: Quad word transfer 0x5: Eight word transfer 0x6�?x7: Reserved, setting this field with a reserved value triggers the error exception"]
    pub mod DSTWIDTH {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Byte transfer"]
            pub const Byte: u32 = 0;
            #[doc = "Half-word transfer"]
            pub const HalfWord: u32 = 0x01;
            #[doc = "Word transfer"]
            pub const Word: u32 = 0x02;
            #[doc = "Double word transfer"]
            pub const DoubleWord: u32 = 0x03;
            #[doc = "Quad word transfer"]
            pub const QuadWord: u32 = 0x04;
            #[doc = "Eight word transfer"]
            pub const EightWord: u32 = 0x05;
        }
    }
    #[doc = "Source transfer width 0x0: Byte transfer 0x1: Half-word transfer 0x2: Word transfer 0x3: Double word transfer 0x4: Quad word transfer 0x5: Eight word transfer 0x6�?x7: Reserved, setting this field with a reserved value triggers the error exception"]
    pub mod SRCWIDTH {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Byte transfer"]
            pub const Byte: u32 = 0;
            #[doc = "Half-word transfer"]
            pub const HalfWord: u32 = 0x01;
            #[doc = "Word transfer"]
            pub const Word: u32 = 0x02;
            #[doc = "Double word transfer"]
            pub const DoubleWord: u32 = 0x03;
            #[doc = "Quad word transfer"]
            pub const QuadWord: u32 = 0x04;
            #[doc = "Eight word transfer"]
            pub const EightWord: u32 = 0x05;
        }
    }
    #[doc = "Source burst size. This field indicates the number of transfers before DMA channel re-arbitration. The burst transfer byte number is (SrcBurstSize * SrcWidth). 0x0: 1 transfer 0x1: 2 transfers 0x2: 4 transfers 0x3: 8 transfers 0x4: 16 transfers 0x5: 32 transfers 0x6: 64 transfers 0x7: 128 transfers 0x8: 256 transfers 0x9:512 transfers 0xa: 1024 transfers 0xb �?0xf: Reserved, setting this field with a reserved value triggers the error exception"]
    pub mod SRCBURSTSIZE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1 transfer"]
            pub const Transfer1: u32 = 0;
            #[doc = "2 transfers"]
            pub const Transfers2: u32 = 0x01;
            #[doc = "4 transfers"]
            pub const Transfers4: u32 = 0x02;
            #[doc = "8 transfers"]
            pub const Transfers8: u32 = 0x03;
            #[doc = "16 transfers"]
            pub const Transfers16: u32 = 0x04;
            #[doc = "32 transfers"]
            pub const Transfers32: u32 = 0x05;
            #[doc = "64 transfers"]
            pub const Transfers64: u32 = 0x06;
            #[doc = "128 transfers"]
            pub const Transfers128: u32 = 0x07;
            #[doc = "256 transfers"]
            pub const Transfers256: u32 = 0x08;
            #[doc = "512 transfers"]
            pub const Transfers512: u32 = 0x09;
            #[doc = "1024 transfers"]
            pub const Transfers1024: u32 = 0x0a;
        }
    }
    #[doc = "Channel priority level 0x0: Lower priority 0x1: Higher priority"]
    pub mod PRIORITY {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Lower priority"]
            pub const Lower: u32 = 0;
            #[doc = "Higher priority"]
            pub const Higher: u32 = 0x01;
        }
    }
    #[doc = "Bus interface index that destination data is written to 0x0: Data is written to bus interface 0 0x1: Data is written to bus interface 1"]
    pub mod DSTBUSINFIDX {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bus interface index that source data is read from 0x0: Data is read from bus interface 0 0x1: Data is read from bus interface"]
    pub mod SRCBUSINFIDX {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel n Transfer Size Register"]
pub mod CHCTRL_CH4_TRANSIZE {
    #[doc = "Total transfer size from source. The total number of transferred bytes is (TranSize * SrcWidth). This register is cleared when the DMA transfer is done. If a channel is enabled with zero total transfer size, the error event will be triggered and the transfer will be terminated."]
    pub mod TRANSIZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel n Source Address Low Part Register"]
pub mod CHCTRL_CH4_SRCADDR {
    #[doc = "Low part of the source starting address. When the transfer completes, the value of {SrcAddrH,SrcAddrL} is updated to the ending address. This address must be aligned to the source transfer size; otherwise, an error event will be triggered."]
    pub mod SRCADDRL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel n Source Address High Part Register"]
pub mod CHCTRL_CH4_SRCADDRH {
    #[doc = "High part of the source starting address. When the transfer completes, the value of {SrcAddrH,SrcAddrL} is updated to the ending address. This register exists only when the address bus width is wider than 32 bits."]
    pub mod SRCADDRH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel n Destination Address Low Part Register"]
pub mod CHCTRL_CH4_DSTADDR {
    #[doc = "Low part of the destination starting address. When the transfer completes, the value of {DstAddrH,DstAddrL} is updated to the ending address. This address must be aligned to the destination transfer size; otherwise the error event will be triggered."]
    pub mod DSTADDRL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel n Destination Address High Part Register"]
pub mod CHCTRL_CH4_DSTADDRH {
    #[doc = "High part of the destination starting address. When the transfer completes, the value of {DstAddrH,DstAddrL} is updated to the ending address. This address must be aligned to the destination transfer size; otherwise the error event will be triggered. This register exists only when the address bus width is wider than 32 bits."]
    pub mod DSTADDRH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel n Linked List Pointer Low Part Register"]
pub mod CHCTRL_CH4_LLPOINTER {
    #[doc = "Bus interface index that the next descriptor is read from 0x0: The next descriptor is read from bus interface 0"]
    pub mod LLDBUSINFIDX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Low part of the pointer to the next descriptor. The pointer must be double word aligned."]
    pub mod LLPOINTERL {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel n Linked List Pointer High Part Register"]
pub mod CHCTRL_CH4_LLPOINTERH {
    #[doc = "High part of the pointer to the next descriptor. This register exists only when the address bus width is wider than 32 bits."]
    pub mod LLPOINTERH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel n Control Register"]
pub mod CHCTRL_CH5_CTRL {
    #[doc = "Channel enable bit 0x0: Disable 0x1: Enable"]
    pub mod ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const Disable: u32 = 0;
            #[doc = "Enable"]
            pub const Enable: u32 = 0x01;
        }
    }
    #[doc = "Channel terminal count interrupt mask 0x0: Allow the terminal count interrupt to be triggered 0x1: Disable the terminal count interrupt"]
    pub mod INTTCMASK {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Allow the terminal count interrupt to be triggered"]
            pub const Enable: u32 = 0;
            #[doc = "Disable the terminal count interrupt"]
            pub const Disable: u32 = 0x01;
        }
    }
    #[doc = "Channel error interrupt mask 0x0: Allow the error interrupt to be triggered 0x1: Disable the error interrupt"]
    pub mod INTERRMASK {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Allow the error interrupt to be triggered"]
            pub const Enable: u32 = 0;
            #[doc = "Disable the error interrupt interrupt"]
            pub const Disable: u32 = 0x01;
        }
    }
    #[doc = "Channel abort interrupt mask 0x0: Allow the abort interrupt to be triggered 0x1: Disable the abort interrupt"]
    pub mod INTABTMASK {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Allow the abort interrupt to be triggered"]
            pub const Enable: u32 = 0;
            #[doc = "Disable the abort interrupt interrupt"]
            pub const Disable: u32 = 0x01;
        }
    }
    #[doc = "Destination DMA request select. Select the request/ack handshake pair that the destination device is connected to."]
    pub mod DSTREQSEL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Source DMA request select. Select the request/ack handshake pair that the source device is connected to."]
    pub mod SRCREQSEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Destination address control 0x0: Increment address 0x1: Decrement address 0x2: Fixed address 0x3: Reserved, setting the field with this value triggers the error exception"]
    pub mod DSTADDRCTRL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Increment address"]
            pub const Increment: u32 = 0;
            #[doc = "Decrement address"]
            pub const Decrement: u32 = 0x01;
            #[doc = "Fixed address"]
            pub const Fixed: u32 = 0x02;
        }
    }
    #[doc = "Source address control 0x0: Increment address 0x1: Decrement address 0x2: Fixed address 0x3: Reserved, setting the field with this value triggers the error exception"]
    pub mod SRCADDRCTRL {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Increment address"]
            pub const Increment: u32 = 0;
            #[doc = "Decrement address"]
            pub const Decrement: u32 = 0x01;
            #[doc = "Fixed address"]
            pub const Fixed: u32 = 0x02;
        }
    }
    #[doc = "Destination DMA handshake mode 0x0: Normal mode 0x1: Handshake mode"]
    pub mod DSTMODE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal mode"]
            pub const Normal: u32 = 0;
            #[doc = "Handshake mode"]
            pub const Handshake: u32 = 0x01;
        }
    }
    #[doc = "Source DMA handshake mode 0x0: Normal mode 0x1: Handshake mode"]
    pub mod SRCMODE {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal mode"]
            pub const Normal: u32 = 0;
            #[doc = "Handshake mode"]
            pub const Handshake: u32 = 0x01;
        }
    }
    #[doc = "Destination transfer width. Both the total transfer byte number and the burst transfer byte number should be aligned to the destination transfer width; otherwise the error event will be triggered. For example, destination transfer width should be set as byte transfer if total transfer byte is not aligned to half-word. See field SrcBurstSize above for the definition of burst transfer byte number and section 3.2.8 for the definition of the total transfer byte number. 0x0: Byte transfer 0x1: Half-word transfer 0x2: Word transfer 0x3: Double word transfer 0x4: Quad word transfer 0x5: Eight word transfer 0x6�?x7: Reserved, setting this field with a reserved value triggers the error exception"]
    pub mod DSTWIDTH {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Byte transfer"]
            pub const Byte: u32 = 0;
            #[doc = "Half-word transfer"]
            pub const HalfWord: u32 = 0x01;
            #[doc = "Word transfer"]
            pub const Word: u32 = 0x02;
            #[doc = "Double word transfer"]
            pub const DoubleWord: u32 = 0x03;
            #[doc = "Quad word transfer"]
            pub const QuadWord: u32 = 0x04;
            #[doc = "Eight word transfer"]
            pub const EightWord: u32 = 0x05;
        }
    }
    #[doc = "Source transfer width 0x0: Byte transfer 0x1: Half-word transfer 0x2: Word transfer 0x3: Double word transfer 0x4: Quad word transfer 0x5: Eight word transfer 0x6�?x7: Reserved, setting this field with a reserved value triggers the error exception"]
    pub mod SRCWIDTH {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Byte transfer"]
            pub const Byte: u32 = 0;
            #[doc = "Half-word transfer"]
            pub const HalfWord: u32 = 0x01;
            #[doc = "Word transfer"]
            pub const Word: u32 = 0x02;
            #[doc = "Double word transfer"]
            pub const DoubleWord: u32 = 0x03;
            #[doc = "Quad word transfer"]
            pub const QuadWord: u32 = 0x04;
            #[doc = "Eight word transfer"]
            pub const EightWord: u32 = 0x05;
        }
    }
    #[doc = "Source burst size. This field indicates the number of transfers before DMA channel re-arbitration. The burst transfer byte number is (SrcBurstSize * SrcWidth). 0x0: 1 transfer 0x1: 2 transfers 0x2: 4 transfers 0x3: 8 transfers 0x4: 16 transfers 0x5: 32 transfers 0x6: 64 transfers 0x7: 128 transfers 0x8: 256 transfers 0x9:512 transfers 0xa: 1024 transfers 0xb �?0xf: Reserved, setting this field with a reserved value triggers the error exception"]
    pub mod SRCBURSTSIZE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1 transfer"]
            pub const Transfer1: u32 = 0;
            #[doc = "2 transfers"]
            pub const Transfers2: u32 = 0x01;
            #[doc = "4 transfers"]
            pub const Transfers4: u32 = 0x02;
            #[doc = "8 transfers"]
            pub const Transfers8: u32 = 0x03;
            #[doc = "16 transfers"]
            pub const Transfers16: u32 = 0x04;
            #[doc = "32 transfers"]
            pub const Transfers32: u32 = 0x05;
            #[doc = "64 transfers"]
            pub const Transfers64: u32 = 0x06;
            #[doc = "128 transfers"]
            pub const Transfers128: u32 = 0x07;
            #[doc = "256 transfers"]
            pub const Transfers256: u32 = 0x08;
            #[doc = "512 transfers"]
            pub const Transfers512: u32 = 0x09;
            #[doc = "1024 transfers"]
            pub const Transfers1024: u32 = 0x0a;
        }
    }
    #[doc = "Channel priority level 0x0: Lower priority 0x1: Higher priority"]
    pub mod PRIORITY {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Lower priority"]
            pub const Lower: u32 = 0;
            #[doc = "Higher priority"]
            pub const Higher: u32 = 0x01;
        }
    }
    #[doc = "Bus interface index that destination data is written to 0x0: Data is written to bus interface 0 0x1: Data is written to bus interface 1"]
    pub mod DSTBUSINFIDX {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bus interface index that source data is read from 0x0: Data is read from bus interface 0 0x1: Data is read from bus interface"]
    pub mod SRCBUSINFIDX {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel n Transfer Size Register"]
pub mod CHCTRL_CH5_TRANSIZE {
    #[doc = "Total transfer size from source. The total number of transferred bytes is (TranSize * SrcWidth). This register is cleared when the DMA transfer is done. If a channel is enabled with zero total transfer size, the error event will be triggered and the transfer will be terminated."]
    pub mod TRANSIZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel n Source Address Low Part Register"]
pub mod CHCTRL_CH5_SRCADDR {
    #[doc = "Low part of the source starting address. When the transfer completes, the value of {SrcAddrH,SrcAddrL} is updated to the ending address. This address must be aligned to the source transfer size; otherwise, an error event will be triggered."]
    pub mod SRCADDRL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel n Source Address High Part Register"]
pub mod CHCTRL_CH5_SRCADDRH {
    #[doc = "High part of the source starting address. When the transfer completes, the value of {SrcAddrH,SrcAddrL} is updated to the ending address. This register exists only when the address bus width is wider than 32 bits."]
    pub mod SRCADDRH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel n Destination Address Low Part Register"]
pub mod CHCTRL_CH5_DSTADDR {
    #[doc = "Low part of the destination starting address. When the transfer completes, the value of {DstAddrH,DstAddrL} is updated to the ending address. This address must be aligned to the destination transfer size; otherwise the error event will be triggered."]
    pub mod DSTADDRL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel n Destination Address High Part Register"]
pub mod CHCTRL_CH5_DSTADDRH {
    #[doc = "High part of the destination starting address. When the transfer completes, the value of {DstAddrH,DstAddrL} is updated to the ending address. This address must be aligned to the destination transfer size; otherwise the error event will be triggered. This register exists only when the address bus width is wider than 32 bits."]
    pub mod DSTADDRH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel n Linked List Pointer Low Part Register"]
pub mod CHCTRL_CH5_LLPOINTER {
    #[doc = "Bus interface index that the next descriptor is read from 0x0: The next descriptor is read from bus interface 0"]
    pub mod LLDBUSINFIDX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Low part of the pointer to the next descriptor. The pointer must be double word aligned."]
    pub mod LLPOINTERL {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel n Linked List Pointer High Part Register"]
pub mod CHCTRL_CH5_LLPOINTERH {
    #[doc = "High part of the pointer to the next descriptor. This register exists only when the address bus width is wider than 32 bits."]
    pub mod LLPOINTERH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel n Control Register"]
pub mod CHCTRL_CH6_CTRL {
    #[doc = "Channel enable bit 0x0: Disable 0x1: Enable"]
    pub mod ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const Disable: u32 = 0;
            #[doc = "Enable"]
            pub const Enable: u32 = 0x01;
        }
    }
    #[doc = "Channel terminal count interrupt mask 0x0: Allow the terminal count interrupt to be triggered 0x1: Disable the terminal count interrupt"]
    pub mod INTTCMASK {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Allow the terminal count interrupt to be triggered"]
            pub const Enable: u32 = 0;
            #[doc = "Disable the terminal count interrupt"]
            pub const Disable: u32 = 0x01;
        }
    }
    #[doc = "Channel error interrupt mask 0x0: Allow the error interrupt to be triggered 0x1: Disable the error interrupt"]
    pub mod INTERRMASK {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Allow the error interrupt to be triggered"]
            pub const Enable: u32 = 0;
            #[doc = "Disable the error interrupt interrupt"]
            pub const Disable: u32 = 0x01;
        }
    }
    #[doc = "Channel abort interrupt mask 0x0: Allow the abort interrupt to be triggered 0x1: Disable the abort interrupt"]
    pub mod INTABTMASK {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Allow the abort interrupt to be triggered"]
            pub const Enable: u32 = 0;
            #[doc = "Disable the abort interrupt interrupt"]
            pub const Disable: u32 = 0x01;
        }
    }
    #[doc = "Destination DMA request select. Select the request/ack handshake pair that the destination device is connected to."]
    pub mod DSTREQSEL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Source DMA request select. Select the request/ack handshake pair that the source device is connected to."]
    pub mod SRCREQSEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Destination address control 0x0: Increment address 0x1: Decrement address 0x2: Fixed address 0x3: Reserved, setting the field with this value triggers the error exception"]
    pub mod DSTADDRCTRL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Increment address"]
            pub const Increment: u32 = 0;
            #[doc = "Decrement address"]
            pub const Decrement: u32 = 0x01;
            #[doc = "Fixed address"]
            pub const Fixed: u32 = 0x02;
        }
    }
    #[doc = "Source address control 0x0: Increment address 0x1: Decrement address 0x2: Fixed address 0x3: Reserved, setting the field with this value triggers the error exception"]
    pub mod SRCADDRCTRL {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Increment address"]
            pub const Increment: u32 = 0;
            #[doc = "Decrement address"]
            pub const Decrement: u32 = 0x01;
            #[doc = "Fixed address"]
            pub const Fixed: u32 = 0x02;
        }
    }
    #[doc = "Destination DMA handshake mode 0x0: Normal mode 0x1: Handshake mode"]
    pub mod DSTMODE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal mode"]
            pub const Normal: u32 = 0;
            #[doc = "Handshake mode"]
            pub const Handshake: u32 = 0x01;
        }
    }
    #[doc = "Source DMA handshake mode 0x0: Normal mode 0x1: Handshake mode"]
    pub mod SRCMODE {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal mode"]
            pub const Normal: u32 = 0;
            #[doc = "Handshake mode"]
            pub const Handshake: u32 = 0x01;
        }
    }
    #[doc = "Destination transfer width. Both the total transfer byte number and the burst transfer byte number should be aligned to the destination transfer width; otherwise the error event will be triggered. For example, destination transfer width should be set as byte transfer if total transfer byte is not aligned to half-word. See field SrcBurstSize above for the definition of burst transfer byte number and section 3.2.8 for the definition of the total transfer byte number. 0x0: Byte transfer 0x1: Half-word transfer 0x2: Word transfer 0x3: Double word transfer 0x4: Quad word transfer 0x5: Eight word transfer 0x6�?x7: Reserved, setting this field with a reserved value triggers the error exception"]
    pub mod DSTWIDTH {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Byte transfer"]
            pub const Byte: u32 = 0;
            #[doc = "Half-word transfer"]
            pub const HalfWord: u32 = 0x01;
            #[doc = "Word transfer"]
            pub const Word: u32 = 0x02;
            #[doc = "Double word transfer"]
            pub const DoubleWord: u32 = 0x03;
            #[doc = "Quad word transfer"]
            pub const QuadWord: u32 = 0x04;
            #[doc = "Eight word transfer"]
            pub const EightWord: u32 = 0x05;
        }
    }
    #[doc = "Source transfer width 0x0: Byte transfer 0x1: Half-word transfer 0x2: Word transfer 0x3: Double word transfer 0x4: Quad word transfer 0x5: Eight word transfer 0x6�?x7: Reserved, setting this field with a reserved value triggers the error exception"]
    pub mod SRCWIDTH {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Byte transfer"]
            pub const Byte: u32 = 0;
            #[doc = "Half-word transfer"]
            pub const HalfWord: u32 = 0x01;
            #[doc = "Word transfer"]
            pub const Word: u32 = 0x02;
            #[doc = "Double word transfer"]
            pub const DoubleWord: u32 = 0x03;
            #[doc = "Quad word transfer"]
            pub const QuadWord: u32 = 0x04;
            #[doc = "Eight word transfer"]
            pub const EightWord: u32 = 0x05;
        }
    }
    #[doc = "Source burst size. This field indicates the number of transfers before DMA channel re-arbitration. The burst transfer byte number is (SrcBurstSize * SrcWidth). 0x0: 1 transfer 0x1: 2 transfers 0x2: 4 transfers 0x3: 8 transfers 0x4: 16 transfers 0x5: 32 transfers 0x6: 64 transfers 0x7: 128 transfers 0x8: 256 transfers 0x9:512 transfers 0xa: 1024 transfers 0xb �?0xf: Reserved, setting this field with a reserved value triggers the error exception"]
    pub mod SRCBURSTSIZE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1 transfer"]
            pub const Transfer1: u32 = 0;
            #[doc = "2 transfers"]
            pub const Transfers2: u32 = 0x01;
            #[doc = "4 transfers"]
            pub const Transfers4: u32 = 0x02;
            #[doc = "8 transfers"]
            pub const Transfers8: u32 = 0x03;
            #[doc = "16 transfers"]
            pub const Transfers16: u32 = 0x04;
            #[doc = "32 transfers"]
            pub const Transfers32: u32 = 0x05;
            #[doc = "64 transfers"]
            pub const Transfers64: u32 = 0x06;
            #[doc = "128 transfers"]
            pub const Transfers128: u32 = 0x07;
            #[doc = "256 transfers"]
            pub const Transfers256: u32 = 0x08;
            #[doc = "512 transfers"]
            pub const Transfers512: u32 = 0x09;
            #[doc = "1024 transfers"]
            pub const Transfers1024: u32 = 0x0a;
        }
    }
    #[doc = "Channel priority level 0x0: Lower priority 0x1: Higher priority"]
    pub mod PRIORITY {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Lower priority"]
            pub const Lower: u32 = 0;
            #[doc = "Higher priority"]
            pub const Higher: u32 = 0x01;
        }
    }
    #[doc = "Bus interface index that destination data is written to 0x0: Data is written to bus interface 0 0x1: Data is written to bus interface 1"]
    pub mod DSTBUSINFIDX {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bus interface index that source data is read from 0x0: Data is read from bus interface 0 0x1: Data is read from bus interface"]
    pub mod SRCBUSINFIDX {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel n Transfer Size Register"]
pub mod CHCTRL_CH6_TRANSIZE {
    #[doc = "Total transfer size from source. The total number of transferred bytes is (TranSize * SrcWidth). This register is cleared when the DMA transfer is done. If a channel is enabled with zero total transfer size, the error event will be triggered and the transfer will be terminated."]
    pub mod TRANSIZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel n Source Address Low Part Register"]
pub mod CHCTRL_CH6_SRCADDR {
    #[doc = "Low part of the source starting address. When the transfer completes, the value of {SrcAddrH,SrcAddrL} is updated to the ending address. This address must be aligned to the source transfer size; otherwise, an error event will be triggered."]
    pub mod SRCADDRL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel n Source Address High Part Register"]
pub mod CHCTRL_CH6_SRCADDRH {
    #[doc = "High part of the source starting address. When the transfer completes, the value of {SrcAddrH,SrcAddrL} is updated to the ending address. This register exists only when the address bus width is wider than 32 bits."]
    pub mod SRCADDRH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel n Destination Address Low Part Register"]
pub mod CHCTRL_CH6_DSTADDR {
    #[doc = "Low part of the destination starting address. When the transfer completes, the value of {DstAddrH,DstAddrL} is updated to the ending address. This address must be aligned to the destination transfer size; otherwise the error event will be triggered."]
    pub mod DSTADDRL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel n Destination Address High Part Register"]
pub mod CHCTRL_CH6_DSTADDRH {
    #[doc = "High part of the destination starting address. When the transfer completes, the value of {DstAddrH,DstAddrL} is updated to the ending address. This address must be aligned to the destination transfer size; otherwise the error event will be triggered. This register exists only when the address bus width is wider than 32 bits."]
    pub mod DSTADDRH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel n Linked List Pointer Low Part Register"]
pub mod CHCTRL_CH6_LLPOINTER {
    #[doc = "Bus interface index that the next descriptor is read from 0x0: The next descriptor is read from bus interface 0"]
    pub mod LLDBUSINFIDX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Low part of the pointer to the next descriptor. The pointer must be double word aligned."]
    pub mod LLPOINTERL {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel n Linked List Pointer High Part Register"]
pub mod CHCTRL_CH6_LLPOINTERH {
    #[doc = "High part of the pointer to the next descriptor. This register exists only when the address bus width is wider than 32 bits."]
    pub mod LLPOINTERH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel n Control Register"]
pub mod CHCTRL_CH7_CTRL {
    #[doc = "Channel enable bit 0x0: Disable 0x1: Enable"]
    pub mod ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const Disable: u32 = 0;
            #[doc = "Enable"]
            pub const Enable: u32 = 0x01;
        }
    }
    #[doc = "Channel terminal count interrupt mask 0x0: Allow the terminal count interrupt to be triggered 0x1: Disable the terminal count interrupt"]
    pub mod INTTCMASK {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Allow the terminal count interrupt to be triggered"]
            pub const Enable: u32 = 0;
            #[doc = "Disable the terminal count interrupt"]
            pub const Disable: u32 = 0x01;
        }
    }
    #[doc = "Channel error interrupt mask 0x0: Allow the error interrupt to be triggered 0x1: Disable the error interrupt"]
    pub mod INTERRMASK {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Allow the error interrupt to be triggered"]
            pub const Enable: u32 = 0;
            #[doc = "Disable the error interrupt interrupt"]
            pub const Disable: u32 = 0x01;
        }
    }
    #[doc = "Channel abort interrupt mask 0x0: Allow the abort interrupt to be triggered 0x1: Disable the abort interrupt"]
    pub mod INTABTMASK {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Allow the abort interrupt to be triggered"]
            pub const Enable: u32 = 0;
            #[doc = "Disable the abort interrupt interrupt"]
            pub const Disable: u32 = 0x01;
        }
    }
    #[doc = "Destination DMA request select. Select the request/ack handshake pair that the destination device is connected to."]
    pub mod DSTREQSEL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Source DMA request select. Select the request/ack handshake pair that the source device is connected to."]
    pub mod SRCREQSEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Destination address control 0x0: Increment address 0x1: Decrement address 0x2: Fixed address 0x3: Reserved, setting the field with this value triggers the error exception"]
    pub mod DSTADDRCTRL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Increment address"]
            pub const Increment: u32 = 0;
            #[doc = "Decrement address"]
            pub const Decrement: u32 = 0x01;
            #[doc = "Fixed address"]
            pub const Fixed: u32 = 0x02;
        }
    }
    #[doc = "Source address control 0x0: Increment address 0x1: Decrement address 0x2: Fixed address 0x3: Reserved, setting the field with this value triggers the error exception"]
    pub mod SRCADDRCTRL {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Increment address"]
            pub const Increment: u32 = 0;
            #[doc = "Decrement address"]
            pub const Decrement: u32 = 0x01;
            #[doc = "Fixed address"]
            pub const Fixed: u32 = 0x02;
        }
    }
    #[doc = "Destination DMA handshake mode 0x0: Normal mode 0x1: Handshake mode"]
    pub mod DSTMODE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal mode"]
            pub const Normal: u32 = 0;
            #[doc = "Handshake mode"]
            pub const Handshake: u32 = 0x01;
        }
    }
    #[doc = "Source DMA handshake mode 0x0: Normal mode 0x1: Handshake mode"]
    pub mod SRCMODE {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal mode"]
            pub const Normal: u32 = 0;
            #[doc = "Handshake mode"]
            pub const Handshake: u32 = 0x01;
        }
    }
    #[doc = "Destination transfer width. Both the total transfer byte number and the burst transfer byte number should be aligned to the destination transfer width; otherwise the error event will be triggered. For example, destination transfer width should be set as byte transfer if total transfer byte is not aligned to half-word. See field SrcBurstSize above for the definition of burst transfer byte number and section 3.2.8 for the definition of the total transfer byte number. 0x0: Byte transfer 0x1: Half-word transfer 0x2: Word transfer 0x3: Double word transfer 0x4: Quad word transfer 0x5: Eight word transfer 0x6�?x7: Reserved, setting this field with a reserved value triggers the error exception"]
    pub mod DSTWIDTH {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Byte transfer"]
            pub const Byte: u32 = 0;
            #[doc = "Half-word transfer"]
            pub const HalfWord: u32 = 0x01;
            #[doc = "Word transfer"]
            pub const Word: u32 = 0x02;
            #[doc = "Double word transfer"]
            pub const DoubleWord: u32 = 0x03;
            #[doc = "Quad word transfer"]
            pub const QuadWord: u32 = 0x04;
            #[doc = "Eight word transfer"]
            pub const EightWord: u32 = 0x05;
        }
    }
    #[doc = "Source transfer width 0x0: Byte transfer 0x1: Half-word transfer 0x2: Word transfer 0x3: Double word transfer 0x4: Quad word transfer 0x5: Eight word transfer 0x6�?x7: Reserved, setting this field with a reserved value triggers the error exception"]
    pub mod SRCWIDTH {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Byte transfer"]
            pub const Byte: u32 = 0;
            #[doc = "Half-word transfer"]
            pub const HalfWord: u32 = 0x01;
            #[doc = "Word transfer"]
            pub const Word: u32 = 0x02;
            #[doc = "Double word transfer"]
            pub const DoubleWord: u32 = 0x03;
            #[doc = "Quad word transfer"]
            pub const QuadWord: u32 = 0x04;
            #[doc = "Eight word transfer"]
            pub const EightWord: u32 = 0x05;
        }
    }
    #[doc = "Source burst size. This field indicates the number of transfers before DMA channel re-arbitration. The burst transfer byte number is (SrcBurstSize * SrcWidth). 0x0: 1 transfer 0x1: 2 transfers 0x2: 4 transfers 0x3: 8 transfers 0x4: 16 transfers 0x5: 32 transfers 0x6: 64 transfers 0x7: 128 transfers 0x8: 256 transfers 0x9:512 transfers 0xa: 1024 transfers 0xb �?0xf: Reserved, setting this field with a reserved value triggers the error exception"]
    pub mod SRCBURSTSIZE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1 transfer"]
            pub const Transfer1: u32 = 0;
            #[doc = "2 transfers"]
            pub const Transfers2: u32 = 0x01;
            #[doc = "4 transfers"]
            pub const Transfers4: u32 = 0x02;
            #[doc = "8 transfers"]
            pub const Transfers8: u32 = 0x03;
            #[doc = "16 transfers"]
            pub const Transfers16: u32 = 0x04;
            #[doc = "32 transfers"]
            pub const Transfers32: u32 = 0x05;
            #[doc = "64 transfers"]
            pub const Transfers64: u32 = 0x06;
            #[doc = "128 transfers"]
            pub const Transfers128: u32 = 0x07;
            #[doc = "256 transfers"]
            pub const Transfers256: u32 = 0x08;
            #[doc = "512 transfers"]
            pub const Transfers512: u32 = 0x09;
            #[doc = "1024 transfers"]
            pub const Transfers1024: u32 = 0x0a;
        }
    }
    #[doc = "Channel priority level 0x0: Lower priority 0x1: Higher priority"]
    pub mod PRIORITY {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Lower priority"]
            pub const Lower: u32 = 0;
            #[doc = "Higher priority"]
            pub const Higher: u32 = 0x01;
        }
    }
    #[doc = "Bus interface index that destination data is written to 0x0: Data is written to bus interface 0 0x1: Data is written to bus interface 1"]
    pub mod DSTBUSINFIDX {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bus interface index that source data is read from 0x0: Data is read from bus interface 0 0x1: Data is read from bus interface"]
    pub mod SRCBUSINFIDX {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel n Transfer Size Register"]
pub mod CHCTRL_CH7_TRANSIZE {
    #[doc = "Total transfer size from source. The total number of transferred bytes is (TranSize * SrcWidth). This register is cleared when the DMA transfer is done. If a channel is enabled with zero total transfer size, the error event will be triggered and the transfer will be terminated."]
    pub mod TRANSIZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel n Source Address Low Part Register"]
pub mod CHCTRL_CH7_SRCADDR {
    #[doc = "Low part of the source starting address. When the transfer completes, the value of {SrcAddrH,SrcAddrL} is updated to the ending address. This address must be aligned to the source transfer size; otherwise, an error event will be triggered."]
    pub mod SRCADDRL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel n Source Address High Part Register"]
pub mod CHCTRL_CH7_SRCADDRH {
    #[doc = "High part of the source starting address. When the transfer completes, the value of {SrcAddrH,SrcAddrL} is updated to the ending address. This register exists only when the address bus width is wider than 32 bits."]
    pub mod SRCADDRH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel n Destination Address Low Part Register"]
pub mod CHCTRL_CH7_DSTADDR {
    #[doc = "Low part of the destination starting address. When the transfer completes, the value of {DstAddrH,DstAddrL} is updated to the ending address. This address must be aligned to the destination transfer size; otherwise the error event will be triggered."]
    pub mod DSTADDRL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel n Destination Address High Part Register"]
pub mod CHCTRL_CH7_DSTADDRH {
    #[doc = "High part of the destination starting address. When the transfer completes, the value of {DstAddrH,DstAddrL} is updated to the ending address. This address must be aligned to the destination transfer size; otherwise the error event will be triggered. This register exists only when the address bus width is wider than 32 bits."]
    pub mod DSTADDRH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel n Linked List Pointer Low Part Register"]
pub mod CHCTRL_CH7_LLPOINTER {
    #[doc = "Bus interface index that the next descriptor is read from 0x0: The next descriptor is read from bus interface 0"]
    pub mod LLDBUSINFIDX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Low part of the pointer to the next descriptor. The pointer must be double word aligned."]
    pub mod LLPOINTERL {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x1fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel n Linked List Pointer High Part Register"]
pub mod CHCTRL_CH7_LLPOINTERH {
    #[doc = "High part of the pointer to the next descriptor. This register exists only when the address bus width is wider than 32 bits."]
    pub mod LLPOINTERH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
