use crossterm::event::{read, Event, KeyEvent, KeyCode, KeyModifiers};

// hex keypad
pub struct Keypad {}

// hex keypad impl
impl Keypad {
    // wait for key
    pub fn wait_for_key() -> Option<u8> {
        loop {
            if let Event::Key(event) = read().ok()? {
                return match event.code {
                    KeyCode::Char('1') | KeyCode::Char('!') => Some(0x1),
                    KeyCode::Char('2') | KeyCode::Char('@') => Some(0x2),
                    KeyCode::Char('3') | KeyCode::Char('#') => Some(0x3),
                    KeyCode::Char('4') | KeyCode::Char('$') => Some(0xC),
                    KeyCode::Char('q') | KeyCode::Char('Q') => Some(0x4),
                    KeyCode::Char('w') | KeyCode::Char('W') => Some(0x5),
                    KeyCode::Char('e') | KeyCode::Char('E') => Some(0x6),
                    KeyCode::Char('r') | KeyCode::Char('R') => Some(0xD),
                    KeyCode::Char('a') | KeyCode::Char('A') => Some(0x7),
                    KeyCode::Char('s') | KeyCode::Char('S') => Some(0x8),
                    KeyCode::Char('d') | KeyCode::Char('D') => Some(0x9),
                    KeyCode::Char('f') | KeyCode::Char('F') => Some(0xE),
                    KeyCode::Char('z') | KeyCode::Char('Z') => Some(0xA),
                    KeyCode::Char('x') | KeyCode::Char('X') => Some(0x0),
                    KeyCode::Char('c') | KeyCode::Char('C') => {
                        if event.modifiers == KeyModifiers::CONTROL {
                            Some(0xFF)
                        } else {
                            Some(0xB)
                        }
                    },
                    KeyCode::Char('v') | KeyCode::Char('V') => Some(0xF),
                    KeyCode::Esc => Some(0xFF),
                    _ => None,
                }
            }
        }
    }
}
