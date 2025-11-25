#![no_std]
#![no_main]

use core::arch::asm;
use cortex_m_rt::entry;
use nrf52833_pac as pac;
use panic_halt as _;

fn delay() {
    for _ in 0..2_000_000 {
        // SAFETY: It's a NOP.
        // NEED: Need the compiler to wait.
        unsafe {
            asm!("nop");
        }
    }
}

fn init_pin(p0: &pac::P0, pin: usize) {
    p0.pin_cnf[pin].write(|w| {
        w.dir().output();
        w.pull().disabled();
        w.drive().s0s1();
        w.sense().disabled();
        w
    });
}

fn set_high(p0: &pac::P0, pin: usize) {
    unsafe {
        p0.outset.write(|w| w.bits(1u32 << pin));
    }
}

fn set_low(p0: &pac::P0, pin: usize) {
    unsafe {
        p0.outclr.write(|w| w.bits(1u32 << pin));
    }
}

#[entry]
fn init() -> ! {
    let p = pac::Peripherals::take().unwrap();
    let p0 = p.P0;
    let col1 = 28;
    let row1 = 21;

    init_pin(&p0, col1);
    set_low(&p0, col1);

    init_pin(&p0, row1);

    loop {
        set_high(&p0, row1);
        delay();
        set_low(&p0, row1);
        delay();
    }
}
