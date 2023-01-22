#![no_std]

pub mod dma;
pub mod spi;
pub mod uart;

use hpm_ral::modify_reg;
use hpm_ral::{ioc, sysctl};

pub fn board_init(sysctl: &sysctl::SYSCTL) {
    modify_reg!(sysctl, sysctl, GROUP0_0_VALUE, GPIO0_1: Linked);
    modify_reg!(sysctl, sysctl, GROUP0_1_VALUE, UARTO: Linked);
    modify_reg!(sysctl, sysctl, GROUP0_0_VALUE, HDMA: Linked);
    // Set UART0 clock source to osc24 and divider to 1 (24 MHz)
    modify_reg!(sysctl, sysctl, CLOCK_CLK_TOP_UART0, MUX: 0, DIV: 0);
}

pub fn board_init_led_pins(ioc: &ioc::IOC0) {
    modify_reg!(ioc, ioc, PAD_PD15_FUNC_CTL, ALT_SELECT: 0);
    modify_reg!(ioc, ioc, PAD_PD15_PAD_CTL, PE: Enable, PS: Down);
}

pub fn board_init_uart_pins(ioc: &ioc::IOC0, pioc: &ioc::PIOC10) {
    modify_reg!(ioc, ioc, PAD_PY06_FUNC_CTL, ALT_SELECT: 2);
    modify_reg!(ioc, ioc, PAD_PY07_FUNC_CTL, ALT_SELECT: 2);
    // PY port IO needs to configure PIOC as well
    modify_reg!(ioc, pioc, PAD_PY06_FUNC_CTL, ALT_SELECT: 3);
    modify_reg!(ioc, pioc, PAD_PY07_FUNC_CTL, ALT_SELECT: 3);
}
