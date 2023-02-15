#![no_std]

use embedded_hal as hal;

pub mod dma;
pub mod gpio;
pub mod prelude;
pub mod spi;
pub mod uart;

use hpm_ral::{ioc, sysctl};
use hpm_ral::{modify_reg, write_reg};

pub fn board_init(sysctl: &sysctl::SYSCTL) {
    modify_reg!(sysctl, sysctl, GROUP0_0_VALUE, GPIO0_1: Linked);
    modify_reg!(sysctl, sysctl, GROUP0_1_VALUE, UARTO: Linked);
    modify_reg!(sysctl, sysctl, GROUP0_1_VALUE, SPI1: Linked);
    modify_reg!(sysctl, sysctl, GROUP0_1_VALUE, SPI3: Linked);
    modify_reg!(sysctl, sysctl, GROUP0_0_VALUE, HDMA: Linked);
    // Set UART0 clock source to osc24 and divider to 1 (24 MHz)
    modify_reg!(sysctl, sysctl, CLOCK_CLK_TOP_UART0, MUX: 0, DIV: 0);
    // Set SPI1 clock source to osc24 and divider to 1 (24 MHz)
    modify_reg!(sysctl, sysctl, CLOCK_CLK_TOP_SPI1, MUX: 0, DIV: 0);
    modify_reg!(sysctl, sysctl, CLOCK_CLK_TOP_SPI3, MUX: 0, DIV: 0);
}

pub fn board_init_uart0_pins(ioc: &ioc::IOC0, pioc: &ioc::PIOC10) {
    modify_reg!(ioc, ioc, PAD_PY06_FUNC_CTL, ALT_SELECT: 2);
    modify_reg!(ioc, ioc, PAD_PY07_FUNC_CTL, ALT_SELECT: 2);
    // PY port IO needs to configure PIOC as well
    modify_reg!(ioc, pioc, PAD_PY06_FUNC_CTL, ALT_SELECT: 3);
    modify_reg!(ioc, pioc, PAD_PY07_FUNC_CTL, ALT_SELECT: 3);
}

pub fn board_init_spi1_pins(ioc: &ioc::IOC0) {
    write_reg!(ioc, ioc, PAD_PA21_FUNC_CTL, ALT_SELECT: 5, LOOP_BACK: Enable); // SPI1 SCLK
    write_reg!(ioc, ioc, PAD_PA16_FUNC_CTL, ALT_SELECT: 5); // SPI1 MOSI
    write_reg!(ioc, ioc, PAD_PA23_FUNC_CTL, ALT_SELECT: 5); // SPI1 MISO
}

pub fn board_init_spi3_pins(ioc: &ioc::IOC0) {
    write_reg!(ioc, ioc, PAD_PC02_FUNC_CTL, ALT_SELECT: 5, LOOP_BACK: Enable); // SPI3 SCLK
    write_reg!(ioc, ioc, PAD_PB30_FUNC_CTL, ALT_SELECT: 5); // SPI3 MOSI
    write_reg!(ioc, ioc, PAD_PB29_FUNC_CTL, ALT_SELECT: 5); // SPI3 CS
}
