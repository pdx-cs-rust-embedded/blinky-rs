//! Demo Blinky in pure PAC for the MicroBit 2 (nRF52833).
//!
//! Much of this code is "borrowed" from the `nrf52833-hal`
//! crate.
//!
//! * With no Cargo features, there will be no delay: the LED
//!   will look half-illuminated.
//! * With `--features=spin`, an approximate spin wait will
//!   be used for delay.
//! * With `--features=timer', a hardware timer will be used
//!   for delay.

#![no_std]
#![no_main]

use cortex_m_rt::entry;
use nrf52833_pac as pac;
use panic_halt as _;

#[cfg(feature="spin")]
/// Timer using pure spin-wait.
mod timer {
    pub struct Timer;

    impl Timer {
        /// Make a dummy timer struct.
        pub fn new() -> Self {
            Self
        }

        /// Delay by spin waiting for approximately 500 ms.
        /// **Note:** Will spin far longer if not compiled
        /// optimized.
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

#[cfg(feature="core_spin")]
/// Timer using pure spin-wait with [core::hints::spin_loop].
mod timer {
    pub struct Timer;

    impl Timer {
        /// Make a dummy timer struct.
        pub fn new() -> Self {
            Self
        }

        /// Delay by spin waiting for approximately 500 ms.
        /// **Note:** Will spin far longer if not compiled
        /// optimized.
        pub fn delay(&self) {
            for _ in 0..20_000_000 {
                core::hint::spin_loop();
            }
        }
    }
}

#[cfg(feature="timer")]
/// Hardware timer.
mod timer {
    use super::*;

    /// Hardware timer struct.
    pub struct Timer(pac::TIMER0);

    impl Timer {
        /// Set up and capture the hardware timer.
        pub fn new(timer0: pac::TIMER0) -> Self {
            // Need 32-bit timer for longer waits.
            timer0.bitmode.write(|w| {
                w.bitmode()._32bit()
            });
            // Scale down peripheral clock by 16× to 1MHz.
            timer0.prescaler.write(|w| unsafe {
                w.prescaler().bits(4)
            });
            // Set up the event system for easy test
            // of timer completion.
            timer0.shorts.write(|w| {
                w.compare0_clear().enabled();
                w.compare0_stop().enabled();
                w
            });
            Self(timer0)
        }

        fn delay_ms(&self, ms: u32) {
            // Calculate and set number of 1MHz ticks to count to.
            let cycles = 1000 * ms;
            self.0.cc[0].write(|w| unsafe {
                w.bits(cycles)
            });
            // Clear the timer.
            self.0.tasks_clear.write(|w| unsafe { w.bits(1) });
            // Start the timer.
            self.0.tasks_start.write(|w| unsafe { w.bits(1) });
            // Spin until the timer completion
            // event. Shortcuts will stop and clear the
            // timer when this event is triggered.
            while self.0.events_compare[0].read().bits() == 0 {
                // SAFETY: It's a NOP.
                // NEED: Need the program to take some time.
                unsafe {
                    core::arch::asm!("nop");
                }
            }
            // Clear the completion event.
            self.0.events_compare[0].reset();
        }

        /// Delay using a 500ms hardware timer wait.
        pub fn delay(&self) {
            self.delay_ms(500);
        }
    }
}

#[cfg(feature = "delay")]
use timer::Timer;

/// Initialize the given GPIO pin.
fn init_pin(p0: &pac::P0, pin: usize) {
    p0.pin_cnf[pin].write(|w| {
        w.dir().output();
        w.pull().disabled();
        w.drive().s0s1();
        w.sense().disabled();
        w
    });
}

/// Set the given GPIO pin high.
fn set_high(p0: &pac::P0, pin: usize) {
    unsafe {
        p0.outset.write(|w| w.bits(1u32 << pin));
    }
}

/// Set the given GPIO pin low.
fn set_low(p0: &pac::P0, pin: usize) {
    unsafe {
        p0.outclr.write(|w| w.bits(1u32 << pin));
    }
}

#[entry]
fn init() -> ! {
    let p = pac::Peripherals::take().unwrap();
    let p0 = p.P0;

    #[cfg(any(feature = "spin", feature = "core_spin"))]
    let timer = Timer::new();

    #[cfg(feature = "timer")]
    let timer = Timer::new(p.TIMER0);

    // Pin numbers on MicroBit v2.
    let col1 = 28;
    let row1 = 21;

    // Set up pins.
    init_pin(&p0, col1);
    set_low(&p0, col1);
    init_pin(&p0, row1);

    loop {
        set_high(&p0, row1);
        #[cfg(feature = "delay")]
        timer.delay();
        set_low(&p0, row1);
        #[cfg(feature = "delay")]
        timer.delay();
    }
}
