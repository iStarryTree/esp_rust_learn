use embedded_graphics::geometry::Point;
use embedded_graphics::mono_font::ascii::FONT_6X9;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::text::Text;
use embedded_graphics::Drawable;
use esp_idf_svc::hal::i2c::{I2cConfig, I2cDriver};
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::hal::prelude::*;
use ssd1306::mode::DisplayConfig;
use ssd1306::{I2CDisplayInterface, Ssd1306};

fn main() -> anyhow::Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    // main
    let peripherals = Peripherals::take().unwrap();

    // i2c设置
    let i2c = peripherals.i2c0;
    let sda = peripherals.pins.gpio6;
    let scl = peripherals.pins.gpio5;
    let config = I2cConfig::new().baudrate(100.kHz().into());
    let i2c = I2cDriver::new(i2c, sda, scl, &config).unwrap();
    let interface = I2CDisplayInterface::new(i2c);

    // 显示接口
    let mut display = Ssd1306::new(
        interface,
        ssd1306::size::DisplaySize128x64,
        ssd1306::rotation::DisplayRotation::Rotate0,
    )
    .into_buffered_graphics_mode();

    // 初始化
    display.init().unwrap();
    let style = MonoTextStyle::new(&FONT_6X9, BinaryColor::On);
    Text::new("Hello, World!", Point::new(10, 10), style)
        .draw(&mut display)
        .unwrap();
    display.flush().unwrap();

    Ok(())
}
