#![no_std]
#![no_main]

use cortex_m_rt::entry;
use microbit::{
    board::Board,
    hal::{
        prelude::*,
        timer::Timer,
    },
};
use panic_halt as _;

#[entry]
fn init() -> ! {
    let mut board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);

    board.display_pins.col1.set_low().unwrap();

    loop {
        board.display_pins.row1.set_high().unwrap();
        timer.delay_ms(500u16);
        board.display_pins.row1.set_low().unwrap();
        timer.delay_ms(500u16);
    }
}
