mod display;
mod keypad;
mod processor;

use crate::display::Display;
use crate::keypad::Keypad;
use crate::processor::Processor;
use std::io::{stdout, Read};
use std::{thread, time};
use std::fs::{File};

fn main() {
    // init
    let mut processor = Processor::new();

    // load rom
    let path = "TEST.rom";
    let mut rom_file = File::open(path).unwrap();
    let mut rom_data = Vec::new();
    let _rom_size = rom_file.read_to_end(&mut rom_data).unwrap();
    processor.load(&rom_data);

    // processor.print_memory_dump();


    // prepare emulation
    let mut display = Display::new(stdout());
    // display.init_console();
    let mut keypad = Keypad::wait_for_key();
    // processor.emulate();

    // cleanup
    display.restore_console();
}
