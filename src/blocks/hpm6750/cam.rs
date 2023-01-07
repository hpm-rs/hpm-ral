#[doc = "CAM0"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Control Register"]
    pub CR1: crate::RWRegister<u32>,
    #[doc = "Interrupt Enable Register"]
    pub INT_EN: crate::RWRegister<u32>,
    _reserved0: [u8; 0x08],
    #[doc = "Control 2 Register"]
    pub CR2: crate::RWRegister<u32>,
    _reserved1: [u8; 0x10],
    #[doc = "Status Register"]
    pub STA: crate::RWRegister<u32>,
    _reserved2: [u8; 0x08],
    #[doc = "Pixel DMA Frame Buffer 1 Address"]
    pub DMASA_FB1: crate::RWRegister<u32>,
    #[doc = "Pixel DMA Frame Buffer 2 Address"]
    pub DMASA_FB2: crate::RWRegister<u32>,
    #[doc = "Buffer Parameters Register"]
    pub BUF_PARA: crate::RWRegister<u32>,
    #[doc = "Ideal Image Size Register"]
    pub IDEAL_WN_SIZE: crate::RWRegister<u32>,
    _reserved3: [u8; 0x0c],
    #[doc = "Control CR18 Register"]
    pub CR18: crate::RWRegister<u32>,
    #[doc = "Pixel UV DMA Frame Buffer 1 Address"]
    pub DMASA_UV1: crate::RWRegister<u32>,
    #[doc = "Pixel UV DMA Frame Buffer 2 Address"]
    pub DMASA_UV2: crate::RWRegister<u32>,
    #[doc = "Control CR20 Register"]
    pub CR20: crate::RWRegister<u32>,
    #[doc = "Max Window Size Register"]
    pub MAX_WN_CYCLE: crate::RWRegister<u32>,
    _reserved4: [u8; 0x10],
    #[doc = "Color Space Conversion Config Register 0"]
    pub CSC_COEF0: crate::RWRegister<u32>,
    #[doc = "Color Space Conversion Config Register 1"]
    pub CSC_COEF1: crate::RWRegister<u32>,
    #[doc = "Color Space Conversion Config Register 2"]
    pub CSC_COEF2: crate::RWRegister<u32>,
    #[doc = "Low Color Key Register"]
    pub CLRKEY_LOW: crate::RWRegister<u32>,
    #[doc = "High Color Key Register"]
    pub CLRKEY_HIGH: crate::RWRegister<u32>,
    _reserved5: [u8; 0x0c],
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA0: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA1: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA2: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA3: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA4: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA5: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA6: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA7: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA8: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA9: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA10: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA11: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA12: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA13: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA14: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA15: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA16: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA17: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA18: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA19: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA20: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA21: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA22: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA23: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA24: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA25: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA26: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA27: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA28: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA29: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA30: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA31: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA32: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA33: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA34: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA35: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA36: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA37: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA38: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA39: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA40: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA41: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA42: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA43: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA44: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA45: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA46: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA47: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA48: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA49: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA50: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA51: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA52: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA53: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA54: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA55: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA56: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA57: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA58: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA59: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA60: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA61: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA62: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA63: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA64: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA65: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA66: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA67: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA68: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA69: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA70: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA71: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA72: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA73: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA74: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA75: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA76: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA77: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA78: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA79: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA80: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA81: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA82: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA83: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA84: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA85: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA86: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA87: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA88: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA89: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA90: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA91: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA92: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA93: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA94: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA95: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA96: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA97: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA98: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA99: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA100: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA101: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA102: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA103: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA104: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA105: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA106: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA107: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA108: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA109: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA110: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA111: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA112: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA113: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA114: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA115: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA116: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA117: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA118: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA119: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA120: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA121: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA122: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA123: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA124: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA125: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA126: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA127: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA128: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA129: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA130: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA131: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA132: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA133: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA134: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA135: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA136: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA137: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA138: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA139: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA140: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA141: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA142: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA143: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA144: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA145: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA146: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA147: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA148: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA149: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA150: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA151: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA152: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA153: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA154: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA155: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA156: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA157: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA158: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA159: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA160: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA161: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA162: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA163: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA164: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA165: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA166: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA167: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA168: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA169: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA170: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA171: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA172: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA173: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA174: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA175: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA176: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA177: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA178: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA179: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA180: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA181: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA182: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA183: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA184: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA185: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA186: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA187: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA188: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA189: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA190: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA191: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA192: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA193: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA194: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA195: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA196: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA197: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA198: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA199: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA200: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA201: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA202: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA203: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA204: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA205: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA206: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA207: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA208: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA209: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA210: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA211: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA212: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA213: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA214: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA215: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA216: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA217: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA218: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA219: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA220: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA221: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA222: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA223: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA224: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA225: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA226: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA227: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA228: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA229: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA230: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA231: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA232: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA233: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA234: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA235: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA236: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA237: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA238: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA239: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA240: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA241: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA242: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA243: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA244: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA245: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA246: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA247: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA248: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA249: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA250: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA251: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA252: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA253: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA254: crate::RWRegister<u32>,
    #[doc = "Histogram Registers"]
    pub HISTOGRAM_FIFO_DATA255: crate::RWRegister<u32>,
}
#[doc = "Control Register"]
pub mod CR1 {
    #[doc = "the bit width of the sensor 0: 8 bits 1: 10 bits Others: Undefined"]
    pub mod SENSOR_BIT_WIDTH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "input color formats: 0010b: 24bit: RGB888 0100b: 16bit: RGB565 0111b: 16bit: YCbCr422 (Y0 Cb Y1 Cr, each 8-bit) YUV YCrCb Note: YUV420 is not supported. 1000b: 24bit: YUV444"]
    pub mod COLOR_FORMATS {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "00: Normal Mode (one plane mode) 01: Two Plane Mode (Y, UV plane) 10: Y-only Mode, byte sequence as Y0,Y1,Y2,Y3 11: Binary Mode, bit sequence is from LSB to MSB when CR20\\[BIG_END\\]=0"]
    pub mod STORAGE_MODE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Invert Data Input. This bit enables or disables internal inverters on the data lines. 0 CAM_D data lines are directly applied to internal circuitry 1 CAM_D data lines are inverted before applied to internal circuitry"]
    pub mod INV_DATA {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SOF Interrupt Polarity. This bit controls the condition that generates an SOF interrupt. 0 SOF interrupt is generated on SOF falling edge 1 SOF interrupt is generated on SOF rising edge"]
    pub mod SOF_INT_POL {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Synchronous Rx FIFO Clear. When asserted, this bit clears RXFIFO on every SOF."]
    pub mod SYNC_RXFIFO_CLR {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ASynchronous Rx FIFO Clear. When asserted, this bit clears RXFIFO immediately. It will be auto-cleared."]
    pub mod ASYNC_RXFIFO_CLR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "force to restart the bus pointer at the every end of the sof period, and at the same time, clr the fifo pointer"]
    pub mod RESTART_BUSPTR {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data Packing Direction. This bit Controls how 8-bit/10-bit image data is packed into 32-bit RX FIFO. 0 Pack from LSB first. For image data, 0x11, 0x22, 0x33, 0x44, it will appear as 0x44332211 in RX FIFO. 1 Pack from MSB first. For image data, 0x11, 0x22, 0x33, 0x44, it will appear as 0x11223344 in RX FIFO."]
    pub mod PACK_DIR {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SWAP 16-Bit Enable. This bit enables the swapping of 16-bit data. Data is packed from 8-bit or 10-bit to 32-bit first (according to the setting of PACK_DIR) and then swapped as 16-bit words before being put into the RX FIFO. The action of the bit only affects the RX FIFO. NOTE: Example of swapping enabled: Data input to FIFO = 0x11223344 Data in RX FIFO = 0x 33441122 NOTE: Example of swapping disabled: Data input to FIFO = 0x11223344 Data in RX FIFO = 0x11223344 0 Disable swapping 1 Enable swapping"]
    pub mod SWAP16_EN {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "invert vsync pad input before it is used"]
    pub mod INV_VSYNC {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "invert hsync pad input before it is used"]
    pub mod INV_HSYNC {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "invert pixclk pad input before it is used"]
    pub mod INV_PIXCLK {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "If asserted, will change the output color to ARGB8888 mode. Used by input color as RGB565, RGB888, YUV888, etc. The byte sequence is B,G,R,A. Depends on correct CR2\\[ClrBitFormat\\] configuration."]
    pub mod COLOR_EXT {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Enable Register"]
pub mod INT_EN {
    #[doc = "Start Of Frame (SOF) Interrupt Enable. This bit enables the SOF interrupt. 0 SOF interrupt disable 1 SOF interrupt enable"]
    pub mod SOF_INT_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Frame Buffer1 DMA Transfer Done Interrupt Enable. This bit enables the interrupt of Frame Buffer1 DMA transfer done. 0 Frame Buffer1 DMA Transfer Done interrupt disable 1 Frame Buffer1 DMA Transfer Done interrupt enable"]
    pub mod FB1_DMA_DONE_INTEN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Frame Buffer2 DMA Transfer Done Interrupt Enable. This bit enables the interrupt of Frame Buffer2 DMA transfer done. 0 Frame Buffer2 DMA Transfer Done interrupt disable 1 Frame Buffer2 DMA Transfer Done interrupt enable"]
    pub mod FB2_DMA_DONE_INTEN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RxFIFO Overrun Interrupt Enable. This bit enables the RX FIFO overrun interrupt. 0 RxFIFO overrun interrupt is disabled 1 RxFIFO overrun interrupt is enabled"]
    pub mod RF_OR_INTEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "End-of-Frame Interrupt Enable. This bit enables and disables the EOF interrupt. 0 EOF interrupt is disabled. 1 EOF interrupt is generated when RX count value is reached."]
    pub mod EOF_INT_EN {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hresponse Error Enable. This bit enables the hresponse error interrupt. 0 Disable hresponse error interrupt 1 Enable hresponse error interrupt"]
    pub mod HRESP_ERR_EN {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable hist done int"]
    pub mod HIST_DONE_INT_EN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The unsupported color (color_formats\\[3:0\\]) and bitwidth (sensor_bit_width\\[2:0\\]) configuation interrupt enable"]
    pub mod ERR_CL_BWID_CFG_INT_EN {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control 2 Register"]
pub mod CR2 {
    #[doc = "Input Byte & bit sequence same as OV5640, except for Raw mode. Used only for internal ARGB conversion."]
    pub mod CLRBITFORMAT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Request Enable for RxFIFO. This bit enables the dma request from RxFIFO to the embedded DMA controller. 0 Disable the dma request 1 Enable the dma request. The UV Rx FIFO is only enabled to filling data in 2 plane mode."]
    pub mod DMA_REQ_EN_RFF {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RxFIFO Full Level. When the number of data in RxFIFO reaches this level, a RxFIFO full interrupt is generated, or an RXFIFO DMA request is sent. 000 4 Double words 001 8 Double words 010 16 Double words 011 24 Double words 100 32 Double words 101 48 Double words 110 64 Double words 111 96 Double words"]
    pub mod RXFF_LEVEL {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Frame Count Reset. Resets the Frame Counter. 0 Do not reset 1 Reset frame counter immediately"]
    pub mod FRMCNT_RST {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Frame Counter. This is a 16-bit Frame Counter (Wraps around automatically after reaching the maximum)"]
    pub mod FRMCNT_15_0 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status Register"]
pub mod STA {
    #[doc = "Hresponse Error Interrupt Status. Indicates that a hresponse error has been detected. (Cleared by writing 1) 0 No hresponse error. 1 Hresponse error is detected."]
    pub mod HRESP_ERR_INT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Start of Frame Interrupt Status. Indicates when SOF is detected. (Cleared by writing 1) 0 SOF is not detected. 1 SOF is detected."]
    pub mod SOF_INT {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "End of Frame (EOF) Interrupt Status. Indicates when EOF is detected. (Cleared by writing 1) 0 EOF is not detected. 1 EOF is detected."]
    pub mod EOF_INT {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Transfer Done in Frame Buffer1. Indicates that the DMA transfer from RxFIFO to Frame Buffer1 is completed. It can trigger an interrupt if the corresponding enable bit is set in CAM_CR1. This bit can be cleared by by writting 1 or reflashing the RxFIFO dma controller in CAM_CR3. (Cleared by writing 1) 0 DMA transfer is not completed. 1 DMA transfer is completed."]
    pub mod DMA_TSF_DONE_FB1 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Transfer Done in Frame Buffer2. Indicates that the DMA transfer from RxFIFO to Frame Buffer2 is completed. It can trigger an interrupt if the corresponding enable bit is set in CAM_CR1. This bit can be cleared by by writting 1 or reflashing the RxFIFO dma controller in CAM_CR3. (Cleared by writing 1) 0 DMA transfer is not completed. 1 DMA transfer is completed."]
    pub mod DMA_TSF_DONE_FB2 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RxFIFO Overrun Interrupt Status. Indicates the overflow status of the RxFIFO register. (Cleared by writing 1) 0 RXFIFO has not overflowed. 1 RXFIFO has overflowed."]
    pub mod RF_OR_INT {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "hist cal done"]
    pub mod HIST_DONE {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The unsupported color (color_formats\\[3:0\\]) and bitwidth (sensor_bit_width\\[2:0\\]) configuation found"]
    pub mod ERR_CL_BWID_CFG {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Pixel DMA Frame Buffer 1 Address"]
pub mod DMASA_FB1 {
    #[doc = "DMA Start Address in Frame Buffer1. Indicates the start address to write data. The embedded DMA controller will read data from RxFIFO and write it from this address through AHB bus. The address should be double words aligned. In Two-Plane Mode, Y buffer1"]
    pub mod PTR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x3fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Pixel DMA Frame Buffer 2 Address"]
pub mod DMASA_FB2 {
    #[doc = "DMA Start Address in Frame Buffer2. Indicates the start address to write data. The embedded DMA controller will read data from RxFIFO and write it from this address through AHB bus. The address should be double words aligned. In Two-Plane Mode, Y buffer2"]
    pub mod PTR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x3fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Buffer Parameters Register"]
pub mod BUF_PARA {
    #[doc = "Line Blank Space Stride. Indicates the space between the end of line image storage and the start of a new line storage in the frame buffer. The width of the line storage in frame buffer(in double words) minus the width of the image(in double words) is the stride. The stride should be double words aligned. The embedded DMA controller will skip the stride before starting to write the next row of the image."]
    pub mod LINEBSP_STRIDE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Ideal Image Size Register"]
pub mod IDEAL_WN_SIZE {
    #[doc = "Image Width. Indicates how many active pixels in a line of the image from the sensor. The number of bytes to be transfered is re-calculated automatically in hardware based on cr1\\[color_ext\\] and cr1\\[store_mode\\]. Default value is 2*pixel number. As the input data from the sensor is 8-bit/pixel format, the IMAGE_WIDTH should be a multiple of 8 pixels."]
    pub mod WIDTH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Image Height. Indicates how many active pixels in a column of the image from the sensor."]
    pub mod HEIGHT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control CR18 Register"]
pub mod CR18 {
    #[doc = "AWQOS for bus fabric arbitration"]
    pub mod AWQOS {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CAM global enable signal. Only when this bit is 1, CAM can start to receive the data and store to memory."]
    pub mod CAM_ENABLE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Pixel UV DMA Frame Buffer 1 Address"]
pub mod DMASA_UV1 {
    #[doc = "Two Plane UV Buffer Start Address 1"]
    pub mod PTR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x3fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Pixel UV DMA Frame Buffer 2 Address"]
pub mod DMASA_UV2 {
    #[doc = "Two Plane UV Buffer Start Address 2"]
    pub mod PTR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x3fff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control CR20 Register"]
pub mod CR20 {
    #[doc = "Threshold to generate binary color. Bin 1 is output if the pixel is greater than the threshold."]
    pub mod THRESHOLD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Asserted when binary output is in big-endian type, which mean the right most data is at the LSBs. Take function only inside the 32-bit word."]
    pub mod BIG_END {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "histogarm enable"]
    pub mod HISTOGRAM_EN {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "binary picture output enable"]
    pub mod BINARY_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Max Window Size Register"]
pub mod MAX_WN_CYCLE {
    #[doc = "Max Height-1"]
    pub mod COL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Max Width-1"]
    pub mod ROW {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Color Space Conversion Config Register 0"]
pub mod CSC_COEF0 {
    #[doc = "Two's compliment amplitude offset implicit in the Y data. For YUV, this is typically 0 and for YCbCr, this is typically -16 (0x1F0)."]
    pub mod Y_OFFSET {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Two's compliment phase offset implicit for CbCr data. Generally used for YCbCr to RGB conversion. YCbCr=0x180, YUV=0x000 (typically -128 or 0x180 to indicate normalized -0.5 to 0.5 range)."]
    pub mod UV_OFFSET {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Two's compliment Y multiplier coefficient. YUV=0x100 (1.000) YCbCr=0x12A (1.164)"]
    pub mod C0 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable the CSC unit 0b - The CSC is bypassed and the input pixels are RGB data already 1b - The CSC is enabled and the pixels will be converted to RGB data"]
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
#[doc = "Color Space Conversion Config Register 1"]
pub mod CSC_COEF1 {
    #[doc = "Two's compliment Blue U/Cb multiplier coefficient. YUV=0x208 (2.032) YCbCr=0x204 (2.017)."]
    pub mod C4 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Two's compliment Red V/Cr multiplier coefficient. YUV=0x123 (1.140) YCbCr=0x198 (1.596)."]
    pub mod C1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Color Space Conversion Config Register 2"]
pub mod CSC_COEF2 {
    #[doc = "Two's compliment Green U/Cb multiplier coefficient. YUV=0x79C (-0.394) YCbCr=0x79C (-0.392)."]
    pub mod C3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Two's compliment Green V/Cr multiplier coefficient. YUV=0x76B (-0.581) YCbCr=0x730 (-0.813)."]
    pub mod C2 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Low Color Key Register"]
pub mod CLRKEY_LOW {
    #[doc = "Low range of color key applied to PS buffer. To disable PS colorkeying, set the low colorkey to 0xFFFFFF and the high colorkey to 0x000000."]
    pub mod LIMIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "High Color Key Register"]
pub mod CLRKEY_HIGH {
    #[doc = "Low range of color key applied to PS buffer. To disable PS colorkeying, set the low colorkey to 0xFFFFFF and the high colorkey to 0x000000."]
    pub mod LIMIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA0 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA1 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA2 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA3 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA4 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA5 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA6 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA7 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA8 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA9 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA10 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA11 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA12 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA13 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA14 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA15 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA16 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA17 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA18 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA19 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA20 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA21 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA22 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA23 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA24 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA25 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA26 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA27 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA28 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA29 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA30 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA31 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA32 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA33 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA34 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA35 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA36 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA37 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA38 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA39 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA40 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA41 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA42 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA43 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA44 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA45 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA46 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA47 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA48 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA49 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA50 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA51 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA52 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA53 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA54 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA55 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA56 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA57 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA58 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA59 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA60 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA61 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA62 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA63 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA64 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA65 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA66 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA67 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA68 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA69 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA70 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA71 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA72 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA73 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA74 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA75 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA76 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA77 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA78 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA79 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA80 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA81 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA82 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA83 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA84 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA85 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA86 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA87 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA88 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA89 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA90 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA91 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA92 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA93 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA94 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA95 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA96 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA97 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA98 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA99 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA100 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA101 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA102 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA103 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA104 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA105 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA106 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA107 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA108 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA109 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA110 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA111 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA112 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA113 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA114 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA115 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA116 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA117 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA118 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA119 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA120 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA121 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA122 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA123 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA124 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA125 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA126 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA127 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA128 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA129 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA130 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA131 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA132 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA133 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA134 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA135 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA136 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA137 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA138 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA139 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA140 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA141 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA142 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA143 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA144 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA145 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA146 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA147 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA148 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA149 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA150 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA151 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA152 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA153 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA154 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA155 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA156 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA157 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA158 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA159 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA160 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA161 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA162 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA163 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA164 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA165 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA166 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA167 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA168 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA169 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA170 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA171 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA172 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA173 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA174 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA175 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA176 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA177 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA178 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA179 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA180 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA181 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA182 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA183 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA184 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA185 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA186 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA187 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA188 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA189 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA190 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA191 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA192 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA193 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA194 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA195 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA196 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA197 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA198 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA199 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA200 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA201 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA202 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA203 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA204 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA205 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA206 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA207 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA208 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA209 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA210 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA211 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA212 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA213 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA214 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA215 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA216 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA217 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA218 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA219 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA220 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA221 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA222 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA223 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA224 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA225 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA226 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA227 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA228 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA229 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA230 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA231 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA232 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA233 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA234 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA235 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA236 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA237 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA238 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA239 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA240 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA241 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA242 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA243 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA244 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA245 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA246 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA247 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA248 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA249 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA250 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA251 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA252 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA253 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA254 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Histogram Registers"]
pub mod HISTOGRAM_FIFO_DATA255 {
    #[doc = "the appearance of bin x (x=(address-DATA0)/4)"]
    pub mod HIST_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
