#![no_std]
#![no_main]

use core::fmt::Debug;

use arduino_hal::prelude::*;
use arduino_hal::adc;
use embedded_dht_rs::dht11::Dht11;
use panic_halt as _;
use ufmt::{uwriteln};

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);


    /*
     * For examples (and inspiration), head to
     *
     *     https://github.com/Rahix/avr-hal/tree/main/examples
     *
     * NOTE: Not all examples were ported to all boards!  There is a good chance though, that code
     * for a different board can be adapted for yours.  The Arduino Uno currently has the most
     * examples available.
     */

    let mut led = pins.d13.into_output();

    let switch = pins.d7.into_pull_up_input();

    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());

    let (vbg, gnd, tmp) = (
        adc.read_blocking(&adc::channel::Vbg),
        adc.read_blocking(&adc::channel::Gnd),
        adc.read_blocking(&adc::channel::Temperature),
    );

    uwriteln!(&mut serial, "Vbandgap: {}", vbg).unwrap_infallible();
    uwriteln!(&mut serial, "GND: {}", gnd).unwrap_infallible();
    uwriteln!(&mut serial, "Temperature: {}", tmp).unwrap_infallible();

    let a0 = pins.a0.into_analog_input(&mut adc);


    let dht11_pin = pins.d2.into_opendrain_high();
    let delay = arduino_hal::Delay::new();
    let mut dht11 = Dht11::new(dht11_pin, delay);

    loop {
        led.toggle();
        let message = if switch.is_high() { "Pin (D7) is HIGH" } else { "Pin (D7) is LOW"};
        uwriteln!(&mut serial, "{}", message).unwrap_infallible();

        let a0_value = a0.analog_read(&mut adc);
        uwriteln!(&mut serial, "Analog A0: {}", a0_value).unwrap_infallible();

        match dht11.read() {
            Ok(measurement) => {
                uwriteln!(&mut serial, "DHT11 Temperature: {}°C", measurement.temperature).unwrap_infallible();
                uwriteln!(&mut serial, "DHT11 Humidity: {}%", measurement.humidity).unwrap_infallible();
            }
            Err(e) => {
                match e {
                    embedded_dht_rs::SensorError::ChecksumMismatch => uwriteln!(&mut serial, "DHT11 Checksum Mismatch").unwrap_infallible(),
                    embedded_dht_rs::SensorError::Timeout => uwriteln!(&mut serial, "DHT11 Timeout Error").unwrap_infallible(),
                    embedded_dht_rs::SensorError::PinError => uwriteln!(&mut serial, "DHT11 Pin Error").unwrap_infallible(),
                }
            }
            
        }
                arduino_hal::delay_ms(1000);

    }
}
