use hpm_ral::_trap::{PLIC_BASE, __EXTERNAL_INTERRUPTS};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref PLIC: &'static plic::Plic = unsafe { &*(PLIC_BASE) };
}

#[export_name = "MachineExternal"]
fn machine_external_handler() {
    extern "C" {
        fn DefaultHandler();
    }

    unsafe {
        if let Some(source) = PLIC.claim(0) {
            let h = &__EXTERNAL_INTERRUPTS[u32::from(source) as usize];
            if h._reserved != 0 {
                (h._handler)();
            } else {
                DefaultHandler();
            }
            PLIC.complete(0, source);
        }
    }
}
