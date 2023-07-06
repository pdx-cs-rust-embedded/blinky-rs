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

    let _col1 = p0parts.p0_28.into_push_pull_output(Level::Low);
    let mut row1 = p0parts.p0_21.into_push_pull_output(Level::Low);

    loop {
        row1.set_high().unwrap();
        delay();
        row1.set_low().unwrap();
        delay();
    }
}
