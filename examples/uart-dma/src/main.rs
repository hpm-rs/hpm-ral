#![no_std]
#![no_main]

extern crate panic_halt;

use core::fmt::Write;
use hpm6750evkmini as bsp;
use hpm_ral as ral;
use riscv_rt::entry;

use bsp::dma::Dma;
use bsp::uart::Uart;

#[entry]
fn main() -> ! {
    let sysctl = unsafe { ral::sysctl::SYSCTL::instance() };
    let ioc = unsafe { ral::ioc::IOC0::instance() };
    let pioc = unsafe { ral::ioc::PIOC10::instance() };
    bsp::board_init(&sysctl);
    bsp::board_init_uart0_pins(&ioc, &pioc);

    let dma = unsafe { Dma::new(ral::dma::HDMA0::instance(), ral::dmamux::DMAMUX::instance()) };
    let mut uart = Uart::new(unsafe { ral::uart::UART0::instance() }, Some(&dma));
    uart.setup(115_200, 24_000_000);

    let mut cnt = 0;
    loop {
        write!(uart, "Hello hpm-ral: {cnt}\r\n").unwrap();
        cnt += 1;
    }
}
