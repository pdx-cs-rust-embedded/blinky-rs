#![no_std]
#![no_main]

use cortex_m_rt::entry;
use nrf52833_hal::{prelude::OutputPin, gpio::{p0, Level}, pac};
use panic_halt as _;

#[inline(never)]
fn delay() {
    for _ in 0..200_000 {
    }
}

#[entry]
fn init() -> ! {
    let p = pac::Peripherals::take().unwrap();
    let p0parts = p0::Parts::new(p.P0);

    let mut led = p0parts.p0_10.into_push_pull_output(Level::Low);

    loop {
        led.set_high().unwrap();
        delay();
        led.set_low().unwrap();
        delay();
    }
}
