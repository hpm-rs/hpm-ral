// #[derive(Copy, Clone, Debug, PartialEq, Eq)]
// pub enum Interrupt {}
// pub type interrupt = Interrupt;
// unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
//     #[inline(always)]
//     fn number(self) -> u16 {
//         self as u16
//     }
// }
#[cfg(feature = "rt")]
mod _vectors {
    extern "C" {}
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    #[cfg_attr(target_os = "none", link_section = ".vector_table.interrupts")]
    #[no_mangle]
    pub static __INTERRUPTS: [Vector; 0] = [];
}
#[path = "."]
pub mod acmp {
    #[doc = "ACMP"]
    pub const ACMP: *const RegisterBlock = 0xf002_0000 as *const RegisterBlock;
    #[path = "blocks/hpm6750/acmp.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type ACMP = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for ACMP {}
    impl crate::Valid for ACMP {}
    impl ACMP {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(ACMP)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, ACMP).then_some(0)
    }
}
#[path = "."]
pub mod adc {
    #[doc = "ADC0"]
    pub const ADC0: *const RegisterBlock = 0xf001_0000 as *const RegisterBlock;
    #[doc = "ADC1"]
    pub const ADC1: *const RegisterBlock = 0xf001_4000 as *const RegisterBlock;
    #[doc = "ADC2"]
    pub const ADC2: *const RegisterBlock = 0xf001_8000 as *const RegisterBlock;
    #[path = "blocks/hpm6750/adc.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type ADC0 = Instance<0>;
    impl crate::private::Sealed for ADC0 {}
    impl crate::Valid for ADC0 {}
    impl ADC0 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(ADC0)
        }
    }
    pub type ADC1 = Instance<1>;
    impl crate::private::Sealed for ADC1 {}
    impl crate::Valid for ADC1 {}
    impl ADC1 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(ADC1)
        }
    }
    pub type ADC2 = Instance<2>;
    impl crate::private::Sealed for ADC2 {}
    impl crate::Valid for ADC2 {}
    impl ADC2 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(ADC2)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(ADC0, 0), (ADC1, 1), (ADC2, 2)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod adc3 {
    #[doc = "ADC3"]
    pub const ADC3: *const RegisterBlock = 0xf001_c000 as *const RegisterBlock;
    #[path = "blocks/hpm6750/adc3.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type ADC3 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for ADC3 {}
    impl crate::Valid for ADC3 {}
    impl ADC3 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(ADC3)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, ADC3).then_some(0)
    }
}
#[path = "."]
pub mod bcfg {
    #[doc = "BCFG"]
    pub const BCFG: *const RegisterBlock = 0xf500_8000 as *const RegisterBlock;
    #[path = "blocks/hpm6750/bcfg.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type BCFG = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for BCFG {}
    impl crate::Valid for BCFG {}
    impl BCFG {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(BCFG)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, BCFG).then_some(0)
    }
}
#[path = "."]
pub mod bgpr {
    #[doc = "BGPR"]
    pub const BGPR: *const RegisterBlock = 0xf501_8000 as *const RegisterBlock;
    #[path = "blocks/hpm6750/bgpr.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type BGPR = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for BGPR {}
    impl crate::Valid for BGPR {}
    impl BGPR {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(BGPR)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, BGPR).then_some(0)
    }
}
#[path = "."]
pub mod bkey {
    #[doc = "BKEY"]
    pub const BKEY: *const RegisterBlock = 0xf504_8000 as *const RegisterBlock;
    #[path = "blocks/hpm6750/bkey.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type BKEY = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for BKEY {}
    impl crate::Valid for BKEY {}
    impl BKEY {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(BKEY)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, BKEY).then_some(0)
    }
}
#[path = "."]
pub mod bmon {
    #[doc = "BMON"]
    pub const BMON: *const RegisterBlock = 0xf504_c000 as *const RegisterBlock;
    #[path = "blocks/hpm6750/bmon.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type BMON = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for BMON {}
    impl crate::Valid for BMON {}
    impl BMON {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(BMON)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, BMON).then_some(0)
    }
}
#[path = "."]
pub mod bpor {
    #[doc = "BPOR"]
    pub const BPOR: *const RegisterBlock = 0xf500_4000 as *const RegisterBlock;
    #[path = "blocks/hpm6750/bpor.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type BPOR = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for BPOR {}
    impl crate::Valid for BPOR {}
    impl BPOR {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(BPOR)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, BPOR).then_some(0)
    }
}
#[path = "."]
pub mod bsec {
    #[doc = "BSEC"]
    pub const BSEC: *const RegisterBlock = 0xf504_0000 as *const RegisterBlock;
    #[path = "blocks/hpm6750/bsec.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type BSEC = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for BSEC {}
    impl crate::Valid for BSEC {}
    impl BSEC {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(BSEC)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, BSEC).then_some(0)
    }
}
#[path = "."]
pub mod butn {
    #[doc = "BUTN"]
    pub const BUTN: *const RegisterBlock = 0xf500_c000 as *const RegisterBlock;
    #[path = "blocks/hpm6750/butn.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type BUTN = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for BUTN {}
    impl crate::Valid for BUTN {}
    impl BUTN {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(BUTN)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, BUTN).then_some(0)
    }
}
#[path = "."]
pub mod cam {
    #[doc = "CAM0"]
    pub const CAM0: *const RegisterBlock = 0xf100_8000 as *const RegisterBlock;
    #[doc = "CAM1"]
    pub const CAM1: *const RegisterBlock = 0xf100_c000 as *const RegisterBlock;
    #[path = "blocks/hpm6750/cam.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type CAM0 = Instance<0>;
    impl crate::private::Sealed for CAM0 {}
    impl crate::Valid for CAM0 {}
    impl CAM0 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(CAM0)
        }
    }
    pub type CAM1 = Instance<1>;
    impl crate::private::Sealed for CAM1 {}
    impl crate::Valid for CAM1 {}
    impl CAM1 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(CAM1)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(CAM0, 0), (CAM1, 1)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod can {
    #[doc = "CAN0"]
    pub const CAN0: *const RegisterBlock = 0xf008_0000 as *const RegisterBlock;
    #[doc = "CAN1"]
    pub const CAN1: *const RegisterBlock = 0xf008_4000 as *const RegisterBlock;
    #[doc = "CAN2"]
    pub const CAN2: *const RegisterBlock = 0xf008_8000 as *const RegisterBlock;
    #[doc = "CAN3"]
    pub const CAN3: *const RegisterBlock = 0xf008_c000 as *const RegisterBlock;
    #[path = "blocks/hpm6750/can.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type CAN0 = Instance<0>;
    impl crate::private::Sealed for CAN0 {}
    impl crate::Valid for CAN0 {}
    impl CAN0 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(CAN0)
        }
    }
    pub type CAN1 = Instance<1>;
    impl crate::private::Sealed for CAN1 {}
    impl crate::Valid for CAN1 {}
    impl CAN1 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(CAN1)
        }
    }
    pub type CAN2 = Instance<2>;
    impl crate::private::Sealed for CAN2 {}
    impl crate::Valid for CAN2 {}
    impl CAN2 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(CAN2)
        }
    }
    pub type CAN3 = Instance<3>;
    impl crate::private::Sealed for CAN3 {}
    impl crate::Valid for CAN3 {}
    impl CAN3 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(CAN3)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(CAN0, 0), (CAN1, 1), (CAN2, 2), (CAN3, 3)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod conctl {
    #[doc = "CONCTL"]
    pub const CONCTL: *const RegisterBlock = 0xf204_0000 as *const RegisterBlock;
    #[path = "blocks/hpm6750/conctl.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type CONCTL = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for CONCTL {}
    impl crate::Valid for CONCTL {}
    impl CONCTL {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(CONCTL)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, CONCTL).then_some(0)
    }
}
#[path = "."]
pub mod dao {
    #[doc = "DAO"]
    pub const DAO: *const RegisterBlock = 0xf011_0000 as *const RegisterBlock;
    #[path = "blocks/hpm6750/dao.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type DAO = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for DAO {}
    impl crate::Valid for DAO {}
    impl DAO {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(DAO)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, DAO).then_some(0)
    }
}
#[path = "."]
pub mod dmamux {
    #[doc = "DMAMUX"]
    pub const DMAMUX: *const RegisterBlock = 0xf00c_0000 as *const RegisterBlock;
    #[path = "blocks/hpm6750/dmamux.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type DMAMUX = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for DMAMUX {}
    impl crate::Valid for DMAMUX {}
    impl DMAMUX {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(DMAMUX)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, DMAMUX).then_some(0)
    }
}
#[path = "."]
pub mod dram {
    #[doc = "DRAM"]
    pub const DRAM: *const RegisterBlock = 0xf305_0000 as *const RegisterBlock;
    #[path = "blocks/hpm6750/dram.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type DRAM = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for DRAM {}
    impl crate::Valid for DRAM {}
    impl DRAM {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(DRAM)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, DRAM).then_some(0)
    }
}
#[path = "."]
pub mod enet {
    #[doc = "ENET0"]
    pub const ENET0: *const RegisterBlock = 0xf200_0000 as *const RegisterBlock;
    #[doc = "ENET1"]
    pub const ENET1: *const RegisterBlock = 0xf200_4000 as *const RegisterBlock;
    #[path = "blocks/hpm6750/enet.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type ENET0 = Instance<0>;
    impl crate::private::Sealed for ENET0 {}
    impl crate::Valid for ENET0 {}
    impl ENET0 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(ENET0)
        }
    }
    pub type ENET1 = Instance<1>;
    impl crate::private::Sealed for ENET1 {}
    impl crate::Valid for ENET1 {}
    impl ENET1 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(ENET1)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(ENET0, 0), (ENET1, 1)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod gpio {
    pub const FGPIO10: *const RegisterBlock = 0x000c_0000 as *const RegisterBlock;
    #[doc = "FGPIO"]
    pub const GPIO0: *const RegisterBlock = 0xf000_0000 as *const RegisterBlock;
    #[doc = "GPIO1"]
    pub const GPIO1: *const RegisterBlock = 0xf000_4000 as *const RegisterBlock;
    #[doc = "PGPIO"]
    pub const PGPIO11: *const RegisterBlock = 0xf40d_c000 as *const RegisterBlock;
    #[doc = "BGPIO"]
    pub const BGPIO12: *const RegisterBlock = 0xf501_4000 as *const RegisterBlock;
    #[path = "blocks/hpm6750/gpio.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type FGPIO10 = Instance<10>;
    impl crate::private::Sealed for FGPIO10 {}
    impl crate::Valid for FGPIO10 {}
    impl FGPIO10 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(FGPIO10)
        }
    }
    pub type GPIO0 = Instance<0>;
    impl crate::private::Sealed for GPIO0 {}
    impl crate::Valid for GPIO0 {}
    impl GPIO0 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(GPIO0)
        }
    }
    pub type GPIO1 = Instance<1>;
    impl crate::private::Sealed for GPIO1 {}
    impl crate::Valid for GPIO1 {}
    impl GPIO1 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(GPIO1)
        }
    }
    pub type PGPIO11 = Instance<11>;
    impl crate::private::Sealed for PGPIO11 {}
    impl crate::Valid for PGPIO11 {}
    impl PGPIO11 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(PGPIO11)
        }
    }
    pub type BGPIO12 = Instance<12>;
    impl crate::private::Sealed for BGPIO12 {}
    impl crate::Valid for BGPIO12 {}
    impl BGPIO12 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(BGPIO12)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [
            (FGPIO10, 10),
            (GPIO0, 0),
            (GPIO1, 1),
            (PGPIO11, 11),
            (BGPIO12, 12),
        ]
        .into_iter()
        .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
        .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod gpiom {
    #[doc = "GPIOM"]
    pub const GPIOM: *const RegisterBlock = 0xf000_8000 as *const RegisterBlock;
    #[path = "blocks/hpm6750/gpiom.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type GPIOM = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for GPIOM {}
    impl crate::Valid for GPIOM {}
    impl GPIOM {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(GPIOM)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, GPIOM).then_some(0)
    }
}
#[path = "."]
pub mod gptmr {
    pub const NTMR10: *const RegisterBlock = 0xf201_0000 as *const RegisterBlock;
    #[doc = "NTMR1"]
    pub const NTMR11: *const RegisterBlock = 0xf201_4000 as *const RegisterBlock;
    #[doc = "NTMR0"]
    pub const GPTMR0: *const RegisterBlock = 0xf300_0000 as *const RegisterBlock;
    #[doc = "GPTMR1"]
    pub const GPTMR1: *const RegisterBlock = 0xf300_4000 as *const RegisterBlock;
    #[doc = "GPTMR2"]
    pub const GPTMR2: *const RegisterBlock = 0xf300_8000 as *const RegisterBlock;
    #[doc = "GPTMR3"]
    pub const GPTMR3: *const RegisterBlock = 0xf300_c000 as *const RegisterBlock;
    #[doc = "GPTMR4"]
    pub const GPTMR4: *const RegisterBlock = 0xf301_0000 as *const RegisterBlock;
    #[doc = "GPTMR5"]
    pub const GPTMR5: *const RegisterBlock = 0xf301_4000 as *const RegisterBlock;
    #[doc = "GPTMR6"]
    pub const GPTMR6: *const RegisterBlock = 0xf301_8000 as *const RegisterBlock;
    #[doc = "GPTMR7"]
    pub const GPTMR7: *const RegisterBlock = 0xf301_c000 as *const RegisterBlock;
    #[doc = "PTMR"]
    pub const PTMR12: *const RegisterBlock = 0xf40e_0000 as *const RegisterBlock;
    #[path = "blocks/hpm6750/gptmr.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type NTMR10 = Instance<10>;
    impl crate::private::Sealed for NTMR10 {}
    impl crate::Valid for NTMR10 {}
    impl NTMR10 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(NTMR10)
        }
    }
    pub type NTMR11 = Instance<11>;
    impl crate::private::Sealed for NTMR11 {}
    impl crate::Valid for NTMR11 {}
    impl NTMR11 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(NTMR11)
        }
    }
    pub type GPTMR0 = Instance<0>;
    impl crate::private::Sealed for GPTMR0 {}
    impl crate::Valid for GPTMR0 {}
    impl GPTMR0 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(GPTMR0)
        }
    }
    pub type GPTMR1 = Instance<1>;
    impl crate::private::Sealed for GPTMR1 {}
    impl crate::Valid for GPTMR1 {}
    impl GPTMR1 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(GPTMR1)
        }
    }
    pub type GPTMR2 = Instance<2>;
    impl crate::private::Sealed for GPTMR2 {}
    impl crate::Valid for GPTMR2 {}
    impl GPTMR2 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(GPTMR2)
        }
    }
    pub type GPTMR3 = Instance<3>;
    impl crate::private::Sealed for GPTMR3 {}
    impl crate::Valid for GPTMR3 {}
    impl GPTMR3 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(GPTMR3)
        }
    }
    pub type GPTMR4 = Instance<4>;
    impl crate::private::Sealed for GPTMR4 {}
    impl crate::Valid for GPTMR4 {}
    impl GPTMR4 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(GPTMR4)
        }
    }
    pub type GPTMR5 = Instance<5>;
    impl crate::private::Sealed for GPTMR5 {}
    impl crate::Valid for GPTMR5 {}
    impl GPTMR5 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(GPTMR5)
        }
    }
    pub type GPTMR6 = Instance<6>;
    impl crate::private::Sealed for GPTMR6 {}
    impl crate::Valid for GPTMR6 {}
    impl GPTMR6 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(GPTMR6)
        }
    }
    pub type GPTMR7 = Instance<7>;
    impl crate::private::Sealed for GPTMR7 {}
    impl crate::Valid for GPTMR7 {}
    impl GPTMR7 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(GPTMR7)
        }
    }
    pub type PTMR12 = Instance<12>;
    impl crate::private::Sealed for PTMR12 {}
    impl crate::Valid for PTMR12 {}
    impl PTMR12 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(PTMR12)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [
            (NTMR10, 10),
            (NTMR11, 11),
            (GPTMR0, 0),
            (GPTMR1, 1),
            (GPTMR2, 2),
            (GPTMR3, 3),
            (GPTMR4, 4),
            (GPTMR5, 5),
            (GPTMR6, 6),
            (GPTMR7, 7),
            (PTMR12, 12),
        ]
        .into_iter()
        .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
        .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod hall {
    #[doc = "HALL0"]
    pub const HALL0: *const RegisterBlock = 0xf020_4000 as *const RegisterBlock;
    #[doc = "HALL1"]
    pub const HALL1: *const RegisterBlock = 0xf021_4000 as *const RegisterBlock;
    #[doc = "HALL2"]
    pub const HALL2: *const RegisterBlock = 0xf022_4000 as *const RegisterBlock;
    #[doc = "HALL3"]
    pub const HALL3: *const RegisterBlock = 0xf023_4000 as *const RegisterBlock;
    #[path = "blocks/hpm6750/hall.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type HALL0 = Instance<0>;
    impl crate::private::Sealed for HALL0 {}
    impl crate::Valid for HALL0 {}
    impl HALL0 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(HALL0)
        }
    }
    pub type HALL1 = Instance<1>;
    impl crate::private::Sealed for HALL1 {}
    impl crate::Valid for HALL1 {}
    impl HALL1 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(HALL1)
        }
    }
    pub type HALL2 = Instance<2>;
    impl crate::private::Sealed for HALL2 {}
    impl crate::Valid for HALL2 {}
    impl HALL2 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(HALL2)
        }
    }
    pub type HALL3 = Instance<3>;
    impl crate::private::Sealed for HALL3 {}
    impl crate::Valid for HALL3 {}
    impl HALL3 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(HALL3)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(HALL0, 0), (HALL1, 1), (HALL2, 2), (HALL3, 3)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod hdma {
    #[doc = "HDMA"]
    pub const HDMA0: *const RegisterBlock = 0xf00c_4000 as *const RegisterBlock;
    #[doc = "XDMA"]
    pub const XDMA1: *const RegisterBlock = 0xf304_8000 as *const RegisterBlock;
    #[path = "blocks/hpm6750/hdma.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type HDMA0 = Instance<0>;
    impl crate::private::Sealed for HDMA0 {}
    impl crate::Valid for HDMA0 {}
    impl HDMA0 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(HDMA0)
        }
    }
    pub type XDMA1 = Instance<1>;
    impl crate::private::Sealed for XDMA1 {}
    impl crate::Valid for XDMA1 {}
    impl XDMA1 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(XDMA1)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(HDMA0, 0), (XDMA1, 1)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod i2c {
    #[doc = "I2C0"]
    pub const I2C0: *const RegisterBlock = 0xf302_0000 as *const RegisterBlock;
    #[doc = "I2C1"]
    pub const I2C1: *const RegisterBlock = 0xf302_4000 as *const RegisterBlock;
    #[doc = "I2C2"]
    pub const I2C2: *const RegisterBlock = 0xf302_8000 as *const RegisterBlock;
    #[doc = "I2C3"]
    pub const I2C3: *const RegisterBlock = 0xf302_c000 as *const RegisterBlock;
    #[path = "blocks/hpm6750/i2c.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type I2C0 = Instance<0>;
    impl crate::private::Sealed for I2C0 {}
    impl crate::Valid for I2C0 {}
    impl I2C0 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(I2C0)
        }
    }
    pub type I2C1 = Instance<1>;
    impl crate::private::Sealed for I2C1 {}
    impl crate::Valid for I2C1 {}
    impl I2C1 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(I2C1)
        }
    }
    pub type I2C2 = Instance<2>;
    impl crate::private::Sealed for I2C2 {}
    impl crate::Valid for I2C2 {}
    impl I2C2 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(I2C2)
        }
    }
    pub type I2C3 = Instance<3>;
    impl crate::private::Sealed for I2C3 {}
    impl crate::Valid for I2C3 {}
    impl I2C3 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(I2C3)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(I2C0, 0), (I2C1, 1), (I2C2, 2), (I2C3, 3)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod i2s {
    #[doc = "I2S0"]
    pub const I2S0: *const RegisterBlock = 0xf010_0000 as *const RegisterBlock;
    #[doc = "I2S1"]
    pub const I2S1: *const RegisterBlock = 0xf010_4000 as *const RegisterBlock;
    #[doc = "I2S2"]
    pub const I2S2: *const RegisterBlock = 0xf010_8000 as *const RegisterBlock;
    #[doc = "I2S3"]
    pub const I2S3: *const RegisterBlock = 0xf010_c000 as *const RegisterBlock;
    #[path = "blocks/hpm6750/i2s.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type I2S0 = Instance<0>;
    impl crate::private::Sealed for I2S0 {}
    impl crate::Valid for I2S0 {}
    impl I2S0 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(I2S0)
        }
    }
    pub type I2S1 = Instance<1>;
    impl crate::private::Sealed for I2S1 {}
    impl crate::Valid for I2S1 {}
    impl I2S1 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(I2S1)
        }
    }
    pub type I2S2 = Instance<2>;
    impl crate::private::Sealed for I2S2 {}
    impl crate::Valid for I2S2 {}
    impl I2S2 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(I2S2)
        }
    }
    pub type I2S3 = Instance<3>;
    impl crate::private::Sealed for I2S3 {}
    impl crate::Valid for I2S3 {}
    impl I2S3 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(I2S3)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(I2S0, 0), (I2S1, 1), (I2S2, 2), (I2S3, 3)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod ioc {
    #[doc = "IOC"]
    pub const IOC0: *const RegisterBlock = 0xf404_0000 as *const RegisterBlock;
    #[doc = "PIOC"]
    pub const PIOC10: *const RegisterBlock = 0xf40d_8000 as *const RegisterBlock;
    #[doc = "BIOC"]
    pub const BIOC11: *const RegisterBlock = 0xf501_0000 as *const RegisterBlock;
    #[path = "blocks/hpm6750/ioc.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type IOC0 = Instance<0>;
    impl crate::private::Sealed for IOC0 {}
    impl crate::Valid for IOC0 {}
    impl IOC0 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(IOC0)
        }
    }
    pub type PIOC10 = Instance<10>;
    impl crate::private::Sealed for PIOC10 {}
    impl crate::Valid for PIOC10 {}
    impl PIOC10 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(PIOC10)
        }
    }
    pub type BIOC11 = Instance<11>;
    impl crate::private::Sealed for BIOC11 {}
    impl crate::Valid for BIOC11 {}
    impl BIOC11 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(BIOC11)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(IOC0, 0), (PIOC10, 10), (BIOC11, 11)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod jpeg {
    #[doc = "JPEG"]
    pub const JPEG: *const RegisterBlock = 0xf101_4000 as *const RegisterBlock;
    #[path = "blocks/hpm6750/jpeg.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type JPEG = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for JPEG {}
    impl crate::Valid for JPEG {}
    impl JPEG {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(JPEG)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, JPEG).then_some(0)
    }
}
#[path = "."]
pub mod keym {
    #[doc = "KEYM"]
    pub const KEYM: *const RegisterBlock = 0xf00c_c000 as *const RegisterBlock;
    #[path = "blocks/hpm6750/keym.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type KEYM = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for KEYM {}
    impl crate::Valid for KEYM {}
    impl KEYM {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(KEYM)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, KEYM).then_some(0)
    }
}
#[path = "."]
pub mod lcdc {
    #[doc = "LCDC"]
    pub const LCDC: *const RegisterBlock = 0xf100_0000 as *const RegisterBlock;
    #[path = "blocks/hpm6750/lcdc.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type LCDC = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for LCDC {}
    impl crate::Valid for LCDC {}
    impl LCDC {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LCDC)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, LCDC).then_some(0)
    }
}
#[path = "."]
pub mod mchtmr {
    #[doc = "MCHTMR"]
    pub const MCHTMR: *const RegisterBlock = 0xe600_0000 as *const RegisterBlock;
    #[path = "blocks/hpm6750/mchtmr.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type MCHTMR = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for MCHTMR {}
    impl crate::Valid for MCHTMR {}
    impl MCHTMR {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(MCHTMR)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, MCHTMR).then_some(0)
    }
}
#[path = "."]
pub mod mono {
    #[doc = "MONO"]
    pub const MONO: *const RegisterBlock = 0xf505_4000 as *const RegisterBlock;
    #[path = "blocks/hpm6750/mono.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type MONO = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for MONO {}
    impl crate::Valid for MONO {}
    impl MONO {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(MONO)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, MONO).then_some(0)
    }
}
#[path = "."]
pub mod otp {
    pub const OTPSHW1: *const RegisterBlock = 0xf408_0000 as *const RegisterBlock;
    #[doc = "OTPSHW"]
    pub const OTP0: *const RegisterBlock = 0xf40c_8000 as *const RegisterBlock;
    #[path = "blocks/hpm6750/otp.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type OTPSHW1 = Instance<1>;
    impl crate::private::Sealed for OTPSHW1 {}
    impl crate::Valid for OTPSHW1 {}
    impl OTPSHW1 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(OTPSHW1)
        }
    }
    pub type OTP0 = Instance<0>;
    impl crate::private::Sealed for OTP0 {}
    impl crate::Valid for OTP0 {}
    impl OTP0 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(OTP0)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(OTPSHW1, 1), (OTP0, 0)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod pcfg {
    #[doc = "PCFG"]
    pub const PCFG: *const RegisterBlock = 0xf40c_4000 as *const RegisterBlock;
    #[path = "blocks/hpm6750/pcfg.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type PCFG = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for PCFG {}
    impl crate::Valid for PCFG {}
    impl PCFG {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(PCFG)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, PCFG).then_some(0)
    }
}
#[path = "."]
pub mod pdm {
    #[doc = "PDM"]
    pub const PDM: *const RegisterBlock = 0xf011_4000 as *const RegisterBlock;
    #[path = "blocks/hpm6750/pdm.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type PDM = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for PDM {}
    impl crate::Valid for PDM {}
    impl PDM {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(PDM)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, PDM).then_some(0)
    }
}
#[path = "."]
pub mod pdma {
    #[doc = "PDMA"]
    pub const PDMA: *const RegisterBlock = 0xf101_0000 as *const RegisterBlock;
    #[path = "blocks/hpm6750/pdma.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type PDMA = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for PDMA {}
    impl crate::Valid for PDMA {}
    impl PDMA {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(PDMA)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, PDMA).then_some(0)
    }
}
#[path = "."]
pub mod pgpr {
    #[doc = "PGPR"]
    pub const PGPR: *const RegisterBlock = 0xf40d_4000 as *const RegisterBlock;
    #[path = "blocks/hpm6750/pgpr.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type PGPR = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for PGPR {}
    impl crate::Valid for PGPR {}
    impl PGPR {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(PGPR)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, PGPR).then_some(0)
    }
}
#[path = "."]
pub mod plic {
    #[doc = "PLIC"]
    pub const PLIC: *const RegisterBlock = 0xe400_0000 as *const RegisterBlock;
    #[path = "blocks/hpm6750/plic.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type PLIC = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for PLIC {}
    impl crate::Valid for PLIC {}
    impl PLIC {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(PLIC)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, PLIC).then_some(0)
    }
}
#[path = "."]
pub mod plicsw {
    #[doc = "PLICSW"]
    pub const PLICSW: *const RegisterBlock = 0xe640_0000 as *const RegisterBlock;
    #[path = "blocks/hpm6750/plicsw.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type PLICSW = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for PLICSW {}
    impl crate::Valid for PLICSW {}
    impl PLICSW {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(PLICSW)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, PLICSW).then_some(0)
    }
}
#[path = "."]
pub mod pllctl {
    #[doc = "PLLCTL"]
    pub const PLLCTL: *const RegisterBlock = 0xf410_0000 as *const RegisterBlock;
    #[path = "blocks/hpm6750/pllctl.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type PLLCTL = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for PLLCTL {}
    impl crate::Valid for PLLCTL {}
    impl PLLCTL {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(PLLCTL)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, PLLCTL).then_some(0)
    }
}
#[path = "."]
pub mod pmon {
    #[doc = "PMON"]
    pub const PMON: *const RegisterBlock = 0xf40d_0000 as *const RegisterBlock;
    #[path = "blocks/hpm6750/pmon.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type PMON = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for PMON {}
    impl crate::Valid for PMON {}
    impl PMON {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(PMON)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, PMON).then_some(0)
    }
}
#[path = "."]
pub mod ppor {
    #[doc = "PPOR"]
    pub const PPOR: *const RegisterBlock = 0xf40c_0000 as *const RegisterBlock;
    #[path = "blocks/hpm6750/ppor.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type PPOR = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for PPOR {}
    impl crate::Valid for PPOR {}
    impl PPOR {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(PPOR)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, PPOR).then_some(0)
    }
}
#[path = "."]
pub mod psec {
    #[doc = "PSEC"]
    pub const PSEC: *const RegisterBlock = 0xf40c_c000 as *const RegisterBlock;
    #[path = "blocks/hpm6750/psec.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type PSEC = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for PSEC {}
    impl crate::Valid for PSEC {}
    impl PSEC {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(PSEC)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, PSEC).then_some(0)
    }
}
#[path = "."]
pub mod ptpc {
    #[doc = "PTPC"]
    pub const PTPC: *const RegisterBlock = 0xf00b_0000 as *const RegisterBlock;
    #[path = "blocks/hpm6750/ptpc.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type PTPC = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for PTPC {}
    impl crate::Valid for PTPC {}
    impl PTPC {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(PTPC)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, PTPC).then_some(0)
    }
}
#[path = "."]
pub mod pwm {
    #[doc = "PWM0"]
    pub const PWM0: *const RegisterBlock = 0xf020_0000 as *const RegisterBlock;
    #[doc = "PWM1"]
    pub const PWM1: *const RegisterBlock = 0xf021_0000 as *const RegisterBlock;
    #[doc = "PWM2"]
    pub const PWM2: *const RegisterBlock = 0xf022_0000 as *const RegisterBlock;
    #[doc = "PWM3"]
    pub const PWM3: *const RegisterBlock = 0xf023_0000 as *const RegisterBlock;
    #[path = "blocks/hpm6750/pwm.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type PWM0 = Instance<0>;
    impl crate::private::Sealed for PWM0 {}
    impl crate::Valid for PWM0 {}
    impl PWM0 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(PWM0)
        }
    }
    pub type PWM1 = Instance<1>;
    impl crate::private::Sealed for PWM1 {}
    impl crate::Valid for PWM1 {}
    impl PWM1 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(PWM1)
        }
    }
    pub type PWM2 = Instance<2>;
    impl crate::private::Sealed for PWM2 {}
    impl crate::Valid for PWM2 {}
    impl PWM2 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(PWM2)
        }
    }
    pub type PWM3 = Instance<3>;
    impl crate::private::Sealed for PWM3 {}
    impl crate::Valid for PWM3 {}
    impl PWM3 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(PWM3)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(PWM0, 0), (PWM1, 1), (PWM2, 2), (PWM3, 3)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod qei {
    #[doc = "QEI0"]
    pub const QEI0: *const RegisterBlock = 0xf020_8000 as *const RegisterBlock;
    #[doc = "QEI1"]
    pub const QEI1: *const RegisterBlock = 0xf021_8000 as *const RegisterBlock;
    #[doc = "QEI2"]
    pub const QEI2: *const RegisterBlock = 0xf022_8000 as *const RegisterBlock;
    #[doc = "QEI3"]
    pub const QEI3: *const RegisterBlock = 0xf023_8000 as *const RegisterBlock;
    #[path = "blocks/hpm6750/qei.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type QEI0 = Instance<0>;
    impl crate::private::Sealed for QEI0 {}
    impl crate::Valid for QEI0 {}
    impl QEI0 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(QEI0)
        }
    }
    pub type QEI1 = Instance<1>;
    impl crate::private::Sealed for QEI1 {}
    impl crate::Valid for QEI1 {}
    impl QEI1 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(QEI1)
        }
    }
    pub type QEI2 = Instance<2>;
    impl crate::private::Sealed for QEI2 {}
    impl crate::Valid for QEI2 {}
    impl QEI2 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(QEI2)
        }
    }
    pub type QEI3 = Instance<3>;
    impl crate::private::Sealed for QEI3 {}
    impl crate::Valid for QEI3 {}
    impl QEI3 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(QEI3)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(QEI0, 0), (QEI1, 1), (QEI2, 2), (QEI3, 3)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod rng {
    #[doc = "RNG"]
    pub const RNG: *const RegisterBlock = 0xf00c_8000 as *const RegisterBlock;
    #[path = "blocks/hpm6750/rng.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type RNG = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for RNG {}
    impl crate::Valid for RNG {}
    impl RNG {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(RNG)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, RNG).then_some(0)
    }
}
#[path = "."]
pub mod rtc {
    pub const RTCSHW1: *const RegisterBlock = 0xf501_c000 as *const RegisterBlock;
    #[doc = "RTCSHW"]
    pub const RTC0: *const RegisterBlock = 0xf504_4000 as *const RegisterBlock;
    #[path = "blocks/hpm6750/rtc.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type RTCSHW1 = Instance<1>;
    impl crate::private::Sealed for RTCSHW1 {}
    impl crate::Valid for RTCSHW1 {}
    impl RTCSHW1 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(RTCSHW1)
        }
    }
    pub type RTC0 = Instance<0>;
    impl crate::private::Sealed for RTC0 {}
    impl crate::Valid for RTC0 {}
    impl RTC0 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(RTC0)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(RTCSHW1, 1), (RTC0, 0)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod sdp {
    #[doc = "SDP"]
    pub const SDP: *const RegisterBlock = 0xf304_c000 as *const RegisterBlock;
    #[path = "blocks/hpm6750/sdp.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type SDP = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for SDP {}
    impl crate::Valid for SDP {}
    impl SDP {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(SDP)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, SDP).then_some(0)
    }
}
#[path = "."]
pub mod sdxc {
    #[doc = "SDXC0"]
    pub const SDXC0: *const RegisterBlock = 0xf203_0000 as *const RegisterBlock;
    #[doc = "SDXC1"]
    pub const SDXC1: *const RegisterBlock = 0xf203_4000 as *const RegisterBlock;
    #[path = "blocks/hpm6750/sdxc.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type SDXC0 = Instance<0>;
    impl crate::private::Sealed for SDXC0 {}
    impl crate::Valid for SDXC0 {}
    impl SDXC0 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(SDXC0)
        }
    }
    pub type SDXC1 = Instance<1>;
    impl crate::private::Sealed for SDXC1 {}
    impl crate::Valid for SDXC1 {}
    impl SDXC1 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(SDXC1)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(SDXC0, 0), (SDXC1, 1)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod spi {
    #[doc = "SPI0"]
    pub const SPI0: *const RegisterBlock = 0xf003_0000 as *const RegisterBlock;
    #[doc = "SPI1"]
    pub const SPI1: *const RegisterBlock = 0xf003_4000 as *const RegisterBlock;
    #[doc = "SPI2"]
    pub const SPI2: *const RegisterBlock = 0xf003_8000 as *const RegisterBlock;
    #[doc = "SPI3"]
    pub const SPI3: *const RegisterBlock = 0xf003_c000 as *const RegisterBlock;
    #[path = "blocks/hpm6750/spi.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type SPI0 = Instance<0>;
    impl crate::private::Sealed for SPI0 {}
    impl crate::Valid for SPI0 {}
    impl SPI0 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(SPI0)
        }
    }
    pub type SPI1 = Instance<1>;
    impl crate::private::Sealed for SPI1 {}
    impl crate::Valid for SPI1 {}
    impl SPI1 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(SPI1)
        }
    }
    pub type SPI2 = Instance<2>;
    impl crate::private::Sealed for SPI2 {}
    impl crate::Valid for SPI2 {}
    impl SPI2 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(SPI2)
        }
    }
    pub type SPI3 = Instance<3>;
    impl crate::private::Sealed for SPI3 {}
    impl crate::Valid for SPI3 {}
    impl SPI3 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(SPI3)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(SPI0, 0), (SPI1, 1), (SPI2, 2), (SPI3, 3)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod synt {
    #[doc = "SYNT"]
    pub const SYNT: *const RegisterBlock = 0xf024_0000 as *const RegisterBlock;
    #[path = "blocks/hpm6750/synt.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type SYNT = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for SYNT {}
    impl crate::Valid for SYNT {}
    impl SYNT {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(SYNT)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, SYNT).then_some(0)
    }
}
#[path = "."]
pub mod sysctl {
    #[doc = "SYSCTL"]
    pub const SYSCTL: *const RegisterBlock = 0xf400_0000 as *const RegisterBlock;
    #[path = "blocks/hpm6750/sysctl.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type SYSCTL = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for SYSCTL {}
    impl crate::Valid for SYSCTL {}
    impl SYSCTL {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(SYSCTL)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, SYSCTL).then_some(0)
    }
}
#[path = "."]
pub mod tamp {
    #[doc = "TAMP"]
    pub const TAMP: *const RegisterBlock = 0xf505_0000 as *const RegisterBlock;
    #[path = "blocks/hpm6750/tamp.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type TAMP = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for TAMP {}
    impl crate::Valid for TAMP {}
    impl TAMP {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(TAMP)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, TAMP).then_some(0)
    }
}
#[path = "."]
pub mod trgm {
    #[doc = "TRGM0"]
    pub const TRGM0: *const RegisterBlock = 0xf020_c000 as *const RegisterBlock;
    #[doc = "TRGM1"]
    pub const TRGM1: *const RegisterBlock = 0xf021_c000 as *const RegisterBlock;
    #[doc = "TRGM2"]
    pub const TRGM2: *const RegisterBlock = 0xf022_c000 as *const RegisterBlock;
    #[doc = "TRGM3"]
    pub const TRGM3: *const RegisterBlock = 0xf023_c000 as *const RegisterBlock;
    #[path = "blocks/hpm6750/trgm.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type TRGM0 = Instance<0>;
    impl crate::private::Sealed for TRGM0 {}
    impl crate::Valid for TRGM0 {}
    impl TRGM0 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(TRGM0)
        }
    }
    pub type TRGM1 = Instance<1>;
    impl crate::private::Sealed for TRGM1 {}
    impl crate::Valid for TRGM1 {}
    impl TRGM1 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(TRGM1)
        }
    }
    pub type TRGM2 = Instance<2>;
    impl crate::private::Sealed for TRGM2 {}
    impl crate::Valid for TRGM2 {}
    impl TRGM2 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(TRGM2)
        }
    }
    pub type TRGM3 = Instance<3>;
    impl crate::private::Sealed for TRGM3 {}
    impl crate::Valid for TRGM3 {}
    impl TRGM3 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(TRGM3)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(TRGM0, 0), (TRGM1, 1), (TRGM2, 2), (TRGM3, 3)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod uart {
    #[doc = "UART0"]
    pub const UART0: *const RegisterBlock = 0xf004_0000 as *const RegisterBlock;
    #[doc = "UART1"]
    pub const UART1: *const RegisterBlock = 0xf004_4000 as *const RegisterBlock;
    #[doc = "UART2"]
    pub const UART2: *const RegisterBlock = 0xf004_8000 as *const RegisterBlock;
    #[doc = "UART3"]
    pub const UART3: *const RegisterBlock = 0xf004_c000 as *const RegisterBlock;
    #[doc = "UART4"]
    pub const UART4: *const RegisterBlock = 0xf005_0000 as *const RegisterBlock;
    #[doc = "UART5"]
    pub const UART5: *const RegisterBlock = 0xf005_4000 as *const RegisterBlock;
    #[doc = "UART6"]
    pub const UART6: *const RegisterBlock = 0xf005_8000 as *const RegisterBlock;
    #[doc = "UART7"]
    pub const UART7: *const RegisterBlock = 0xf005_c000 as *const RegisterBlock;
    #[doc = "UART8"]
    pub const UART8: *const RegisterBlock = 0xf006_0000 as *const RegisterBlock;
    #[doc = "UART9"]
    pub const UART9: *const RegisterBlock = 0xf006_4000 as *const RegisterBlock;
    #[doc = "UART10"]
    pub const UART10: *const RegisterBlock = 0xf006_8000 as *const RegisterBlock;
    #[doc = "UART11"]
    pub const UART11: *const RegisterBlock = 0xf006_c000 as *const RegisterBlock;
    #[doc = "UART12"]
    pub const UART12: *const RegisterBlock = 0xf007_0000 as *const RegisterBlock;
    #[doc = "UART13"]
    pub const UART13: *const RegisterBlock = 0xf007_4000 as *const RegisterBlock;
    #[doc = "UART14"]
    pub const UART14: *const RegisterBlock = 0xf007_8000 as *const RegisterBlock;
    #[doc = "UART15"]
    pub const UART15: *const RegisterBlock = 0xf007_c000 as *const RegisterBlock;
    #[doc = "PUART"]
    pub const PUART20: *const RegisterBlock = 0xf40e_4000 as *const RegisterBlock;
    #[path = "blocks/hpm6750/uart.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type UART0 = Instance<0>;
    impl crate::private::Sealed for UART0 {}
    impl crate::Valid for UART0 {}
    impl UART0 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(UART0)
        }
    }
    pub type UART1 = Instance<1>;
    impl crate::private::Sealed for UART1 {}
    impl crate::Valid for UART1 {}
    impl UART1 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(UART1)
        }
    }
    pub type UART2 = Instance<2>;
    impl crate::private::Sealed for UART2 {}
    impl crate::Valid for UART2 {}
    impl UART2 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(UART2)
        }
    }
    pub type UART3 = Instance<3>;
    impl crate::private::Sealed for UART3 {}
    impl crate::Valid for UART3 {}
    impl UART3 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(UART3)
        }
    }
    pub type UART4 = Instance<4>;
    impl crate::private::Sealed for UART4 {}
    impl crate::Valid for UART4 {}
    impl UART4 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(UART4)
        }
    }
    pub type UART5 = Instance<5>;
    impl crate::private::Sealed for UART5 {}
    impl crate::Valid for UART5 {}
    impl UART5 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(UART5)
        }
    }
    pub type UART6 = Instance<6>;
    impl crate::private::Sealed for UART6 {}
    impl crate::Valid for UART6 {}
    impl UART6 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(UART6)
        }
    }
    pub type UART7 = Instance<7>;
    impl crate::private::Sealed for UART7 {}
    impl crate::Valid for UART7 {}
    impl UART7 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(UART7)
        }
    }
    pub type UART8 = Instance<8>;
    impl crate::private::Sealed for UART8 {}
    impl crate::Valid for UART8 {}
    impl UART8 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(UART8)
        }
    }
    pub type UART9 = Instance<9>;
    impl crate::private::Sealed for UART9 {}
    impl crate::Valid for UART9 {}
    impl UART9 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(UART9)
        }
    }
    pub type UART10 = Instance<10>;
    impl crate::private::Sealed for UART10 {}
    impl crate::Valid for UART10 {}
    impl UART10 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(UART10)
        }
    }
    pub type UART11 = Instance<11>;
    impl crate::private::Sealed for UART11 {}
    impl crate::Valid for UART11 {}
    impl UART11 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(UART11)
        }
    }
    pub type UART12 = Instance<12>;
    impl crate::private::Sealed for UART12 {}
    impl crate::Valid for UART12 {}
    impl UART12 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(UART12)
        }
    }
    pub type UART13 = Instance<13>;
    impl crate::private::Sealed for UART13 {}
    impl crate::Valid for UART13 {}
    impl UART13 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(UART13)
        }
    }
    pub type UART14 = Instance<14>;
    impl crate::private::Sealed for UART14 {}
    impl crate::Valid for UART14 {}
    impl UART14 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(UART14)
        }
    }
    pub type UART15 = Instance<15>;
    impl crate::private::Sealed for UART15 {}
    impl crate::Valid for UART15 {}
    impl UART15 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(UART15)
        }
    }
    pub type PUART20 = Instance<20>;
    impl crate::private::Sealed for PUART20 {}
    impl crate::Valid for PUART20 {}
    impl PUART20 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(PUART20)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [
            (UART0, 0),
            (UART1, 1),
            (UART2, 2),
            (UART3, 3),
            (UART4, 4),
            (UART5, 5),
            (UART6, 6),
            (UART7, 7),
            (UART8, 8),
            (UART9, 9),
            (UART10, 10),
            (UART11, 11),
            (UART12, 12),
            (UART13, 13),
            (UART14, 14),
            (UART15, 15),
            (PUART20, 20),
        ]
        .into_iter()
        .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
        .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod usb {
    #[doc = "USB0"]
    pub const USB0: *const RegisterBlock = 0xf202_0000 as *const RegisterBlock;
    #[doc = "USB1"]
    pub const USB1: *const RegisterBlock = 0xf202_4000 as *const RegisterBlock;
    #[path = "blocks/hpm6750/usb.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type USB0 = Instance<0>;
    impl crate::private::Sealed for USB0 {}
    impl crate::Valid for USB0 {}
    impl USB0 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(USB0)
        }
    }
    pub type USB1 = Instance<1>;
    impl crate::private::Sealed for USB1 {}
    impl crate::Valid for USB1 {}
    impl USB1 {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(USB1)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(USB0, 0), (USB1, 1)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod vad {
    #[doc = "VAD"]
    pub const VAD: *const RegisterBlock = 0xf40e_c000 as *const RegisterBlock;
    #[path = "blocks/hpm6750/vad.rs"]
    mod blocks;
    pub use blocks::*;
    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;
    pub type VAD = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::private::Sealed for VAD {}
    impl crate::Valid for VAD {}
    impl VAD {
        #[doc = r" Acquire a vaild, but possibly aliased, instance."]
        #[doc = r""]
        #[doc = r" # Safety"]
        #[doc = r""]
        #[doc = r" See [the struct-level safety documentation](crate::Instance)."]
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(VAD)
        }
    }
    #[doc = r" Returns the instance number `N` for a peripheral instance."]
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, VAD).then_some(0)
    }
}
#[doc = r"Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 7;
#[doc = r" Instances for all of this device's peripherals."]
#[doc = r""]
#[doc = r" Use this if you want a single way to acquire *all* instances"]
#[doc = r" for your device."]
pub struct Instances {
    pub ACMP: acmp::ACMP,
    pub ADC0: adc::ADC0,
    pub ADC1: adc::ADC1,
    pub ADC2: adc::ADC2,
    pub ADC3: adc3::ADC3,
    pub BCFG: bcfg::BCFG,
    pub BGPR: bgpr::BGPR,
    pub BKEY: bkey::BKEY,
    pub BMON: bmon::BMON,
    pub BPOR: bpor::BPOR,
    pub BSEC: bsec::BSEC,
    pub BUTN: butn::BUTN,
    pub CAM0: cam::CAM0,
    pub CAM1: cam::CAM1,
    pub CAN0: can::CAN0,
    pub CAN1: can::CAN1,
    pub CAN2: can::CAN2,
    pub CAN3: can::CAN3,
    pub CONCTL: conctl::CONCTL,
    pub DAO: dao::DAO,
    pub DMAMUX: dmamux::DMAMUX,
    pub DRAM: dram::DRAM,
    pub ENET0: enet::ENET0,
    pub ENET1: enet::ENET1,
    pub FGPIO10: gpio::FGPIO10,
    pub GPIO0: gpio::GPIO0,
    pub GPIO1: gpio::GPIO1,
    pub PGPIO11: gpio::PGPIO11,
    pub BGPIO12: gpio::BGPIO12,
    pub GPIOM: gpiom::GPIOM,
    pub NTMR10: gptmr::NTMR10,
    pub NTMR11: gptmr::NTMR11,
    pub GPTMR0: gptmr::GPTMR0,
    pub GPTMR1: gptmr::GPTMR1,
    pub GPTMR2: gptmr::GPTMR2,
    pub GPTMR3: gptmr::GPTMR3,
    pub GPTMR4: gptmr::GPTMR4,
    pub GPTMR5: gptmr::GPTMR5,
    pub GPTMR6: gptmr::GPTMR6,
    pub GPTMR7: gptmr::GPTMR7,
    pub PTMR12: gptmr::PTMR12,
    pub HALL0: hall::HALL0,
    pub HALL1: hall::HALL1,
    pub HALL2: hall::HALL2,
    pub HALL3: hall::HALL3,
    pub HDMA0: hdma::HDMA0,
    pub XDMA1: hdma::XDMA1,
    pub I2C0: i2c::I2C0,
    pub I2C1: i2c::I2C1,
    pub I2C2: i2c::I2C2,
    pub I2C3: i2c::I2C3,
    pub I2S0: i2s::I2S0,
    pub I2S1: i2s::I2S1,
    pub I2S2: i2s::I2S2,
    pub I2S3: i2s::I2S3,
    pub IOC0: ioc::IOC0,
    pub PIOC10: ioc::PIOC10,
    pub BIOC11: ioc::BIOC11,
    pub JPEG: jpeg::JPEG,
    pub KEYM: keym::KEYM,
    pub LCDC: lcdc::LCDC,
    pub MCHTMR: mchtmr::MCHTMR,
    pub MONO: mono::MONO,
    pub OTPSHW1: otp::OTPSHW1,
    pub OTP0: otp::OTP0,
    pub PCFG: pcfg::PCFG,
    pub PDM: pdm::PDM,
    pub PDMA: pdma::PDMA,
    pub PGPR: pgpr::PGPR,
    pub PLIC: plic::PLIC,
    pub PLICSW: plicsw::PLICSW,
    pub PLLCTL: pllctl::PLLCTL,
    pub PMON: pmon::PMON,
    pub PPOR: ppor::PPOR,
    pub PSEC: psec::PSEC,
    pub PTPC: ptpc::PTPC,
    pub PWM0: pwm::PWM0,
    pub PWM1: pwm::PWM1,
    pub PWM2: pwm::PWM2,
    pub PWM3: pwm::PWM3,
    pub QEI0: qei::QEI0,
    pub QEI1: qei::QEI1,
    pub QEI2: qei::QEI2,
    pub QEI3: qei::QEI3,
    pub RNG: rng::RNG,
    pub RTCSHW1: rtc::RTCSHW1,
    pub RTC0: rtc::RTC0,
    pub SDP: sdp::SDP,
    pub SDXC0: sdxc::SDXC0,
    pub SDXC1: sdxc::SDXC1,
    pub SPI0: spi::SPI0,
    pub SPI1: spi::SPI1,
    pub SPI2: spi::SPI2,
    pub SPI3: spi::SPI3,
    pub SYNT: synt::SYNT,
    pub SYSCTL: sysctl::SYSCTL,
    pub TAMP: tamp::TAMP,
    pub TRGM0: trgm::TRGM0,
    pub TRGM1: trgm::TRGM1,
    pub TRGM2: trgm::TRGM2,
    pub TRGM3: trgm::TRGM3,
    pub UART0: uart::UART0,
    pub UART1: uart::UART1,
    pub UART2: uart::UART2,
    pub UART3: uart::UART3,
    pub UART4: uart::UART4,
    pub UART5: uart::UART5,
    pub UART6: uart::UART6,
    pub UART7: uart::UART7,
    pub UART8: uart::UART8,
    pub UART9: uart::UART9,
    pub UART10: uart::UART10,
    pub UART11: uart::UART11,
    pub UART12: uart::UART12,
    pub UART13: uart::UART13,
    pub UART14: uart::UART14,
    pub UART15: uart::UART15,
    pub PUART20: uart::PUART20,
    pub USB0: usb::USB0,
    pub USB1: usb::USB1,
    pub VAD: vad::VAD,
}
impl Instances {
    #[doc = r" Acquire all peripheral instances."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Since this calls `instance()` to initialize each of its members,"]
    #[doc = r" the `instance()` safety contract applies. See [the `Instance` safety"]
    #[doc = r" documentation](crate::Instance) for more information."]
    #[inline]
    pub const unsafe fn instances() -> Self {
        Self {
            ACMP: acmp::ACMP::instance(),
            ADC0: adc::ADC0::instance(),
            ADC1: adc::ADC1::instance(),
            ADC2: adc::ADC2::instance(),
            ADC3: adc3::ADC3::instance(),
            BCFG: bcfg::BCFG::instance(),
            BGPR: bgpr::BGPR::instance(),
            BKEY: bkey::BKEY::instance(),
            BMON: bmon::BMON::instance(),
            BPOR: bpor::BPOR::instance(),
            BSEC: bsec::BSEC::instance(),
            BUTN: butn::BUTN::instance(),
            CAM0: cam::CAM0::instance(),
            CAM1: cam::CAM1::instance(),
            CAN0: can::CAN0::instance(),
            CAN1: can::CAN1::instance(),
            CAN2: can::CAN2::instance(),
            CAN3: can::CAN3::instance(),
            CONCTL: conctl::CONCTL::instance(),
            DAO: dao::DAO::instance(),
            DMAMUX: dmamux::DMAMUX::instance(),
            DRAM: dram::DRAM::instance(),
            ENET0: enet::ENET0::instance(),
            ENET1: enet::ENET1::instance(),
            FGPIO10: gpio::FGPIO10::instance(),
            GPIO0: gpio::GPIO0::instance(),
            GPIO1: gpio::GPIO1::instance(),
            PGPIO11: gpio::PGPIO11::instance(),
            BGPIO12: gpio::BGPIO12::instance(),
            GPIOM: gpiom::GPIOM::instance(),
            NTMR10: gptmr::NTMR10::instance(),
            NTMR11: gptmr::NTMR11::instance(),
            GPTMR0: gptmr::GPTMR0::instance(),
            GPTMR1: gptmr::GPTMR1::instance(),
            GPTMR2: gptmr::GPTMR2::instance(),
            GPTMR3: gptmr::GPTMR3::instance(),
            GPTMR4: gptmr::GPTMR4::instance(),
            GPTMR5: gptmr::GPTMR5::instance(),
            GPTMR6: gptmr::GPTMR6::instance(),
            GPTMR7: gptmr::GPTMR7::instance(),
            PTMR12: gptmr::PTMR12::instance(),
            HALL0: hall::HALL0::instance(),
            HALL1: hall::HALL1::instance(),
            HALL2: hall::HALL2::instance(),
            HALL3: hall::HALL3::instance(),
            HDMA0: hdma::HDMA0::instance(),
            XDMA1: hdma::XDMA1::instance(),
            I2C0: i2c::I2C0::instance(),
            I2C1: i2c::I2C1::instance(),
            I2C2: i2c::I2C2::instance(),
            I2C3: i2c::I2C3::instance(),
            I2S0: i2s::I2S0::instance(),
            I2S1: i2s::I2S1::instance(),
            I2S2: i2s::I2S2::instance(),
            I2S3: i2s::I2S3::instance(),
            IOC0: ioc::IOC0::instance(),
            PIOC10: ioc::PIOC10::instance(),
            BIOC11: ioc::BIOC11::instance(),
            JPEG: jpeg::JPEG::instance(),
            KEYM: keym::KEYM::instance(),
            LCDC: lcdc::LCDC::instance(),
            MCHTMR: mchtmr::MCHTMR::instance(),
            MONO: mono::MONO::instance(),
            OTPSHW1: otp::OTPSHW1::instance(),
            OTP0: otp::OTP0::instance(),
            PCFG: pcfg::PCFG::instance(),
            PDM: pdm::PDM::instance(),
            PDMA: pdma::PDMA::instance(),
            PGPR: pgpr::PGPR::instance(),
            PLIC: plic::PLIC::instance(),
            PLICSW: plicsw::PLICSW::instance(),
            PLLCTL: pllctl::PLLCTL::instance(),
            PMON: pmon::PMON::instance(),
            PPOR: ppor::PPOR::instance(),
            PSEC: psec::PSEC::instance(),
            PTPC: ptpc::PTPC::instance(),
            PWM0: pwm::PWM0::instance(),
            PWM1: pwm::PWM1::instance(),
            PWM2: pwm::PWM2::instance(),
            PWM3: pwm::PWM3::instance(),
            QEI0: qei::QEI0::instance(),
            QEI1: qei::QEI1::instance(),
            QEI2: qei::QEI2::instance(),
            QEI3: qei::QEI3::instance(),
            RNG: rng::RNG::instance(),
            RTCSHW1: rtc::RTCSHW1::instance(),
            RTC0: rtc::RTC0::instance(),
            SDP: sdp::SDP::instance(),
            SDXC0: sdxc::SDXC0::instance(),
            SDXC1: sdxc::SDXC1::instance(),
            SPI0: spi::SPI0::instance(),
            SPI1: spi::SPI1::instance(),
            SPI2: spi::SPI2::instance(),
            SPI3: spi::SPI3::instance(),
            SYNT: synt::SYNT::instance(),
            SYSCTL: sysctl::SYSCTL::instance(),
            TAMP: tamp::TAMP::instance(),
            TRGM0: trgm::TRGM0::instance(),
            TRGM1: trgm::TRGM1::instance(),
            TRGM2: trgm::TRGM2::instance(),
            TRGM3: trgm::TRGM3::instance(),
            UART0: uart::UART0::instance(),
            UART1: uart::UART1::instance(),
            UART2: uart::UART2::instance(),
            UART3: uart::UART3::instance(),
            UART4: uart::UART4::instance(),
            UART5: uart::UART5::instance(),
            UART6: uart::UART6::instance(),
            UART7: uart::UART7::instance(),
            UART8: uart::UART8::instance(),
            UART9: uart::UART9::instance(),
            UART10: uart::UART10::instance(),
            UART11: uart::UART11::instance(),
            UART12: uart::UART12::instance(),
            UART13: uart::UART13::instance(),
            UART14: uart::UART14::instance(),
            UART15: uart::UART15::instance(),
            PUART20: uart::PUART20::instance(),
            USB0: usb::USB0::instance(),
            USB1: usb::USB1::instance(),
            VAD: vad::VAD::instance(),
        }
    }
}
