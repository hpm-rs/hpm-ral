#![no_std]
#![no_main]

extern crate panic_halt;

use hpm6750evkmini as bsp;
use hpm_ral as ral;

use bsp::gpio::{Gpio, PinState};
use embedded_hal::blocking::delay::DelayMs;
use hpm_rt::entry;

use riscv::delay::McycleDelay;

#[entry]
fn main() -> ! {
    let sysctl = unsafe { ral::sysctl::SYSCTL::instance() };
    let gpio = unsafe { Gpio::new(ral::gpio::GPIO0::instance(), ral::ioc::IOC0::instance()) };
    let mut delay = McycleDelay::new(324_000_000);

    bsp::board_init(&sysctl);
    let led_g = gpio.pb18.into_push_pull_output();
    let led_r = gpio.pb19.into_push_pull_output();
    let led_b = gpio.pb20.into_push_pull_output();

    led_g.set_state(PinState::High);
    led_r.set_state(PinState::High);

    loop {
        led_b.toggle();
        delay.delay_ms(500);
    }
}
