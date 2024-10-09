#![no_std]
#![no_main]

use arduino_hal::{delay_ms, prelude::*};
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let mut led_tx = pins.led_tx.into_output();
    let mut led_rx = pins.led_rx.into_output();

    loop {
        led_tx.set_low();
        led_rx.set_high();
        delay_ms(1000);
        led_tx.set_high();
        led_rx.set_low();
        delay_ms(1000);
        // ufmt::uwriteln!(&mut serial, "Hello from Arduino!\r").unwrap_infallible();
        serial.write_str("test").unwrap_infallible();
        serial.flush();
        // // Read a byte from the serial connection
        _ = nb::block!(serial.read()).unwrap_infallible();

        // // Answer
        // ufmt::uwriteln!(&mut serial, "Got {}!\r", b).unwrap_infallible();
    }
}
