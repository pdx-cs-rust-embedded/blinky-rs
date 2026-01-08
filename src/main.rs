#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::digital::OutputPin;
use nrf52833_hal::{
    gpio::{Level, p0},
    pac,
};
use panic_halt as _;

fn delay() {
    for _ in 0..2_000_000 {
        unsafe {
            core::arch::asm!("nop");
        }
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
