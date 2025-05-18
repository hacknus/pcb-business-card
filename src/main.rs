#![no_main]
#![no_std]

extern crate panic_halt;

use cortex_m_rt::entry;
use embedded_hal::digital::v2::OutputPin;
use sht4x::Address::Address0x44;
use sht4x::{Precision, Sht4x};
use stm32l0xx_hal::{delay::Delay, pac, prelude::*, rcc::Config, rtc, rtc::{Rtc}};
use stm32l0xx_hal::exti::{ConfigurableLine, Exti, TriggerEdge};
use stm32l0xx_hal::pwr::PWR;
use stm32l0xx_hal::rcc::MSIRange;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.freeze(Config::msi(MSIRange::Range0));
    let mut scb = cp.SCB;
    let mut exti = Exti::new(dp.EXTI);
    let mut pwr = PWR::new(dp.PWR, &mut rcc);

    let mut rtc = Rtc::new(dp.RTC, &mut rcc, &mut pwr, None).unwrap();

    // GPIO setup
    let gpioa = dp.GPIOA.split(&mut rcc);
    let gpiob = dp.GPIOB.split(&mut rcc);

    let mut delay = Delay::new(cp.SYST, rcc.clocks);

    // LED outputs
    let mut rh30 = gpioa.pa7.into_push_pull_output();
    let mut rh40 = gpioa.pa6.into_push_pull_output();
    let mut rh60 = gpioa.pa5.into_push_pull_output();
    let mut rh80 = gpioa.pa4.into_push_pull_output();
    let mut rh90 = gpioa.pa3.into_push_pull_output();

    // reduce power by setting all pins to low:

    let _ = gpioa.pa0.into_analog();
    let _ = gpioa.pa1.into_analog();
    let _ = gpioa.pa2.into_analog();
    let _ = gpioa.pa11.into_analog();
    let _ = gpioa.pa12.into_analog();
    let _ = gpiob.pb0.into_analog();
    let _ = gpiob.pb1.into_analog();
    let _ = gpiob.pb6.into_analog();
    let _ = gpiob.pb7.into_analog();


    // I2C setup
    let sda = gpioa.pa10.into_open_drain_output();
    let scl = gpioa.pa9.into_open_drain_output();
    let i2c1 = dp.I2C1.i2c(sda, scl, 100_000.Hz(), &mut rcc);
    let mut sht40 = Sht4x::new_with_address(i2c1, Address0x44);

    let exti_line = ConfigurableLine::RtcWakeup;

    rtc.enable_interrupts(rtc::Interrupts {
        wakeup_timer: true,
        ..rtc::Interrupts::default()
    });
    exti.listen_configurable(exti_line, TriggerEdge::Rising);

    loop {
        if let Ok(measurement) = sht40.measure(Precision::Low, &mut delay) {
            let humidity: f32 = measurement.humidity_percent().to_num();

            // Light only one LED briefly to reduce power
            if humidity <= 30.0 {
                rh30.set_high().ok();
            } else if humidity <= 40.0 {
                rh40.set_high().ok();
            } else if humidity <= 60.0 {
                rh60.set_high().ok();
            } else if humidity <= 80.0 {
                rh80.set_high().ok();
            } else {
                rh90.set_high().ok();
            }

            delay.delay(150.milliseconds());

            // Turn off all LEDs to save power
            rh30.set_low().ok();
            rh40.set_low().ok();
            rh60.set_low().ok();
            rh80.set_low().ok();
            rh90.set_low().ok();
        }

        rtc.wakeup_timer().start(1u32);

        exti.wait_for_irq(exti_line, pwr.standby_mode(&mut scb));
    }
}
