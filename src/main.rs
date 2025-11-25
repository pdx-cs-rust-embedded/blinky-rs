#![no_std]
#![no_main]

use cortex_m_rt::entry;
use nrf52833_pac as pac;
use panic_halt as _;

#[cfg(feature="spin")]
fn delay() {
    for _ in 0..4_000_000 {
        // SAFETY: It's a NOP.
        // NEED: Need the program to take some time.
        unsafe {
            core::arch::asm!("nop");
        }
    }
}

#[cfg(not(feature="spin"))]
fn init_timer(timer0: &pac::TIMER0) {
    timer0.bitmode.write(|w| {
        w.bitmode()._32bit()
    });
    timer0.prescaler.write(|w| unsafe {
        w.prescaler().bits(4)
    });
    timer0.shorts.write(|w| {
        w.compare0_clear().enabled().compare0_stop().enabled()
    });
}

#[cfg(not(feature="spin"))]
fn delay_ms(timer0: &pac::TIMER0, ms: u32) {
    let cycles = 1000 * ms;
    timer0.cc[0].write(|w| unsafe {
        w.bits(cycles)
    });
    timer0.tasks_clear.write(|w| unsafe { w.bits(1) });
    timer0.tasks_start.write(|w| unsafe { w.bits(1) });
    while timer0.events_compare[0].read().bits() == 0 {
        // SAFETY: It's a NOP.
        // NEED: Need the program to take some time.
        unsafe {
            core::arch::asm!("nop");
        }
    }
    timer0.events_compare[0].reset();
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
    #[cfg(not(feature = "spin"))]
    let timer0 = p.TIMER0;

    #[cfg(not(feature = "spin"))]
    init_timer(&timer0);

    let col1 = 28;
    let row1 = 21;
    init_pin(&p0, col1);
    set_low(&p0, col1);
    init_pin(&p0, row1);

    #[cfg(not(feature="spin"))]
    let wait = || delay_ms(&timer0, 500);
    #[cfg(feature="spin")]
    let wait = || delay();

    loop {
        set_high(&p0, row1);
        wait();
        set_low(&p0, row1);
        wait();
    }
}
