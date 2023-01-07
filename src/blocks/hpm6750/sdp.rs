#[doc = "SDP"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "SDP control register"]
    pub SDPCR: crate::RWRegister<u32>,
    #[doc = "Mod control register."]
    pub MODCTRL: crate::RWRegister<u32>,
    #[doc = "packet counter registers."]
    pub PKTCNT: crate::RWRegister<u32>,
    #[doc = "Status Registers"]
    pub STA: crate::RWRegister<u32>,
    #[doc = "Key Address"]
    pub KEYADDR: crate::RWRegister<u32>,
    #[doc = "Key Data"]
    pub KEYDAT: crate::RWRegister<u32>,
    #[doc = "Cipher Initializtion Vector 0"]
    pub CIPHIV_CIPHIV0: crate::RWRegister<u32>,
    #[doc = "Cipher Initializtion Vector 1"]
    pub CIPHIV_CIPHIV1: crate::RWRegister<u32>,
    #[doc = "Cipher Initializtion Vector 2"]
    pub CIPHIV_CIPHIV2: crate::RWRegister<u32>,
    #[doc = "Cipher Initializtion Vector 3"]
    pub CIPHIV_CIPHIV3: crate::RWRegister<u32>,
    #[doc = "Hash Data Word 0"]
    pub HASWRD_HASWRD0: crate::RWRegister<u32>,
    #[doc = "Hash Data Word 1"]
    pub HASWRD_HASWRD1: crate::RWRegister<u32>,
    #[doc = "Hash Data Word 2"]
    pub HASWRD_HASWRD2: crate::RWRegister<u32>,
    #[doc = "Hash Data Word 3"]
    pub HASWRD_HASWRD3: crate::RWRegister<u32>,
    #[doc = "Hash Data Word 4"]
    pub HASWRD_HASWRD4: crate::RWRegister<u32>,
    #[doc = "Hash Data Word 5"]
    pub HASWRD_HASWRD5: crate::RWRegister<u32>,
    #[doc = "Hash Data Word 6"]
    pub HASWRD_HASWRD6: crate::RWRegister<u32>,
    #[doc = "Hash Data Word 7"]
    pub HASWRD_HASWRD7: crate::RWRegister<u32>,
    #[doc = "Command Pointer"]
    pub CMDPTR: crate::RWRegister<u32>,
    #[doc = "Next Packet Address Pointer"]
    pub NPKTPTR: crate::RWRegister<u32>,
    #[doc = "Packet Control Registers"]
    pub PKTCTL: crate::RWRegister<u32>,
    #[doc = "Packet Memory Source Address"]
    pub PKTSRC: crate::RWRegister<u32>,
    #[doc = "Packet Memory Destination Address"]
    pub PKTDST: crate::RWRegister<u32>,
    #[doc = "Packet buffer size."]
    pub PKTBUF: crate::RWRegister<u32>,
}
#[doc = "SDP control register"]
pub mod SDPCR {
    #[doc = "Interrupt Enablement, controlled by SW. 1, SDP interrupt is enabled. 0, SDP interrupt is disabled."]
    pub mod INTEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "when set to \"1\", the 1st data packet descriptor loacted in the register(CMDPTR, NPKTPTR, ...) when set to \"0\", the 1st data packet descriptor loacted in the memeory(pointed by CMDPTR)"]
    pub mod RDSCEN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Test purpose for interrupt when Packet counter reachs \"0\", but CHAIN=1 in the current packet."]
    pub mod TSTPKT0IRQ {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Decryption Disable bit, Write to 1 to disable the decryption."]
    pub mod DCRPDI {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Constant Fill to memory, controlled by SW. 1, Constant fill is Enabled. 0, Constant fill is Disabled."]
    pub mod CONFEN {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Memory Copy Enablement, controlled by SW. 1, Memory copy is Enabled. 0, Memory copy is Disabled."]
    pub mod MCPEN {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "HASH Enablement, controlled by SW. 1, HASH is Enabled. 0, HASH is Disabled."]
    pub mod HASHEN {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Cipher Enablement, controlled by SW. 1, Cipher is Enabled. 0, Cipher is Disabled."]
    pub mod CIPHEN {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "HASH Disable, read the info, whether the HASH features is besing disable in this chip or not. 1, HASH is disabled in this chip. 0, HASH is enabled in this chip."]
    pub mod HASDIS {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Cipher Disable, read the info, whether the CIPHER features is besing disable in this chip or not. 1, Cipher is disabled in this chip. 0, Cipher is enabled in this chip."]
    pub mod CIPDIS {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Clock Gate for the SDP main logic. Write to 1 will clock gate for most logic of the SDP block, dynamic power saving when not use SDP block."]
    pub mod CLKGAT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "soft reset. Write 1 then 0, to reset the SDP block."]
    pub mod SFTRST {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Mod control register."]
pub mod MODCTRL {
    #[doc = "Decide whether the SDP byteswaps the Key (big-endian data). When all bits are set, the data is assumed to be in the big-endian format"]
    pub mod KEYSWP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Decide whether the SDP byteswaps the output data (big-endian data); When all bits are set, the data is assumed to be in the big-endian format"]
    pub mod DOUTSWP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Decide whether the SDP byteswaps the input data (big-endian data); When all bits are set, the data is assumed to be in the big-endian format"]
    pub mod DINSWP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When hashing is enabled, this bit controls the input or output data of the AES engine is hashed. 0 INPUT HASH 1 OUTPUT HASH"]
    pub mod HASOUT {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "HASH Check Enable Bit. 1x1, HASH check need, hash result will compare with the HASHRSLT 0-7 registers; 1x0, HASH check is not enabled, HASHRSLT0-7 store the HASH result. For SHA1, will use HASHRSLT0-3 words, and HASH 256 will use HASH0-7 words."]
    pub mod HASCHK {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CRC enable. 1x1, CRC is enabled. 1x0, CRC is disabled."]
    pub mod CRCEN {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "HASH Algorithem selection. 0x0 SHA1 — 0x1 CRC32 — 0x2 SHA256 —"]
    pub mod HASALG {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AES direction 1x1, AES Decryption 1x0, AES Encryption."]
    pub mod AESDIR {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AES Key Selection. These regisgers are being used to select the AES key that stored in the 16x128 key ram of the SDP, or select the key from the OTP. Detail as following: 0x00: key from the 16x128, this is the key read address, valid for AES128; AES256 will use 128 bit from this address and 128 bit key from next address as 256 bit AES key. 0x01: key from the 16x128, this is the key read address, valid for AES128, not valid for AES286. .... 0x0E: key from the 16x128, this is the key read address, valid for AES128; AES256 will use 128 from this add and 128 from next add for the AES key. 0x0F: key from the 16x128, this is the key read address, valid for AES128, not valid for AES286. 0x20: kman_sk0\\[127:0\\] from the key manager for AES128; AES256 will use kman_sk0\\[255:0\\] as AES key. 0x21: kman_sk0\\[255:128\\] from the key manager for AES128; not valid for AES256. 0x22: kman_sk1\\[127:0\\] from the key manager for AES128; AES256 will use kman_sk1\\[255:0\\] as AES key. 0x23: kman_sk1\\[255:128\\] from the key manager for AES128; not valid for AES256. 0x24: kman_sk2\\[127:0\\] from the key manager for AES128; AES256 will use kman_sk2\\[255:0\\] as AES key. 0x25: kman_sk2\\[255:128\\] from the key manager for AES128; not valid for AES256. 0x26: kman_sk3\\[127:0\\] from the key manager for AES128; AES256 will use kman_sk3\\[255:0\\] as AES key. 0x27: kman_sk3\\[255:128\\] from the key manager for AES128; not valid for AES256. 0x30: exip0_key\\[127:0\\] from OTP for AES128; AES256 will use exip0_key\\[255:0\\] as AES key. 0x31: exip0_key\\[255:128\\] from OTP for AES128; not valid for AES256. 0x32: exip1_key\\[127:0\\] from OTP for AES128; AES256 will use exip1_key\\[255:0\\] as AES key. 0x33: exip1_key\\[255:128\\] from OTP for AES128; not valid for AES256. Other values, reserved."]
    pub mod AESKS {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AES mode selection. 0x0 = ECB; 0x1 = CBC; Others, reserved."]
    pub mod AESMOD {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AES algorithem selection. 0x0 = AES 128; 0x1 = AES 256; Others, reserved."]
    pub mod AESALG {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "packet counter registers."]
pub mod PKTCNT {
    #[doc = "The value written to this field is added to the spacket count."]
    pub mod CNTINCR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This read-only field shows the current (instantaneous) value of the packet counter"]
    pub mod CNTVAL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status Registers"]
pub mod STA {
    #[doc = "buffer chain error happen when packet's CHAIN bit=0, but the Packet counter is still not zero."]
    pub mod ERRCHAIN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hashing Check Error"]
    pub mod ERRHAS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Destination Buffer Error"]
    pub mod ERRDST {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Source Buffer Access Error"]
    pub mod ERRSRC {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Packet head access error, or status update error."]
    pub mod ERRPKT {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Working mode setup error."]
    pub mod ERRSET {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Packet processing done, will trigger this itnerrrupt when the \"PKTINT\" bit set in the packet control word."]
    pub mod PKTDON {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Packet Counter registers reachs to ZERO now."]
    pub mod PKTCNT0 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hashing Busy"]
    pub mod HASBSY {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AES Busy"]
    pub mod AESBSY {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "the chain buffer \"chain\" bit is \"1\", while packet counter is \"0\", now, waiting for new buffer data."]
    pub mod CHN1PKT0 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "interrupt Request, requested when error happen, or when packet processing done, packet counter reach to zero."]
    pub mod IRQ {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "packet tag."]
    pub mod TAG {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Key Address"]
pub mod KEYADDR {
    #[doc = "Key subword pointer. The valid indices are 0-3. After each write to the key data register, this field increments; To write a key, the software must first write the desired key index/subword to this register."]
    pub mod SUBWRD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "To write a key to the SDP KEY RAM, the software must first write the desired key index/subword to this register. Key index pointer. The valid indices are 0-\\[number_keys\\]. In the SDP, there is a 16x128 key ram can store 16 AES128 keys or 8 AES 256 Keys; this index is for addressing the 16 128-bit key addresses."]
    pub mod INDEX {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Key Data"]
pub mod KEYDAT {
    #[doc = "This register provides the write access to the key/key subword specified by the key index register. Writing this location updates the selected subword for the key located at the index specified by the key index register. The write also triggers the SUBWORD field of the KEY register to increment to the next higher word in the key"]
    pub mod KEYDAT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Cipher Initializtion Vector 0"]
pub mod CIPHIV_CIPHIV0 {
    #[doc = "cipher initialization vector."]
    pub mod CIPHIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Cipher Initializtion Vector 1"]
pub mod CIPHIV_CIPHIV1 {
    #[doc = "cipher initialization vector."]
    pub mod CIPHIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Cipher Initializtion Vector 2"]
pub mod CIPHIV_CIPHIV2 {
    #[doc = "cipher initialization vector."]
    pub mod CIPHIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Cipher Initializtion Vector 3"]
pub mod CIPHIV_CIPHIV3 {
    #[doc = "cipher initialization vector."]
    pub mod CIPHIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Hash Data Word 0"]
pub mod HASWRD_HASWRD0 {
    #[doc = "Hash Data Word x - HASH result bit; will store the expected hash result bit if hash check enabled; when hash check is not enabled, the hash engine will store the final hash result\\[31:0\\] here. If CRC mode enabled, this work store the CRC expected result if the check enabled, or store the final calcuated CRC result."]
    pub mod HASWRD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Hash Data Word 1"]
pub mod HASWRD_HASWRD1 {
    #[doc = "Hash Data Word x - HASH result bit; will store the expected hash result bit if hash check enabled; when hash check is not enabled, the hash engine will store the final hash result\\[31:0\\] here. If CRC mode enabled, this work store the CRC expected result if the check enabled, or store the final calcuated CRC result."]
    pub mod HASWRD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Hash Data Word 2"]
pub mod HASWRD_HASWRD2 {
    #[doc = "Hash Data Word x - HASH result bit; will store the expected hash result bit if hash check enabled; when hash check is not enabled, the hash engine will store the final hash result\\[31:0\\] here. If CRC mode enabled, this work store the CRC expected result if the check enabled, or store the final calcuated CRC result."]
    pub mod HASWRD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Hash Data Word 3"]
pub mod HASWRD_HASWRD3 {
    #[doc = "Hash Data Word x - HASH result bit; will store the expected hash result bit if hash check enabled; when hash check is not enabled, the hash engine will store the final hash result\\[31:0\\] here. If CRC mode enabled, this work store the CRC expected result if the check enabled, or store the final calcuated CRC result."]
    pub mod HASWRD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Hash Data Word 4"]
pub mod HASWRD_HASWRD4 {
    #[doc = "Hash Data Word x - HASH result bit; will store the expected hash result bit if hash check enabled; when hash check is not enabled, the hash engine will store the final hash result\\[31:0\\] here. If CRC mode enabled, this work store the CRC expected result if the check enabled, or store the final calcuated CRC result."]
    pub mod HASWRD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Hash Data Word 5"]
pub mod HASWRD_HASWRD5 {
    #[doc = "Hash Data Word x - HASH result bit; will store the expected hash result bit if hash check enabled; when hash check is not enabled, the hash engine will store the final hash result\\[31:0\\] here. If CRC mode enabled, this work store the CRC expected result if the check enabled, or store the final calcuated CRC result."]
    pub mod HASWRD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Hash Data Word 6"]
pub mod HASWRD_HASWRD6 {
    #[doc = "Hash Data Word x - HASH result bit; will store the expected hash result bit if hash check enabled; when hash check is not enabled, the hash engine will store the final hash result\\[31:0\\] here. If CRC mode enabled, this work store the CRC expected result if the check enabled, or store the final calcuated CRC result."]
    pub mod HASWRD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Hash Data Word 7"]
pub mod HASWRD_HASWRD7 {
    #[doc = "Hash Data Word x - HASH result bit; will store the expected hash result bit if hash check enabled; when hash check is not enabled, the hash engine will store the final hash result\\[31:0\\] here. If CRC mode enabled, this work store the CRC expected result if the check enabled, or store the final calcuated CRC result."]
    pub mod HASWRD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Command Pointer"]
pub mod CMDPTR {
    #[doc = "current command addresses the register points to the multiword descriptor that is to be executed (or is currently being executed)"]
    pub mod CMDPTR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Next Packet Address Pointer"]
pub mod NPKTPTR {
    #[doc = "Next Packet Address Pointer"]
    pub mod NPKTPTR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Packet Control Registers"]
pub mod PKTCTL {
    #[doc = "Reflects whether the channel must issue an interrupt upon the completion of the packet"]
    pub mod PKTINT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "whether the channel's semaphore must be decremented at the end of the current operation. When the semaphore reaches a value of zero, no more operations are issued from the channel."]
    pub mod DCRSEMA {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "whether the next command pointer register must be loaded into the channel's current descriptor pointer."]
    pub mod CHAIN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hash Initialization packat"]
    pub mod HASINI {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hash Termination packet"]
    pub mod HASFNL {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Load Initial Vector for the AES in this packet."]
    pub mod CIPHIV {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "packet tag"]
    pub mod PKTTAG {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Packet Memory Source Address"]
pub mod PKTSRC {
    #[doc = "Packet Memory Source Address"]
    pub mod PKTSRC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Packet Memory Destination Address"]
pub mod PKTDST {
    #[doc = "Packet Memory Destination Address"]
    pub mod PKTDST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Packet buffer size."]
pub mod PKTBUF {
    #[doc = "No description avaiable"]
    pub mod PKTBUF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
