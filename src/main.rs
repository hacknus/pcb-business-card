//#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate panic_halt;

use cortex_m_rt::exception;
use cortex_m_rt::{entry, ExceptionFrame};
use sht4x::Address::Address0x44;
use sht4x::{Precision, Sht4x};
use stm32l0xx_hal::rcc::Config;
use stm32l0xx_hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.freeze(Config::default());

    // initialize ports
    let gpioa = dp.GPIOA.split(&mut rcc);
    let gpiob = dp.GPIOB.split(&mut rcc);

    // Get the delay provider.
    let mut delay = cp.SYST.delay(rcc.clocks);

    // initialize leds
    let mut t1 = gpiob.pb4.into_push_pull_output();
    let mut t2 = gpioa.pa15.into_push_pull_output();
    let mut t3 = gpiob.pb3.into_push_pull_output();
    let mut t4 = gpiob.pb5.into_push_pull_output();

    let mut rh30 = gpioa.pa7.into_push_pull_output();
    let mut rh40 = gpioa.pa6.into_push_pull_output();
    let mut rh60 = gpioa.pa5.into_push_pull_output();
    let mut rh80 = gpioa.pa4.into_push_pull_output();
    let mut rh90 = gpioa.pa3.into_push_pull_output();

    // initialize sw
    let sw = gpioa.pa8.into_pull_down_input();

    delay.delay(100.milliseconds());

    let sda = gpioa.pa10.into_open_drain_output();
    let scl = gpioa.pa9.into_open_drain_output();
    let i2c1 = dp.I2C1.i2c(sda, scl, 100_000.Hz(), &mut rcc);

    //let mut sht40 = Sht4x::new(i2c1);
    let mut sht40 = Sht4x::new_with_address(i2c1, Address0x44);

    loop {
        let measurement = sht40.measure(Precision::Low, &mut delay);
        // Convert temperature measurand into different formats for further
        // processing.
        if let Ok(measurement) = measurement {
            let _temperature: f32 = measurement.temperature_celsius().to_num();
            let humidity: f32 = measurement.humidity_percent().to_num();

            match humidity {
                0.0..=40.0 => {
                    rh30.set_high().unwrap();
                }
                40.0..=60.0 => {
                    rh40.set_high().unwrap();
                }
                60.0..=80.0 => {
                    rh60.set_high().unwrap();
                }
                80.0..=90.0 => {
                    rh80.set_high().unwrap();
                }
                90.0..=100.0 => {
                    rh90.set_high().unwrap();
                }
                _ => {}
            }
        }
        if sw.is_high().unwrap() {
            t1.set_low().unwrap();
            t2.set_low().unwrap();
            t3.set_high().unwrap();
            t4.set_high().unwrap();
        } else {
            t1.set_high().unwrap();
            t2.set_high().unwrap();
            t3.set_low().unwrap();
            t4.set_low().unwrap();
        }
        delay.delay(100.milliseconds());
    }
}

#[exception]
#[allow(non_snake_case)]
unsafe fn DefaultHandler(_irqn: i16) {
    // custom default handler
    // irqn is negative for Cortex-M exceptions
    // irqn is positive for device specific (line IRQ)
    // panic!("Exception: {}", irqn);
}

#[exception]
#[allow(non_snake_case)]
unsafe fn HardFault(_ef: &ExceptionFrame) -> ! {
    loop {}
}
