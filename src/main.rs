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
use rtt_target::{rtt_init_print, rprintln};                                   
use panic_rtt_target as _;                                                    


#[entry]
fn init() -> ! {
    rtt_init_print!();
    let mut board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);

    board.display_pins.col1.set_low().unwrap();

    loop {
        board.display_pins.row1.set_high().unwrap();
        rprintln!("high");
        timer.delay_ms(500u16);
        board.display_pins.row1.set_low().unwrap();
        rprintln!("low");
        timer.delay_ms(500u16);
    }
}
