#![no_std]
#![no_main]

mod ext_int;

extern crate panic_halt;

use core::cell::RefCell;
use core::fmt::Write;

use critical_section::Mutex;
use hpm_ral as ral;
use riscv::interrupt;
use riscv::register;
use riscv_rt::entry;

use ext_int::PLIC;

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
        // Word length to 8 bits
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

#[no_mangle]
static __FLAG: Mutex<RefCell<bool>> = Mutex::new(RefCell::new(false));

fn board_init(sysctl: &ral::sysctl::SYSCTL) {
    ral::modify_reg!(ral::sysctl, sysctl, GROUP0_0_VALUE, AXI_SRAM1: Linked);
    ral::modify_reg!(ral::sysctl, sysctl, GROUP0_0_VALUE, GPIO0_1: Linked);
    ral::modify_reg!(ral::sysctl, sysctl, GROUP0_1_VALUE, UARTO: Linked);
    // Set UART0 clock source to osc24 and divider to 1 (24 MHz)
    ral::modify_reg!(ral::sysctl, sysctl, CLOCK_CLK_TOP_UART0, MUX: 0, DIV: 0);

    unsafe {
        register::mie::set_mext();
        interrupt::enable();
        PLIC.enable(ral::ExternalInterrupt::Gpio0PortD, 0);
        PLIC.set_priority(ral::ExternalInterrupt::Gpio0PortD, 1);
    }
}

fn board_init_gpio_pins(ioc: &ral::ioc::IOC0, gpio: &ral::gpio::GPIO0) {
    // Select GPIO function
    ral::modify_reg!(ral::ioc, ioc, PAD_PD19_FUNC_CTL, ALT_SELECT: 0);
    ral::modify_reg!(ral::ioc, ioc, PAD_PD13_FUNC_CTL, ALT_SELECT: 0);
    ral::modify_reg!(ral::ioc, ioc, PAD_PD25_FUNC_CTL, ALT_SELECT: 0);
    // Enable pull-up resistor
    ral::modify_reg!(ral::ioc, ioc, PAD_PD19_PAD_CTL, PS: Up, PE: Enable);
    ral::modify_reg!(ral::ioc, ioc, PAD_PD23_PAD_CTL, PS: Up, PE: Enable);
    ral::modify_reg!(ral::ioc, ioc, PAD_PD25_PAD_CTL, PS: Up, PE: Enable);
    // Set GPIO as input
    ral::write_reg!(
        ral::gpio,
        gpio,
        OE_GPIOD_CLEAR,
        (1 << 19) | (1 << 23) | (1 << 25)
    );
    // Enable interrupt, falling edge trigger
    ral::write_reg!(ral::gpio, gpio, TP_GPIOD_SET, 1 << 19);
    ral::write_reg!(ral::gpio, gpio, PL_GPIOD_SET, 1 << 19);
    ral::write_reg!(ral::gpio, gpio, IE_GPIOD_SET, 1 << 19);
}

fn board_init_uart_pins(ioc: &ral::ioc::IOC0, pioc: &ral::ioc::PIOC10) {
    ral::modify_reg!(ral::ioc, ioc, PAD_PY06_FUNC_CTL, ALT_SELECT: 2);
    ral::modify_reg!(ral::ioc, ioc, PAD_PY07_FUNC_CTL, ALT_SELECT: 2);
    // PY port IO needs to configure PIOC as well
    ral::modify_reg!(ral::ioc, pioc, PAD_PY06_FUNC_CTL, ALT_SELECT: 3);
    ral::modify_reg!(ral::ioc, pioc, PAD_PY07_FUNC_CTL, ALT_SELECT: 3);
}

#[export_name = "Gpio0PortDHandler"]
fn gpio0_portd_handler() {
    critical_section::with(|cs| unsafe {
        ral::write_reg!(ral::gpio, GPIO0, IF_GPIOD_VALUE, 1 << 19);
        *__FLAG.borrow_ref_mut(cs) = true;
    });
}

#[entry]
fn main() -> ! {
    let sysctl = unsafe { ral::sysctl::SYSCTL::instance() };
    let ioc = unsafe { ral::ioc::IOC0::instance() };
    let pioc = unsafe { ral::ioc::PIOC10::instance() };
    let gpio0 = unsafe { ral::gpio::GPIO0::instance() };
    let mut uart = Uart::new(unsafe { ral::uart::UART0::instance() });
    let mut cnt = 0;

    board_init(&sysctl);
    board_init_gpio_pins(&ioc, &gpio0);
    board_init_uart_pins(&ioc, &pioc);
    uart.init(115_200, 24_000_000);

    loop {
        critical_section::with(|cs| {
            if *__FLAG.borrow_ref(cs) {
                write!(uart, "value: {cnt}\r\n").unwrap();
                cnt += 1;
                *__FLAG.borrow_ref_mut(cs) = false;
            }
        });
    }
}
