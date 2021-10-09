mod display;

use crate::display::Display;
use std::io::stdout;
use std::{thread, time};

fn main() {
    let mut display = Display::new(stdout());
    display.init_console();
    for _ in 0..10 {
        display.render();
        thread::sleep(time::Duration::from_millis(100));
    }
    display.restore_console();
}
