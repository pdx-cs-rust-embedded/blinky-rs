#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::{delay::DelayNs, digital::{InputPin, OutputPin}};
use microbit::{
    board::Board,
    hal::timer::Timer,
};
use rtt_target::{rtt_init_print, rprintln};                                   
use panic_rtt_target as _;                                                    


enum State {
    LedOn,
    LedOff,
}

#[entry]
fn init() -> ! {
    rtt_init_print!();
    let mut board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut button = board.buttons.button_a;

    board.display_pins.col1.set_low().unwrap();

    let mut state = State::LedOn;

    loop {
        let pressed = button.is_low().unwrap();
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
