mod display;
mod font;
mod keypad;
mod processor;

use crate::display::Display;
use crate::keypad::Keypad;
use crate::processor::Processor;
use std::io::{stdout, Read};
use std::time::{Duration, Instant};
use std::{thread, time};
use std::fs::{File};

fn main() {
    // init
    let display = Display::new(stdout());
    let mut processor = Processor::new(display);

    // load rom
    let path = "KEYPAD.rom";
    let mut rom_file = File::open(path).unwrap();
    let mut rom_data = Vec::new();
    let _rom_size = rom_file.read_to_end(&mut rom_data).unwrap();
    processor.load(&rom_data);

    // start emulation
    processor.emulate();
}
