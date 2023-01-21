#![no_std]
#![no_main]

mod dma;
mod uart;

extern crate panic_halt;

use core::fmt::Write;
use hpm_ral as ral;
use riscv_rt::entry;

use dma::Dma;
use uart::Uart;

fn board_init(sysctl: &ral::sysctl::SYSCTL) {
    ral::modify_reg!(ral::sysctl, sysctl, GROUP0_0_VALUE, GPIO0_1: Linked);
    ral::modify_reg!(ral::sysctl, sysctl, GROUP0_1_VALUE, UARTO: Linked);
    ral::modify_reg!(ral::sysctl, sysctl, GROUP0_0_VALUE, HDMA: Linked);
    // Set UART0 clock source to osc24 and divider to 1 (24 MHz)
    ral::modify_reg!(ral::sysctl, sysctl, CLOCK_CLK_TOP_UART0, MUX: 0, DIV: 0);
}

fn board_init_uart_pins(ioc: &ral::ioc::IOC0, pioc: &ral::ioc::PIOC10) {
    ral::modify_reg!(ral::ioc, ioc, PAD_PY06_FUNC_CTL, ALT_SELECT: 2);
    ral::modify_reg!(ral::ioc, ioc, PAD_PY07_FUNC_CTL, ALT_SELECT: 2);
    // PY port IO needs to configure PIOC as well
    ral::modify_reg!(ral::ioc, pioc, PAD_PY06_FUNC_CTL, ALT_SELECT: 3);
    ral::modify_reg!(ral::ioc, pioc, PAD_PY07_FUNC_CTL, ALT_SELECT: 3);
}

#[entry]
fn main() -> ! {
    let sysctl = unsafe { ral::sysctl::SYSCTL::instance() };
    let ioc = unsafe { ral::ioc::IOC0::instance() };
    let pioc = unsafe { ral::ioc::PIOC10::instance() };
    board_init(&sysctl);
    board_init_uart_pins(&ioc, &pioc);

    let dma = unsafe { Dma::new(ral::dma::HDMA0::instance(), ral::dmamux::DMAMUX::instance()) };
    let mut uart = Uart::new(unsafe { ral::uart::UART0::instance() }, &dma);
    uart.init(115_200, 24_000_000);

    let mut cnt = 0;
    loop {
        write!(uart, "Hello hpm-ral: {cnt}\r\n").unwrap();
        cnt += 1;
        for _ in 0..50000 {}
    }
}
