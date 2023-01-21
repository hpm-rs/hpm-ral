#![no_std]
#![no_main]

extern crate panic_halt;

use core::fmt::Write;

use hpm_ral as ral;
use riscv_rt::entry;

struct Uart<const N: u8> {
    inner: ral::uart::Instance<N>,
}

impl<const N: u8> Uart<N> {
    fn new(uart: ral::uart::Instance<N>) -> Self {
        Self { inner: uart }
    }

    fn init(&self, buadrate: u32, clock_src_freq: u32) {
        // Disable all interrupt
        ral::write_reg!(ral::uart, self.inner, DLM, 0);
        // Set DLAB to 1
        ral::modify_reg!(ral::uart, self.inner, LCR, DLAB: 1);

        let div = clock_src_freq / (buadrate * 16);
        ral::modify_reg!(ral::uart, self.inner, DLL, DLL: div);
        ral::modify_reg!(ral::uart, self.inner, DLM, DLM: div >> 8);

        // Set DLAB to 0
        ral::modify_reg!(ral::uart, self.inner, LCR, DLAB: 0);
        // Word length to 8
        ral::modify_reg!(ral::uart, self.inner, LCR, WLS: Bits8);
        // Enable TX and RX FIFO
        ral::modify_reg!(ral::uart, self.inner, FCR, FIFOE: 1);
    }

    fn send_byte(&self, byte: u8) {
        while ral::read_reg!(ral::uart, self.inner, LSR, THRE) == 0 {}
        ral::write_reg!(ral::uart, self.inner, DLL, DLL: byte as u32);
    }
}

impl<const N: u8> Write for Uart<N> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for &ch in s.as_bytes() {
            self.send_byte(ch);
        }
        Ok(())
    }
}

fn board_init(sysctl: &ral::sysctl::SYSCTL) {
    ral::modify_reg!(ral::sysctl, sysctl, GROUP0_0_VALUE, AXI_SRAM1: Linked);
    ral::modify_reg!(ral::sysctl, sysctl, GROUP0_0_VALUE, GPIO0_1: Linked);
    ral::modify_reg!(ral::sysctl, sysctl, GROUP0_1_VALUE, UARTO: Linked);
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
    let mut uart = Uart::new(unsafe { ral::uart::UART0::instance() });
    let mut cnt = 0;

    board_init(&sysctl);
    board_init_uart_pins(&ioc, &pioc);
    uart.init(115_200, 24_000_000);

    loop {
        write!(uart, "Hello, hpm-ral: {cnt}\r\n").unwrap();
        cnt += 1;
    }
}
