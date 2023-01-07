#[doc = "SDXC0"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "No description avaiable"]
    pub SDMASA: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub BLK_ATTR: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CMD_ARG: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CMD_XFER: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub RESP_RESP01: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub RESP_RESP23: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub RESP_RESP45: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub RESP_RESP67: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub BUF_DATA: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub PSTATE: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub PROT_CTRL: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub SYS_CTRL: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub INT_STAT: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub INT_STAT_EN: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub INT_SIGNAL_EN: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub AC_HOST_CTRL: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CAPABILITIES1: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CAPABILITIES2: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CURR_CAPABILITIES1: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CURR_CAPABILITIES2: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub FORCE_EVENT: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub ADMA_ERR_STAT: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub ADMA_SYS_ADDR: crate::RWRegister<u32>,
    _reserved0: [u8; 0x04],
    #[doc = "No description avaiable"]
    pub PRESET_INIT: crate::RWRegister<u16>,
    #[doc = "No description avaiable"]
    pub PRESET_DS: crate::RWRegister<u16>,
    #[doc = "No description avaiable"]
    pub PRESET_HS: crate::RWRegister<u16>,
    #[doc = "No description avaiable"]
    pub PRESET_SDR12: crate::RWRegister<u16>,
    #[doc = "No description avaiable"]
    pub PRESET_SDR25: crate::RWRegister<u16>,
    #[doc = "No description avaiable"]
    pub PRESET_SDR50: crate::RWRegister<u16>,
    #[doc = "No description avaiable"]
    pub PRESET_SDR104: crate::RWRegister<u16>,
    #[doc = "No description avaiable"]
    pub PRESET_DDR50: crate::RWRegister<u16>,
    _reserved1: [u8; 0x04],
    #[doc = "No description avaiable"]
    pub PRESET_UHS2: crate::RWRegister<u16>,
    _reserved2: [u8; 0x02],
    #[doc = "No description avaiable"]
    pub ADMA_ID_ADDR: crate::RWRegister<u32>,
    _reserved3: [u8; 0x6a],
    #[doc = "No description avaiable"]
    pub P_EMBEDDED_CNTRL: crate::RWRegister<u16>,
    #[doc = "No description avaiable"]
    pub P_VENDOR_SPECIFIC_AREA: crate::RWRegister<u16>,
    #[doc = "No description avaiable"]
    pub P_VENDOR2_SPECIFIC_AREA: crate::RWRegister<u16>,
    _reserved4: [u8; 0x10],
    #[doc = "No description avaiable"]
    pub SLOT_INTR_STATUS: crate::RWRegister<u16>,
    _reserved5: [u8; 0x82],
    #[doc = "No description avaiable"]
    pub CQVER: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CQCAP: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CQCFG: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CQCTL: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CQIS: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CQISE: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CQISGE: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CQIC: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CQTDLBA: crate::RWRegister<u32>,
    _reserved6: [u8; 0x04],
    #[doc = "No description avaiable"]
    pub CQTDBR: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CQTCN: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CQDQS: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CQDPT: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CQTCLR: crate::RWRegister<u32>,
    _reserved7: [u8; 0x04],
    #[doc = "No description avaiable"]
    pub CQSSC1: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CQSSC2: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CQCRDCT: crate::RWRegister<u32>,
    _reserved8: [u8; 0x04],
    #[doc = "No description avaiable"]
    pub CQRMEM: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CQTERRI: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CQCRI: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub CQCRA: crate::RWRegister<u32>,
    _reserved9: [u8; 0x0320],
    #[doc = "No description avaiable"]
    pub MSHC_VER_ID: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub MSHC_VER_TYPE: crate::RWRegister<u32>,
    _reserved10: [u8; 0x08],
    #[doc = "Y"]
    pub MBIU_CTRL: crate::RWRegister<u32>,
    _reserved11: [u8; 0x18],
    #[doc = "No description avaiable"]
    pub EMMC_BOOT_CTRL: crate::RWRegister<u32>,
    _reserved12: [u8; 0x10],
    #[doc = "No description avaiable"]
    pub AUTO_TUNING_CTRL: crate::RWRegister<u32>,
    #[doc = "No description avaiable"]
    pub AUTO_TUNING_STAT: crate::RWRegister<u32>,
}
#[doc = "No description avaiable"]
pub mod SDMASA {
    #[doc = "32-bit Block Count (SDMA System Address) - SDMA System Address (Host Version 4 Enable = 0): This register contains the system memory address for an SDMA transfer in the 32-bit addressing mode. When the Host Controller stops an SDMA transfer, this register points to the system address of the next contiguous data position. It can be accessed only if no transaction is executing. Reading this register during data transfers may return an invalid value. - 32-bit Block Count (Host Version 4 Enable = 1): From the Host Controller Version 4.10 specification, this register is redefined as 32-bit Block Count. The Host Controller decrements the block count of this register for every block transfer and the data transfer stops when the count reaches zero. This register must be accessed when no transaction is executing. Reading this register during data transfers may return invalid value. Following are the values for BLOCKCNT_SDMASA: - 0xFFFF_FFFF: 4G - 1 Block - - 0x0000_0002: 2 Blocks - 0x0000_0001: 1 Block - 0x0000_0000: Stop Count Note: - For Host Version 4 Enable = 0, the Host driver does not program the system address in this register while operating in ADMA mode. The system address must be programmed in the ADMA System Address register. - For Host Version 4 Enable = 0, the Host driver programs a non-zero 32-bit block count value in this register when Auto CMD23 is enabled for non-DMA and ADMA modes. Auto CMD23 cannot be used with SDMA. - This register must be programmed with a non-zero value for data transfer if the 32-bit Block count register is used instead of the 16-bit Block count register."]
    pub mod BLOCKCNT_SDMASA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod BLK_ATTR {
    #[doc = "Transfer Block Size These bits specify the block size of data transfers. In case of memory, it is set to 512 bytes. It can be accessed only if no transaction is executing. Read operations during transfers may return an invalid value, and write operations are ignored. Following are the values for XFER_BLOCK_SIZE: - 0x1: 1 byte - 0x2: 2 bytes - 0x3: 3 bytes - . - 0x1FF: 511 byte - 0x200: 512 byt es - . - 0x800: 2048 bytes Note: This register must be programmed with a non-zero value for data transfer."]
    pub mod XFER_BLOCK_SIZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SDMA Buffer Boundary These bits specify the size of contiguous buffer in system memory. The SDMA transfer waits at every boundary specified by these fields and the Host Controller generates the DMA interrupt to request the Host Driver to update the SDMA System Address register. Values: - 0x0 (BYTES_4K): 4K bytes SDMA Buffer Boundary - 0x1 (BYTES_8K): 8K bytes SDMA Buffer Boundary - 0x2 (BYTES_16K): 16K bytes SDMA Buffer Boundary - 0x3 (BYTES_32K): 32K bytes SDMA Buffer Boundary - 0x4 (BYTES_64K): 64K bytes SDMA Buffer Boundary - 0x5 (BYTES_128K): 128K bytes SDMA Buffer Boundary - 0x6 (BYTES_256K): 256K bytes SDMA Buffer Boundary - 0x7 (BYTES_512K): 512K bytes SDMA Buffer Boundary"]
    pub mod SDMA_BUF_BDARY {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "16-bit Block Count - If the Host Version 4 Enable bit is set 0 or the 16-bit Block Count register is set to non-zero, the 16-bit Block Count register is selected. - If the Host Version 4 Enable bit is set 1 and the 16-bit Block Count register is set to zero, the 32-bit Block Count register is selected. Following are the values for BLOCK_CNT: - 0x0: Stop Count - 0x1: 1 Block - 0x2: 2 Blocks - . - 0xFFFF: 65535 Blocks Note: For Host Version 4 Enable = 0, this register must be set to 0000h before programming the 32-bit block count register when Auto CMD23 is enabled for non-DMA and ADMA modes."]
    pub mod BLOCK_CNT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CMD_ARG {
    #[doc = "Command Argument These bits specify the SD/eMMC command argument that is specified in bits 39-8 of the Command format."]
    pub mod ARGUMNET {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CMD_XFER {
    #[doc = "DMA Enable This bit enables the DMA functionality. If this bit is set to 1, a DMA operation begins when the Host Driver writes to the Command register. You can select one of the DMA modes by using DMA Select in the Host Control 1 register. Values: 0x1 (ENABLED): DMA Data transfer 0x0 (DISABLED): No data transfer or Non-DMA data transfer"]
    pub mod DMA_ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Block Count Enable This bit is used to enable the Block Count register, which is relevant for multiple block transfers. If this bit is set to 0, the Block Count register is disabled, which is useful in executing an infinite transfer. The Host Driver must set this bit to 0 when ADMA is used. Values: 0x1 (ENABLED): Enable 0x0 (DISABLED): Disable"]
    pub mod BLOCK_COUNT_ENABLE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Auto Command Enable This field determines use of Auto Command functions. Note: In SDIO, this field must be set as 00b (Auto Command Disabled). Values: 0x0 (AUTO_CMD_DISABLED): Auto Command Disabled 0x1 (AUTO_CMD12_ENABLED): Auto CMD12 Enable 0x2 (AUTO_CMD23_ENABLED): Auto CMD23 Enable 0x3 (AUTO_CMD_AUTO_SEL): Auto CMD Auto Sel"]
    pub mod AUTO_CMD_ENABLE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data Transfer Direction Select This bit defines the direction of DAT line data transfers. This bit is set to 1 by the Host Driver to transfer data from the SD/eMMC card to the Host Controller and it is set to 0 for all other commands. Values: 0x1 (READ): Read (Card to Host) 0x0 (WRITE): Write (Host to Card)"]
    pub mod DATA_XFER_DIR {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Multi/Single Block Select This bit is set when issuing multiple-block transfer commands using the DAT line. If this bit is set to 0, it is not necessary to set the Block Count register. Values: 0x0 (SINGLE): Single Block 0x1 (MULTI): Multiple Block"]
    pub mod MULTI_BLK_SEL {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Response Type R1/R5 This bit selects either R1 or R5 as a response type when the Response Error Check is selected. Error statuses checked in R1: OUT_OF_RANGE ADDRESS_ERROR BLOCK_LEN_ERROR WP_VIOLATION CARD_IS_LOCKED COM_CRC_ERROR CARD_ECC_FAILED CC_ERROR ERROR Response Flags checked in R5: COM_CRC_ERROR ERROR FUNCTION_NUMBER OUT_OF_RANGE Values: 0x0 (RESP_R1): R1 (Memory) 0x1 (RESP_R5): R5 (SDIO)"]
    pub mod RESP_TYPE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Response Error Check Enable The Host Controller supports response check function to avoid overhead of response error check by Host driver. Response types of only R1 and R5 can be checked by the Controller. If the Host Controller checks the response error, set this bit to 1 and set Response Interrupt Disable to 1. If an error is detected, the Response Error interrupt is generated in the Error Interrupt Status register. Note: - Response error check must not be enabled for any response type other than R1 and R5. - Response check must not be enabled for the tuning command. Values: - 0x0 (DISABLED): Response Error Check is disabled - 0x1 (ENABLED): Response Error Check is enabled"]
    pub mod RESP_ERR_CHK_ENABLE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Response Interrupt Disable The Host Controller supports response check function to avoid overhead of response error check by the Host driver. Response types of only R1 and R5 can be checked by the Controller. If Host Driver checks the response error, set this bit to 0 and wait for Command Complete Interrupt and then check the response register. If the Host Controller checks the response error, set this bit to 1 and set the Response Error Check Enable bit to 1. The Command Complete Interrupt is disabled by this bit regardless of the Command Complete Signal Enable. Note: During tuning (when the Execute Tuning bit in the Host Control2 register is set), the Command Complete Interrupt is not generated irrespective of the Response Interrupt Disable setting. Values: - 0x0 (ENABLED): Response Interrupt is enabled - 0x1 (DISABLED): Response Interrupt is disabled"]
    pub mod RESP_INT_DISABLE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Response Type Select This bit indicates the type of response expected from the card. Values: 0x0 (NO_RESP): No Response 0x1 (RESP_LEN_136): Response Length 136 0x2 (RESP_LEN_48): Response Length 48 0x3 (RESP_LEN_48B): Response Length 48; Check Busy after response"]
    pub mod RESP_TYPE_SELECT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Sub Command Flag This bit distinguishes between a main command and a sub command. Values: 0x0 (MAIN): Main Command 0x1 (SUB): Sub Command"]
    pub mod SUB_CMD_FLAG {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Command CRC Check Enable This bit enables the Host Controller to check the CRC field in the response. If an error is detected, it is reported as a Command CRC error. Note: CRC Check enable must be set to 0 for the command with no response, R3 response, and R4 response. For the tuning command, this bit must always be set to 1 to enable the CRC check. Values: 0x0 (DISABLED): Disable 0x1 (ENABLED): Enable"]
    pub mod CMD_CRC_CHK_ENABLE {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Command Index Check Enable This bit enables the Host Controller to check the index field in the response to verify if it has the same value as the command index. If the value is not the same, it is reported as a Command Index error. Note: Index Check enable must be set to 0 for the command with no response, R2 response, R3 response and R4 response. For the tuning command, this bit must always be set to enable the index check. Values: 0x0 (DISABLED): Disable 0x1 (ENABLED): Enable"]
    pub mod CMD_IDX_CHK_ENABLE {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data Present Select This bit is set to 1 to indicate that data is present and that the data is transferred using the DAT line. This bit is set to 0 in the following instances: Command using the CMD line Command with no data transfer but using busy signal on the DAT\\[0\\] line Resume Command Values: 0x0 (NO_DATA): No Data Present 0x1 (DATA): Data Present"]
    pub mod DATA_PRESENT_SEL {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Command Type These bits indicate the command type. Note: While issuing Abort CMD using CMD12/CMD52 or reset CMD using CMD0/CMD52, CMD_TYPE field shall be set to 0x3. Values: 0x3 (ABORT_CMD): Abort 0x2 (RESUME_CMD): Resume 0x1 (SUSPEND_CMD): Suspend 0x0 (NORMAL_CMD): Normal"]
    pub mod CMD_TYPE {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Command Index These bits are set to the command number that is specified in bits 45-40 of the Command Format."]
    pub mod CMD_INDEX {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod RESP_RESP01 {
    #[doc = "Command Response These bits reflect 39-8 bits of SD/eMMC Response Field. Note: For Auto CMD, the 32-bit response (bits 39-8 of the Response Field) is updated in the RESP67_R register."]
    pub mod RESP01 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod RESP_RESP23 {
    #[doc = "Command Response These bits reflect 39-8 bits of SD/eMMC Response Field. Note: For Auto CMD, the 32-bit response (bits 39-8 of the Response Field) is updated in the RESP67_R register."]
    pub mod RESP01 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod RESP_RESP45 {
    #[doc = "Command Response These bits reflect 39-8 bits of SD/eMMC Response Field. Note: For Auto CMD, the 32-bit response (bits 39-8 of the Response Field) is updated in the RESP67_R register."]
    pub mod RESP01 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod RESP_RESP67 {
    #[doc = "Command Response These bits reflect 39-8 bits of SD/eMMC Response Field. Note: For Auto CMD, the 32-bit response (bits 39-8 of the Response Field) is updated in the RESP67_R register."]
    pub mod RESP01 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod BUF_DATA {
    #[doc = "Buffer Data These bits enable access to the Host Controller packet buffer."]
    pub mod BUF_DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PSTATE {
    #[doc = "Command Inhibit (CMD) This bit indicates the following : If this bit is set to 0, it indicates that the CMD line is not in use and the Host controller can issue an SD/eMMC command using the CMD line. This bit is set when the command register is written. This bit is cleared when the command response is received. This bit is not cleared by the response of auto CMD12/23 but cleared by the response of read/write command. Values: 0x0 (READY): Host Controller is ready to issue a command 0x1 (NOT_READY): Host Controller is not ready to issue a command"]
    pub mod CMD_INHIBIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Command Inhibit (DAT) This bit is generated if either DAT line active or Read transfer active is set to 1. If this bit is set to 0, it indicates that the Host Controller can issue subsequent SD/eMMC commands. Values: 0x0 (READY): Can issue command which used DAT line 0x1 (NOT_READY): Cannot issue command which used DAT line"]
    pub mod DAT_INHIBIT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DAT Line Active ( This bit indicates whether one of the DAT lines on the SD/eMMC bus is in use. In the case of read transactions, this bit indicates whether a read transfer is executing on the SD/eMMC bus. In the case of write transactions, this bit indicates whether a write transfer is executing on the SD/eMMC bus. For a command with busy, this status indicates whether the command executing busy is executing on an SD or eMMC bus. Values: 0x0 (INACTIVE): DAT Line Inactive 0x1 (ACTIVE): DAT Line Active"]
    pub mod DAT_LINE_ACTIVE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Re-Tuning Request SDXC does not generate retuning request. The software must maintain the Retuning timer."]
    pub mod RE_TUNE_REQ {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DAT\\[7:4\\] Line Signal Level This bit is used to check the DAT line level to recover from errors and for debugging. These bits reflect the value of the sd_dat_in (upper nibble) signal."]
    pub mod DAT_7_4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write Transfer Active This status indicates whether a write transfer is active for SD/eMMC mode. Values: 0x0 (INACTIVE): No valid data 0x1 (ACTIVE): Transferring data"]
    pub mod WR_XFER_ACTIVE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read Transfer Active This bit indicates whether a read transfer is active for SD/eMMC mode. Values: 0x0 (INACTIVE): No valid data 0x1 (ACTIVE): Transferring data"]
    pub mod RD_XFER_ACTIVE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Buffer Write Enable This bit is used for non-DMA transfers. This bit is set if space is available for writing data. Values: 0x0 (DISABLED): Write disable 0x1 (ENABLED): Write enable"]
    pub mod BUF_WR_ENABLE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Buffer Read Enable This bit is used for non-DMA transfers. This bit is set if valid data exists in the Host buffer. Values: 0x0 (DISABLED): Read disable 0x1 (ENABLED): Read enable"]
    pub mod BUF_RD_ENABLE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Card Inserted This bit indicates whether a card has been inserted. The Host Controller debounces this signal so that Host Driver need not wait for it to stabilize. Values: 0x0 (FALSE): Reset, Debouncing, or No card 0x1 (TRUE): Card Inserted"]
    pub mod CARD_INSERTED {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Card Stable This bit indicates the stability of the Card Detect Pin Level. A card is not detected if this bit is set to 1 and the value of the CARD_INSERTED bit is 0. Values: 0x0 (FALSE): Reset or Debouncing 0x1 (TRUE): No Card or Inserted"]
    pub mod CARD_STABLE {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Card Detect Pin Level This bit reflects the inverse synchronized value of the card_detect_n signal. Values: 0x0 (FALSE): No card present 0x1 (TRUE): Card Present"]
    pub mod CARD_DETECT_PIN_LEVEL {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write Protect Switch Pin Level This bit is supported only for memory and combo cards. This bit reflects the synchronized value of the card_write_prot signal. Values: 0x0 (FALSE): Write protected 0x1 (TRUE): Write enabled"]
    pub mod WR_PROTECT_SW_LVL {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DAT\\[3:0\\] Line Signal Level This bit is used to check the DAT line level to recover from errors and for debugging. These bits reflect the value of the sd_dat_in (lower nibble) signal."]
    pub mod DAT_3_0 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Command-Line Signal Level This bit is used to check the CMD line level to recover from errors and for debugging. These bits reflect the value of the sd_cmd_in signal."]
    pub mod CMD_LINE_LVL {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Command Not Issued by Error This bit is set if a command cannot be issued after setting the command register due to an error except the Auto CMD12 error. Values: 0x0 (FALSE): No error for issuing a command 0x1 (TRUE): Command cannot be issued"]
    pub mod CMD_ISSUE_ERR {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Sub Command Status This bit is used to distinguish between a main command and a sub command status. Values: 0x0 (FALSE): Main Command Status 0x1 (TRUE): Sub Command Status"]
    pub mod SUB_CMD_STAT {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PROT_CTRL {
    #[doc = "Data Transfer Width For SD/eMMC mode,this bit selects the data transfer width of the Host Controller. The Host Driver sets it to match the data width of the SD/eMMC card. In UHS-II mode, this bit is irrelevant. Values: 0x1 (FOUR_BIT): 4-bit mode 0x0 (ONE_BIT): 1-bit mode"]
    pub mod DAT_XFER_WIDTH {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "High Speed Enable this bit is used to determine the selection of preset value for High Speed mode. Before setting this bit, the Host Driver checks the High Speed Support in the Capabilities register. Note: SDXC always outputs the sd_cmd_out and sd_dat_out lines at the rising edge of cclk_tx clock irrespective of this bit. Values: 0x1 (HIGH_SPEED): High Speed mode 0x0 (NORMAL_SPEED): Normal Speed mode"]
    pub mod HIGH_SPEED_EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Select This field is used to select the DMA type. When Host Version 4 Enable is 1 in Host Control 2 register: 0x0 : SDMA is selected 0x1 : Reserved 0x2 : ADMA2 is selected 0x3 : ADMA2 or ADMA3 is selected When Host Version 4 Enable is 0 in Host Control 2 register: 0x0 : SDMA is selected 0x1 : Reserved 0x2 : 32-bit Address ADMA2 is selected 0x3 : 64-bit Address ADMA2 is selected Values: 0x0 (SDMA): SDMA is selected 0x1 (RSVD_BIT): Reserved 0x2 (ADMA2): ADMA2 is selected 0x3 (ADMA2_3): ADMA2 or ADMA3 is selected"]
    pub mod DMA_SEL {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Transfer Width This bit controls 8-bit bus width mode of embedded device. Values: 0x1 (EIGHT_BIT): 8-bit Bus Width 0x0 (DEFAULT): Bus Width is selected by the Data Transfer Width"]
    pub mod EXT_DAT_XFER {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SD Bus Voltage Select for VDD1/eMMC Bus Voltage Select for VDD These bits enable the Host Driver to select the voltage level for an SD/eMMC card. Before setting this register, the Host Driver checks the Voltage Support bits in the Capabilities register. If an unsupported voltage is selected, the Host System does not supply the SD Bus voltage. The value set in this field is available on the SDXC output signal (sd_vdd1_sel), which is used by the voltage switching circuitry. SD Bus Voltage Select options: 0x7 : 3.3V(Typical) 0x6 : 3.0V(Typical) 0x5 : 1.8V(Typical) for Embedded 0x4 : 0x0 - Reserved eMMC Bus Voltage Select options: 0x7 : 3.3V(Typical) 0x6 : 1.8V(Typical) 0x5 : 1.2V(Typical) 0x4 : 0x0 - Reserved Values: 0x7 (V_3_3): 3.3V (Typ.) 0x6 (V_3_0): 3.0V (Typ.) 0x5 (V_1_8): 1.8V (Typ.) for Embedded 0x4 (RSVD4): Reserved 0x3 (RSVD3): Reserved 0x2 (RSVD2): Reserved 0x1 (RSVD1): Reserved 0x0 (RSVD0): Reserved"]
    pub mod SD_BUS_VOL_VDD1 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Stop At Block Gap Request This bit is used to stop executing read and write transactions at the next block gap for non-DMA, SDMA, and ADMA transfers. Values: 0x0 (XFER): Transfer 0x1 (STOP): Stop"]
    pub mod STOP_BG_REQ {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Continue Request This bit is used to restart the transaction, which was stopped using the Stop At Block Gap Request. The Host Controller automatically clears this bit when the transaction restarts. If stop at block gap request is set to 1, any write to this bit is ignored. Values: 0x0 (NO_AFFECT): No Affect 0x1 (RESTART): Restart"]
    pub mod CONTINUE_REQ {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read Wait Control This bit is used to enable the read wait protocol to stop read data using DAT\\[2\\] line if the card supports read wait. Otherwise, the Host Controller has to stop the card clock to hold the read data. In UHS-II mode, Read Wait is disabled. Values: 0x0 (DISABLE): Disable Read Wait Control 0x1 (ENABLE): Enable Read Wait Control"]
    pub mod RD_WAIT_CTRL {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Interrupt At Block Gap This bit is valid only in the 4-bit mode of an SDIO card and is used to select a sample point in the interrupt cycle. Setting to 1 enables interrupt detection at the block gap for a multiple block transfer. Values: 0x0 (DISABLE): Disabled 0x1 (ENABLE): Enabled"]
    pub mod INT_AT_BGAP {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Wakeup Event Enable on Card Interrupt This bit enables wakeup event through a Card Interrupt assertion in the Normal Interrupt Status register. This bit can be set to 1 if FN_WUS (Wake Up Support) in CIS is set to 1. Values: 0x0 (DISABLED): Disable 0x1 (ENABLED): Enable"]
    pub mod CARD_INT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Wakeup Event Enable on SD Card Insertion This bit enables wakeup event through Card Insertion assertion in the Normal Interrupt Status register. FN_WUS (Wake Up Support) in CIS does not affect this bit. Values: 0x0 (DISABLED): Disable 0x1 (ENABLED): Enable"]
    pub mod CARD_INSERT {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Wakeup Event Enable on SD Card Removal This bit enables wakeup event through Card Removal assertion in the Normal Interrupt Status register. For the SDIO card, Wake Up Support (FN_WUS) in the Card Information Structure (CIS) register does not affect this bit. Values: 0x0 (DISABLED): Disable 0x1 (ENABLED): Enable"]
    pub mod CARD_REMOVAL {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod SYS_CTRL {
    #[doc = "Internal Clock Enable This bit is set to 0 when the Host Driver is not using the Host Controller or the Host Controller awaits a wakeup interrupt. The Host Controller must stop its internal clock to enter a very low power state. However, registers can still be read and written to. The value is reflected on the intclk_en signal. Note: If this bit is not used to control the internal clock (base clock and master clock), it is recommended to set this bit to '1' . Values: 0x0 (FALSE): Stop 0x1 (TRUE): Oscillate"]
    pub mod INTERNAL_CLK_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Internal Clock Stable This bit enables the Host Driver to check the clock stability twice after the Internal Clock Enable bit is set and after the PLL Enable bit is set. This bit reflects the synchronized value of the intclk_stable signal after the Internal Clock Enable bit is set to 1, and also reflects the synchronized value of the card_clk_stable signal after the PLL Enable bit is set to 1. Values: 0x0 (FALSE): Not Ready 0x1 (TRUE): Ready"]
    pub mod INTERNAL_CLK_STABLE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SD/eMMC Clock Enable This bit stops the SDCLK or RCLK when set to 0. The SDCLK/RCLK Frequency Select bit can be changed when this bit is set to 0. The value is reflected on the clk2card_on pin. Values: 0x0 (FALSE): Disable providing SDCLK/RCLK 0x1 (TRUE): Enable providing SDCLK/RCLK"]
    pub mod SD_CLK_EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PLL Enable This bit is used to activate the PLL (applicable when Host Version 4 Enable = 1). When Host Version 4 Enable = 0, INTERNAL_CLK_EN bit may be used to activate PLL. The value is reflected on the card_clk_en signal. Note: If this bit is not used to to active the PLL when Host Version 4 Enable = 1, it is recommended to set this bit to '1' . Values: 0x0 (FALSE): PLL is in low power mode 0x1 (TRUE): PLL is enabled"]
    pub mod PLL_ENABLE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Clock Generator Select This bit is used to select the clock generator mode in SDCLK/RCLK Frequency Select. If Preset Value Enable = 0, this bit is set by the Host Driver. If Preset Value Enable = 1, this bit is automatically set to a value specified in one of the Preset Value registers. The value is reflected on the card_clk_gen_sel signal. Values: 0x0 (FALSE): Divided Clock Mode 0x1 (TRUE): Programmable Clock Mode"]
    pub mod CLK_GEN_SELECT {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "These bits specify the upper 2 bits of 10-bit SDCLK/RCLK Frequency Select control. The value is reflected on the upper 2 bits of the card_clk_freq_sel signal."]
    pub mod UPPER_FREQ_SEL {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SDCLK/RCLK Frequency Select These bits are used to select the frequency of the SDCLK signal. These bits depend on setting of Preset Value Enable in the Host Control 2 register. If Preset Value Enable = 0, these bits are set by the Host Driver. If Preset Value Enable = 1, these bits are automatically set to a value specified in one of the Preset Value register. The value is reflected on the lower 8-bit of the card_clk_freq_selsignal. 10-bit Divided Clock Mode: 0x3FF : 1/2046 Divided clock .......... N : 1/2N Divided Clock .......... 0x002 : 1/4 Divided Clock 0x001 : 1/2 Divided Clock 0x000 : Base clock (10MHz - 255 MHz) Programmable Clock Mode : Enables the Host System to select a fine grain SD clock frequency: 0x3FF : Base clock * M /1024 .......... N-1 : Base clock * M /N .......... 0x002 : Base clock * M /3 0x001 : Base clock * M /2 0x000 : Base clock * M"]
    pub mod FREQ_SEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data Timeout Counter Value. This value determines the interval by which DAT line timeouts are detected. The Timeout clock frequency is generated by dividing the base clock TMCLK value by this value. When setting this register, prevent inadvertent timeout events by clearing the Data Timeout Error Status Enable (in the Error Interrupt Status Enable register). The values for these bits are: 0xF : Reserved 0xE : TMCLK x 2^27 ......... 0x1 : TMCLK x 2^14 0x0 : TMCLK x 2^13 Note: During a boot operating in an eMMC mode, an application must configure the boot data timeout value (approximately 1 sec) in this bit."]
    pub mod TOUT_CNT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Software Reset For All This reset affects the entire Host Controller except for the card detection circuit. During its initialization, the Host Driver sets this bit to 1 to reset the Host Controller. All registers are reset except the capabilities register. If this bit is set to 1, the Host Driver must issue reset command and reinitialize the card. Values: 0x0 (FALSE): Work 0x1 (TRUE): Reset"]
    pub mod SW_RST_ALL {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Software Reset For CMD line This bit resets only a part of the command circuit to be able to issue a command. It bit is also used to initialize a UHS-II command circuit. This reset is effective only for a command issuing circuit (including response error statuses related to Command Inhibit (CMD) control) and does not affect the data transfer circuit. Host Controller can continue data transfer even after this reset is executed while handling subcommand-response errors. The following registers and bits are cleared by this bit: Present State register : Command Inhibit (CMD) bit Normal Interrupt Status register : Command Complete bit Error Interrupt Status : Response error statuses related to Command Inhibit (CMD) bit Values: 0x0 (FALSE): Work 0x1 (TRUE): Reset"]
    pub mod SW_RST_CMD {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Software Reset For DAT line This bit is used in SD/eMMC mode and it resets only a part of the data circuit and the DMA circuit is also reset. The following registers and bits are cleared by this bit: Buffer Data Port register -Buffer is cleared and initialized. Present state register -Buffer Read Enable -Buffer Write Enable -Read Transfer Active -Write Transfer Active -DAT Line Active -Command Inhibit (DAT) Block Gap Control register -Continue Request -Stop At Block Gap Request Normal Interrupt status register -Buffer Read Ready -Buffer Write Ready -DMA Interrupt -Block Gap Event -Transfer Complete In UHS-II mode, this bit shall be set to 0 Values: 0x0 (FALSE): Work 0x1 (TRUE): Reset"]
    pub mod SW_RST_DAT {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod INT_STAT {
    #[doc = "Command Complete In an SD/eMMC Mode, this bit is set when the end bit of a response except for Auto CMD12 and Auto CMD23. This interrupt is not generated when the Response Interrupt Disable in Transfer Mode Register is set to 1. Values: 0x0 (FALSE): No command complete 0x1 (TRUE): Command Complete"]
    pub mod CMD_COMPLETE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transfer Complete This bit is set when a read/write transfer and a command with status busy is completed. Values: 0x0 (FALSE): Not complete 0x1 (TRUE): Command execution is completed"]
    pub mod XFER_COMPLETE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Block Gap Event This bit is set when both read/write transaction is stopped at block gap due to a Stop at Block Gap Request. Values: 0x0 (FALSE): No Block Gap Event 0x1 (TRUE): Transaction stopped at block gap"]
    pub mod BGAP_EVENT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Interrupt This bit is set if the Host Controller detects the SDMA Buffer Boundary during transfer. In case of ADMA, by setting the Int field in the descriptor table, the Host controller generates this interrupt. This interrupt is not generated after a Transfer Complete. Values: 0x0 (FALSE): No DMA Interrupt 0x1 (TRUE): DMA Interrupt is generated"]
    pub mod DMA_INTERRUPT {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Buffer Write Ready This bit is set if the Buffer Write Enable changes from 0 to 1. Values: 0x0 (FALSE): Not ready to write buffer 0x1 (TRUE): Ready to write buffer"]
    pub mod BUF_WR_READY {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Buffer Read Ready This bit is set if the Buffer Read Enable changes from 0 to 1. Values: 0x0 (FALSE): Not ready to read buffer 0x1 (TRUE): Ready to read buffer"]
    pub mod BUF_RD_READY {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Card Insertion This bit is set if the Card Inserted in the Present State register changes from 0 to 1. Values: 0x0 (FALSE): Card state stable or Debouncing 0x1 (TRUE): Card Inserted"]
    pub mod CARD_INSERTION {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Card Removal This bit is set if the Card Inserted in the Present State register changes from 1 to 0. Values: 0x0 (FALSE): Card state stable or Debouncing 0x1 (TRUE): Card Removed"]
    pub mod CARD_REMOVAL {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Card Interrupt This bit reflects the synchronized value of: DAT\\[1\\] Interrupt Input for SD Mode DAT\\[2\\] Interrupt Input for UHS-II Mode Values: 0x0 (FALSE): No Card Interrupt 0x1 (TRUE): Generate Card Interrupt"]
    pub mod CARD_INTERRUPT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Re-tuning Event This bit is set if the Re-Tuning Request changes from 0 to 1. Re-Tuning request is not supported."]
    pub mod RE_TUNE_EVENT {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FX Event This status is set when R\\[14\\] of response register is set to 1 and Response Type R1/R5 is set to 0 in Transfer Mode register. This interrupt is used with response check function. Values: 0x0 (FALSE): No Event 0x1 (TRUE): FX Event is detected"]
    pub mod FX_EVENT {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Command Queuing Event This status is set if Command Queuing/Crypto related event has occurred in eMMC/SD mode. Read CQHCI's CQIS/CRNQIS register for more details. Values: 0x0 (FALSE): No Event 0x1 (TRUE): Command Queuing Event is detected"]
    pub mod CQE_EVENT {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error Interrupt If any of the bits in the Error Interrupt Status register are set, then this bit is set. Values: 0x0 (FALSE): No Error 0x1 (TRUE): Error"]
    pub mod ERR_INTERRUPT {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Command Timeout Error In SD/eMMC Mode,this bit is set only if no response is returned within 64 SD clock cycles from the end bit of the command. If the Host Controller detects a CMD line conflict, along with Command CRC Error bit, this bit is set to 1, without waiting for 64 SD/eMMC card clock cycles. Values: 0x0 (FALSE): No error 0x1 (TRUE): Time out"]
    pub mod CMD_TOUT_ERR {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Command CRC Error Command CRC Error is generated in SD/eMMC mode for following two cases. If a response is returned and the Command Timeout Error is set to 0 (indicating no timeout), this bit is set to 1 when detecting a CRC error in the command response. The Host Controller detects a CMD line conflict by monitoring the CMD line when a command is issued. If the Host Controller drives the CMD line to 1 level, but detects 0 level on the CMD line at the next SD clock edge, then the Host Controller aborts the command (stop driving CMD line) and set this bit to 1. The Command Timeout Error is also set to 1 to distinguish a CMD line conflict. Values: 0x0 (FALSE): No error 0x1 (TRUE): CRC error generated"]
    pub mod CMD_CRC_ERR {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Command End Bit Error This bit is set when detecting that the end bit of a command response is 0 in SD/eMMC mode. Values: 0x0 (FALSE): No error 0x1 (TRUE): End Bit error generated"]
    pub mod CMD_END_BIT_ERR {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Command Index Error This bit is set if a Command Index error occurs in the command respons in SD/eMMC mode. Values: 0x0 (FALSE): No error 0x1 (TRUE): Error"]
    pub mod CMD_IDX_ERR {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data Timeout Error This bit is set in SD/eMMC mode when detecting one of the following timeout conditions: Busy timeout for R1b, R5b type Busy timeout after Write CRC status Write CRC Status timeout Read Data timeout Values: 0x0 (FALSE): No error 0x1 (TRUE): Time out"]
    pub mod DATA_TOUT_ERR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data CRC Error This error occurs in SD/eMMC mode when detecting CRC error when transferring read data which uses the DAT line, when detecting the Write CRC status having a value of other than 010 or when write CRC status timeout. Values: 0x0 (FALSE): No error 0x1 (TRUE): Error"]
    pub mod DATA_CRC_ERR {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data End Bit Error This error occurs in SD/eMMC mode either when detecting 0 at the end bit position of read data that uses the DAT line or at the end bit position of the CRC status. Values: 0x0 (FALSE): No error 0x1 (TRUE): Error"]
    pub mod DATA_END_BIT_ERR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Current Limit Error By setting the SD Bus Power bit in the Power Control register, the Host Controller is requested to supply power for the SD Bus. If the Host Controller supports the Current Limit function, it can be protected from an illegal card by stopping power supply to the card in which case this bit indicates a failure status. A reading of 1 for this bit means that the Host Controller is not supplying power to the SD card due to some failure. A reading of 0 for this bit means that the Host Controller is supplying power and no error has occurred. The Host Controller may require some sampling time to detect the current limit. SDXC Host Controller does not support this function, this bit is always set to 0. Values: 0x0 (FALSE): No error 0x1 (TRUE): Power Fail"]
    pub mod CUR_LMT_ERR {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Auto CMD Error This error status is used by Auto CMD12 and Auto CMD23 in SD/eMMC mode. This bit is set when detecting that any of the bits D00 to D05 in Auto CMD Error Status register has changed from 0 to 1. D07 is effective in case of Auto CMD12. Auto CMD Error Status register is valid while this bit is set to 1 and may be cleared by clearing of this bit. Values: 0x0 (FALSE): No error 0x1 (TRUE): Error"]
    pub mod AUTO_CMD_ERR {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ADMA Error This bit is set when the Host Controller detects error during ADMA-based data transfer. The error could be due to following reasons: Error response received from System bus (Master I/F) ADMA3,ADMA2 Descriptors invalid CQE Task or Transfer descriptors invalid When the error occurs, the state of the ADMA is saved in the ADMA Error Status register. In eMMC CQE mode: The Host Controller generates this Interrupt when it detects an invalid descriptor data (Valid=0) at the ST_FDS state. ADMA Error State in the ADMA Error Status indicates that an error has occurred in ST_FDS state. The Host Driver may find that Valid bit is not set at the error descriptor. Values: 0x0 (FALSE): No error 0x1 (TRUE): Error"]
    pub mod ADMA_ERR {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Tuning Error This bit is set when an unrecoverable error is detected in a tuning circuit except during the tuning procedure (occurrence of an error during tuning procedure is indicated by Sampling Clock Select in the Host Control 2 register). By detecting Tuning Error, Host Driver needs to abort a command executing and perform tuning. To reset tuning circuit, Sampling Clock Select is set to 0 before executing tuning procedure. The Tuning Error is higher priority than the other error interrupts generated during data transfer. By detecting Tuning Error, the Host Driver must discard data transferred by a current read/write command and retry data transfer after the Host Controller retrieved from the tuning circuit error. This is applicable in SD/eMMC mode. Values: 0x0 (FALSE): No error 0x1 (TRUE): Error"]
    pub mod TUNING_ERR {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Response Error Host Controller Version 4.00 supports response error check function to avoid overhead of response error check by Host Driver during DMA execution. If Response Error Check Enable is set to 1 in the Transfer Mode register, Host Controller Checks R1 or R5 response. If an error is detected in a response, this bit is set to 1.This is applicable in SD/eMMC mode. Values: 0x0 (FALSE): No error 0x1 (TRUE): Error"]
    pub mod RESP_ERR {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Boot Acknowledgement Error This bit is set when there is a timeout for boot acknowledgement or when detecting boot ack status having a value other than 010. This is applicable only when boot acknowledgement is expected in eMMC mode. In SD/UHS-II mode, this bit is irrelevant."]
    pub mod BOOT_ACK_ERR {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod INT_STAT_EN {
    #[doc = "Command Complete Status Enable Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    pub mod CMD_COMPLETE_STAT_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transfer Complete Status Enable Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    pub mod XFER_COMPLETE_STAT_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Block Gap Event Status Enable Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    pub mod BGAP_EVENT_STAT_EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Interrupt Status Enable Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    pub mod DMA_INTERRUPT_STAT_EN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Buffer Write Ready Status Enable Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    pub mod BUF_WR_READY_STAT_EN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Buffer Read Ready Status Enable Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    pub mod BUF_RD_READY_STAT_EN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Card Insertion Status Enable Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    pub mod CARD_INSERTION_STAT_EN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Card Removal Status Enable Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    pub mod CARD_REMOVAL_STAT_EN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Card Interrupt Status Enable If this bit is set to 0, the Host Controller clears the interrupt request to the System. The Card Interrupt detection is stopped when this bit is cleared and restarted when this bit is set to 1. The Host Driver may clear the Card Interrupt Status Enable before servicing the Card Interrupt and may set this bit again after all interrupt requests from the card are cleared to prevent inadvertent interrupts. By setting this bit to 0, interrupt input must be masked by implementation so that the interrupt input is not affected by external signal in any state (for example, floating). Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    pub mod CARD_INTERRUPT_STAT_EN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Re-Tuning Event (UHS-I only) Status Enable Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    pub mod RE_TUNE_EVENT_STAT_EN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FX Event Status Enable This bit is added from Version 4.10. Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    pub mod FX_EVENT_STAT_EN {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CQE Event Status Enable Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    pub mod CQE_EVENT_STAT_EN {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Command Timeout Error Status Enable (SD/eMMC Mode only). Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    pub mod CMD_TOUT_ERR_STAT_EN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Command CRC Error Status Enable (SD/eMMC Mode only) Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    pub mod CMD_CRC_ERR_STAT_EN {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Command End Bit Error Status Enable (SD/eMMC Mode only) Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    pub mod CMD_END_BIT_ERR_STAT_EN {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Command Index Error Status Enable (SD/eMMC Mode only) Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    pub mod CMD_IDX_ERR_STAT_EN {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data Timeout Error Status Enable (SD/eMMC Mode only) Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    pub mod DATA_TOUT_ERR_STAT_EN {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data CRC Error Status Enable (SD/eMMC Mode only) Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    pub mod DATA_CRC_ERR_STAT_EN {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data End Bit Error Status Enable (SD/eMMC Mode only). Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    pub mod DATA_END_BIT_ERR_STAT_EN {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Current Limit Error Status Enable Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    pub mod CUR_LMT_ERR_STAT_EN {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Auto CMD Error Status Enable (SD/eMMC Mode only). Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    pub mod AUTO_CMD_ERR_STAT_EN {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ADMA Error Status Enable Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    pub mod ADMA_ERR_STAT_EN {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Tuning Error Status Enable (UHS-I Mode only) Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    pub mod TUNING_ERR_STAT_EN {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Response Error Status Enable (SD Mode only) Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    pub mod RESP_ERR_STAT_EN {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Boot Acknowledgment Error (eMMC Mode only) Setting this bit to 1 enables setting of Boot Acknowledgment Error in Error Interrupt Status register (ERROR_INT_STAT_R). Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    pub mod BOOT_ACK_ERR_STAT_EN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod INT_SIGNAL_EN {
    #[doc = "Command Complete Signal Enable Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    pub mod CMD_COMPLETE_SIGNAL_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transfer Complete Signal Enable Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    pub mod XFER_COMPLETE_SIGNAL_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Block Gap Event Signal Enable Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    pub mod BGAP_EVENT_SIGNAL_EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Interrupt Signal Enable Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    pub mod DMA_INTERRUPT_SIGNAL_EN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Buffer Write Ready Signal Enable Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    pub mod BUF_WR_READY_SIGNAL_EN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Buffer Read Ready Signal Enable Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    pub mod BUF_RD_READY_SIGNAL_EN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Card Insertion Signal Enable Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    pub mod CARD_INSERTION_SIGNAL_EN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Card Removal Signal Enable Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    pub mod CARD_REMOVAL_SIGNAL_EN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Card Interrupt Signal Enable Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    pub mod CARD_INTERRUPT_SIGNAL_EN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Re-Tuning Event (UHS-I only) Signal Enable. Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    pub mod RE_TUNE_EVENT_SIGNAL_EN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FX Event Signal Enable Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    pub mod FX_EVENT_SIGNAL_EN {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Command Queuing Engine Event Signal Enable Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    pub mod CQE_EVENT_SIGNAL_EN {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Command Timeout Error Signal Enable (SD/eMMC Mode only) Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    pub mod CMD_TOUT_ERR_SIGNAL_EN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Command CRC Error Signal Enable (SD/eMMC Mode only) Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    pub mod CMD_CRC_ERR_SIGNAL_EN {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Command End Bit Error Signal Enable (SD/eMMC Mode only) Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    pub mod CMD_END_BIT_ERR_SIGNAL_EN {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Command Index Error Signal Enable (SD/eMMC Mode only) Values: 0x0 (FALSE): No error 0x1 (TRUE): Error"]
    pub mod CMD_IDX_ERR_SIGNAL_EN {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data Timeout Error Signal Enable (SD/eMMC Mode only) Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    pub mod DATA_TOUT_ERR_SIGNAL_EN {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data CRC Error Signal Enable (SD/eMMC Mode only) Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    pub mod DATA_CRC_ERR_SIGNAL_EN {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data End Bit Error Signal Enable (SD/eMMC Mode only) Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    pub mod DATA_END_BIT_ERR_SIGNAL_EN {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Current Limit Error Signal Enable Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    pub mod CUR_LMT_ERR_SIGNAL_EN {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Auto CMD Error Signal Enable (SD/eMMC Mode only) Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    pub mod AUTO_CMD_ERR_SIGNAL_EN {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ADMA Error Signal Enable Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    pub mod ADMA_ERR_SIGNAL_EN {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Tuning Error Signal Enable (UHS-I Mode only) Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    pub mod TUNING_ERR_SIGNAL_EN {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Response Error Signal Enable (SD Mode only) Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    pub mod RESP_ERR_SIGNAL_EN {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Boot Acknowledgment Error (eMMC Mode only). Setting this bit to 1 enables generating interrupt signal when Boot Acknowledgement Error in Error Interrupt Status register is set. Values: 0x0 (FALSE): Masked 0x1 (TRUE): Enabled"]
    pub mod BOOT_ACK_ERR_SIGNAL_EN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod AC_HOST_CTRL {
    #[doc = "Auto CMD12 Not Executed If multiple memory block data transfer is not started due to a command error, this bit is not set because it is not necessary to issue an Auto CMD12. Setting this bit to 1 means that the Host Controller cannot issue Auto CMD12 to stop multiple memory block data transfer, due to some error. If this bit is set to 1, error status bits (D04-D01) is meaningless. This bit is set to 0 when Auto CMD Error is generated by Auto CMD23. Values: 0x1 (TRUE): Not Executed 0x0 (FALSE): Executed"]
    pub mod AUTO_CMD12_NOT_EXEC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Auto CMD Timeout Error This bit is set if no response is returned with 64 SDCLK cycles from the end bit of the command. If this bit is set to 1, error status bits (D04-D01) are meaningless. Values: 0x1 (TRUE): Time out 0x0 (FALSE): No Error"]
    pub mod AUTO_CMD_TOUT_ERR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Auto CMD CRC Error This bit is set when detecting a CRC error in the command response. Values: 0x1 (TRUE): CRC Error Generated 0x0 (FALSE): No Error"]
    pub mod AUTO_CMD_CRC_ERR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Auto CMD End Bit Error This bit is set when detecting that the end bit of command response is 0. Values: 0x1 (TRUE): End Bit Error Generated 0x0 (FALSE): No Error"]
    pub mod AUTO_CMD_EBIT_ERR {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Auto CMD Index Error This bit is set if the command index error occurs in response to a command. Values: 0x1 (TRUE): Error 0x0 (FALSE): No Error"]
    pub mod AUTO_CMD_IDX_ERR {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Auto CMD Response Error This bit is set when Response Error Check Enable in the Transfer Mode register is set to 1 and an error is detected in R1 response of either Auto CMD12 or CMD13. This status is ignored if any bit between D00 to D04 is set to 1. Values: 0x1 (TRUE): Error 0x0 (FALSE): No Error"]
    pub mod AUTO_CMD_RESP_ERR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Command Not Issued By Auto CMD12 Error If this bit is set to 1, CMD_wo_DAT is not executed due to an Auto CMD12 Error (D04-D01) in this register. This bit is set to 0 when Auto CMD Error is generated by Auto CMD23. Values: 0x1 (TRUE): Not Issued 0x0 (FALSE): No Error"]
    pub mod CMD_NOT_ISSUED_AUTO_CMD12 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "UHS Mode/eMMC Speed Mode Select These bits are used to select UHS mode in the SD mode of operation. In eMMC mode, these bits are used to select eMMC Speed mode. UHS Mode (SD/UHS-II mode only): 0x0 (SDR12): SDR12/Legacy 0x1 (SDR25): SDR25/High Speed SDR 0x2 (SDR50): SDR50 0x3 (SDR104): SDR104/HS200 0x4 (DDR50): DDR50/High Speed DDR 0x5 (RSVD5): Reserved 0x6 (RSVD6): Reserved 0x7 (UHS2): UHS-II/HS400 eMMC Speed Mode (eMMC mode only): 0x0: Legacy 0x1: High Speed SDR 0x2: Reserved 0x3: HS200 0x4: High Speed DDR 0x5: Reserved 0x6: Reserved 0x7: HS400"]
    pub mod UHS_MODE_SEL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1.8V Signaling Enable This bit controls voltage regulator for I/O cell in UHS-I/eMMC speed modes. Setting this bit from 0 to 1 starts changing the signal voltage from 3.3V to 1.8V. Host Controller clears this bit if switching to 1.8 signaling fails. The value is reflected on the uhs1_swvolt_en pin. Note: This bit must be set for all UHS-I speed modes (SDR12/SDR25/SDR50/SDR104/DDR50). Values: 0x0 (V_3_3): 3.3V Signalling 0x1 (V_1_8): 1.8V Signalling"]
    pub mod SIGNALING_EN {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Execute Tuning This bit is set to 1 to start the tuning procedure in UHS-I/eMMC speed modes and this bit is automatically cleared when tuning procedure is completed. Values: 0x0 (FALSE): Not Tuned or Tuning completed 0x1 (TRUE): Execute Tuning"]
    pub mod EXEC_TUNING {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Sampling Clock Select This bit is used by the Host Controller to select the sampling clock in SD/eMMC mode to receive CMD and DAT. This bit is set by the tuning procedure and is valid after the completion of tuning (when Execute Tuning is cleared). Setting this bit to 1 means that tuning is completed successfully and setting this bit to 0 means that tuning has failed. The value is reflected on the sample_cclk_sel pin. Values: 0x0 (FALSE): Fixed clock is used to sample data 0x1 (TRUE): Tuned clock is used to sample data"]
    pub mod SAMPLE_CLK_SEL {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ADMA2 Length Mode This bit selects ADMA2 Length mode to be either 16-bit or 26-bit. Values: 0x0 (FALSE): 16-bit Data Length Mode 0x1 (TRUE): 26-bit Data Length Mode"]
    pub mod ADMA2_LEN_MODE {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CMD23 Enable If the card supports CMD23, this bit is set to 1. This bit is used to select Auto CMD23 or Auto CMD12 for ADMA3 data transfer. Values: 0x0 (FALSE): Auto CMD23 is disabled 0x1 (TRUE): Auto CMD23 is enabled"]
    pub mod CMD23_ENABLE {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Host Version 4 Enable This bit selects either Version 3.00 compatible mode or Version 4 mode. Functions of following fields are modified for Host Version 4 mode: SDMA Address: SDMA uses ADMA System Address (05Fh-058h) instead of SDMA System Address register (003h-000h) ADMA2/ADMA3 selection: ADMA3 is selected by DMA select in Host Control 1 register 64-bit ADMA Descriptor Size: 128-bit descriptor is used instead of 96-bit descriptor when 64-bit Addressing is set to 1 Selection of 32-bit/64-bit System Addressing: Either 32-bit or 64-bit system addressing is selected by 64-bit Addressing bit in this register 32-bit Block Count: SDMA System Address register (003h-000h) is modified to 32-bit Block Count register Note: It is recommended not to program ADMA3 Integrated Descriptor Address registers, UHS-II registers and Command Queuing registers (if applicable) while operating in Host version less than 4 mode (Host Version 4 Enable = 0). Values: 0x0 (FALSE): Version 3.00 compatible mode 0x1 (TRUE): Version 4 mode"]
    pub mod HOST_VER4_ENABLE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Asynchronous Interrupt Enable This bit can be set if a card supports asynchronous interrupts and Asynchronous Interrupt Support is set to 1 in the Capabilities register. Values: 0x0 (FALSE): Disabled 0x1 (TRUE): Enabled"]
    pub mod ASYNC_INT_ENABLE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Preset Value Enable This bit enables automatic selection of SDCLK frequency and Driver strength Preset Value registers. When Preset Value Enable is set, SDCLK frequency generation (Frequency Select and Clock Generator Select) and the driver strength selection are performed by the controller. These values are selected from set of Preset Value registers based on selected speed mode. Values: 0x0 (FALSE): SDCLK and Driver Strength are controlled by Host Driver 0x1 (TRUE): Automatic Selection by Preset Value are Enabled"]
    pub mod PRESET_VAL_ENABLE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CAPABILITIES1 {
    #[doc = "Timeout Clock Frequency This bit shows the base clock frequency used to detect Data Timeout Error. The Timeout Clock unit defines the unit of timeout clock frequency. It can be KHz or MHz. 0x00 : Get information through another method 0x01 : 1KHz / 1MHz 0x02 : 2KHz / 2MHz 0x03 : 3KHz / 3MHz ........... 0x3F : 63KHz / 63MHz"]
    pub mod TOUT_CLK_FREQ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Timeout Clock Unit This bit shows the unit of base clock frequency used to detect Data TImeout Error. Values: 0x0 (KHZ): KHz 0x1 (MHZ): MHz"]
    pub mod TOUT_CLK_UNIT {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Base Clock Frequency for SD clock These bits indicate the base (maximum) clock frequency for the SD Clock. The definition of these bits depend on the Host Controller Version. 6-Bit Base Clock Frequency: This mode is supported by the Host Controller version 1.00 and 2.00. The upper 2 bits are not effective and are always 0. The unit values are 1 MHz. The supported clock range is 10 MHz to 63 MHz. -0x00 : Get information through another method -0x01 : 1 MHz -0x02 : 2 MHz -............. -0x3F : 63 MHz -0x40-0xFF : Not Supported 8-Bit Base Clock Frequency: This mode is supported by the Host Controller version 3.00. The unit values are 1 MHz. The supported clock range is 10 MHz to 255 MHz. -0x00 : Get information through another method -0x01 : 1 MHz -0x02 : 2 MHz -............ -0xFF : 255 MHz If the frequency is 16.5 MHz, the larger value is set to 0001001b (17 MHz) because the Host Driver uses this value to calculate the clock divider value and it does not exceed the upper limit of the SD Clock frequency. If these bits are all 0, the Host system has to get information using a different method."]
    pub mod BASE_CLK_FREQ {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Maximum Block Length This bit indicates the maximum block size that the Host driver can read and write to the buffer in the Host Controller. The buffer transfers this block size without wait cycles. The transfer block length is always 512 bytes for the SD Memory irrespective of this bit Values: 0x0 (ZERO): 512 Byte 0x1 (ONE): 1024 Byte 0x2 (TWO): 2048 Byte 0x3 (THREE): Reserved"]
    pub mod MAX_BLK_LEN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "8-bit Support for Embedded Device This bit indicates whether the Host Controller is capable of using an 8-bit bus width mode. This bit is not effective when the Slot Type is set to 10b. Values: 0x0 (FALSE): 8-bit Bus Width not Supported 0x1 (TRUE): 8-bit Bus Width Supported"]
    pub mod EMBEDDED_8_BIT {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ADMA2 Support This bit indicates whether the Host Controller is capable of using ADMA2. Values: 0x0 (FALSE): ADMA2 not Supported 0x1 (TRUE): ADMA2 Supported"]
    pub mod ADMA2_SUPPORT {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "High Speed Support This bit indicates whether the Host Controller and the Host System supports High Speed mode and they can supply the SD Clock frequency from 25 MHz to 50 MHz. Values: 0x0 (FALSE): High Speed not Supported 0x1 (TRUE): High Speed Supported"]
    pub mod HIGH_SPEED_SUPPORT {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SDMA Support This bit indicates whether the Host Controller is capable of using SDMA to transfer data between the system memory and the Host Controller directly. Values: 0x0 (FALSE): SDMA not Supported 0x1 (TRUE): SDMA Supported"]
    pub mod SDMA_SUPPORT {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Suspense/Resume Support This bit indicates whether the Host Controller supports Suspend/Resume functionality. If this bit is 0, the Host Driver does not issue either Suspend or Resume commands because the Suspend and Resume mechanism is not supported. Values: 0x0 (FALSE): Not Supported 0x1 (TRUE): Supported"]
    pub mod SUS_RES_SUPPORT {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Voltage Support for 3.3V Values: 0x0 (FALSE): 3.3V Not Supported 0x1 (TRUE): 3.3V Supported"]
    pub mod VOLT_33 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Voltage Support for SD 3.0V or Embedded 1.2V Values: 0x0 (FALSE): SD 3.0V or Embedded 1.2V Not Supported 0x1 (TRUE): SD 3.0V or Embedded Supported"]
    pub mod VOLT_30 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Voltage Support for 1.8V Values: 0x0 (FALSE): 1.8V Not Supported 0x1 (TRUE): 1.8V Supported"]
    pub mod VOLT_18 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Asynchronous Interrupt Support (SD Mode only) Values: 0x0 (FALSE): Asynchronous Interrupt Not Supported 0x1 (TRUE): Asynchronous Interrupt Supported"]
    pub mod ASYNC_INT_SUPPORT {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Slot Type These bits indicate usage of a slot by a specific Host System. Values: 0x0 (REMOVABLE_SLOT): Removable Card Slot 0x1 (EMBEDDED_SLOT): Embedded Slot for one Device 0x2 (SHARED_SLOT): Shared Bus Slot (SD mode) 0x3 (UHS2_EMBEDDED_SLOT): UHS-II Multiple Embedded Devices"]
    pub mod SLOT_TYPE_R {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CAPABILITIES2 {
    #[doc = "SDR50 Support (UHS-I only) This bit indicates that SDR50 is supported. The bit 13 (USE_TUNING_SDR50) indicates whether SDR50 requires tuning or not. Values: 0x0 (FALSE): SDR50 is not supported 0x1 (TRUE): SDR50 is supported"]
    pub mod SDR50_SUPPORT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SDR104 Support (UHS-I only) This bit mentions that SDR104 requires tuning. Values: 0x0 (FALSE): SDR104 is not supported 0x1 (TRUE): SDR104 is supported"]
    pub mod SDR104_SUPPORT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DDR50 Support (UHS-I only) Values: 0x0 (FALSE): DDR50 is not supported 0x1 (TRUE): DDR50 is supported"]
    pub mod DDR50_SUPPORT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "UHS-II Support (UHS-II only) This bit indicates whether Host Controller supports UHS-II. Values: 0x0 (FALSE): UHS-II is not supported 0x1 (TRUE): UHS-II is supported"]
    pub mod UHS2_SUPPORT {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Driver Type A Support (UHS-I only) This bit indicates support of Driver Type A for 1.8 Signaling. Values: 0x0 (FALSE): Driver Type A is not supported 0x1 (TRUE): Driver Type A is supported"]
    pub mod DRV_TYPEA {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Driver Type C Support (UHS-I only) This bit indicates support of Driver Type C for 1.8 Signaling. Values: 0x0 (FALSE): Driver Type C is not supported 0x1 (TRUE): Driver Type C is supported"]
    pub mod DRV_TYPEC {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Driver Type D Support (UHS-I only) This bit indicates support of Driver Type D for 1.8 Signaling. Values: 0x0 (FALSE): Driver Type D is not supported 0x1 (TRUE): Driver Type D is supported"]
    pub mod DRV_TYPED {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Timer Count for Re-Tuning (UHS-I only) 0x0: Re-Tuning Timer disabled 0x1: 1 seconds 0x2: 2 seconds 0x3: 4 seconds ........ 0xB: 1024 seconds 0xC: Reserved 0xD: Reserved 0xE: Reserved 0xF: Get information from other source"]
    pub mod RETUNE_CNT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Use Tuning for SDR50 (UHS-I only) Values: 0x0 (ZERO): SDR50 does not require tuning 0x1 (ONE): SDR50 requires tuning"]
    pub mod USE_TUNING_SDR50 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Re-Tuning Modes (UHS-I only) These bits select the re-tuning method and limit the maximum data length. Values: 0x0 (MODE1): Timer 0x1 (MODE2): Timer and Re-Tuning Request (Not supported) 0x2 (MODE3): Auto Re-Tuning (for transfer) 0x3 (RSVD_MODE): Reserved"]
    pub mod RE_TUNING_MODES {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Clock Multiplier These bits indicate the clock multiplier of the programmable clock generator. Setting these bits to 0 means that the Host Controller does not support a programmable clock generator. 0x0: Clock Multiplier is not Supported 0x1: Clock Multiplier M = 2 0x2: Clock Multiplier M = 3 ......... 0xFF: Clock Multiplier M = 256"]
    pub mod CLK_MUL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ADMA3 Support This bit indicates whether the Host Controller is capable of using ADMA3. Values: 0x0 (FALSE): ADMA3 not Supported 0x1 (TRUE): ADMA3 Supported"]
    pub mod ADMA3_SUPPORT {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1.8V VDD2 Support This bit indicates support of VDD2 for the Host System. 0x0 (FALSE): 1.8V VDD2 is not Supported 0x1 (TRUE): 1.8V VDD2 is Supported"]
    pub mod VDD2_18V_SUPPORT {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CURR_CAPABILITIES1 {
    #[doc = "Maximum Current for 3.3V This bit specifies the Maximum Current for 3.3V VDD1 power supply for the card. 0: Get information through another method 1: 4mA 2: 8mA 3: 13mA ....... 255: 1020mA"]
    pub mod MAX_CUR_33V {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Maximum Current for 3.0V This bit specifies the Maximum Current for 3.0V VDD1 power supply for the card. 0: Get information through another method 1: 4mA 2: 8mA 3: 13mA ....... 255: 1020mA"]
    pub mod MAX_CUR_30V {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Maximum Current for 1.8V This bit specifies the Maximum Current for 1.8V VDD1 power supply for the card. 0: Get information through another method 1: 4mA 2: 8mA 3: 13mA ....... 255: 1020mA"]
    pub mod MAX_CUR_18V {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CURR_CAPABILITIES2 {
    #[doc = "Maximum Current for 1.8V VDD2 This bit specifies the Maximum Current for 1.8V VDD2 power supply for the UHS-II card. 0: Get information through another method 1: 4mA 2: 8mA 3: 13mA ....... 255: 1020mA"]
    pub mod MAX_CUR_VDD2_18V {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod FORCE_EVENT {
    #[doc = "Force Event for Auto CMD12 Not Executed Values: 0x1 (TRUE): Auto CMD12 Not Executed Status is set 0x0 (FALSE): Not Affected"]
    pub mod FORCE_AUTO_CMD12_NOT_EXEC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Force Event for Auto CMD Timeout Error Values: 0x1 (TRUE): Auto CMD Timeout Error Status is set 0x0 (FALSE): Not Affected"]
    pub mod FORCE_AUTO_CMD_TOUT_ERR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Force Event for Auto CMD CRC Error Values: 0x1 (TRUE): Auto CMD CRC Error Status is set 0x0 (FALSE): Not Affected"]
    pub mod FORCE_AUTO_CMD_CRC_ERR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Force Event for Auto CMD End Bit Error Values: 0x1 (TRUE): Auto CMD End Bit Error Status is set 0x0 (FALSE): Not Affected"]
    pub mod FORCE_AUTO_CMD_EBIT_ERR {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Force Event for Auto CMD Index Error Values: 0x1 (TRUE): Auto CMD Index Error Status is set 0x0 (FALSE): Not Affected"]
    pub mod FORCE_AUTO_CMD_IDX_ERR {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Force Event for Auto CMD Response Error Values: 0x1 (TRUE): Auto CMD Response Error Status is set 0x0 (FALSE): Not Affected"]
    pub mod FORCE_AUTO_CMD_RESP_ERR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Force Event for Command Not Issued By Auto CMD12 Error Values: 0x1 (TRUE): Command Not Issued By Auto CMD12 Error Status is set 0x0 (FALSE): Not Affected"]
    pub mod FORCE_CMD_NOT_ISSUED_AUTO_CMD12 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Force Event for Command Timeout Error (SD/eMMC Mode only) Values: 0x0 (FALSE): Not Affected 0x1 (TRUE): Command Timeout Error Status is set"]
    pub mod FORCE_CMD_TOUT_ERR {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Force Event for Command CRC Error (SD/eMMC Mode only) Values: 0x0 (FALSE): Not Affected 0x1 (TRUE): Command CRC Error Status is set"]
    pub mod FORCE_CMD_CRC_ERR {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Force Event for Command End Bit Error (SD/eMMC Mode only) Values: 0x0 (FALSE): Not Affected 0x1 (TRUE): Command End Bit Error Status is set"]
    pub mod FORCE_CMD_END_BIT_ERR {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Force Event for Command Index Error (SD/eMMC Mode only) Values: 0x0 (FALSE): Not Affected 0x1 (TRUE): Command Index Error Status is set"]
    pub mod FORCE_CMD_IDX_ERR {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Force Event for Data Timeout Error (SD/eMMC Mode only) Values: 0x0 (FALSE): Not Affected 0x1 (TRUE): Data Timeout Error Status is set"]
    pub mod FORCE_DATA_TOUT_ERR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Force Event for Data CRC Error (SD/eMMC Mode only) Values: 0x0 (FALSE): Not Affected 0x1 (TRUE): Data CRC Error Status is set"]
    pub mod FORCE_DATA_CRC_ERR {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Force Event for Data End Bit Error (SD/eMMC Mode only) Values: 0x0 (FALSE): Not Affected 0x1 (TRUE): Data End Bit Error Status is set"]
    pub mod FORCE_DATA_END_BIT_ERR {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Force Event for Current Limit Error Values: 0x0 (FALSE): Not Affected 0x1 (TRUE): Current Limit Error Status is set"]
    pub mod FORCE_CUR_LMT_ERR {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Force Event for Auto CMD Error (SD/eMMC Mode only) Values: 0x0 (FALSE): Not Affected 0x1 (TRUE): Auto CMD Error Status is set"]
    pub mod FORCE_AUTO_CMD_ERR {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Force Event for ADMA Error Values: 0x0 (FALSE): Not Affected 0x1 (TRUE): ADMA Error Status is set"]
    pub mod FORCE_ADMA_ERR {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Force Event for Tuning Error (UHS-I Mode only) Values: 0x0 (FALSE): Not Affected 0x1 (TRUE): Tuning Error Status is set"]
    pub mod FORCE_TUNING_ERR {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Force Event for Response Error (SD Mode only) Values: 0x0 (FALSE): Not Affected 0x1 (TRUE): Response Error Status is set"]
    pub mod FORCE_RESP_ERR {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Force Event for Boot Ack error Values: 0x0 (FALSE): Not Affected 0x1 (TRUE): Boot ack Error Status is set"]
    pub mod FORCE_BOOT_ACK_ERR {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod ADMA_ERR_STAT {
    #[doc = "ADMA Error States These bits indicate the state of ADMA when an error occurs during ADMA data transfer. Values: 0x0 (ST_STOP): Stop DMA - SYS_ADR register points to a location next to the error descriptor 0x1 (ST_FDS): Fetch Descriptor - SYS_ADR register points to the error descriptor 0x2 (UNUSED): Never set this state 0x3 (ST_TFR): Transfer Data - SYS_ADR register points to a location next to the error descriptor"]
    pub mod ADMA_ERR_STATES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ADMA Length Mismatch Error States This error occurs in the following instances: While the Block Count Enable is being set, the total data length specified by the Descriptor table is different from that specified by the Block Count and Block Length When the total data length cannot be divided by the block length Values: 0x0 (NO_ERR): No Error 0x1 (ERROR): Error"]
    pub mod ADMA_LEN_ERR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod ADMA_SYS_ADDR {
    #[doc = "ADMA System Address These bits indicate the lower 32 bits of the ADMA system address. SDMA: If Host Version 4 Enable is set to 1, this register stores the system address of the data location ADMA2: This register stores the byte address of the executing command of the descriptor table ADMA3: This register is set by ADMA3. ADMA2 increments the address of this register that points to the next line, every time a Descriptor line is fetched."]
    pub mod ADMA_SA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PRESET_INIT {
    #[doc = "SDCLK/RCLK Frequency Select Value 10-bit preset value to be set in SDCLK/RCLK Frequency Select field of the Clock Control register described by a Host System."]
    pub mod FREQ_SEL_VAL {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Clock Generator Select Value This bit is effective when the Host Controller supports a programmable clock generator. Values: 0x0 (FALSE): Host Controller Ver2.0 Compatible Clock Generator 0x1 (PROG): Programmable Clock Generator"]
    pub mod CLK_GEN_SEL_VAL {
        pub const offset: u16 = 10;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PRESET_DS {
    #[doc = "SDCLK/RCLK Frequency Select Value 10-bit preset value to be set in SDCLK/RCLK Frequency Select field of the Clock Control register described by a Host System."]
    pub mod FREQ_SEL_VAL {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Clock Generator Select Value This bit is effective when the Host Controller supports a programmable clock generator. Values: 0x0 (FALSE): Host Controller Ver2.0 Compatible Clock Generator 0x1 (PROG): Programmable Clock Generator"]
    pub mod CLK_GEN_SEL_VAL {
        pub const offset: u16 = 10;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PRESET_HS {
    #[doc = "SDCLK/RCLK Frequency Select Value 10-bit preset value to be set in SDCLK/RCLK Frequency Select field of the Clock Control register described by a Host System."]
    pub mod FREQ_SEL_VAL {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Clock Generator Select Value This bit is effective when the Host Controller supports a programmable clock generator. Values: 0x0 (FALSE): Host Controller Ver2.0 Compatible Clock Generator 0x1 (PROG): Programmable Clock Generator"]
    pub mod CLK_GEN_SEL_VAL {
        pub const offset: u16 = 10;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PRESET_SDR12 {
    #[doc = "SDCLK/RCLK Frequency Select Value 10-bit preset value to be set in SDCLK/RCLK Frequency Select field of the Clock Control register described by a Host System."]
    pub mod FREQ_SEL_VAL {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Clock Generator Select Value This bit is effective when the Host Controller supports a programmable clock generator. Values: 0x0 (FALSE): Host Controller Ver2.0 Compatible Clock Generator 0x1 (PROG): Programmable Clock Generator"]
    pub mod CLK_GEN_SEL_VAL {
        pub const offset: u16 = 10;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PRESET_SDR25 {
    #[doc = "SDCLK/RCLK Frequency Select Value 10-bit preset value to be set in SDCLK/RCLK Frequency Select field of the Clock Control register described by a Host System."]
    pub mod FREQ_SEL_VAL {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Clock Generator Select Value This bit is effective when the Host Controller supports a programmable clock generator. Values: 0x0 (FALSE): Host Controller Ver2.0 Compatible Clock Generator 0x1 (PROG): Programmable Clock Generator"]
    pub mod CLK_GEN_SEL_VAL {
        pub const offset: u16 = 10;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PRESET_SDR50 {
    #[doc = "SDCLK/RCLK Frequency Select Value 10-bit preset value to be set in SDCLK/RCLK Frequency Select field of the Clock Control register described by a Host System."]
    pub mod FREQ_SEL_VAL {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Clock Generator Select Value This bit is effective when the Host Controller supports a programmable clock generator. Values: 0x0 (FALSE): Host Controller Ver2.0 Compatible Clock Generator 0x1 (PROG): Programmable Clock Generator"]
    pub mod CLK_GEN_SEL_VAL {
        pub const offset: u16 = 10;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PRESET_SDR104 {
    #[doc = "SDCLK/RCLK Frequency Select Value 10-bit preset value to be set in SDCLK/RCLK Frequency Select field of the Clock Control register described by a Host System."]
    pub mod FREQ_SEL_VAL {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Clock Generator Select Value This bit is effective when the Host Controller supports a programmable clock generator. Values: 0x0 (FALSE): Host Controller Ver2.0 Compatible Clock Generator 0x1 (PROG): Programmable Clock Generator"]
    pub mod CLK_GEN_SEL_VAL {
        pub const offset: u16 = 10;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PRESET_DDR50 {
    #[doc = "SDCLK/RCLK Frequency Select Value 10-bit preset value to be set in SDCLK/RCLK Frequency Select field of the Clock Control register described by a Host System."]
    pub mod FREQ_SEL_VAL {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Clock Generator Select Value This bit is effective when the Host Controller supports a programmable clock generator. Values: 0x0 (FALSE): Host Controller Ver2.0 Compatible Clock Generator 0x1 (PROG): Programmable Clock Generator"]
    pub mod CLK_GEN_SEL_VAL {
        pub const offset: u16 = 10;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod PRESET_UHS2 {
    #[doc = "SDCLK/RCLK Frequency Select Value 10-bit preset value to be set in SDCLK/RCLK Frequency Select field of the Clock Control register described by a Host System."]
    pub mod FREQ_SEL_VAL {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Clock Generator Select Value This bit is effective when the Host Controller supports a programmable clock generator. Values: 0x0 (FALSE): Host Controller Ver2.0 Compatible Clock Generator 0x1 (PROG): Programmable Clock Generator"]
    pub mod CLK_GEN_SEL_VAL {
        pub const offset: u16 = 10;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod ADMA_ID_ADDR {
    #[doc = "ADMA Integrated Descriptor Address These bits indicate the lower 32-bit of the ADMA Integrated Descriptor address. The start address of Integrated Descriptor is set to these register bits. The ADMA3 fetches one Descriptor Address and increments these bits to indicate the next Descriptor address."]
    pub mod ADMA_ID_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod P_EMBEDDED_CNTRL {
    #[doc = "Offset Address of Embedded Control register."]
    pub mod REG_OFFSET_ADDR {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod P_VENDOR_SPECIFIC_AREA {
    #[doc = "Base offset Address for Vendor-Specific registers."]
    pub mod REG_OFFSET_ADDR {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod P_VENDOR2_SPECIFIC_AREA {
    #[doc = "Base offset Address for Command Queuing registers."]
    pub mod REG_OFFSET_ADDR {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod SLOT_INTR_STATUS {
    #[doc = "Interrupt signal for each Slot These status bits indicate the logical OR of Interrupt signal and Wakeup signal for each slot. A maximum of 8 slots can be defined. If one interrupt signal is associated with multiple slots, the Host Driver can identify the interrupt that is generated by reading these bits. By a power on reset or by setting Software Reset For All bit, the interrupt signals are de-asserted and this status reads 00h. Bit 00: Slot 1 Bit 01: Slot 2 Bit 02: Slot 3 .......... .......... Bit 07: Slot 8 Note: MSHC Host Controller support single card slot. This register shall always return 0."]
    pub mod INTR_SLOT {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CQVER {
    #[doc = "This bit indicates the eMMC version suffix (2nd digit right of decimal point) in BCD format."]
    pub mod EMMC_VER_SUFFIX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates the eMMC minor version (1st digit right of decimal point) in BCD format."]
    pub mod EMMC_VER_MINOR {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates the eMMC major version (1st digit left of decimal point) in BCD format."]
    pub mod EMMC_VER_MAHOR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CQCAP {
    #[doc = "Internal Timer Clock Frequency Value (ITCFVAL) This field scales the frequency of the timer clock provided by ITCFMUL. The Final clock frequency of actual timer clock is calculated as ITCFVAL* ITCFMUL."]
    pub mod ITCFVAL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Internal Timer Clock Frequency Multiplier (ITCFMUL) This field indicates the frequency of the clock used for interrupt coalescing timer and for determining the SQS polling period. See ITCFVAL definition for details. Values 0x5 to 0xF are reserved. Values: 0x0 (CLK_1KHz): 1KHz clock 0x1 (CLK_10KHz): 10KHz clock 0x2 (CLK_100KHz): 100KHz clock 0x3 (CLK_1MHz): 1MHz clock 0x4 (CLK_10MHz): 10MHz clock"]
    pub mod ITCFMUL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Crypto Support This bit indicates whether the Host Controller supports cryptographic operations. Values: 0x0 (FALSE): Crypto not Supported 0x1 (TRUE): Crypto Supported"]
    pub mod CRYPTO_SUPPORT {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CQCFG {
    #[doc = "No description avaiable"]
    pub mod CQ_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Value Description This bit indicates the size of task descriptor used in host memory. This bit can only be configured when Command Queuing Enable bit is 0 (command queuing is disabled). Values: 0x1 (TASK_DESC_128b): Task descriptor size is 128 bits 0x0 (TASK_DESC_64b): Task descriptor size is 64 bit"]
    pub mod TASK_DESC_SIZE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit indicates to the hardware whether the Task Descriptor in slot #31 of the TDL is a data transfer descriptor or a direct-command descriptor. CQE uses this bit when a task is issued in slot #31, to determine how to decode the Task Descriptor. Values: 0x1 (SLOT31_DCMD_ENABLE): Task descriptor in slot #31 is a DCMD Task Descriptor 0x0 (SLOT31_DCMD_DISABLE): Task descriptor in slot #31 is a data Transfer Task Descriptor"]
    pub mod DCMD_EN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CQCTL {
    #[doc = "Halt request and resume Values: 0x1 (HALT_CQE): Software writes 1 to this bit when it wants to acquire software control over the eMMC bus and to disable CQE from issuing command on the bus. For example, issuing a Discard Task command (CMDQ_TASK_MGMT). When the software writes 1, CQE completes the ongoing task (if any in progress). After the task is completed and the CQE is in idle state, CQE does not issue new commands and indicates to the software by setting this bit to 1. The software can poll on this bit until it is set to 1 and only then send commands on the eMMC bus. 0x0 (RESUME_CQE): Software writes 0 to this bit to exit from the halt state and resume CQE activity"]
    pub mod HALT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Clear all tasks This bit can only be written when the controller is halted. This bit does not clear tasks in the device. The software has to use the CMDQ_TASK_MGMT command to clear device's queue. Values: 0x1 (CLEAR_ALL_TASKS): Clears all the tasks in the controller 0x0 (NO_EFFECT): Programming 0 has no effect"]
    pub mod CLR_ALL_TASKS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CQIS {
    #[doc = "Halt complete interrupt This status bit is asserted (only if CQISE.HAC_STE=1) when halt bit in the CQCTL register transitions from 0 to 1 indicating that the host controller has completed its current ongoing task and has entered halt state. A value of 1 clears this status bit. Values: 0x1 (SET): HAC Interrupt is set 0x0 (NOTSET): HAC Interrupt is not set"]
    pub mod HAC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Task complete interrupt This status bit is asserted (if CQISE.TCC_STE=1) when at least one of the following conditions are met: A task is completed and the INT bit is set in its Task Descriptor Interrupt caused by Interrupt Coalescing logic due to timeout Interrupt Coalescing logic reached the configured threshold A value of 1 clears this status bit"]
    pub mod TCC {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Response error detected interrupt This status bit is asserted (if CQISE.RED_STE=1) when a response is received with an error bit set in the device status field. Configure the CQRMEM register to identify device status bit fields that may trigger an interrupt and that are masked. A value of 1 clears this status bit. Values: 0x1 (SET): RED Interrupt is set 0x0 (NOTSET): RED Interrupt is not set"]
    pub mod RED {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Task cleared interrupt This status bit is asserted (if CQISE.TCL_STE=1) when a task clear operation is completed by CQE. The completed task clear operation is either an individual task clear (by writing CQTCLR) or clearing of all tasks (by writing CQCTL). A value of 1 clears this status bit. Values: 0x1 (SET): TCL Interrupt is set 0x0 (NOTSET): TCL Interrupt is not set"]
    pub mod TCL {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CQISE {
    #[doc = "Halt complete interrupt status enable Values: 0x1 (INT_STS_ENABLE): CQIS.HAC is set when its interrupt condition is active 0x0 (INT_STS_DISABLE): CQIS.HAC is disabled"]
    pub mod HAC_STE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Task complete interrupt status enable Values: 0x1 (INT_STS_ENABLE): CQIS.TCC is set when its interrupt condition is active 0x0 (INT_STS_DISABLE): CQIS.TCC is disabled"]
    pub mod TCC_STE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Response error detected interrupt status enable Values: 0x1 (INT_STS_ENABLE): CQIS.RED is set when its interrupt condition is active 0x0 (INT_STS_DISABLE): CQIS.RED is disabled"]
    pub mod RED_STE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Task cleared interrupt status enable Values: 0x1 (INT_STS_ENABLE): CQIS.TCL is set when its interrupt condition is active 0x0 (INT_STS_DISABLE): CQIS.TCL is disabled"]
    pub mod TCL_STE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CQISGE {
    #[doc = "Halt complete interrupt signal enable Values: 0x1 (INT_SIG_ENABLE): CQIS.HAC interrupt signal generation is active 0x0 (INT_SIG_DISABLE): CQIS.HAC interrupt signal generation is disabled"]
    pub mod HAC_SGE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Task complete interrupt signal enable Values: 0x1 (INT_SIG_ENABLE): CQIS.TCC interrupt signal generation is active 0x0 (INT_SIG_DISABLE): CQIS.TCC interrupt signal generation is disabled"]
    pub mod TCC_SGE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Response error detected interrupt signal enable Values: 0x1 (INT_SIG_ENABLE): CQIS.RED interrupt signal generation is active 0x0 (INT_SIG_DISABLE): CQIS.RED interrupt signal generation is disabled"]
    pub mod RED_SGE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Task cleared interrupt signal enable Values: 0x1 (INT_SIG_ENABLE): CQIS.TCL interrupt signal generation is active 0x0 (INT_SIG_DISABLE): CQIS.TCL interrupt signal generation is disabled"]
    pub mod TCL_SGE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CQIC {
    #[doc = "Interrupt Coalescing Timeout Value Software uses this field to configure the maximum time allowed between the completion of a task on the bus and the generation of an interrupt. Timer Operation: The timer is reset by software during the interrupt service routine. It starts running when the first data transfer task with INT=0 is completed, after the timer was reset. When the timer reaches the value configured in ICTOVAL field, it generates an interrupt and stops. The timer's unit is equal to 1024 clock periods of the clock whose frequency is specified in the Internal Timer Clock Frequency field CQCAP register. 0x0: Timer is disabled. Timeout-based interrupt is not generated 0x1: Timeout on 01x1024 cycles of timer clock frequency 0x2: Timeout on 02x1024 cycles of timer clock frequency ........ 0x7f: Timeout on 127x1024 cycles of timer clock frequency In order to write to this field, the TOUT_VAL_WEN bit must be set at the same write operation."]
    pub mod TOUT_VAL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When software writes 1 to this bit, the value TOUT_VAL is updated with the contents written on the same cycle. Values: 0x1 (WEN_SET): Sets TOUT_VAL_WEN 0x0 (WEN_CLR): clears TOUT_VAL_WEN"]
    pub mod TOUT_VAL_WEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Interrupt Coalescing Counter Threshold filed Software uses this field to configure the number of task completions (only tasks with INT=0 in the Task Descriptor), which are required in order to generate an interrupt. Counter Operation: As data transfer tasks with INT=0 complete, they are counted by CQE. The counter is reset by software during the interrupt service routine. The counter stops counting when it reaches the value configured in INTC_TH, and generates interrupt. 0x0: Interrupt coalescing feature disabled 0x1: Interrupt coalescing interrupt generated after 1 task when INT=0 completes 0x2: Interrupt coalescing interrupt generated after 2 tasks when INT=0 completes ........ 0x1f: Interrupt coalescing interrupt generated after 31 tasks when INT=0 completes To write to this field, the INTC_TH_WEN bit must be set during the same write operation."]
    pub mod INTC_TH {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Interrupt Coalescing Counter Threshold Write Enable When software writes 1 to this bit, the value INTC_TH is updated with the contents written on the same cycle. Values: 0x1 (WEN_SET): Sets INTC_TH_WEN 0x0 (WEN_CLR): Clears INTC_TH_WEN"]
    pub mod INTC_TH_WEN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Counter and Timer Reset When host driver writes 1, the interrupt coalescing timer and counter are reset. Values: 0x1 (ASSERT_INTC_RESET): Interrupt coalescing timer and counter are reset 0x0 (NO_EFFECT): No Effect"]
    pub mod INTC_RST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Interrupt Coalescing Status Bit This bit indicates to the software whether any tasks (with INT=0) have completed and counted towards interrupt coalescing (that is, this is set if and only if INTC counter > 0). Values: 0x1 (INTC_ATLEAST1_COMP): At least one INT0 task completion has been counted (INTC counter > 0) 0x0 (INTC_NO_TASK_COMP): INT0 Task completions have not occurred since last counter reset (INTC counter == 0)"]
    pub mod INTC_STAT {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Interrupt Coalescing Enable Bit Values: 0x1 (ENABLE_INT_COALESCING): Interrupt coalescing mechanism is active. Interrupts are counted and timed, and coalesced interrupts are generated 0x0 (DISABLE_INT_COALESCING): Interrupt coalescing mechanism is disabled (Default)"]
    pub mod INTC_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CQTDLBA {
    #[doc = "This register stores the LSB bits (31:0) of the byte address of the head of the Task Descriptor List in system memory. The size of the task descriptor list is 32 * (Task Descriptor size + Transfer Descriptor size) as configured by the host driver. This address is set on 1 KB boundary. The lower 10 bits of this register are set to 0 by the software and are ignored by CQE"]
    pub mod TDLBA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CQTDBR {
    #[doc = "The software configures TDLBA and TDLBAU, and enable CQE in CQCFG before using this register. Writing 1 to bit n of this register triggers CQE to start processing the task encoded in slot n of the TDL. Writing 0 by the software does not have any impact on the hardware, and does not change the value of the register bit. CQE always processes tasks according to the order submitted to the list by CQTDBR write transactions. CQE processes Data Transfer tasks by reading the Task Descriptor and sending QUEUED_TASK_PARAMS (CMD44) and QUEUED_TASK_ADDRESS (CMD45) commands to the device. CQE processes DCMD tasks (in slot #31, when enabled) by reading the Task Descriptor, and generating the command encoded by its index and argument. The corresponding bit is cleared to 0 by CQE in one of the following events: A task execution is completed (with success or error). The task is cleared using CQTCLR register. All tasks are cleared using CQCTL register. CQE is disabled using CQCFG register. Software may initiate multiple tasks at the same time (batch submission) by writing 1 to multiple bits of this register in the same transaction. In the case of batch submission, CQE processes the tasks in order of the task index, starting with the lowest index. If one or more tasks in the batch are marked with QBR, the ordering of execution is based on said processing order."]
    pub mod DBR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CQTCN {
    #[doc = "Task Completion Notification Each of the 32 bits are bit mapped to the 32 tasks. Bit-N(1): Task-N has completed execution (with success or errors) Bit-N(0): Task-N has not completed, could be pending or not submitted. On task completion, software may read this register to know tasks that have completed. After reading this register, software may clear the relevant bit fields by writing 1 to the corresponding bits."]
    pub mod TCN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CQDQS {
    #[doc = "Device Queue Status Each of the 32 bits are bit mapped to the 32 tasks. Bit-N(1): Device has marked task N as ready for execution Bit-N(0): Task-N is not ready for execution. This task could be pending in device or not submitted. Host controller updates this register with response of the Device Queue Status command."]
    pub mod DQS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CQDPT {
    #[doc = "Device-Pending Tasks Each of the 32 bits are bit mapped to the 32 tasks. Bit-N(1): Task-N has been successfully queued into the device and is awaiting execution Bit-N(0): Task-N is not yet queued. Bit n of this register is set if and only if QUEUED_TASK_PARAMS (CMD44) and QUEUED_TASK_ADDRESS (CMD45) were sent for this specific task and if this task has not been executed. The controller sets this bit after receiving a successful response for CMD45. CQE clears this bit after the task has completed execution. Software reads this register in the task-discard procedure to determine if the task is queued in the device"]
    pub mod DPT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CQTCLR {
    #[doc = "Writing 1 to bit n of this register orders CQE to clear a task that the software has previously issued. This bit can only be written when CQE is in Halt state as indicated in CQCFG register Halt bit. When software writes 1 to a bit in this register, CQE updates the value to 1, and starts clearing the data structures related to the task. CQE clears the bit fields (sets a value of 0) in CQTCLR and in CQTDBR once the clear operation is complete. Software must poll on the CQTCLR until it is leared to verify that a clear operation was done."]
    pub mod TCLR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CQSSC1 {
    #[doc = "This field configures the polling period to be used when using periodic SEND_QUEUE_STATUS (CMD13) polling. Periodic polling is used when tasks are pending in the device, but no data transfer is in progress. When a SEND_QUEUE_STATUS response indicates that no task is ready for execution, CQE counts the configured time until it issues the next SEND_QUEUE_STATUS. Timer units are clock periods of the clock whose frequency is specified in the Internal Timer Clock Frequency field CQCAP register. The minimum value is 0001h (1 clock period) and the maximum value is FFFFh (65535 clock periods). For example, a CQCAP field value of 0 indicates a 19.2 MHz clock frequency (period = 52.08 ns). If the setting in CQSSC1.CIT is 1000h, the calculated polling period is 4096*52.08 ns= 213.33 us. Should be programmed only when CQCFG.CQ_EN is '0'"]
    pub mod SQSCMD_IDLE_TMR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This field indicates when SQS CMD is sent while data transfer is in progress. A value of 'n' indicates that CQE sends status command on the CMD line, during the transfer of data block BLOCK_CNTn, on the data lines, where BLOCK_CNT is the number of blocks in the current transaction. 0x0: SEND_QUEUE_STATUS (CMD13) command is not sent during the transaction. Instead, it is sent only when the data lines are idle. 0x1: SEND_QUEUE_STATUS command is to be sent during the last block of the transaction. 0x2: SEND_QUEUE_STATUS command when last 2 blocks are pending. 0x3: SEND_QUEUE_STATUS command when last 3 blocks are pending. ........ 0xf: SEND_QUEUE_STATUS command when last 15 blocks are pending. Should be programmed only when CQCFG.CQ_EN is 0"]
    pub mod SQSCMD_BLK_CNT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CQSSC2 {
    #[doc = "This field provides CQE with the contents of the 16-bit RCA field in SEND_QUEUE_STATUS (CMD13) command argument. CQE copies this field to bits 31:16 of the argument when transmitting SEND_ QUEUE_STATUS (CMD13) command."]
    pub mod SQSCMD_RCA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CQCRDCT {
    #[doc = "This register contains the response of the command generated by the last direct command (DCMD) task that was sent. Contents of this register are valid only after bit 31 of CQTDBR register is cleared by the controller."]
    pub mod DCMD_RESP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CQRMEM {
    #[doc = "The bits of this field are bit mapped to the device response. This bit is used as an interrupt mask on the device status filed that is received in R1/R1b responses. 1: When a R1/R1b response is received, with a bit i in the device status set, a RED interrupt is generated. 0: When a R1/R1b response is received, bit i in the device status is ignored. The reset value of this register is set to trigger an interrupt on all \"Error\" type bits in the device status. Note: Responses to CMD13 (SQS) encode the QSR so that they are ignored by this logic."]
    pub mod RESP_ERR_MASK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CQTERRI {
    #[doc = "This field captures the index of the command that was executed on the command line when the error occurred"]
    pub mod RESP_ERR_CMD_INDX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This field captures the ID of the task which was executed on the command line when the error occurred."]
    pub mod RESP_ERR_TASKID {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This bit is updated when an error is detected while a command transaction was in progress. Values: 0x1 (SET): Response-related error is detected. Check contents of RESP_ERR_TASKID and RESP_ERR_CMD_INDX fields 0x0 (NOT_SET): Ignore contents of RESP_ERR_TASKID and RESP_ERR_CMD_INDX"]
    pub mod RESP_ERR_FIELDS_VALID {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This field captures the index of the command that was executed and whose data transfer has errors."]
    pub mod TRANS_ERR_CMD_INDX {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This field captures the ID of the task that was executed and whose data transfer has errors."]
    pub mod TRANS_ERR_TASKID {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CQCRI {
    #[doc = "Last Command Response index This field stores the index of the last received command response. Controller updates the value every time a command response is received"]
    pub mod CMD_RESP_INDX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod CQCRA {
    #[doc = "Last Command Response argument This field stores the argument of the last received command response. Controller updates the value every time a command response is received."]
    pub mod CMD_RESP_ARG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod MSHC_VER_ID {
    #[doc = "No description avaiable"]
    pub mod VER_ID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod MSHC_VER_TYPE {
    #[doc = "No description avaiable"]
    pub mod VER_TYPE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Y"]
pub mod MBIU_CTRL {
    #[doc = "No description avaiable"]
    pub mod UNDEFL_INCR_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "No description avaiable"]
    pub mod BUSRT_INCR4_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "No description avaiable"]
    pub mod BURST_INCR8_EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "No description avaiable"]
    pub mod BURST_INCR16_EN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod EMMC_BOOT_CTRL {
    #[doc = "eMMC Card present This bit indicates the type of card connected. An application program this bit based on the card connected to SDXC. Values: 0x1 (EMMC_CARD): Card connected to SDXC is an eMMC card 0x0 (NON_EMMC_CARD): Card connected to SDXCis a non-eMMC card"]
    pub mod CARD_IS_EMMC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Disable Data CRC Check This bit controls masking of CRC16 error for Card Write in eMMC mode. This is useful in bus testing (CMD19) for an eMMC device. In bus testing, an eMMC card does not send CRC status for a block, which may generate CRC error. This CRC error can be masked using this bit during bus testing. Values: 0x1 (DISABLE): DATA CRC check is disabled 0x0 (ENABLE): DATA CRC check is enabled"]
    pub mod DISABLE_DATA_CRC_CHK {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "EMMC Device Reset signal control. This register field controls the sd_rst_n output of SDXC Values: 0x1 (RST_DEASSERT): Reset to eMMC device is deasserted 0x0 (RST_ASSERT): Reset to eMMC device asserted (active low)"]
    pub mod EMMC_RST_N {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Output Enable control for EMMC Device Reset signal PAD control. This field drived sd_rst_n_oe output of SDXC Values: 0x1 (ENABLE): sd_rst_n_oe is 1 0x0 (DISABLE): sd_rst_n_oe is 0"]
    pub mod EMMC_RST_N_OE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enhanced Strobe Enable This bit instructs SDXC to sample the CMD line using data strobe for HS400 mode. Values: 0x1 (ENH_STB_FOR_CMD): CMD line is sampled using data strobe for HS400 mode 0x0 (NO_STB_FOR_CMD): CMD line is sampled using cclk_rx for HS400 mode"]
    pub mod ENH_STROBE_ENABLE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Scheduler algorithm selected for execution This bit selects the Algorithm used for selecting one of the many ready tasks for execution. Values: 0x0 (PRI_REORDER_PLUS_FCFS): Priority based reordering with FCFS to resolve equal priority tasks 0x1 (FCFS_ONLY): First come First serve, in the order of DBR rings"]
    pub mod CQE_ALGO_SEL {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable or Disable CQE's PREFETCH feature This field allows Software to disable CQE's data prefetch feature when set to 1. Values: 0x0 (PREFETCH_ENABLE): CQE can Prefetch data for sucessive WRITE transfers and pipeline sucessive READ transfers 0x1 (PREFETCH_DISABLE): Prefetch for WRITE and Pipeline for READ are disabled"]
    pub mod CQE_PREFETCH_DISABLE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Mandatory Boot Enable This bit is used to initiate the mandatory boot operation. The application sets this bit along with VALIDATE_BOOT bit. Writing 0 is ignored. The SDXC clears this bit after the boot transfer is completed or terminated. Values: 0x1 (MAN_BOOT_EN): Mandatory boot enable 0x0 (MAN_BOOT_DIS): Mandatory boot disable"]
    pub mod MAN_BOOT_EN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Validate Mandatory Boot Enable bit This bit is used to validate the MAN_BOOT_EN bit. Values: 0x1 (TRUE): Validate Mandatory boot enable bit 0x0 (FALSE): Ignore Mandatory boot Enable bit"]
    pub mod VALIDATE_BOOT {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Boot Acknowledge Enable When this bit set, SDXC checks for boot acknowledge start pattern of 0-1-0 during boot operation. This bit is applicable for both mandatory and alternate boot mode. Values: 0x1 (TRUE): Boot Ack enable 0x0 (FALSE): Boot Ack disable"]
    pub mod BOOT_ACK_ENABLE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Boot Ack Timeout Counter Value. This value determines the interval by which boot ack timeout (50 ms) is detected when boot ack is expected during boot operation. 0xF : Reserved 0xE : TMCLK x 2^27 ............ 0x1 : TMCLK x 2^14 0x0 : TMCLK x 2^13"]
    pub mod BOOT_TOUT_CNT {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod AUTO_TUNING_CTRL {
    #[doc = "Setting this bit enables Auto tuning engine. This bit is enabled by default when core is configured with mode3 retuning support. Clear this bit to 0 when core is configured to have Mode3 re-tuning but SW wishes to disable mode3 retuning. This field should be programmed only when CLK_CTRL_R.SD_CLK_EN is 0. Values: 0x1 (AT_ENABLE): AutoTuning is enabled 0x0 (AT_DISABLE): AutoTuning is disabled"]
    pub mod AT_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Selects the interval when the corrected center phase select code can be driven on tuning_cclk_sel output. Values: 0x0 (WHEN_IN_BLK_GAP): Driven in block gap interval 0x1 (WHEN_IN_IDLE): Driven at the end of the transfer"]
    pub mod CI_SEL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Sampling window Threshold enable Selects the tuning mode Field should be programmed only when SAMPLE_CLK_SEL is '0' Values: 0x1 (THRESHOLD_MODE): Tuning engine selects the first complete sampling window that meets the threshold set by SWIN_TH_VAL field 0x0 (LARGEST_WIN_MODE): Tuning engine sweeps all taps and settles at the largest window"]
    pub mod SWIN_TH_EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Framing errors are not generated when executing tuning. This debug bit allows users to report these errors. Values: 0x1 (DEBUG_ERRORS): Debug mode for reporting framing errors 0x0 (ERRORS_DISABLED): Default mode where as per SDXC no errors are reported."]
    pub mod RPT_TUNE_ERR {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This fields enables software-managed tuning flow. Values: 0x1 (SW_TUNING_ENABLE): Software-managed tuning enabled. AT_STAT_R.CENTER_PH_CODE Field is now writable. 0x0 (SW_TUNING_DISABLE): Software-managed tuning disabled"]
    pub mod SW_TUNE_EN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This field sets the phase for Left and Right edges for drift monitoring. \\[Left edge offset + Right edge offset\\] must not be less than total taps of delayLine. 0x0: User selection disabled. Tuning calculated edges are used. 0x1: Right edge Phase is center + 2 stages, Left edge Phase is center - 2 stages. 0x2: Right edge Phase is center + 3 stages, Left edge Phase is center - 3 stagess ... 0xF: Right edge Phase is center + 16 stages, Left edge Phase is center - 16 stages."]
    pub mod WIN_EDGE_SEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Clock stopping control for Tuning and auto-tuning circuit. When enabled, clock gate control output of SDXC (clk2card_on) is pulled low before changing phase select codes on tuning_cclk_sel and autotuning_cclk_sel. This effectively stops the Device/Card clock, cclk_rx and also drift_cclk_rx. Changing phase code when clocks are stopped ensures glitch free phase switching. Set this bit to 0 if the PHY or delayline can guarantee glitch free switching. Values: 0x1 (ENABLE_CLK_STOPPING): Clocks stopped during phase code change 0x0 (DISABLE_CLK_STOPPING): Clocks not stopped. PHY ensures glitch free phase switching"]
    pub mod TUNE_CLK_STOP_EN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Maximum Latency specification between cclk_tx and cclk_rx. Values: 0x0 (LATENCY_LT_1): Less than 1-cycle latency 0x1 (LATENCY_LT_2): Less than 2-cycle latency 0x2 (LATENCY_LT_3): Less than 3-cycle latency 0x3 (LATENCY_LT_4): Less than 4-cycle latency"]
    pub mod PRE_CHANGE_DLY {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Time taken for phase switching and stable clock output. Specifies the maximum time (in terms of cclk cycles) that the delay line can take to switch its output phase after a change in tuning_cclk_sel or autotuning_cclk_sel. Values: 0x0 (LATENCY_LT_1): Less than 1-cycle latency 0x1 (LATENCY_LT_2): Less than 2-cycle latency 0x2 (LATENCY_LT_3): Less than 3-cycle latency 0x3 (LATENCY_LT_4): Less than 4-cycle latency"]
    pub mod POST_CHANGE_DLY {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Sampling window threshold value setting The maximum value that can be set here depends on the length of delayline used for tuning. A delayLine with 32 taps can use values from 0x0 to 0x1F. This field is valid only when SWIN_TH_EN is '1'. Should be programmed only when SAMPLE_CLK_SEL is '0' 0x0 : Threshold values is 0x1, windows of length 1 tap and above can be selected as sampling window. 0x1 : Threshold values is 0x2, windows of length 2 taps and above can be selected as sampling window. 0x2 : Threshold values is 0x1, windows of length 3 taps and above can be selected as sampling window. ........ 0x1F : Threshold values is 0x1, windows of length 32 taps and above can be selected as sampling window."]
    pub mod SWIN_TH_VAL {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "No description avaiable"]
pub mod AUTO_TUNING_STAT {
    #[doc = "Centered Phase code. Reading this field returns the current value on tuning_cclk_sel output. Setting AT_CTRL_R.SW_TUNE_EN enables software to write to this field and its contents are reflected on tuning_cclk_sel"]
    pub mod CENTER_PH_CODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Right Edge Phase code. Reading this field returns the phase code value used by Auto-tuning engine to sample data on Right edge of sampling window."]
    pub mod R_EDGE_PH_CODE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Left Edge Phase code. Reading this field returns the phase code value used by Auto-tuning engine to sample data on Left edge of sampling window."]
    pub mod L_EDGE_PH_CODE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
