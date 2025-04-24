#![no_std]
#![no_main]

use core::sync::atomic::{
    AtomicBool,
    Ordering::{Acquire, Release},
};

use cortex_m_rt::entry;
use embedded_hal::{
    delay::DelayNs,
    digital::{InputPin, OutputPin},
};
use microbit::{
    board::Board,
    hal::gpio::{Floating, Input, Pin},
    hal::gpiote::Gpiote,
    hal::pac::{self, interrupt},
    hal::timer::Timer,
};
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use critical_section_lock_mut::LockMut;
static APP_STATE: LockMut<AppState> = LockMut::new();

static BUTTON_STATE: AtomicBool = AtomicBool::new(false);

struct AppState {
    gpiote: Gpiote,
    button: Pin<Input<Floating>>,
}

enum State {
    LedOn,
    LedOff,
}

#[interrupt]
fn GPIOTE() {
    APP_STATE.with_lock(|app_state| {
        let button_a_changed = app_state.gpiote.channel0().is_event_triggered();
        if button_a_changed {
            let button_value = app_state.button.is_low().unwrap();
            BUTTON_STATE.store(button_value, Release);
        }
        app_state.gpiote.channel0().reset_events();
    });
}

#[entry]
fn init() -> ! {
    rtt_init_print!();
    let mut board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut button = board.buttons.button_a.degrade();
    let gpiote = Gpiote::new(board.GPIOTE);

    unsafe { pac::NVIC::unmask(pac::Interrupt::GPIOTE) };
    pac::NVIC::unpend(pac::Interrupt::GPIOTE);

    let channel = gpiote.channel0();
    channel.input_pin(&button).toggle().enable_interrupt();
    channel.reset_events();

    let button_state = button.is_low().unwrap();
    BUTTON_STATE.store(button_state, Release);

    let app_state = AppState { button, gpiote };
    APP_STATE.init(app_state);

    board.display_pins.col1.set_low().unwrap();

    let mut state = State::LedOn;

    loop {
        let pressed = BUTTON_STATE.load(Acquire);
        state = match (pressed, state) {
            (true, State::LedOn) => {
                board.display_pins.row1.set_high().unwrap();
                rprintln!("high");
                State::LedOff
            }
            _ => {
                board.display_pins.row1.set_low().unwrap();
                rprintln!("low");
                State::LedOn
            }
        };
        timer.delay_ms(500);
    }
}
