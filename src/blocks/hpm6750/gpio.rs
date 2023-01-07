#[doc = "FGPIO"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "GPIO input value"]
    pub DI_GPIOA_VALUE: crate::RWRegister<u32>,
    #[doc = "GPIO input set"]
    pub DI_GPIOA_SET: crate::RWRegister<u32>,
    #[doc = "GPIO input clear"]
    pub DI_GPIOA_CLEAR: crate::RWRegister<u32>,
    #[doc = "GPIO input toggle"]
    pub DI_GPIOA_TOGGLE: crate::RWRegister<u32>,
    #[doc = "GPIOB input"]
    pub DI_GPIOB_VALUE: crate::RWRegister<u32>,
    #[doc = "GPIO input set"]
    pub DI_GPIOB_SET: crate::RWRegister<u32>,
    #[doc = "GPIO input clear"]
    pub DI_GPIOB_CLEAR: crate::RWRegister<u32>,
    #[doc = "GPIO input toggle"]
    pub DI_GPIOB_TOGGLE: crate::RWRegister<u32>,
    #[doc = "GPIOC input"]
    pub DI_GPIOC_VALUE: crate::RWRegister<u32>,
    #[doc = "GPIO input set"]
    pub DI_GPIOC_SET: crate::RWRegister<u32>,
    #[doc = "GPIO input clear"]
    pub DI_GPIOC_CLEAR: crate::RWRegister<u32>,
    #[doc = "GPIO input toggle"]
    pub DI_GPIOC_TOGGLE: crate::RWRegister<u32>,
    #[doc = "GPIOD input"]
    pub DI_GPIOD_VALUE: crate::RWRegister<u32>,
    #[doc = "GPIO input set"]
    pub DI_GPIOD_SET: crate::RWRegister<u32>,
    #[doc = "GPIO input clear"]
    pub DI_GPIOD_CLEAR: crate::RWRegister<u32>,
    #[doc = "GPIO input toggle"]
    pub DI_GPIOD_TOGGLE: crate::RWRegister<u32>,
    #[doc = "GPIOE input"]
    pub DI_GPIOE_VALUE: crate::RWRegister<u32>,
    #[doc = "GPIO input set"]
    pub DI_GPIOE_SET: crate::RWRegister<u32>,
    #[doc = "GPIO input clear"]
    pub DI_GPIOE_CLEAR: crate::RWRegister<u32>,
    #[doc = "GPIO input toggle"]
    pub DI_GPIOE_TOGGLE: crate::RWRegister<u32>,
    #[doc = "GPIOF input"]
    pub DI_GPIOF_VALUE: crate::RWRegister<u32>,
    #[doc = "GPIO input set"]
    pub DI_GPIOF_SET: crate::RWRegister<u32>,
    #[doc = "GPIO input clear"]
    pub DI_GPIOF_CLEAR: crate::RWRegister<u32>,
    #[doc = "GPIO input toggle"]
    pub DI_GPIOF_TOGGLE: crate::RWRegister<u32>,
    _reserved0: [u8; 0x70],
    #[doc = "GPIOX input"]
    pub DI_GPIOX_VALUE: crate::RWRegister<u32>,
    #[doc = "GPIO input set"]
    pub DI_GPIOX_SET: crate::RWRegister<u32>,
    #[doc = "GPIO input clear"]
    pub DI_GPIOX_CLEAR: crate::RWRegister<u32>,
    #[doc = "GPIO input toggle"]
    pub DI_GPIOX_TOGGLE: crate::RWRegister<u32>,
    #[doc = "GPIOY input"]
    pub DI_GPIOY_VALUE: crate::RWRegister<u32>,
    #[doc = "GPIO input set"]
    pub DI_GPIOY_SET: crate::RWRegister<u32>,
    #[doc = "GPIO input clear"]
    pub DI_GPIOY_CLEAR: crate::RWRegister<u32>,
    #[doc = "GPIO input toggle"]
    pub DI_GPIOY_TOGGLE: crate::RWRegister<u32>,
    #[doc = "GPIOZ input"]
    pub DI_GPIOZ_VALUE: crate::RWRegister<u32>,
    #[doc = "GPIO input set"]
    pub DI_GPIOZ_SET: crate::RWRegister<u32>,
    #[doc = "GPIO input clear"]
    pub DI_GPIOZ_CLEAR: crate::RWRegister<u32>,
    #[doc = "GPIO input toggle"]
    pub DI_GPIOZ_TOGGLE: crate::RWRegister<u32>,
    #[doc = "GPIO output value"]
    pub DO_GPIOA_VALUE: crate::RWRegister<u32>,
    #[doc = "GPIO output set"]
    pub DO_GPIOA_SET: crate::RWRegister<u32>,
    #[doc = "GPIO output clear"]
    pub DO_GPIOA_CLEAR: crate::RWRegister<u32>,
    #[doc = "GPIO output toggle"]
    pub DO_GPIOA_TOGGLE: crate::RWRegister<u32>,
    #[doc = "GPIOB output"]
    pub DO_GPIOB_VALUE: crate::RWRegister<u32>,
    #[doc = "GPIO output set"]
    pub DO_GPIOB_SET: crate::RWRegister<u32>,
    #[doc = "GPIO output clear"]
    pub DO_GPIOB_CLEAR: crate::RWRegister<u32>,
    #[doc = "GPIO output toggle"]
    pub DO_GPIOB_TOGGLE: crate::RWRegister<u32>,
    #[doc = "GPIOC output"]
    pub DO_GPIOC_VALUE: crate::RWRegister<u32>,
    #[doc = "GPIO output set"]
    pub DO_GPIOC_SET: crate::RWRegister<u32>,
    #[doc = "GPIO output clear"]
    pub DO_GPIOC_CLEAR: crate::RWRegister<u32>,
    #[doc = "GPIO output toggle"]
    pub DO_GPIOC_TOGGLE: crate::RWRegister<u32>,
    #[doc = "GPIOD output"]
    pub DO_GPIOD_VALUE: crate::RWRegister<u32>,
    #[doc = "GPIO output set"]
    pub DO_GPIOD_SET: crate::RWRegister<u32>,
    #[doc = "GPIO output clear"]
    pub DO_GPIOD_CLEAR: crate::RWRegister<u32>,
    #[doc = "GPIO output toggle"]
    pub DO_GPIOD_TOGGLE: crate::RWRegister<u32>,
    #[doc = "GPIOE output"]
    pub DO_GPIOE_VALUE: crate::RWRegister<u32>,
    #[doc = "GPIO output set"]
    pub DO_GPIOE_SET: crate::RWRegister<u32>,
    #[doc = "GPIO output clear"]
    pub DO_GPIOE_CLEAR: crate::RWRegister<u32>,
    #[doc = "GPIO output toggle"]
    pub DO_GPIOE_TOGGLE: crate::RWRegister<u32>,
    #[doc = "GPIOF output"]
    pub DO_GPIOF_VALUE: crate::RWRegister<u32>,
    #[doc = "GPIO output set"]
    pub DO_GPIOF_SET: crate::RWRegister<u32>,
    #[doc = "GPIO output clear"]
    pub DO_GPIOF_CLEAR: crate::RWRegister<u32>,
    #[doc = "GPIO output toggle"]
    pub DO_GPIOF_TOGGLE: crate::RWRegister<u32>,
    _reserved1: [u8; 0x70],
    #[doc = "GPIOX output"]
    pub DO_GPIOX_VALUE: crate::RWRegister<u32>,
    #[doc = "GPIO output set"]
    pub DO_GPIOX_SET: crate::RWRegister<u32>,
    #[doc = "GPIO output clear"]
    pub DO_GPIOX_CLEAR: crate::RWRegister<u32>,
    #[doc = "GPIO output toggle"]
    pub DO_GPIOX_TOGGLE: crate::RWRegister<u32>,
    #[doc = "GPIOY output"]
    pub DO_GPIOY_VALUE: crate::RWRegister<u32>,
    #[doc = "GPIO output set"]
    pub DO_GPIOY_SET: crate::RWRegister<u32>,
    #[doc = "GPIO output clear"]
    pub DO_GPIOY_CLEAR: crate::RWRegister<u32>,
    #[doc = "GPIO output toggle"]
    pub DO_GPIOY_TOGGLE: crate::RWRegister<u32>,
    #[doc = "GPIOZ output"]
    pub DO_GPIOZ_VALUE: crate::RWRegister<u32>,
    #[doc = "GPIO output set"]
    pub DO_GPIOZ_SET: crate::RWRegister<u32>,
    #[doc = "GPIO output clear"]
    pub DO_GPIOZ_CLEAR: crate::RWRegister<u32>,
    #[doc = "GPIO output toggle"]
    pub DO_GPIOZ_TOGGLE: crate::RWRegister<u32>,
    #[doc = "GPIO direction value"]
    pub OE_GPIOA_VALUE: crate::RWRegister<u32>,
    #[doc = "GPIO direction set"]
    pub OE_GPIOA_SET: crate::RWRegister<u32>,
    #[doc = "GPIO direction clear"]
    pub OE_GPIOA_CLEAR: crate::RWRegister<u32>,
    #[doc = "GPIO direction toggle"]
    pub OE_GPIOA_TOGGLE: crate::RWRegister<u32>,
    #[doc = "GPIOB direction"]
    pub OE_GPIOB_VALUE: crate::RWRegister<u32>,
    #[doc = "GPIO direction set"]
    pub OE_GPIOB_SET: crate::RWRegister<u32>,
    #[doc = "GPIO direction clear"]
    pub OE_GPIOB_CLEAR: crate::RWRegister<u32>,
    #[doc = "GPIO direction toggle"]
    pub OE_GPIOB_TOGGLE: crate::RWRegister<u32>,
    #[doc = "GPIOC direction"]
    pub OE_GPIOC_VALUE: crate::RWRegister<u32>,
    #[doc = "GPIO direction set"]
    pub OE_GPIOC_SET: crate::RWRegister<u32>,
    #[doc = "GPIO direction clear"]
    pub OE_GPIOC_CLEAR: crate::RWRegister<u32>,
    #[doc = "GPIO direction toggle"]
    pub OE_GPIOC_TOGGLE: crate::RWRegister<u32>,
    #[doc = "GPIOD direction"]
    pub OE_GPIOD_VALUE: crate::RWRegister<u32>,
    #[doc = "GPIO direction set"]
    pub OE_GPIOD_SET: crate::RWRegister<u32>,
    #[doc = "GPIO direction clear"]
    pub OE_GPIOD_CLEAR: crate::RWRegister<u32>,
    #[doc = "GPIO direction toggle"]
    pub OE_GPIOD_TOGGLE: crate::RWRegister<u32>,
    #[doc = "GPIOE direction"]
    pub OE_GPIOE_VALUE: crate::RWRegister<u32>,
    #[doc = "GPIO direction set"]
    pub OE_GPIOE_SET: crate::RWRegister<u32>,
    #[doc = "GPIO direction clear"]
    pub OE_GPIOE_CLEAR: crate::RWRegister<u32>,
    #[doc = "GPIO direction toggle"]
    pub OE_GPIOE_TOGGLE: crate::RWRegister<u32>,
    #[doc = "GPIOF direction"]
    pub OE_GPIOF_VALUE: crate::RWRegister<u32>,
    #[doc = "GPIO direction set"]
    pub OE_GPIOF_SET: crate::RWRegister<u32>,
    #[doc = "GPIO direction clear"]
    pub OE_GPIOF_CLEAR: crate::RWRegister<u32>,
    #[doc = "GPIO direction toggle"]
    pub OE_GPIOF_TOGGLE: crate::RWRegister<u32>,
    _reserved2: [u8; 0x70],
    #[doc = "GPIOX direction"]
    pub OE_GPIOX_VALUE: crate::RWRegister<u32>,
    #[doc = "GPIO direction set"]
    pub OE_GPIOX_SET: crate::RWRegister<u32>,
    #[doc = "GPIO direction clear"]
    pub OE_GPIOX_CLEAR: crate::RWRegister<u32>,
    #[doc = "GPIO direction toggle"]
    pub OE_GPIOX_TOGGLE: crate::RWRegister<u32>,
    #[doc = "GPIOY direction"]
    pub OE_GPIOY_VALUE: crate::RWRegister<u32>,
    #[doc = "GPIO direction set"]
    pub OE_GPIOY_SET: crate::RWRegister<u32>,
    #[doc = "GPIO direction clear"]
    pub OE_GPIOY_CLEAR: crate::RWRegister<u32>,
    #[doc = "GPIO direction toggle"]
    pub OE_GPIOY_TOGGLE: crate::RWRegister<u32>,
    #[doc = "GPIOZ direction"]
    pub OE_GPIOZ_VALUE: crate::RWRegister<u32>,
    #[doc = "GPIO direction set"]
    pub OE_GPIOZ_SET: crate::RWRegister<u32>,
    #[doc = "GPIO direction clear"]
    pub OE_GPIOZ_CLEAR: crate::RWRegister<u32>,
    #[doc = "GPIO direction toggle"]
    pub OE_GPIOZ_TOGGLE: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt flag value"]
    pub IF_GPIOA_VALUE: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt flag set"]
    pub IF_GPIOA_SET: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt flag clear"]
    pub IF_GPIOA_CLEAR: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt flag toggle"]
    pub IF_GPIOA_TOGGLE: crate::RWRegister<u32>,
    #[doc = "GPIOB interrupt flag"]
    pub IF_GPIOB_VALUE: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt flag set"]
    pub IF_GPIOB_SET: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt flag clear"]
    pub IF_GPIOB_CLEAR: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt flag toggle"]
    pub IF_GPIOB_TOGGLE: crate::RWRegister<u32>,
    #[doc = "GPIOC interrupt flag"]
    pub IF_GPIOC_VALUE: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt flag set"]
    pub IF_GPIOC_SET: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt flag clear"]
    pub IF_GPIOC_CLEAR: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt flag toggle"]
    pub IF_GPIOC_TOGGLE: crate::RWRegister<u32>,
    #[doc = "GPIOD interrupt flag"]
    pub IF_GPIOD_VALUE: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt flag set"]
    pub IF_GPIOD_SET: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt flag clear"]
    pub IF_GPIOD_CLEAR: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt flag toggle"]
    pub IF_GPIOD_TOGGLE: crate::RWRegister<u32>,
    #[doc = "GPIOE interrupt flag"]
    pub IF_GPIOE_VALUE: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt flag set"]
    pub IF_GPIOE_SET: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt flag clear"]
    pub IF_GPIOE_CLEAR: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt flag toggle"]
    pub IF_GPIOE_TOGGLE: crate::RWRegister<u32>,
    #[doc = "GPIOF interrupt flag"]
    pub IF_GPIOF_VALUE: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt flag set"]
    pub IF_GPIOF_SET: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt flag clear"]
    pub IF_GPIOF_CLEAR: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt flag toggle"]
    pub IF_GPIOF_TOGGLE: crate::RWRegister<u32>,
    _reserved3: [u8; 0x70],
    #[doc = "GPIOX interrupt flag"]
    pub IF_GPIOX_VALUE: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt flag set"]
    pub IF_GPIOX_SET: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt flag clear"]
    pub IF_GPIOX_CLEAR: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt flag toggle"]
    pub IF_GPIOX_TOGGLE: crate::RWRegister<u32>,
    #[doc = "GPIOY interrupt flag"]
    pub IF_GPIOY_VALUE: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt flag set"]
    pub IF_GPIOY_SET: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt flag clear"]
    pub IF_GPIOY_CLEAR: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt flag toggle"]
    pub IF_GPIOY_TOGGLE: crate::RWRegister<u32>,
    #[doc = "GPIOZ interrupt flag"]
    pub IF_GPIOZ_VALUE: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt flag set"]
    pub IF_GPIOZ_SET: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt flag clear"]
    pub IF_GPIOZ_CLEAR: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt flag toggle"]
    pub IF_GPIOZ_TOGGLE: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt enable value"]
    pub IE_GPIOA_VALUE: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt enable set"]
    pub IE_GPIOA_SET: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt enable clear"]
    pub IE_GPIOA_CLEAR: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt enable toggle"]
    pub IE_GPIOA_TOGGLE: crate::RWRegister<u32>,
    #[doc = "GPIOB interrupt enable"]
    pub IE_GPIOB_VALUE: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt enable set"]
    pub IE_GPIOB_SET: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt enable clear"]
    pub IE_GPIOB_CLEAR: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt enable toggle"]
    pub IE_GPIOB_TOGGLE: crate::RWRegister<u32>,
    #[doc = "GPIOC interrupt enable"]
    pub IE_GPIOC_VALUE: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt enable set"]
    pub IE_GPIOC_SET: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt enable clear"]
    pub IE_GPIOC_CLEAR: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt enable toggle"]
    pub IE_GPIOC_TOGGLE: crate::RWRegister<u32>,
    #[doc = "GPIOD interrupt enable"]
    pub IE_GPIOD_VALUE: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt enable set"]
    pub IE_GPIOD_SET: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt enable clear"]
    pub IE_GPIOD_CLEAR: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt enable toggle"]
    pub IE_GPIOD_TOGGLE: crate::RWRegister<u32>,
    #[doc = "GPIOE interrupt enable"]
    pub IE_GPIOE_VALUE: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt enable set"]
    pub IE_GPIOE_SET: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt enable clear"]
    pub IE_GPIOE_CLEAR: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt enable toggle"]
    pub IE_GPIOE_TOGGLE: crate::RWRegister<u32>,
    #[doc = "GPIOF interrupt enable"]
    pub IE_GPIOF_VALUE: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt enable set"]
    pub IE_GPIOF_SET: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt enable clear"]
    pub IE_GPIOF_CLEAR: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt enable toggle"]
    pub IE_GPIOF_TOGGLE: crate::RWRegister<u32>,
    _reserved4: [u8; 0x70],
    #[doc = "GPIOX interrupt enable"]
    pub IE_GPIOX_VALUE: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt enable set"]
    pub IE_GPIOX_SET: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt enable clear"]
    pub IE_GPIOX_CLEAR: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt enable toggle"]
    pub IE_GPIOX_TOGGLE: crate::RWRegister<u32>,
    #[doc = "GPIOY interrupt enable"]
    pub IE_GPIOY_VALUE: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt enable set"]
    pub IE_GPIOY_SET: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt enable clear"]
    pub IE_GPIOY_CLEAR: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt enable toggle"]
    pub IE_GPIOY_TOGGLE: crate::RWRegister<u32>,
    #[doc = "GPIOZ interrupt enable"]
    pub IE_GPIOZ_VALUE: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt enable set"]
    pub IE_GPIOZ_SET: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt enable clear"]
    pub IE_GPIOZ_CLEAR: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt enable toggle"]
    pub IE_GPIOZ_TOGGLE: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt polarity value"]
    pub PL_GPIOA_VALUE: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt polarity set"]
    pub PL_GPIOA_SET: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt polarity clear"]
    pub PL_GPIOA_CLEAR: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt polarity toggle"]
    pub PL_GPIOA_TOGGLE: crate::RWRegister<u32>,
    #[doc = "GPIOB interrupt polarity"]
    pub PL_GPIOB_VALUE: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt polarity set"]
    pub PL_GPIOB_SET: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt polarity clear"]
    pub PL_GPIOB_CLEAR: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt polarity toggle"]
    pub PL_GPIOB_TOGGLE: crate::RWRegister<u32>,
    #[doc = "GPIOC interrupt polarity"]
    pub PL_GPIOC_VALUE: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt polarity set"]
    pub PL_GPIOC_SET: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt polarity clear"]
    pub PL_GPIOC_CLEAR: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt polarity toggle"]
    pub PL_GPIOC_TOGGLE: crate::RWRegister<u32>,
    #[doc = "GPIOD interrupt polarity"]
    pub PL_GPIOD_VALUE: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt polarity set"]
    pub PL_GPIOD_SET: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt polarity clear"]
    pub PL_GPIOD_CLEAR: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt polarity toggle"]
    pub PL_GPIOD_TOGGLE: crate::RWRegister<u32>,
    #[doc = "GPIOE interrupt polarity"]
    pub PL_GPIOE_VALUE: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt polarity set"]
    pub PL_GPIOE_SET: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt polarity clear"]
    pub PL_GPIOE_CLEAR: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt polarity toggle"]
    pub PL_GPIOE_TOGGLE: crate::RWRegister<u32>,
    #[doc = "GPIOF interrupt polarity"]
    pub PL_GPIOF_VALUE: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt polarity set"]
    pub PL_GPIOF_SET: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt polarity clear"]
    pub PL_GPIOF_CLEAR: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt polarity toggle"]
    pub PL_GPIOF_TOGGLE: crate::RWRegister<u32>,
    _reserved5: [u8; 0x70],
    #[doc = "GPIOX interrupt polarity"]
    pub PL_GPIOX_VALUE: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt polarity set"]
    pub PL_GPIOX_SET: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt polarity clear"]
    pub PL_GPIOX_CLEAR: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt polarity toggle"]
    pub PL_GPIOX_TOGGLE: crate::RWRegister<u32>,
    #[doc = "GPIOY interrupt polarity"]
    pub PL_GPIOY_VALUE: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt polarity set"]
    pub PL_GPIOY_SET: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt polarity clear"]
    pub PL_GPIOY_CLEAR: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt polarity toggle"]
    pub PL_GPIOY_TOGGLE: crate::RWRegister<u32>,
    #[doc = "GPIOZ interrupt polarity"]
    pub PL_GPIOZ_VALUE: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt polarity set"]
    pub PL_GPIOZ_SET: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt polarity clear"]
    pub PL_GPIOZ_CLEAR: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt polarity toggle"]
    pub PL_GPIOZ_TOGGLE: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt type value"]
    pub TP_GPIOA_VALUE: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt type set"]
    pub TP_GPIOA_SET: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt type clear"]
    pub TP_GPIOA_CLEAR: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt type toggle"]
    pub TP_GPIOA_TOGGLE: crate::RWRegister<u32>,
    #[doc = "GPIOB interrupt type"]
    pub TP_GPIOB_VALUE: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt type set"]
    pub TP_GPIOB_SET: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt type clear"]
    pub TP_GPIOB_CLEAR: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt type toggle"]
    pub TP_GPIOB_TOGGLE: crate::RWRegister<u32>,
    #[doc = "GPIOC interrupt type"]
    pub TP_GPIOC_VALUE: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt type set"]
    pub TP_GPIOC_SET: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt type clear"]
    pub TP_GPIOC_CLEAR: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt type toggle"]
    pub TP_GPIOC_TOGGLE: crate::RWRegister<u32>,
    #[doc = "GPIOD interrupt type"]
    pub TP_GPIOD_VALUE: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt type set"]
    pub TP_GPIOD_SET: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt type clear"]
    pub TP_GPIOD_CLEAR: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt type toggle"]
    pub TP_GPIOD_TOGGLE: crate::RWRegister<u32>,
    #[doc = "GPIOE interrupt type"]
    pub TP_GPIOE_VALUE: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt type set"]
    pub TP_GPIOE_SET: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt type clear"]
    pub TP_GPIOE_CLEAR: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt type toggle"]
    pub TP_GPIOE_TOGGLE: crate::RWRegister<u32>,
    #[doc = "GPIOF interrupt type"]
    pub TP_GPIOF_VALUE: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt type set"]
    pub TP_GPIOF_SET: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt type clear"]
    pub TP_GPIOF_CLEAR: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt type toggle"]
    pub TP_GPIOF_TOGGLE: crate::RWRegister<u32>,
    _reserved6: [u8; 0x70],
    #[doc = "GPIOX interrupt type"]
    pub TP_GPIOX_VALUE: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt type set"]
    pub TP_GPIOX_SET: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt type clear"]
    pub TP_GPIOX_CLEAR: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt type toggle"]
    pub TP_GPIOX_TOGGLE: crate::RWRegister<u32>,
    #[doc = "GPIOY interrupt type"]
    pub TP_GPIOY_VALUE: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt type set"]
    pub TP_GPIOY_SET: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt type clear"]
    pub TP_GPIOY_CLEAR: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt type toggle"]
    pub TP_GPIOY_TOGGLE: crate::RWRegister<u32>,
    #[doc = "GPIOZ interrupt type"]
    pub TP_GPIOZ_VALUE: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt type set"]
    pub TP_GPIOZ_SET: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt type clear"]
    pub TP_GPIOZ_CLEAR: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt type toggle"]
    pub TP_GPIOZ_TOGGLE: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt asynchronous value"]
    pub AS_GPIOA_VALUE: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt asynchronous set"]
    pub AS_GPIOA_SET: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt asynchronous clear"]
    pub AS_GPIOA_CLEAR: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt asynchronous toggle"]
    pub AS_GPIOA_TOGGLE: crate::RWRegister<u32>,
    #[doc = "GPIOB interrupt asynchronous"]
    pub AS_GPIOB_VALUE: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt asynchronous set"]
    pub AS_GPIOB_SET: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt asynchronous clear"]
    pub AS_GPIOB_CLEAR: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt asynchronous toggle"]
    pub AS_GPIOB_TOGGLE: crate::RWRegister<u32>,
    #[doc = "GPIOC interrupt asynchronous"]
    pub AS_GPIOC_VALUE: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt asynchronous set"]
    pub AS_GPIOC_SET: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt asynchronous clear"]
    pub AS_GPIOC_CLEAR: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt asynchronous toggle"]
    pub AS_GPIOC_TOGGLE: crate::RWRegister<u32>,
    #[doc = "GPIOD interrupt asynchronous"]
    pub AS_GPIOD_VALUE: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt asynchronous set"]
    pub AS_GPIOD_SET: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt asynchronous clear"]
    pub AS_GPIOD_CLEAR: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt asynchronous toggle"]
    pub AS_GPIOD_TOGGLE: crate::RWRegister<u32>,
    #[doc = "GPIOE interrupt asynchronous"]
    pub AS_GPIOE_VALUE: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt asynchronous set"]
    pub AS_GPIOE_SET: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt asynchronous clear"]
    pub AS_GPIOE_CLEAR: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt asynchronous toggle"]
    pub AS_GPIOE_TOGGLE: crate::RWRegister<u32>,
    #[doc = "GPIOF interrupt asynchronous"]
    pub AS_GPIOF_VALUE: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt asynchronous set"]
    pub AS_GPIOF_SET: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt asynchronous clear"]
    pub AS_GPIOF_CLEAR: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt asynchronous toggle"]
    pub AS_GPIOF_TOGGLE: crate::RWRegister<u32>,
    _reserved7: [u8; 0x70],
    #[doc = "GPIOX interrupt asynchronous"]
    pub AS_GPIOX_VALUE: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt asynchronous set"]
    pub AS_GPIOX_SET: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt asynchronous clear"]
    pub AS_GPIOX_CLEAR: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt asynchronous toggle"]
    pub AS_GPIOX_TOGGLE: crate::RWRegister<u32>,
    #[doc = "GPIOY interrupt asynchronous"]
    pub AS_GPIOY_VALUE: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt asynchronous set"]
    pub AS_GPIOY_SET: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt asynchronous clear"]
    pub AS_GPIOY_CLEAR: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt asynchronous toggle"]
    pub AS_GPIOY_TOGGLE: crate::RWRegister<u32>,
    #[doc = "GPIOZ interrupt asynchronous"]
    pub AS_GPIOZ_VALUE: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt asynchronous set"]
    pub AS_GPIOZ_SET: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt asynchronous clear"]
    pub AS_GPIOZ_CLEAR: crate::RWRegister<u32>,
    #[doc = "GPIO interrupt asynchronous toggle"]
    pub AS_GPIOZ_TOGGLE: crate::RWRegister<u32>,
}
#[doc = "GPIO input value"]
pub mod DI_GPIOA_VALUE {
    #[doc = "GPIO input bus value, each bit represents a bus bit 0: low level presents on chip pin 1: high level presents on chip pin"]
    pub mod INPUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO input set"]
pub mod DI_GPIOA_SET {
    #[doc = "GPIO input bus value, each bit represents a bus bit 0: low level presents on chip pin 1: high level presents on chip pin"]
    pub mod INPUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO input clear"]
pub mod DI_GPIOA_CLEAR {
    #[doc = "GPIO input bus value, each bit represents a bus bit 0: low level presents on chip pin 1: high level presents on chip pin"]
    pub mod INPUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO input toggle"]
pub mod DI_GPIOA_TOGGLE {
    #[doc = "GPIO input bus value, each bit represents a bus bit 0: low level presents on chip pin 1: high level presents on chip pin"]
    pub mod INPUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIOB input"]
pub mod DI_GPIOB_VALUE {
    #[doc = "GPIO input bus value, each bit represents a bus bit 0: low level presents on chip pin 1: high level presents on chip pin"]
    pub mod INPUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO input set"]
pub mod DI_GPIOB_SET {
    #[doc = "GPIO input bus value, each bit represents a bus bit 0: low level presents on chip pin 1: high level presents on chip pin"]
    pub mod INPUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO input clear"]
pub mod DI_GPIOB_CLEAR {
    #[doc = "GPIO input bus value, each bit represents a bus bit 0: low level presents on chip pin 1: high level presents on chip pin"]
    pub mod INPUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO input toggle"]
pub mod DI_GPIOB_TOGGLE {
    #[doc = "GPIO input bus value, each bit represents a bus bit 0: low level presents on chip pin 1: high level presents on chip pin"]
    pub mod INPUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIOC input"]
pub mod DI_GPIOC_VALUE {
    #[doc = "GPIO input bus value, each bit represents a bus bit 0: low level presents on chip pin 1: high level presents on chip pin"]
    pub mod INPUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO input set"]
pub mod DI_GPIOC_SET {
    #[doc = "GPIO input bus value, each bit represents a bus bit 0: low level presents on chip pin 1: high level presents on chip pin"]
    pub mod INPUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO input clear"]
pub mod DI_GPIOC_CLEAR {
    #[doc = "GPIO input bus value, each bit represents a bus bit 0: low level presents on chip pin 1: high level presents on chip pin"]
    pub mod INPUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO input toggle"]
pub mod DI_GPIOC_TOGGLE {
    #[doc = "GPIO input bus value, each bit represents a bus bit 0: low level presents on chip pin 1: high level presents on chip pin"]
    pub mod INPUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIOD input"]
pub mod DI_GPIOD_VALUE {
    #[doc = "GPIO input bus value, each bit represents a bus bit 0: low level presents on chip pin 1: high level presents on chip pin"]
    pub mod INPUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO input set"]
pub mod DI_GPIOD_SET {
    #[doc = "GPIO input bus value, each bit represents a bus bit 0: low level presents on chip pin 1: high level presents on chip pin"]
    pub mod INPUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO input clear"]
pub mod DI_GPIOD_CLEAR {
    #[doc = "GPIO input bus value, each bit represents a bus bit 0: low level presents on chip pin 1: high level presents on chip pin"]
    pub mod INPUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO input toggle"]
pub mod DI_GPIOD_TOGGLE {
    #[doc = "GPIO input bus value, each bit represents a bus bit 0: low level presents on chip pin 1: high level presents on chip pin"]
    pub mod INPUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIOE input"]
pub mod DI_GPIOE_VALUE {
    #[doc = "GPIO input bus value, each bit represents a bus bit 0: low level presents on chip pin 1: high level presents on chip pin"]
    pub mod INPUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO input set"]
pub mod DI_GPIOE_SET {
    #[doc = "GPIO input bus value, each bit represents a bus bit 0: low level presents on chip pin 1: high level presents on chip pin"]
    pub mod INPUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO input clear"]
pub mod DI_GPIOE_CLEAR {
    #[doc = "GPIO input bus value, each bit represents a bus bit 0: low level presents on chip pin 1: high level presents on chip pin"]
    pub mod INPUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO input toggle"]
pub mod DI_GPIOE_TOGGLE {
    #[doc = "GPIO input bus value, each bit represents a bus bit 0: low level presents on chip pin 1: high level presents on chip pin"]
    pub mod INPUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIOF input"]
pub mod DI_GPIOF_VALUE {
    #[doc = "GPIO input bus value, each bit represents a bus bit 0: low level presents on chip pin 1: high level presents on chip pin"]
    pub mod INPUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO input set"]
pub mod DI_GPIOF_SET {
    #[doc = "GPIO input bus value, each bit represents a bus bit 0: low level presents on chip pin 1: high level presents on chip pin"]
    pub mod INPUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO input clear"]
pub mod DI_GPIOF_CLEAR {
    #[doc = "GPIO input bus value, each bit represents a bus bit 0: low level presents on chip pin 1: high level presents on chip pin"]
    pub mod INPUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO input toggle"]
pub mod DI_GPIOF_TOGGLE {
    #[doc = "GPIO input bus value, each bit represents a bus bit 0: low level presents on chip pin 1: high level presents on chip pin"]
    pub mod INPUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIOX input"]
pub mod DI_GPIOX_VALUE {
    #[doc = "GPIO input bus value, each bit represents a bus bit 0: low level presents on chip pin 1: high level presents on chip pin"]
    pub mod INPUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO input set"]
pub mod DI_GPIOX_SET {
    #[doc = "GPIO input bus value, each bit represents a bus bit 0: low level presents on chip pin 1: high level presents on chip pin"]
    pub mod INPUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO input clear"]
pub mod DI_GPIOX_CLEAR {
    #[doc = "GPIO input bus value, each bit represents a bus bit 0: low level presents on chip pin 1: high level presents on chip pin"]
    pub mod INPUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO input toggle"]
pub mod DI_GPIOX_TOGGLE {
    #[doc = "GPIO input bus value, each bit represents a bus bit 0: low level presents on chip pin 1: high level presents on chip pin"]
    pub mod INPUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIOY input"]
pub mod DI_GPIOY_VALUE {
    #[doc = "GPIO input bus value, each bit represents a bus bit 0: low level presents on chip pin 1: high level presents on chip pin"]
    pub mod INPUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO input set"]
pub mod DI_GPIOY_SET {
    #[doc = "GPIO input bus value, each bit represents a bus bit 0: low level presents on chip pin 1: high level presents on chip pin"]
    pub mod INPUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO input clear"]
pub mod DI_GPIOY_CLEAR {
    #[doc = "GPIO input bus value, each bit represents a bus bit 0: low level presents on chip pin 1: high level presents on chip pin"]
    pub mod INPUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO input toggle"]
pub mod DI_GPIOY_TOGGLE {
    #[doc = "GPIO input bus value, each bit represents a bus bit 0: low level presents on chip pin 1: high level presents on chip pin"]
    pub mod INPUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIOZ input"]
pub mod DI_GPIOZ_VALUE {
    #[doc = "GPIO input bus value, each bit represents a bus bit 0: low level presents on chip pin 1: high level presents on chip pin"]
    pub mod INPUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO input set"]
pub mod DI_GPIOZ_SET {
    #[doc = "GPIO input bus value, each bit represents a bus bit 0: low level presents on chip pin 1: high level presents on chip pin"]
    pub mod INPUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO input clear"]
pub mod DI_GPIOZ_CLEAR {
    #[doc = "GPIO input bus value, each bit represents a bus bit 0: low level presents on chip pin 1: high level presents on chip pin"]
    pub mod INPUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO input toggle"]
pub mod DI_GPIOZ_TOGGLE {
    #[doc = "GPIO input bus value, each bit represents a bus bit 0: low level presents on chip pin 1: high level presents on chip pin"]
    pub mod INPUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO output value"]
pub mod DO_GPIOA_VALUE {
    #[doc = "GPIO output register value, each bit represents a bus bit 0: chip pin output low level when direction is output 1: chip pin output high level when direction is output"]
    pub mod OUTPUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO output set"]
pub mod DO_GPIOA_SET {
    #[doc = "GPIO output register value, each bit represents a bus bit 0: chip pin output low level when direction is output 1: chip pin output high level when direction is output"]
    pub mod OUTPUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO output clear"]
pub mod DO_GPIOA_CLEAR {
    #[doc = "GPIO output register value, each bit represents a bus bit 0: chip pin output low level when direction is output 1: chip pin output high level when direction is output"]
    pub mod OUTPUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO output toggle"]
pub mod DO_GPIOA_TOGGLE {
    #[doc = "GPIO output register value, each bit represents a bus bit 0: chip pin output low level when direction is output 1: chip pin output high level when direction is output"]
    pub mod OUTPUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIOB output"]
pub mod DO_GPIOB_VALUE {
    #[doc = "GPIO output register value, each bit represents a bus bit 0: chip pin output low level when direction is output 1: chip pin output high level when direction is output"]
    pub mod OUTPUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO output set"]
pub mod DO_GPIOB_SET {
    #[doc = "GPIO output register value, each bit represents a bus bit 0: chip pin output low level when direction is output 1: chip pin output high level when direction is output"]
    pub mod OUTPUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO output clear"]
pub mod DO_GPIOB_CLEAR {
    #[doc = "GPIO output register value, each bit represents a bus bit 0: chip pin output low level when direction is output 1: chip pin output high level when direction is output"]
    pub mod OUTPUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO output toggle"]
pub mod DO_GPIOB_TOGGLE {
    #[doc = "GPIO output register value, each bit represents a bus bit 0: chip pin output low level when direction is output 1: chip pin output high level when direction is output"]
    pub mod OUTPUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIOC output"]
pub mod DO_GPIOC_VALUE {
    #[doc = "GPIO output register value, each bit represents a bus bit 0: chip pin output low level when direction is output 1: chip pin output high level when direction is output"]
    pub mod OUTPUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO output set"]
pub mod DO_GPIOC_SET {
    #[doc = "GPIO output register value, each bit represents a bus bit 0: chip pin output low level when direction is output 1: chip pin output high level when direction is output"]
    pub mod OUTPUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO output clear"]
pub mod DO_GPIOC_CLEAR {
    #[doc = "GPIO output register value, each bit represents a bus bit 0: chip pin output low level when direction is output 1: chip pin output high level when direction is output"]
    pub mod OUTPUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO output toggle"]
pub mod DO_GPIOC_TOGGLE {
    #[doc = "GPIO output register value, each bit represents a bus bit 0: chip pin output low level when direction is output 1: chip pin output high level when direction is output"]
    pub mod OUTPUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIOD output"]
pub mod DO_GPIOD_VALUE {
    #[doc = "GPIO output register value, each bit represents a bus bit 0: chip pin output low level when direction is output 1: chip pin output high level when direction is output"]
    pub mod OUTPUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO output set"]
pub mod DO_GPIOD_SET {
    #[doc = "GPIO output register value, each bit represents a bus bit 0: chip pin output low level when direction is output 1: chip pin output high level when direction is output"]
    pub mod OUTPUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO output clear"]
pub mod DO_GPIOD_CLEAR {
    #[doc = "GPIO output register value, each bit represents a bus bit 0: chip pin output low level when direction is output 1: chip pin output high level when direction is output"]
    pub mod OUTPUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO output toggle"]
pub mod DO_GPIOD_TOGGLE {
    #[doc = "GPIO output register value, each bit represents a bus bit 0: chip pin output low level when direction is output 1: chip pin output high level when direction is output"]
    pub mod OUTPUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIOE output"]
pub mod DO_GPIOE_VALUE {
    #[doc = "GPIO output register value, each bit represents a bus bit 0: chip pin output low level when direction is output 1: chip pin output high level when direction is output"]
    pub mod OUTPUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO output set"]
pub mod DO_GPIOE_SET {
    #[doc = "GPIO output register value, each bit represents a bus bit 0: chip pin output low level when direction is output 1: chip pin output high level when direction is output"]
    pub mod OUTPUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO output clear"]
pub mod DO_GPIOE_CLEAR {
    #[doc = "GPIO output register value, each bit represents a bus bit 0: chip pin output low level when direction is output 1: chip pin output high level when direction is output"]
    pub mod OUTPUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO output toggle"]
pub mod DO_GPIOE_TOGGLE {
    #[doc = "GPIO output register value, each bit represents a bus bit 0: chip pin output low level when direction is output 1: chip pin output high level when direction is output"]
    pub mod OUTPUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIOF output"]
pub mod DO_GPIOF_VALUE {
    #[doc = "GPIO output register value, each bit represents a bus bit 0: chip pin output low level when direction is output 1: chip pin output high level when direction is output"]
    pub mod OUTPUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO output set"]
pub mod DO_GPIOF_SET {
    #[doc = "GPIO output register value, each bit represents a bus bit 0: chip pin output low level when direction is output 1: chip pin output high level when direction is output"]
    pub mod OUTPUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO output clear"]
pub mod DO_GPIOF_CLEAR {
    #[doc = "GPIO output register value, each bit represents a bus bit 0: chip pin output low level when direction is output 1: chip pin output high level when direction is output"]
    pub mod OUTPUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO output toggle"]
pub mod DO_GPIOF_TOGGLE {
    #[doc = "GPIO output register value, each bit represents a bus bit 0: chip pin output low level when direction is output 1: chip pin output high level when direction is output"]
    pub mod OUTPUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIOX output"]
pub mod DO_GPIOX_VALUE {
    #[doc = "GPIO output register value, each bit represents a bus bit 0: chip pin output low level when direction is output 1: chip pin output high level when direction is output"]
    pub mod OUTPUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO output set"]
pub mod DO_GPIOX_SET {
    #[doc = "GPIO output register value, each bit represents a bus bit 0: chip pin output low level when direction is output 1: chip pin output high level when direction is output"]
    pub mod OUTPUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO output clear"]
pub mod DO_GPIOX_CLEAR {
    #[doc = "GPIO output register value, each bit represents a bus bit 0: chip pin output low level when direction is output 1: chip pin output high level when direction is output"]
    pub mod OUTPUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO output toggle"]
pub mod DO_GPIOX_TOGGLE {
    #[doc = "GPIO output register value, each bit represents a bus bit 0: chip pin output low level when direction is output 1: chip pin output high level when direction is output"]
    pub mod OUTPUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIOY output"]
pub mod DO_GPIOY_VALUE {
    #[doc = "GPIO output register value, each bit represents a bus bit 0: chip pin output low level when direction is output 1: chip pin output high level when direction is output"]
    pub mod OUTPUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO output set"]
pub mod DO_GPIOY_SET {
    #[doc = "GPIO output register value, each bit represents a bus bit 0: chip pin output low level when direction is output 1: chip pin output high level when direction is output"]
    pub mod OUTPUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO output clear"]
pub mod DO_GPIOY_CLEAR {
    #[doc = "GPIO output register value, each bit represents a bus bit 0: chip pin output low level when direction is output 1: chip pin output high level when direction is output"]
    pub mod OUTPUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO output toggle"]
pub mod DO_GPIOY_TOGGLE {
    #[doc = "GPIO output register value, each bit represents a bus bit 0: chip pin output low level when direction is output 1: chip pin output high level when direction is output"]
    pub mod OUTPUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIOZ output"]
pub mod DO_GPIOZ_VALUE {
    #[doc = "GPIO output register value, each bit represents a bus bit 0: chip pin output low level when direction is output 1: chip pin output high level when direction is output"]
    pub mod OUTPUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO output set"]
pub mod DO_GPIOZ_SET {
    #[doc = "GPIO output register value, each bit represents a bus bit 0: chip pin output low level when direction is output 1: chip pin output high level when direction is output"]
    pub mod OUTPUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO output clear"]
pub mod DO_GPIOZ_CLEAR {
    #[doc = "GPIO output register value, each bit represents a bus bit 0: chip pin output low level when direction is output 1: chip pin output high level when direction is output"]
    pub mod OUTPUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO output toggle"]
pub mod DO_GPIOZ_TOGGLE {
    #[doc = "GPIO output register value, each bit represents a bus bit 0: chip pin output low level when direction is output 1: chip pin output high level when direction is output"]
    pub mod OUTPUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO direction value"]
pub mod OE_GPIOA_VALUE {
    #[doc = "GPIO direction, each bit represents a bus bit 0: input 1: output"]
    pub mod DIRECTION {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO direction set"]
pub mod OE_GPIOA_SET {
    #[doc = "GPIO direction, each bit represents a bus bit 0: input 1: output"]
    pub mod DIRECTION {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO direction clear"]
pub mod OE_GPIOA_CLEAR {
    #[doc = "GPIO direction, each bit represents a bus bit 0: input 1: output"]
    pub mod DIRECTION {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO direction toggle"]
pub mod OE_GPIOA_TOGGLE {
    #[doc = "GPIO direction, each bit represents a bus bit 0: input 1: output"]
    pub mod DIRECTION {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIOB direction"]
pub mod OE_GPIOB_VALUE {
    #[doc = "GPIO direction, each bit represents a bus bit 0: input 1: output"]
    pub mod DIRECTION {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO direction set"]
pub mod OE_GPIOB_SET {
    #[doc = "GPIO direction, each bit represents a bus bit 0: input 1: output"]
    pub mod DIRECTION {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO direction clear"]
pub mod OE_GPIOB_CLEAR {
    #[doc = "GPIO direction, each bit represents a bus bit 0: input 1: output"]
    pub mod DIRECTION {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO direction toggle"]
pub mod OE_GPIOB_TOGGLE {
    #[doc = "GPIO direction, each bit represents a bus bit 0: input 1: output"]
    pub mod DIRECTION {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIOC direction"]
pub mod OE_GPIOC_VALUE {
    #[doc = "GPIO direction, each bit represents a bus bit 0: input 1: output"]
    pub mod DIRECTION {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO direction set"]
pub mod OE_GPIOC_SET {
    #[doc = "GPIO direction, each bit represents a bus bit 0: input 1: output"]
    pub mod DIRECTION {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO direction clear"]
pub mod OE_GPIOC_CLEAR {
    #[doc = "GPIO direction, each bit represents a bus bit 0: input 1: output"]
    pub mod DIRECTION {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO direction toggle"]
pub mod OE_GPIOC_TOGGLE {
    #[doc = "GPIO direction, each bit represents a bus bit 0: input 1: output"]
    pub mod DIRECTION {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIOD direction"]
pub mod OE_GPIOD_VALUE {
    #[doc = "GPIO direction, each bit represents a bus bit 0: input 1: output"]
    pub mod DIRECTION {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO direction set"]
pub mod OE_GPIOD_SET {
    #[doc = "GPIO direction, each bit represents a bus bit 0: input 1: output"]
    pub mod DIRECTION {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO direction clear"]
pub mod OE_GPIOD_CLEAR {
    #[doc = "GPIO direction, each bit represents a bus bit 0: input 1: output"]
    pub mod DIRECTION {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO direction toggle"]
pub mod OE_GPIOD_TOGGLE {
    #[doc = "GPIO direction, each bit represents a bus bit 0: input 1: output"]
    pub mod DIRECTION {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIOE direction"]
pub mod OE_GPIOE_VALUE {
    #[doc = "GPIO direction, each bit represents a bus bit 0: input 1: output"]
    pub mod DIRECTION {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO direction set"]
pub mod OE_GPIOE_SET {
    #[doc = "GPIO direction, each bit represents a bus bit 0: input 1: output"]
    pub mod DIRECTION {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO direction clear"]
pub mod OE_GPIOE_CLEAR {
    #[doc = "GPIO direction, each bit represents a bus bit 0: input 1: output"]
    pub mod DIRECTION {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO direction toggle"]
pub mod OE_GPIOE_TOGGLE {
    #[doc = "GPIO direction, each bit represents a bus bit 0: input 1: output"]
    pub mod DIRECTION {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIOF direction"]
pub mod OE_GPIOF_VALUE {
    #[doc = "GPIO direction, each bit represents a bus bit 0: input 1: output"]
    pub mod DIRECTION {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO direction set"]
pub mod OE_GPIOF_SET {
    #[doc = "GPIO direction, each bit represents a bus bit 0: input 1: output"]
    pub mod DIRECTION {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO direction clear"]
pub mod OE_GPIOF_CLEAR {
    #[doc = "GPIO direction, each bit represents a bus bit 0: input 1: output"]
    pub mod DIRECTION {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO direction toggle"]
pub mod OE_GPIOF_TOGGLE {
    #[doc = "GPIO direction, each bit represents a bus bit 0: input 1: output"]
    pub mod DIRECTION {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIOX direction"]
pub mod OE_GPIOX_VALUE {
    #[doc = "GPIO direction, each bit represents a bus bit 0: input 1: output"]
    pub mod DIRECTION {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO direction set"]
pub mod OE_GPIOX_SET {
    #[doc = "GPIO direction, each bit represents a bus bit 0: input 1: output"]
    pub mod DIRECTION {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO direction clear"]
pub mod OE_GPIOX_CLEAR {
    #[doc = "GPIO direction, each bit represents a bus bit 0: input 1: output"]
    pub mod DIRECTION {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO direction toggle"]
pub mod OE_GPIOX_TOGGLE {
    #[doc = "GPIO direction, each bit represents a bus bit 0: input 1: output"]
    pub mod DIRECTION {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIOY direction"]
pub mod OE_GPIOY_VALUE {
    #[doc = "GPIO direction, each bit represents a bus bit 0: input 1: output"]
    pub mod DIRECTION {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO direction set"]
pub mod OE_GPIOY_SET {
    #[doc = "GPIO direction, each bit represents a bus bit 0: input 1: output"]
    pub mod DIRECTION {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO direction clear"]
pub mod OE_GPIOY_CLEAR {
    #[doc = "GPIO direction, each bit represents a bus bit 0: input 1: output"]
    pub mod DIRECTION {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO direction toggle"]
pub mod OE_GPIOY_TOGGLE {
    #[doc = "GPIO direction, each bit represents a bus bit 0: input 1: output"]
    pub mod DIRECTION {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIOZ direction"]
pub mod OE_GPIOZ_VALUE {
    #[doc = "GPIO direction, each bit represents a bus bit 0: input 1: output"]
    pub mod DIRECTION {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO direction set"]
pub mod OE_GPIOZ_SET {
    #[doc = "GPIO direction, each bit represents a bus bit 0: input 1: output"]
    pub mod DIRECTION {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO direction clear"]
pub mod OE_GPIOZ_CLEAR {
    #[doc = "GPIO direction, each bit represents a bus bit 0: input 1: output"]
    pub mod DIRECTION {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO direction toggle"]
pub mod OE_GPIOZ_TOGGLE {
    #[doc = "GPIO direction, each bit represents a bus bit 0: input 1: output"]
    pub mod DIRECTION {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt flag value"]
pub mod IF_GPIOA_VALUE {
    #[doc = "GPIO interrupt flag, write 1 to clear this flag 0: no irq 1: irq pending"]
    pub mod IRQ_FLAG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt flag set"]
pub mod IF_GPIOA_SET {
    #[doc = "GPIO interrupt flag, write 1 to clear this flag 0: no irq 1: irq pending"]
    pub mod IRQ_FLAG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt flag clear"]
pub mod IF_GPIOA_CLEAR {
    #[doc = "GPIO interrupt flag, write 1 to clear this flag 0: no irq 1: irq pending"]
    pub mod IRQ_FLAG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt flag toggle"]
pub mod IF_GPIOA_TOGGLE {
    #[doc = "GPIO interrupt flag, write 1 to clear this flag 0: no irq 1: irq pending"]
    pub mod IRQ_FLAG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIOB interrupt flag"]
pub mod IF_GPIOB_VALUE {
    #[doc = "GPIO interrupt flag, write 1 to clear this flag 0: no irq 1: irq pending"]
    pub mod IRQ_FLAG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt flag set"]
pub mod IF_GPIOB_SET {
    #[doc = "GPIO interrupt flag, write 1 to clear this flag 0: no irq 1: irq pending"]
    pub mod IRQ_FLAG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt flag clear"]
pub mod IF_GPIOB_CLEAR {
    #[doc = "GPIO interrupt flag, write 1 to clear this flag 0: no irq 1: irq pending"]
    pub mod IRQ_FLAG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt flag toggle"]
pub mod IF_GPIOB_TOGGLE {
    #[doc = "GPIO interrupt flag, write 1 to clear this flag 0: no irq 1: irq pending"]
    pub mod IRQ_FLAG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIOC interrupt flag"]
pub mod IF_GPIOC_VALUE {
    #[doc = "GPIO interrupt flag, write 1 to clear this flag 0: no irq 1: irq pending"]
    pub mod IRQ_FLAG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt flag set"]
pub mod IF_GPIOC_SET {
    #[doc = "GPIO interrupt flag, write 1 to clear this flag 0: no irq 1: irq pending"]
    pub mod IRQ_FLAG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt flag clear"]
pub mod IF_GPIOC_CLEAR {
    #[doc = "GPIO interrupt flag, write 1 to clear this flag 0: no irq 1: irq pending"]
    pub mod IRQ_FLAG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt flag toggle"]
pub mod IF_GPIOC_TOGGLE {
    #[doc = "GPIO interrupt flag, write 1 to clear this flag 0: no irq 1: irq pending"]
    pub mod IRQ_FLAG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIOD interrupt flag"]
pub mod IF_GPIOD_VALUE {
    #[doc = "GPIO interrupt flag, write 1 to clear this flag 0: no irq 1: irq pending"]
    pub mod IRQ_FLAG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt flag set"]
pub mod IF_GPIOD_SET {
    #[doc = "GPIO interrupt flag, write 1 to clear this flag 0: no irq 1: irq pending"]
    pub mod IRQ_FLAG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt flag clear"]
pub mod IF_GPIOD_CLEAR {
    #[doc = "GPIO interrupt flag, write 1 to clear this flag 0: no irq 1: irq pending"]
    pub mod IRQ_FLAG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt flag toggle"]
pub mod IF_GPIOD_TOGGLE {
    #[doc = "GPIO interrupt flag, write 1 to clear this flag 0: no irq 1: irq pending"]
    pub mod IRQ_FLAG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIOE interrupt flag"]
pub mod IF_GPIOE_VALUE {
    #[doc = "GPIO interrupt flag, write 1 to clear this flag 0: no irq 1: irq pending"]
    pub mod IRQ_FLAG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt flag set"]
pub mod IF_GPIOE_SET {
    #[doc = "GPIO interrupt flag, write 1 to clear this flag 0: no irq 1: irq pending"]
    pub mod IRQ_FLAG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt flag clear"]
pub mod IF_GPIOE_CLEAR {
    #[doc = "GPIO interrupt flag, write 1 to clear this flag 0: no irq 1: irq pending"]
    pub mod IRQ_FLAG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt flag toggle"]
pub mod IF_GPIOE_TOGGLE {
    #[doc = "GPIO interrupt flag, write 1 to clear this flag 0: no irq 1: irq pending"]
    pub mod IRQ_FLAG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIOF interrupt flag"]
pub mod IF_GPIOF_VALUE {
    #[doc = "GPIO interrupt flag, write 1 to clear this flag 0: no irq 1: irq pending"]
    pub mod IRQ_FLAG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt flag set"]
pub mod IF_GPIOF_SET {
    #[doc = "GPIO interrupt flag, write 1 to clear this flag 0: no irq 1: irq pending"]
    pub mod IRQ_FLAG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt flag clear"]
pub mod IF_GPIOF_CLEAR {
    #[doc = "GPIO interrupt flag, write 1 to clear this flag 0: no irq 1: irq pending"]
    pub mod IRQ_FLAG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt flag toggle"]
pub mod IF_GPIOF_TOGGLE {
    #[doc = "GPIO interrupt flag, write 1 to clear this flag 0: no irq 1: irq pending"]
    pub mod IRQ_FLAG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIOX interrupt flag"]
pub mod IF_GPIOX_VALUE {
    #[doc = "GPIO interrupt flag, write 1 to clear this flag 0: no irq 1: irq pending"]
    pub mod IRQ_FLAG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt flag set"]
pub mod IF_GPIOX_SET {
    #[doc = "GPIO interrupt flag, write 1 to clear this flag 0: no irq 1: irq pending"]
    pub mod IRQ_FLAG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt flag clear"]
pub mod IF_GPIOX_CLEAR {
    #[doc = "GPIO interrupt flag, write 1 to clear this flag 0: no irq 1: irq pending"]
    pub mod IRQ_FLAG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt flag toggle"]
pub mod IF_GPIOX_TOGGLE {
    #[doc = "GPIO interrupt flag, write 1 to clear this flag 0: no irq 1: irq pending"]
    pub mod IRQ_FLAG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIOY interrupt flag"]
pub mod IF_GPIOY_VALUE {
    #[doc = "GPIO interrupt flag, write 1 to clear this flag 0: no irq 1: irq pending"]
    pub mod IRQ_FLAG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt flag set"]
pub mod IF_GPIOY_SET {
    #[doc = "GPIO interrupt flag, write 1 to clear this flag 0: no irq 1: irq pending"]
    pub mod IRQ_FLAG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt flag clear"]
pub mod IF_GPIOY_CLEAR {
    #[doc = "GPIO interrupt flag, write 1 to clear this flag 0: no irq 1: irq pending"]
    pub mod IRQ_FLAG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt flag toggle"]
pub mod IF_GPIOY_TOGGLE {
    #[doc = "GPIO interrupt flag, write 1 to clear this flag 0: no irq 1: irq pending"]
    pub mod IRQ_FLAG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIOZ interrupt flag"]
pub mod IF_GPIOZ_VALUE {
    #[doc = "GPIO interrupt flag, write 1 to clear this flag 0: no irq 1: irq pending"]
    pub mod IRQ_FLAG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt flag set"]
pub mod IF_GPIOZ_SET {
    #[doc = "GPIO interrupt flag, write 1 to clear this flag 0: no irq 1: irq pending"]
    pub mod IRQ_FLAG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt flag clear"]
pub mod IF_GPIOZ_CLEAR {
    #[doc = "GPIO interrupt flag, write 1 to clear this flag 0: no irq 1: irq pending"]
    pub mod IRQ_FLAG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt flag toggle"]
pub mod IF_GPIOZ_TOGGLE {
    #[doc = "GPIO interrupt flag, write 1 to clear this flag 0: no irq 1: irq pending"]
    pub mod IRQ_FLAG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt enable value"]
pub mod IE_GPIOA_VALUE {
    #[doc = "GPIO interrupt enable, each bit represents a bus bit 0: irq is disabled 1: irq is enable"]
    pub mod IRQ_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt enable set"]
pub mod IE_GPIOA_SET {
    #[doc = "GPIO interrupt enable, each bit represents a bus bit 0: irq is disabled 1: irq is enable"]
    pub mod IRQ_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt enable clear"]
pub mod IE_GPIOA_CLEAR {
    #[doc = "GPIO interrupt enable, each bit represents a bus bit 0: irq is disabled 1: irq is enable"]
    pub mod IRQ_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt enable toggle"]
pub mod IE_GPIOA_TOGGLE {
    #[doc = "GPIO interrupt enable, each bit represents a bus bit 0: irq is disabled 1: irq is enable"]
    pub mod IRQ_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIOB interrupt enable"]
pub mod IE_GPIOB_VALUE {
    #[doc = "GPIO interrupt enable, each bit represents a bus bit 0: irq is disabled 1: irq is enable"]
    pub mod IRQ_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt enable set"]
pub mod IE_GPIOB_SET {
    #[doc = "GPIO interrupt enable, each bit represents a bus bit 0: irq is disabled 1: irq is enable"]
    pub mod IRQ_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt enable clear"]
pub mod IE_GPIOB_CLEAR {
    #[doc = "GPIO interrupt enable, each bit represents a bus bit 0: irq is disabled 1: irq is enable"]
    pub mod IRQ_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt enable toggle"]
pub mod IE_GPIOB_TOGGLE {
    #[doc = "GPIO interrupt enable, each bit represents a bus bit 0: irq is disabled 1: irq is enable"]
    pub mod IRQ_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIOC interrupt enable"]
pub mod IE_GPIOC_VALUE {
    #[doc = "GPIO interrupt enable, each bit represents a bus bit 0: irq is disabled 1: irq is enable"]
    pub mod IRQ_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt enable set"]
pub mod IE_GPIOC_SET {
    #[doc = "GPIO interrupt enable, each bit represents a bus bit 0: irq is disabled 1: irq is enable"]
    pub mod IRQ_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt enable clear"]
pub mod IE_GPIOC_CLEAR {
    #[doc = "GPIO interrupt enable, each bit represents a bus bit 0: irq is disabled 1: irq is enable"]
    pub mod IRQ_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt enable toggle"]
pub mod IE_GPIOC_TOGGLE {
    #[doc = "GPIO interrupt enable, each bit represents a bus bit 0: irq is disabled 1: irq is enable"]
    pub mod IRQ_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIOD interrupt enable"]
pub mod IE_GPIOD_VALUE {
    #[doc = "GPIO interrupt enable, each bit represents a bus bit 0: irq is disabled 1: irq is enable"]
    pub mod IRQ_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt enable set"]
pub mod IE_GPIOD_SET {
    #[doc = "GPIO interrupt enable, each bit represents a bus bit 0: irq is disabled 1: irq is enable"]
    pub mod IRQ_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt enable clear"]
pub mod IE_GPIOD_CLEAR {
    #[doc = "GPIO interrupt enable, each bit represents a bus bit 0: irq is disabled 1: irq is enable"]
    pub mod IRQ_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt enable toggle"]
pub mod IE_GPIOD_TOGGLE {
    #[doc = "GPIO interrupt enable, each bit represents a bus bit 0: irq is disabled 1: irq is enable"]
    pub mod IRQ_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIOE interrupt enable"]
pub mod IE_GPIOE_VALUE {
    #[doc = "GPIO interrupt enable, each bit represents a bus bit 0: irq is disabled 1: irq is enable"]
    pub mod IRQ_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt enable set"]
pub mod IE_GPIOE_SET {
    #[doc = "GPIO interrupt enable, each bit represents a bus bit 0: irq is disabled 1: irq is enable"]
    pub mod IRQ_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt enable clear"]
pub mod IE_GPIOE_CLEAR {
    #[doc = "GPIO interrupt enable, each bit represents a bus bit 0: irq is disabled 1: irq is enable"]
    pub mod IRQ_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt enable toggle"]
pub mod IE_GPIOE_TOGGLE {
    #[doc = "GPIO interrupt enable, each bit represents a bus bit 0: irq is disabled 1: irq is enable"]
    pub mod IRQ_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIOF interrupt enable"]
pub mod IE_GPIOF_VALUE {
    #[doc = "GPIO interrupt enable, each bit represents a bus bit 0: irq is disabled 1: irq is enable"]
    pub mod IRQ_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt enable set"]
pub mod IE_GPIOF_SET {
    #[doc = "GPIO interrupt enable, each bit represents a bus bit 0: irq is disabled 1: irq is enable"]
    pub mod IRQ_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt enable clear"]
pub mod IE_GPIOF_CLEAR {
    #[doc = "GPIO interrupt enable, each bit represents a bus bit 0: irq is disabled 1: irq is enable"]
    pub mod IRQ_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt enable toggle"]
pub mod IE_GPIOF_TOGGLE {
    #[doc = "GPIO interrupt enable, each bit represents a bus bit 0: irq is disabled 1: irq is enable"]
    pub mod IRQ_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIOX interrupt enable"]
pub mod IE_GPIOX_VALUE {
    #[doc = "GPIO interrupt enable, each bit represents a bus bit 0: irq is disabled 1: irq is enable"]
    pub mod IRQ_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt enable set"]
pub mod IE_GPIOX_SET {
    #[doc = "GPIO interrupt enable, each bit represents a bus bit 0: irq is disabled 1: irq is enable"]
    pub mod IRQ_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt enable clear"]
pub mod IE_GPIOX_CLEAR {
    #[doc = "GPIO interrupt enable, each bit represents a bus bit 0: irq is disabled 1: irq is enable"]
    pub mod IRQ_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt enable toggle"]
pub mod IE_GPIOX_TOGGLE {
    #[doc = "GPIO interrupt enable, each bit represents a bus bit 0: irq is disabled 1: irq is enable"]
    pub mod IRQ_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIOY interrupt enable"]
pub mod IE_GPIOY_VALUE {
    #[doc = "GPIO interrupt enable, each bit represents a bus bit 0: irq is disabled 1: irq is enable"]
    pub mod IRQ_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt enable set"]
pub mod IE_GPIOY_SET {
    #[doc = "GPIO interrupt enable, each bit represents a bus bit 0: irq is disabled 1: irq is enable"]
    pub mod IRQ_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt enable clear"]
pub mod IE_GPIOY_CLEAR {
    #[doc = "GPIO interrupt enable, each bit represents a bus bit 0: irq is disabled 1: irq is enable"]
    pub mod IRQ_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt enable toggle"]
pub mod IE_GPIOY_TOGGLE {
    #[doc = "GPIO interrupt enable, each bit represents a bus bit 0: irq is disabled 1: irq is enable"]
    pub mod IRQ_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIOZ interrupt enable"]
pub mod IE_GPIOZ_VALUE {
    #[doc = "GPIO interrupt enable, each bit represents a bus bit 0: irq is disabled 1: irq is enable"]
    pub mod IRQ_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt enable set"]
pub mod IE_GPIOZ_SET {
    #[doc = "GPIO interrupt enable, each bit represents a bus bit 0: irq is disabled 1: irq is enable"]
    pub mod IRQ_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt enable clear"]
pub mod IE_GPIOZ_CLEAR {
    #[doc = "GPIO interrupt enable, each bit represents a bus bit 0: irq is disabled 1: irq is enable"]
    pub mod IRQ_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt enable toggle"]
pub mod IE_GPIOZ_TOGGLE {
    #[doc = "GPIO interrupt enable, each bit represents a bus bit 0: irq is disabled 1: irq is enable"]
    pub mod IRQ_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt polarity value"]
pub mod PL_GPIOA_VALUE {
    #[doc = "GPIO interrupt polarity, each bit represents a bus bit 0: irq is high level or rising edge 1: irq is low level or falling edge"]
    pub mod IRQ_POL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt polarity set"]
pub mod PL_GPIOA_SET {
    #[doc = "GPIO interrupt polarity, each bit represents a bus bit 0: irq is high level or rising edge 1: irq is low level or falling edge"]
    pub mod IRQ_POL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt polarity clear"]
pub mod PL_GPIOA_CLEAR {
    #[doc = "GPIO interrupt polarity, each bit represents a bus bit 0: irq is high level or rising edge 1: irq is low level or falling edge"]
    pub mod IRQ_POL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt polarity toggle"]
pub mod PL_GPIOA_TOGGLE {
    #[doc = "GPIO interrupt polarity, each bit represents a bus bit 0: irq is high level or rising edge 1: irq is low level or falling edge"]
    pub mod IRQ_POL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIOB interrupt polarity"]
pub mod PL_GPIOB_VALUE {
    #[doc = "GPIO interrupt polarity, each bit represents a bus bit 0: irq is high level or rising edge 1: irq is low level or falling edge"]
    pub mod IRQ_POL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt polarity set"]
pub mod PL_GPIOB_SET {
    #[doc = "GPIO interrupt polarity, each bit represents a bus bit 0: irq is high level or rising edge 1: irq is low level or falling edge"]
    pub mod IRQ_POL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt polarity clear"]
pub mod PL_GPIOB_CLEAR {
    #[doc = "GPIO interrupt polarity, each bit represents a bus bit 0: irq is high level or rising edge 1: irq is low level or falling edge"]
    pub mod IRQ_POL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt polarity toggle"]
pub mod PL_GPIOB_TOGGLE {
    #[doc = "GPIO interrupt polarity, each bit represents a bus bit 0: irq is high level or rising edge 1: irq is low level or falling edge"]
    pub mod IRQ_POL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIOC interrupt polarity"]
pub mod PL_GPIOC_VALUE {
    #[doc = "GPIO interrupt polarity, each bit represents a bus bit 0: irq is high level or rising edge 1: irq is low level or falling edge"]
    pub mod IRQ_POL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt polarity set"]
pub mod PL_GPIOC_SET {
    #[doc = "GPIO interrupt polarity, each bit represents a bus bit 0: irq is high level or rising edge 1: irq is low level or falling edge"]
    pub mod IRQ_POL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt polarity clear"]
pub mod PL_GPIOC_CLEAR {
    #[doc = "GPIO interrupt polarity, each bit represents a bus bit 0: irq is high level or rising edge 1: irq is low level or falling edge"]
    pub mod IRQ_POL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt polarity toggle"]
pub mod PL_GPIOC_TOGGLE {
    #[doc = "GPIO interrupt polarity, each bit represents a bus bit 0: irq is high level or rising edge 1: irq is low level or falling edge"]
    pub mod IRQ_POL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIOD interrupt polarity"]
pub mod PL_GPIOD_VALUE {
    #[doc = "GPIO interrupt polarity, each bit represents a bus bit 0: irq is high level or rising edge 1: irq is low level or falling edge"]
    pub mod IRQ_POL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt polarity set"]
pub mod PL_GPIOD_SET {
    #[doc = "GPIO interrupt polarity, each bit represents a bus bit 0: irq is high level or rising edge 1: irq is low level or falling edge"]
    pub mod IRQ_POL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt polarity clear"]
pub mod PL_GPIOD_CLEAR {
    #[doc = "GPIO interrupt polarity, each bit represents a bus bit 0: irq is high level or rising edge 1: irq is low level or falling edge"]
    pub mod IRQ_POL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt polarity toggle"]
pub mod PL_GPIOD_TOGGLE {
    #[doc = "GPIO interrupt polarity, each bit represents a bus bit 0: irq is high level or rising edge 1: irq is low level or falling edge"]
    pub mod IRQ_POL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIOE interrupt polarity"]
pub mod PL_GPIOE_VALUE {
    #[doc = "GPIO interrupt polarity, each bit represents a bus bit 0: irq is high level or rising edge 1: irq is low level or falling edge"]
    pub mod IRQ_POL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt polarity set"]
pub mod PL_GPIOE_SET {
    #[doc = "GPIO interrupt polarity, each bit represents a bus bit 0: irq is high level or rising edge 1: irq is low level or falling edge"]
    pub mod IRQ_POL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt polarity clear"]
pub mod PL_GPIOE_CLEAR {
    #[doc = "GPIO interrupt polarity, each bit represents a bus bit 0: irq is high level or rising edge 1: irq is low level or falling edge"]
    pub mod IRQ_POL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt polarity toggle"]
pub mod PL_GPIOE_TOGGLE {
    #[doc = "GPIO interrupt polarity, each bit represents a bus bit 0: irq is high level or rising edge 1: irq is low level or falling edge"]
    pub mod IRQ_POL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIOF interrupt polarity"]
pub mod PL_GPIOF_VALUE {
    #[doc = "GPIO interrupt polarity, each bit represents a bus bit 0: irq is high level or rising edge 1: irq is low level or falling edge"]
    pub mod IRQ_POL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt polarity set"]
pub mod PL_GPIOF_SET {
    #[doc = "GPIO interrupt polarity, each bit represents a bus bit 0: irq is high level or rising edge 1: irq is low level or falling edge"]
    pub mod IRQ_POL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt polarity clear"]
pub mod PL_GPIOF_CLEAR {
    #[doc = "GPIO interrupt polarity, each bit represents a bus bit 0: irq is high level or rising edge 1: irq is low level or falling edge"]
    pub mod IRQ_POL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt polarity toggle"]
pub mod PL_GPIOF_TOGGLE {
    #[doc = "GPIO interrupt polarity, each bit represents a bus bit 0: irq is high level or rising edge 1: irq is low level or falling edge"]
    pub mod IRQ_POL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIOX interrupt polarity"]
pub mod PL_GPIOX_VALUE {
    #[doc = "GPIO interrupt polarity, each bit represents a bus bit 0: irq is high level or rising edge 1: irq is low level or falling edge"]
    pub mod IRQ_POL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt polarity set"]
pub mod PL_GPIOX_SET {
    #[doc = "GPIO interrupt polarity, each bit represents a bus bit 0: irq is high level or rising edge 1: irq is low level or falling edge"]
    pub mod IRQ_POL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt polarity clear"]
pub mod PL_GPIOX_CLEAR {
    #[doc = "GPIO interrupt polarity, each bit represents a bus bit 0: irq is high level or rising edge 1: irq is low level or falling edge"]
    pub mod IRQ_POL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt polarity toggle"]
pub mod PL_GPIOX_TOGGLE {
    #[doc = "GPIO interrupt polarity, each bit represents a bus bit 0: irq is high level or rising edge 1: irq is low level or falling edge"]
    pub mod IRQ_POL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIOY interrupt polarity"]
pub mod PL_GPIOY_VALUE {
    #[doc = "GPIO interrupt polarity, each bit represents a bus bit 0: irq is high level or rising edge 1: irq is low level or falling edge"]
    pub mod IRQ_POL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt polarity set"]
pub mod PL_GPIOY_SET {
    #[doc = "GPIO interrupt polarity, each bit represents a bus bit 0: irq is high level or rising edge 1: irq is low level or falling edge"]
    pub mod IRQ_POL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt polarity clear"]
pub mod PL_GPIOY_CLEAR {
    #[doc = "GPIO interrupt polarity, each bit represents a bus bit 0: irq is high level or rising edge 1: irq is low level or falling edge"]
    pub mod IRQ_POL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt polarity toggle"]
pub mod PL_GPIOY_TOGGLE {
    #[doc = "GPIO interrupt polarity, each bit represents a bus bit 0: irq is high level or rising edge 1: irq is low level or falling edge"]
    pub mod IRQ_POL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIOZ interrupt polarity"]
pub mod PL_GPIOZ_VALUE {
    #[doc = "GPIO interrupt polarity, each bit represents a bus bit 0: irq is high level or rising edge 1: irq is low level or falling edge"]
    pub mod IRQ_POL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt polarity set"]
pub mod PL_GPIOZ_SET {
    #[doc = "GPIO interrupt polarity, each bit represents a bus bit 0: irq is high level or rising edge 1: irq is low level or falling edge"]
    pub mod IRQ_POL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt polarity clear"]
pub mod PL_GPIOZ_CLEAR {
    #[doc = "GPIO interrupt polarity, each bit represents a bus bit 0: irq is high level or rising edge 1: irq is low level or falling edge"]
    pub mod IRQ_POL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt polarity toggle"]
pub mod PL_GPIOZ_TOGGLE {
    #[doc = "GPIO interrupt polarity, each bit represents a bus bit 0: irq is high level or rising edge 1: irq is low level or falling edge"]
    pub mod IRQ_POL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt type value"]
pub mod TP_GPIOA_VALUE {
    #[doc = "GPIO interrupt type, each bit represents a bus bit 0: irq is triggered by level 1: irq is triggered by edge"]
    pub mod IRQ_TYPE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt type set"]
pub mod TP_GPIOA_SET {
    #[doc = "GPIO interrupt type, each bit represents a bus bit 0: irq is triggered by level 1: irq is triggered by edge"]
    pub mod IRQ_TYPE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt type clear"]
pub mod TP_GPIOA_CLEAR {
    #[doc = "GPIO interrupt type, each bit represents a bus bit 0: irq is triggered by level 1: irq is triggered by edge"]
    pub mod IRQ_TYPE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt type toggle"]
pub mod TP_GPIOA_TOGGLE {
    #[doc = "GPIO interrupt type, each bit represents a bus bit 0: irq is triggered by level 1: irq is triggered by edge"]
    pub mod IRQ_TYPE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIOB interrupt type"]
pub mod TP_GPIOB_VALUE {
    #[doc = "GPIO interrupt type, each bit represents a bus bit 0: irq is triggered by level 1: irq is triggered by edge"]
    pub mod IRQ_TYPE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt type set"]
pub mod TP_GPIOB_SET {
    #[doc = "GPIO interrupt type, each bit represents a bus bit 0: irq is triggered by level 1: irq is triggered by edge"]
    pub mod IRQ_TYPE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt type clear"]
pub mod TP_GPIOB_CLEAR {
    #[doc = "GPIO interrupt type, each bit represents a bus bit 0: irq is triggered by level 1: irq is triggered by edge"]
    pub mod IRQ_TYPE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt type toggle"]
pub mod TP_GPIOB_TOGGLE {
    #[doc = "GPIO interrupt type, each bit represents a bus bit 0: irq is triggered by level 1: irq is triggered by edge"]
    pub mod IRQ_TYPE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIOC interrupt type"]
pub mod TP_GPIOC_VALUE {
    #[doc = "GPIO interrupt type, each bit represents a bus bit 0: irq is triggered by level 1: irq is triggered by edge"]
    pub mod IRQ_TYPE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt type set"]
pub mod TP_GPIOC_SET {
    #[doc = "GPIO interrupt type, each bit represents a bus bit 0: irq is triggered by level 1: irq is triggered by edge"]
    pub mod IRQ_TYPE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt type clear"]
pub mod TP_GPIOC_CLEAR {
    #[doc = "GPIO interrupt type, each bit represents a bus bit 0: irq is triggered by level 1: irq is triggered by edge"]
    pub mod IRQ_TYPE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt type toggle"]
pub mod TP_GPIOC_TOGGLE {
    #[doc = "GPIO interrupt type, each bit represents a bus bit 0: irq is triggered by level 1: irq is triggered by edge"]
    pub mod IRQ_TYPE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIOD interrupt type"]
pub mod TP_GPIOD_VALUE {
    #[doc = "GPIO interrupt type, each bit represents a bus bit 0: irq is triggered by level 1: irq is triggered by edge"]
    pub mod IRQ_TYPE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt type set"]
pub mod TP_GPIOD_SET {
    #[doc = "GPIO interrupt type, each bit represents a bus bit 0: irq is triggered by level 1: irq is triggered by edge"]
    pub mod IRQ_TYPE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt type clear"]
pub mod TP_GPIOD_CLEAR {
    #[doc = "GPIO interrupt type, each bit represents a bus bit 0: irq is triggered by level 1: irq is triggered by edge"]
    pub mod IRQ_TYPE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt type toggle"]
pub mod TP_GPIOD_TOGGLE {
    #[doc = "GPIO interrupt type, each bit represents a bus bit 0: irq is triggered by level 1: irq is triggered by edge"]
    pub mod IRQ_TYPE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIOE interrupt type"]
pub mod TP_GPIOE_VALUE {
    #[doc = "GPIO interrupt type, each bit represents a bus bit 0: irq is triggered by level 1: irq is triggered by edge"]
    pub mod IRQ_TYPE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt type set"]
pub mod TP_GPIOE_SET {
    #[doc = "GPIO interrupt type, each bit represents a bus bit 0: irq is triggered by level 1: irq is triggered by edge"]
    pub mod IRQ_TYPE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt type clear"]
pub mod TP_GPIOE_CLEAR {
    #[doc = "GPIO interrupt type, each bit represents a bus bit 0: irq is triggered by level 1: irq is triggered by edge"]
    pub mod IRQ_TYPE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt type toggle"]
pub mod TP_GPIOE_TOGGLE {
    #[doc = "GPIO interrupt type, each bit represents a bus bit 0: irq is triggered by level 1: irq is triggered by edge"]
    pub mod IRQ_TYPE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIOF interrupt type"]
pub mod TP_GPIOF_VALUE {
    #[doc = "GPIO interrupt type, each bit represents a bus bit 0: irq is triggered by level 1: irq is triggered by edge"]
    pub mod IRQ_TYPE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt type set"]
pub mod TP_GPIOF_SET {
    #[doc = "GPIO interrupt type, each bit represents a bus bit 0: irq is triggered by level 1: irq is triggered by edge"]
    pub mod IRQ_TYPE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt type clear"]
pub mod TP_GPIOF_CLEAR {
    #[doc = "GPIO interrupt type, each bit represents a bus bit 0: irq is triggered by level 1: irq is triggered by edge"]
    pub mod IRQ_TYPE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt type toggle"]
pub mod TP_GPIOF_TOGGLE {
    #[doc = "GPIO interrupt type, each bit represents a bus bit 0: irq is triggered by level 1: irq is triggered by edge"]
    pub mod IRQ_TYPE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIOX interrupt type"]
pub mod TP_GPIOX_VALUE {
    #[doc = "GPIO interrupt type, each bit represents a bus bit 0: irq is triggered by level 1: irq is triggered by edge"]
    pub mod IRQ_TYPE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt type set"]
pub mod TP_GPIOX_SET {
    #[doc = "GPIO interrupt type, each bit represents a bus bit 0: irq is triggered by level 1: irq is triggered by edge"]
    pub mod IRQ_TYPE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt type clear"]
pub mod TP_GPIOX_CLEAR {
    #[doc = "GPIO interrupt type, each bit represents a bus bit 0: irq is triggered by level 1: irq is triggered by edge"]
    pub mod IRQ_TYPE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt type toggle"]
pub mod TP_GPIOX_TOGGLE {
    #[doc = "GPIO interrupt type, each bit represents a bus bit 0: irq is triggered by level 1: irq is triggered by edge"]
    pub mod IRQ_TYPE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIOY interrupt type"]
pub mod TP_GPIOY_VALUE {
    #[doc = "GPIO interrupt type, each bit represents a bus bit 0: irq is triggered by level 1: irq is triggered by edge"]
    pub mod IRQ_TYPE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt type set"]
pub mod TP_GPIOY_SET {
    #[doc = "GPIO interrupt type, each bit represents a bus bit 0: irq is triggered by level 1: irq is triggered by edge"]
    pub mod IRQ_TYPE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt type clear"]
pub mod TP_GPIOY_CLEAR {
    #[doc = "GPIO interrupt type, each bit represents a bus bit 0: irq is triggered by level 1: irq is triggered by edge"]
    pub mod IRQ_TYPE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt type toggle"]
pub mod TP_GPIOY_TOGGLE {
    #[doc = "GPIO interrupt type, each bit represents a bus bit 0: irq is triggered by level 1: irq is triggered by edge"]
    pub mod IRQ_TYPE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIOZ interrupt type"]
pub mod TP_GPIOZ_VALUE {
    #[doc = "GPIO interrupt type, each bit represents a bus bit 0: irq is triggered by level 1: irq is triggered by edge"]
    pub mod IRQ_TYPE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt type set"]
pub mod TP_GPIOZ_SET {
    #[doc = "GPIO interrupt type, each bit represents a bus bit 0: irq is triggered by level 1: irq is triggered by edge"]
    pub mod IRQ_TYPE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt type clear"]
pub mod TP_GPIOZ_CLEAR {
    #[doc = "GPIO interrupt type, each bit represents a bus bit 0: irq is triggered by level 1: irq is triggered by edge"]
    pub mod IRQ_TYPE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt type toggle"]
pub mod TP_GPIOZ_TOGGLE {
    #[doc = "GPIO interrupt type, each bit represents a bus bit 0: irq is triggered by level 1: irq is triggered by edge"]
    pub mod IRQ_TYPE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt asynchronous value"]
pub mod AS_GPIOA_VALUE {
    #[doc = "GPIO interrupt asynchronous, each bit represents a bus bit 0: irq is triggered base on system clock 1: irq is triggered combinational Note: combinational interrupt is sensitive to environment noise"]
    pub mod IRQ_ASYNC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt asynchronous set"]
pub mod AS_GPIOA_SET {
    #[doc = "GPIO interrupt asynchronous, each bit represents a bus bit 0: irq is triggered base on system clock 1: irq is triggered combinational Note: combinational interrupt is sensitive to environment noise"]
    pub mod IRQ_ASYNC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt asynchronous clear"]
pub mod AS_GPIOA_CLEAR {
    #[doc = "GPIO interrupt asynchronous, each bit represents a bus bit 0: irq is triggered base on system clock 1: irq is triggered combinational Note: combinational interrupt is sensitive to environment noise"]
    pub mod IRQ_ASYNC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt asynchronous toggle"]
pub mod AS_GPIOA_TOGGLE {
    #[doc = "GPIO interrupt asynchronous, each bit represents a bus bit 0: irq is triggered base on system clock 1: irq is triggered combinational Note: combinational interrupt is sensitive to environment noise"]
    pub mod IRQ_ASYNC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIOB interrupt asynchronous"]
pub mod AS_GPIOB_VALUE {
    #[doc = "GPIO interrupt asynchronous, each bit represents a bus bit 0: irq is triggered base on system clock 1: irq is triggered combinational Note: combinational interrupt is sensitive to environment noise"]
    pub mod IRQ_ASYNC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt asynchronous set"]
pub mod AS_GPIOB_SET {
    #[doc = "GPIO interrupt asynchronous, each bit represents a bus bit 0: irq is triggered base on system clock 1: irq is triggered combinational Note: combinational interrupt is sensitive to environment noise"]
    pub mod IRQ_ASYNC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt asynchronous clear"]
pub mod AS_GPIOB_CLEAR {
    #[doc = "GPIO interrupt asynchronous, each bit represents a bus bit 0: irq is triggered base on system clock 1: irq is triggered combinational Note: combinational interrupt is sensitive to environment noise"]
    pub mod IRQ_ASYNC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt asynchronous toggle"]
pub mod AS_GPIOB_TOGGLE {
    #[doc = "GPIO interrupt asynchronous, each bit represents a bus bit 0: irq is triggered base on system clock 1: irq is triggered combinational Note: combinational interrupt is sensitive to environment noise"]
    pub mod IRQ_ASYNC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIOC interrupt asynchronous"]
pub mod AS_GPIOC_VALUE {
    #[doc = "GPIO interrupt asynchronous, each bit represents a bus bit 0: irq is triggered base on system clock 1: irq is triggered combinational Note: combinational interrupt is sensitive to environment noise"]
    pub mod IRQ_ASYNC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt asynchronous set"]
pub mod AS_GPIOC_SET {
    #[doc = "GPIO interrupt asynchronous, each bit represents a bus bit 0: irq is triggered base on system clock 1: irq is triggered combinational Note: combinational interrupt is sensitive to environment noise"]
    pub mod IRQ_ASYNC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt asynchronous clear"]
pub mod AS_GPIOC_CLEAR {
    #[doc = "GPIO interrupt asynchronous, each bit represents a bus bit 0: irq is triggered base on system clock 1: irq is triggered combinational Note: combinational interrupt is sensitive to environment noise"]
    pub mod IRQ_ASYNC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt asynchronous toggle"]
pub mod AS_GPIOC_TOGGLE {
    #[doc = "GPIO interrupt asynchronous, each bit represents a bus bit 0: irq is triggered base on system clock 1: irq is triggered combinational Note: combinational interrupt is sensitive to environment noise"]
    pub mod IRQ_ASYNC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIOD interrupt asynchronous"]
pub mod AS_GPIOD_VALUE {
    #[doc = "GPIO interrupt asynchronous, each bit represents a bus bit 0: irq is triggered base on system clock 1: irq is triggered combinational Note: combinational interrupt is sensitive to environment noise"]
    pub mod IRQ_ASYNC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt asynchronous set"]
pub mod AS_GPIOD_SET {
    #[doc = "GPIO interrupt asynchronous, each bit represents a bus bit 0: irq is triggered base on system clock 1: irq is triggered combinational Note: combinational interrupt is sensitive to environment noise"]
    pub mod IRQ_ASYNC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt asynchronous clear"]
pub mod AS_GPIOD_CLEAR {
    #[doc = "GPIO interrupt asynchronous, each bit represents a bus bit 0: irq is triggered base on system clock 1: irq is triggered combinational Note: combinational interrupt is sensitive to environment noise"]
    pub mod IRQ_ASYNC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt asynchronous toggle"]
pub mod AS_GPIOD_TOGGLE {
    #[doc = "GPIO interrupt asynchronous, each bit represents a bus bit 0: irq is triggered base on system clock 1: irq is triggered combinational Note: combinational interrupt is sensitive to environment noise"]
    pub mod IRQ_ASYNC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIOE interrupt asynchronous"]
pub mod AS_GPIOE_VALUE {
    #[doc = "GPIO interrupt asynchronous, each bit represents a bus bit 0: irq is triggered base on system clock 1: irq is triggered combinational Note: combinational interrupt is sensitive to environment noise"]
    pub mod IRQ_ASYNC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt asynchronous set"]
pub mod AS_GPIOE_SET {
    #[doc = "GPIO interrupt asynchronous, each bit represents a bus bit 0: irq is triggered base on system clock 1: irq is triggered combinational Note: combinational interrupt is sensitive to environment noise"]
    pub mod IRQ_ASYNC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt asynchronous clear"]
pub mod AS_GPIOE_CLEAR {
    #[doc = "GPIO interrupt asynchronous, each bit represents a bus bit 0: irq is triggered base on system clock 1: irq is triggered combinational Note: combinational interrupt is sensitive to environment noise"]
    pub mod IRQ_ASYNC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt asynchronous toggle"]
pub mod AS_GPIOE_TOGGLE {
    #[doc = "GPIO interrupt asynchronous, each bit represents a bus bit 0: irq is triggered base on system clock 1: irq is triggered combinational Note: combinational interrupt is sensitive to environment noise"]
    pub mod IRQ_ASYNC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIOF interrupt asynchronous"]
pub mod AS_GPIOF_VALUE {
    #[doc = "GPIO interrupt asynchronous, each bit represents a bus bit 0: irq is triggered base on system clock 1: irq is triggered combinational Note: combinational interrupt is sensitive to environment noise"]
    pub mod IRQ_ASYNC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt asynchronous set"]
pub mod AS_GPIOF_SET {
    #[doc = "GPIO interrupt asynchronous, each bit represents a bus bit 0: irq is triggered base on system clock 1: irq is triggered combinational Note: combinational interrupt is sensitive to environment noise"]
    pub mod IRQ_ASYNC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt asynchronous clear"]
pub mod AS_GPIOF_CLEAR {
    #[doc = "GPIO interrupt asynchronous, each bit represents a bus bit 0: irq is triggered base on system clock 1: irq is triggered combinational Note: combinational interrupt is sensitive to environment noise"]
    pub mod IRQ_ASYNC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt asynchronous toggle"]
pub mod AS_GPIOF_TOGGLE {
    #[doc = "GPIO interrupt asynchronous, each bit represents a bus bit 0: irq is triggered base on system clock 1: irq is triggered combinational Note: combinational interrupt is sensitive to environment noise"]
    pub mod IRQ_ASYNC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIOX interrupt asynchronous"]
pub mod AS_GPIOX_VALUE {
    #[doc = "GPIO interrupt asynchronous, each bit represents a bus bit 0: irq is triggered base on system clock 1: irq is triggered combinational Note: combinational interrupt is sensitive to environment noise"]
    pub mod IRQ_ASYNC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt asynchronous set"]
pub mod AS_GPIOX_SET {
    #[doc = "GPIO interrupt asynchronous, each bit represents a bus bit 0: irq is triggered base on system clock 1: irq is triggered combinational Note: combinational interrupt is sensitive to environment noise"]
    pub mod IRQ_ASYNC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt asynchronous clear"]
pub mod AS_GPIOX_CLEAR {
    #[doc = "GPIO interrupt asynchronous, each bit represents a bus bit 0: irq is triggered base on system clock 1: irq is triggered combinational Note: combinational interrupt is sensitive to environment noise"]
    pub mod IRQ_ASYNC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt asynchronous toggle"]
pub mod AS_GPIOX_TOGGLE {
    #[doc = "GPIO interrupt asynchronous, each bit represents a bus bit 0: irq is triggered base on system clock 1: irq is triggered combinational Note: combinational interrupt is sensitive to environment noise"]
    pub mod IRQ_ASYNC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIOY interrupt asynchronous"]
pub mod AS_GPIOY_VALUE {
    #[doc = "GPIO interrupt asynchronous, each bit represents a bus bit 0: irq is triggered base on system clock 1: irq is triggered combinational Note: combinational interrupt is sensitive to environment noise"]
    pub mod IRQ_ASYNC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt asynchronous set"]
pub mod AS_GPIOY_SET {
    #[doc = "GPIO interrupt asynchronous, each bit represents a bus bit 0: irq is triggered base on system clock 1: irq is triggered combinational Note: combinational interrupt is sensitive to environment noise"]
    pub mod IRQ_ASYNC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt asynchronous clear"]
pub mod AS_GPIOY_CLEAR {
    #[doc = "GPIO interrupt asynchronous, each bit represents a bus bit 0: irq is triggered base on system clock 1: irq is triggered combinational Note: combinational interrupt is sensitive to environment noise"]
    pub mod IRQ_ASYNC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt asynchronous toggle"]
pub mod AS_GPIOY_TOGGLE {
    #[doc = "GPIO interrupt asynchronous, each bit represents a bus bit 0: irq is triggered base on system clock 1: irq is triggered combinational Note: combinational interrupt is sensitive to environment noise"]
    pub mod IRQ_ASYNC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIOZ interrupt asynchronous"]
pub mod AS_GPIOZ_VALUE {
    #[doc = "GPIO interrupt asynchronous, each bit represents a bus bit 0: irq is triggered base on system clock 1: irq is triggered combinational Note: combinational interrupt is sensitive to environment noise"]
    pub mod IRQ_ASYNC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt asynchronous set"]
pub mod AS_GPIOZ_SET {
    #[doc = "GPIO interrupt asynchronous, each bit represents a bus bit 0: irq is triggered base on system clock 1: irq is triggered combinational Note: combinational interrupt is sensitive to environment noise"]
    pub mod IRQ_ASYNC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt asynchronous clear"]
pub mod AS_GPIOZ_CLEAR {
    #[doc = "GPIO interrupt asynchronous, each bit represents a bus bit 0: irq is triggered base on system clock 1: irq is triggered combinational Note: combinational interrupt is sensitive to environment noise"]
    pub mod IRQ_ASYNC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GPIO interrupt asynchronous toggle"]
pub mod AS_GPIOZ_TOGGLE {
    #[doc = "GPIO interrupt asynchronous, each bit represents a bus bit 0: irq is triggered base on system clock 1: irq is triggered combinational Note: combinational interrupt is sensitive to environment noise"]
    pub mod IRQ_ASYNC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
