#![no_std]
#![no_main]

extern crate panic_halt;

use hpm6750evkmini as bsp;
use hpm_ral as ral;
use riscv_rt::entry;

fn init_led_gpio(gpio0: &ral::gpio::GPIO0) {
    ral::write_reg!(ral::gpio, gpio0, DO_GPIOD_SET, 1 << 15);
    ral::write_reg!(ral::gpio, gpio0, OE_GPIOD_SET, 1 << 15);
}

#[entry]
fn main() -> ! {
    let sysctl = unsafe { ral::sysctl::SYSCTL::instance() };
    let gpio0 = unsafe { ral::gpio::GPIO0::instance() };
    let ioc = unsafe { ral::ioc::IOC0::instance() };

    bsp::board_init(&sysctl);
    bsp::board_init_led_pins(&ioc);
    init_led_gpio(&gpio0);

    loop {
        ral::write_reg!(ral::gpio, gpio0, DO_GPIOD_TOGGLE, 1 << 15);
        for _ in 0..50000 {}
    }
}
