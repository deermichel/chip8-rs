use crossterm::event::{poll, read, Event, KeyEvent, KeyCode, KeyModifiers};
use std::time::Duration;

// input
#[derive(Debug)]
pub enum Input {
    Key(u8),
    Quit,
}

// hex keypad
pub struct Keypad {}

// hex keypad impl
impl Keypad {
    // get keypress (if available)
    pub fn get_input() -> Option<Input> {
        if poll(Duration::from_millis(0)).ok()? {
            if let Event::Key(event) = read().ok()? {
                return Keypad::match_key(event);
            }
        }
        None
    }

    // wait for keypress
    pub fn wait_for_input() -> Input {
        loop {
            if let Event::Key(event) = read().unwrap() {
                if let Some(input) = Keypad::match_key(event) {
                    return input;
                }
            }
        }
    }

    // match key
    fn match_key(event: KeyEvent) -> Option<Input> {
        match event.code {
            KeyCode::Char('1') | KeyCode::Char('!') => Some(Input::Key(0x1)),
            KeyCode::Char('2') | KeyCode::Char('@') => Some(Input::Key(0x2)),
            KeyCode::Char('3') | KeyCode::Char('#') => Some(Input::Key(0x3)),
            KeyCode::Char('4') | KeyCode::Char('$') => Some(Input::Key(0xC)),
            KeyCode::Char('q') | KeyCode::Char('Q') => Some(Input::Key(0x4)),
            KeyCode::Char('w') | KeyCode::Char('W') => Some(Input::Key(0x5)),
            KeyCode::Char('e') | KeyCode::Char('E') => Some(Input::Key(0x6)),
            KeyCode::Char('r') | KeyCode::Char('R') => Some(Input::Key(0xD)),
            KeyCode::Char('a') | KeyCode::Char('A') => Some(Input::Key(0x7)),
            KeyCode::Char('s') | KeyCode::Char('S') => Some(Input::Key(0x8)),
            KeyCode::Char('d') | KeyCode::Char('D') => Some(Input::Key(0x9)),
            KeyCode::Char('f') | KeyCode::Char('F') => Some(Input::Key(0xE)),
            KeyCode::Char('z') | KeyCode::Char('Z') => Some(Input::Key(0xA)),
            KeyCode::Char('x') | KeyCode::Char('X') => Some(Input::Key(0x0)),
            KeyCode::Char('c') | KeyCode::Char('C') => {
                if event.modifiers == KeyModifiers::CONTROL {
                    Some(Input::Quit)
                } else {
                    Some(Input::Key(0xB))
                }
            },
            KeyCode::Char('v') | KeyCode::Char('V') => Some(Input::Key(0xF)),
            KeyCode::Esc => Some(Input::Quit),
            _ => None,
        }
    }
}
