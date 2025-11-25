#![no_std]
#![no_main]

use cortex_m_rt::entry;
use nrf52833_pac as pac;
use panic_halt as _;

#[cfg(feature="spin")]
mod timer {
    pub struct Timer;

    impl Timer {
        pub fn new() -> Self {
            Self
        }

        pub fn delay(&self) {
            for _ in 0..4_000_000 {
                // SAFETY: It's a NOP.
                // NEED: Need the program to take some time.
                unsafe {
                    core::arch::asm!("nop");
                }
            }
        }
    }
}

#[cfg(not(feature="spin"))]
mod timer {
    use super::*;

    pub struct Timer(pac::TIMER0);

    impl Timer {
        pub fn new(timer0: pac::TIMER0) -> Self {
            timer0.bitmode.write(|w| {
                w.bitmode()._32bit()
            });
            timer0.prescaler.write(|w| unsafe {
                w.prescaler().bits(4)
            });
            timer0.shorts.write(|w| {
                w.compare0_clear().enabled().compare0_stop().enabled()
            });
            Self(timer0)
        }

        fn delay_ms(&self, ms: u32) {
            let cycles = 1000 * ms;
            self.0.cc[0].write(|w| unsafe {
                w.bits(cycles)
            });
            self.0.tasks_clear.write(|w| unsafe { w.bits(1) });
            self.0.tasks_start.write(|w| unsafe { w.bits(1) });
            while self.0.events_compare[0].read().bits() == 0 {
                // SAFETY: It's a NOP.
                // NEED: Need the program to take some time.
                unsafe {
                    core::arch::asm!("nop");
                }
            }
            self.0.events_compare[0].reset();
        }

        pub fn delay(&self) {
            self.delay_ms(500);
        }
    }
}

use timer::Timer;

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

    #[cfg(feature = "spin")]
    let timer = Timer::new();

    #[cfg(not(feature = "spin"))]
    let timer = Timer::new(p.TIMER0);

    let col1 = 28;
    let row1 = 21;
    init_pin(&p0, col1);
    set_low(&p0, col1);
    init_pin(&p0, row1);

    loop {
        set_high(&p0, row1);
        timer.delay();
        set_low(&p0, row1);
        timer.delay();
    }
}
