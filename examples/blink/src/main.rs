#![no_std]
#![no_main]

extern crate panic_halt;

use hpm_ral as ral;
use riscv_rt::entry;

fn board_init() {
    let sysctl = unsafe { ral::sysctl::SYSCTL::instance() };

    ral::modify_reg!(ral::sysctl, sysctl, GROUP0_0_VALUE, AXI_SRAM1: Linked);
    ral::modify_reg!(ral::sysctl, sysctl, GROUP0_0_VALUE, GPIO0_1: Linked);

    board_turn_off_rgb_led();
}

fn board_turn_off_rgb_led() {
    let ioc = unsafe { ral::ioc::IOC0::instance() };

    ral::modify_reg!(ral::ioc, ioc, PAD_PD15_FUNC_CTL, ALT_SELECT: 0);
    ral::modify_reg!(ral::ioc, ioc, PAD_PD15_PAD_CTL, PE: Enable, PS: Down);
}

fn board_init_led_pins() {
    let gpio0 = unsafe { ral::gpio::GPIO0::instance() };

    ral::write_reg!(ral::gpio, gpio0, DO_GPIOD_SET, 1 << 15);
    ral::write_reg!(ral::gpio, gpio0, OE_GPIOD_SET, 1 << 15);
}

#[entry]
fn main() -> ! {
    board_init();
    board_init_led_pins();

    let gpio0 = unsafe { ral::gpio::GPIO0::instance() };

    loop {
        ral::write_reg!(ral::gpio, gpio0, DO_GPIOD_TOGGLE, 1 << 15);
        for _ in 0..50000 {}
    }
}
