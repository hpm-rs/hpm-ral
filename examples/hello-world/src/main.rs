#![no_std]
#![no_main]

extern crate panic_halt;

use core::fmt::Write;

use bsp::uart::Uart;
use hpm6750evkmini as bsp;
use hpm_ral as ral;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let sysctl = unsafe { ral::sysctl::SYSCTL::instance() };
    let ioc = unsafe { ral::ioc::IOC0::instance() };
    let pioc = unsafe { ral::ioc::PIOC10::instance() };
    let mut uart = Uart::new(unsafe { ral::uart::UART0::instance() }, None);
    let mut cnt = 0;

    bsp::board_init(&sysctl);
    bsp::board_init_uart0_pins(&ioc, &pioc);
    uart.setup(115_200, 24_000_000);

    loop {
        write!(uart, "Hello, hpm-ral: {cnt}\r\n").unwrap();
        cnt += 1;
        for _ in 0..50000 {}
    }
}
