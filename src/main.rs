use std::io::{self, Write};

use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::*;
use esp_idf_hal::peripherals::Peripherals;

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();
    // let mut led = PinDriver::output(peripherals.pins.gpio4)?;
    let mut led = PinDriver::output(peripherals.pins.gpio2)?; // built-in led

    println!("Hello rusty blinky!");
    loop {
        led.set_high()?;
        // we are sleeping here to make sure the watchdog isn't triggered
        FreeRtos::delay_ms(1000);
        print!(".");
        io::stdout().flush()?;
        led.set_low()?;
        FreeRtos::delay_ms(1000);
    }
}