# blinky-rs
Bart Massey 2023

This Rust app for the BBC micro:bit v2 is a bare-minimum
blinky light program.

## Build and Run

You can follow the instructions from the embedded micro:bit
[*Discovery Book*](https://docs.rust-embedded.org/discovery/microbit/index.html)
to set up your build environment.  Then you can say

    cargo embed

to flash and run this.

Note that this app will not work with `--release`: the delay
spin loop will be optimized away.

## License

This work is made available under the "MIT License". Please
see the file `LICENSE.txt` in this distribution for license
terms.

## Acknowledgements

The hardware-using code and build infrastructure here were
heavily derived from the examples in the amazing `microbit`
crate.
