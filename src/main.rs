#![no_std]
#![no_main]

use panic_halt as _;
use arduino_hal::{prelude::*, adc::ReferenceVoltage};

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());

    let sensor = pins.a0.into_analog_input(&mut adc);
    let mut led = pins.d13.into_output();

    loop {
        let adc_value = sensor.analog_read(&mut adc);
        let voltage = adc_value * 50 / 1023; // 5 V * 10 = 50 -> 3.8 V == 38 V
        let voltage_int = voltage / 10; // 3 V
        let voltage_float = voltage % 10; // 0.8 V

        ufmt::uwriteln!(&mut serial, "Adc value: {}", adc_value).void_unwrap();
        ufmt::uwriteln!(&mut serial, "Voltage: {}.{}", voltage_int, voltage_float).void_unwrap();

        led.toggle();
        arduino_hal::delay_ms(1000);
    }
}
