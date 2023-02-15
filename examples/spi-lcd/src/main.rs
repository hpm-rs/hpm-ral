#![no_std]
#![no_main]

extern crate panic_halt;

use hpm6750evkmini as bsp;

use riscv::delay::McycleDelay;
use riscv_rt::entry;

use bsp::gpio::Gpio;
use bsp::spi::{FormatConfig, Spi, TimingConfig};
use bsp::uart::Uart;
use hpm_ral as ral;

use display_interface_spi::SPIInterfaceNoCS;
use embedded_graphics::{
    draw_target::DrawTarget,
    image::{Image, ImageRawLE},
    mono_font::{ascii::FONT_9X18_BOLD, MonoTextStyle},
    prelude::*,
    text::{Alignment, Text},
};
use st7789::{Orientation, ST7789};

#[entry]
fn main() -> ! {
    let sysctl = unsafe { ral::sysctl::SYSCTL::instance() };
    let ioc = unsafe { ral::ioc::IOC0::instance() };
    let pioc = unsafe { ral::ioc::PIOC10::instance() };
    let gpio0 = unsafe { ral::gpio::GPIO0::instance() };
    let uart0 = unsafe { ral::uart::UART0::instance() };
    let spi3 = unsafe { ral::spi::SPI3::instance() };
    let mut delay = McycleDelay::new(324_000_000);

    bsp::board_init(&sysctl);
    bsp::board_init_spi3_pins(&ioc);
    bsp::board_init_uart0_pins(&ioc, &pioc);

    let gpio = Gpio::new(gpio0, ioc);
    let bl_pin = gpio.pd14.into_push_pull_output();
    let dc_pin = gpio.pc03.into_push_pull_output();
    let rst_pin = gpio.pd15.into_push_pull_output();

    let mut uart0 = Uart::new(uart0, None);
    uart0.setup(115_200, 24_000_000);

    let timing_cfg = TimingConfig::with_sclk_freq(12_000_000);
    let format_cfg = FormatConfig::default();
    let spi3 = Spi::new(spi3, timing_cfg, format_cfg);

    let spi_intf = SPIInterfaceNoCS::new(spi3, dc_pin);
    let mut st7789 = ST7789::new(spi_intf, Some(rst_pin), Some(bl_pin), 135, 240);

    st7789.init(&mut delay).unwrap();
    st7789
        .set_orientation(Orientation::LandscapeSwapped)
        .unwrap();
    st7789.clear(RgbColor::BLACK).unwrap();

    let raw_image_data = ImageRawLE::new(include_bytes!("../assets/ferris.raw"), 86);
    let ferris = Image::new(&raw_image_data, Point::new(117, 75));
    ferris.draw(&mut st7789).unwrap();

    let text = "HPM6750 with Rust";
    Text::with_alignment(
        text,
        Point::new(160, 160),
        MonoTextStyle::new(&FONT_9X18_BOLD, RgbColor::WHITE),
        Alignment::Center,
    )
    .draw(&mut st7789)
    .unwrap();

    loop {}
}
