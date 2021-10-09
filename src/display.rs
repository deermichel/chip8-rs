use std::io::{Write, Stdout};
use crossterm::{QueueableCommand, ExecutableCommand, style, cursor, terminal};

// display constants
const PIXEL_CHAR: &str = "██";
const PIXEL_WIDTH: usize = 2;
pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

// console display
pub struct Display {
    stdout: Stdout,
}

// console display impl
impl Display {
    // constructor
    pub fn new(stdout: Stdout) -> Self {
        Self { stdout }
    }

    // prepare console
    pub fn init_console(&mut self) {
        terminal::enable_raw_mode().unwrap();
        self.stdout
            .execute(terminal::EnterAlternateScreen).unwrap()
            .execute(cursor::Hide).unwrap()
            .execute(terminal::DisableLineWrap).unwrap();
    }

    // render framebuffer to console
    pub fn render(&mut self, framebuffer: &[u8]) {
        // clear screen
        self.stdout.queue(terminal::Clear(terminal::ClearType::All)).unwrap();

        // center screen
        let x_offset = (terminal::size().unwrap().0 - (SCREEN_WIDTH * PIXEL_WIDTH) as u16) / 2;
        let y_offset = (terminal::size().unwrap().1 - (SCREEN_HEIGHT) as u16) / 2;

        // render pixels
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                if framebuffer[y * SCREEN_WIDTH + x] > 0 {
                    self.stdout
                        .queue(cursor::MoveTo(x_offset + (x * PIXEL_WIDTH) as u16, y_offset + y as u16)).unwrap()
                        .queue(style::Print(PIXEL_CHAR)).unwrap();
                }
            }
        }
        self.stdout.queue(cursor::MoveTo(0, 0)).unwrap();

        // commit results
        self.stdout.flush().unwrap();
    }

    // restore console behavior
    pub fn restore_console(&mut self) {
        self.stdout
            .execute(terminal::LeaveAlternateScreen).unwrap()
            .execute(cursor::Show).unwrap()
            .execute(terminal::EnableLineWrap).unwrap();
        terminal::disable_raw_mode().unwrap();
    }
}
