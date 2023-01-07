#[doc = "PSEC"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Secure state"]
    pub SECURE_STATE: crate::RWRegister<u32>,
    #[doc = "secure state configuration"]
    pub SECURE_STATE_CONFIG: crate::RWRegister<u32>,
    #[doc = "Security violation config"]
    pub VIOLATION_CONFIG: crate::RWRegister<u32>,
    #[doc = "Escalate behavior on security event"]
    pub ESCALATE_CONFIG: crate::RWRegister<u32>,
    #[doc = "Event and escalate status"]
    pub EVENT: crate::RWRegister<u32>,
    #[doc = "Lifecycle"]
    pub LIFECYCLE: crate::RWRegister<u32>,
}
#[doc = "Secure state"]
pub mod SECURE_STATE {
    #[doc = "PMIC secure state one hot indicator 0: secure state is not in inspect state 1: secure state is in inspect state"]
    pub mod PMIC_INS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PMIC secure state one hot indicator 0: secure state is not in secure state 1: secure state is in secure state"]
    pub mod PMIC_SEC {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PMIC secure state one hot indicator 0: secure state is not in non-secure state 1: secure state is in non-secure state"]
    pub mod PMIC_NSC {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PMIC secure state one hot indicator 0: secure state is not in fail state 1: secure state is in fail state"]
    pub mod PMIC_FAIL {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Secure state allow 0: system is not healthy to enter secure state, request to enter non-secure state will cause a fail state 1: system is healthy to enter secure state"]
    pub mod ALLOW_SEC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Non-secure state allow 0: system is not healthy to enter non-secure state, request to enter non-secure state will cause a fail state 1: system is healthy to enter non-secure state"]
    pub mod ALLOW_NSC {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "secure state configuration"]
pub mod SECURE_STATE_CONFIG {
    #[doc = "allow secure state restart from fail state 0: restart is not allowed, only hardware reset can recover secure state 1: software is allowed to switch to inspect state from fail state"]
    pub mod ALLOW_RESTART {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Lock bit of allow restart setting, once locked, lock bit itself and configuration register will keep value until next reset 0: not locked, register can be modified 1: register locked, write access to the register is ignored"]
    pub mod LOCK {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Security violation config"]
pub mod VIOLATION_CONFIG {
    #[doc = "configuration of secure state violations, each bit represents one security event 0: event is not a security violation 1: event is a security violation"]
    pub mod SEC_VIO_CFG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Lock bit secure violation setting, once locked, lock bit itself and configuration will keep value until next reset 0: not locked, configuration can be modified 1: register locked, write access to the configuration is ignored"]
    pub mod LOCK_SEC {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "configuration of non-secure state violations, each bit represents one security event 0: event is not a security violation 1: event is a security violation"]
    pub mod NSC_VIO_CFG {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x7fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Lock bit non-secure violation setting, once locked, lock bit itself and configuration will keep value until next reset 0: not locked, configuration can be modified 1: register locked, write access to the configuration is ignored"]
    pub mod LOCK_NSC {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Escalate behavior on security event"]
pub mod ESCALATE_CONFIG {
    #[doc = "configuration of secure state escalates, each bit represents one security event 0: event is not a security escalate 1: event is a security escalate"]
    pub mod SEC_VIO_CFG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Lock bit secure escalate setting, once locked, lock bit itself and configuration will keep value until next reset 0: not locked, configuration can be modified 1: register locked, write access to the configuration is ignored"]
    pub mod LOCK_SEC {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "configuration of non-secure state escalates, each bit represents one security event 0: event is not a security escalate 1: event is a security escalate"]
    pub mod NSC_VIO_CFG {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x7fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Lock bit non-secure escalate setting, once locked, lock bit itself and configuration will keep value until next reset 0: not locked, configuration can be modified 1: register locked, write access to the configuration is ignored"]
    pub mod LOCK_NSC {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Event and escalate status"]
pub mod EVENT {
    #[doc = "PMIC is escalting secure event"]
    pub mod PMIC_ESC_SEC {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PMIC is escalating non-secure event"]
    pub mod PMIC_ESC_NSC {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "local event statue, each bit represents one security event"]
    pub mod EVENT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Lifecycle"]
pub mod LIFECYCLE {
    #[doc = "lifecycle status, bit7: lifecycle_debate, bit6: lifecycle_scribe, bit5: lifecycle_no_ret, bit4: lifecycle_return, bit3: lifecycle_secure, bit2: lifecycle_nonsec, bit1: lifecycle_create, bit0: lifecycle_unknow"]
    pub mod LIFECYCLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
