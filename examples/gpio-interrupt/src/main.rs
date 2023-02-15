#![no_std]
#![no_main]

mod ext_int;

extern crate panic_halt;

use core::cell::RefCell;
use core::fmt::Write;

use bsp::uart::Uart;
use critical_section::Mutex;
use hpm6750evkmini as bsp;
use hpm_ral as ral;
use riscv::interrupt;
use riscv::register;
use riscv_rt::entry;

use ext_int::PLIC;

#[no_mangle]
static __FLAG: Mutex<RefCell<bool>> = Mutex::new(RefCell::new(false));

fn enable_gpio0_int() {
    unsafe {
        register::mie::set_mext();
        interrupt::enable();
        PLIC.enable(ral::ExternalInterrupt::Gpio0PortD, 0);
        PLIC.set_priority(ral::ExternalInterrupt::Gpio0PortD, 1);
    }
}

fn init_gpio_pins(ioc: &ral::ioc::IOC0, gpio: &ral::gpio::GPIO0) {
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
    let mut uart = Uart::new(unsafe { ral::uart::UART0::instance() }, None);
    let mut cnt = 0;

    bsp::board_init(&sysctl);
    bsp::board_init_uart0_pins(&ioc, &pioc);
    init_gpio_pins(&ioc, &gpio0);

    uart.setup(115_200, 24_000_000);
    enable_gpio0_int();

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
