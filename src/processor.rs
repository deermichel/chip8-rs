use crate::display::{Display, SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::keypad::{Keypad, Input};
use crate::font::FONT_SET;
use std::time::{Duration, Instant};
use std::panic;
use rand::Rng;

// processor constants
const BASE_ADDR: usize = 0x200;
const FONTS_ADDR: usize = 0x050;
const RAM_SIZE: usize = 4096;
const STACK_SIZE: usize = 16;
const VRAM_SIZE: usize = SCREEN_WIDTH * SCREEN_HEIGHT;

// chip8 processor
pub struct Processor {
    delay_timer: u8,            // delay timer
    display: Display,           // display adapter
    halt: bool,                 // stop emulation
    i: usize,                   // 12-bit index register
    last_tick: Instant,         // last timer update
    keypad: [u8; 16],           // hex keyboard state
    pc: usize,                  // program counter
    ram: [u8; RAM_SIZE],        // main memory (4096 bytes)
    sound_timer: u8,            // sound timer
    sp: usize,                  // stack pointer
    stack: [usize; STACK_SIZE], // stack (16 12-bit addresses)
    v: [u8; 16],                // 16 8-bit registers
    vram: [u8; VRAM_SIZE],      // framebuffer (64x32)
    vram_dirty: bool,           // render on next frame
}

// processor impl
impl Processor {
    // constructor
    pub fn new(display: Display) -> Self {
        // preload font set
        let mut ram = [0; RAM_SIZE];
        for i in 0..FONT_SET.len() {
            ram[i + FONTS_ADDR] = FONT_SET[i];
        }

        // init
        Self {
            delay_timer: 0,
            display,
            halt: false,
            i: 0,
            last_tick: Instant::now(),
            keypad: [0; 16],
            pc: BASE_ADDR,
            ram,
            sound_timer: 0,
            sp: 0,
            stack: [0; STACK_SIZE],
            v: [0; 16],
            vram: [0; VRAM_SIZE],
            vram_dirty: false,
        }
    }

    // load into memory
    pub fn load(&mut self, data: &[u8]) {
        for (i, &byte) in data.iter().enumerate() {
            let addr = BASE_ADDR + i;
            if addr < RAM_SIZE {
                self.ram[addr] = byte;
            } else {
                break;
            }
        }
    }

    // emulation loop
    pub fn emulate(&mut self) {
        self.display.init_console();
        while !self.halt && self.pc < RAM_SIZE {
            // execute
            let opcode = self.fetch_instruction();
            self.process_opcode(opcode);
            self.pc += 2;

            // render
            if self.vram_dirty {
                self.vram_dirty = false;
                self.display.render(&self.vram);
            }

            // fetch keys
            self.keypad = [0; 16];
            if let Some(input) = Keypad::get_input() {
                match input {
                    Input::Key(k) => self.keypad[k as usize] = 1,
                    Input::Quit => self.halt = true,
                }
            }

            // update timers
            if self.last_tick.elapsed() > Duration::from_millis(16) {
                self.last_tick = Instant::now();
                if self.delay_timer > 0 { self.delay_timer -= 1; }
                if self.sound_timer > 0 { self.sound_timer -= 1; }
            }
        }
        self.display.restore_console();
    }

    // print memory dump
    pub fn _print_memory_dump(&self) {
        for i in 0..RAM_SIZE {
            if i % 32 == 0 { print!("{:#05X}  |  ", i); }
            print!("{:02X}  ", self.ram[i]);
            if i % 32 == 31 { println!(); }
        }
    }

    // fetch instruction
    fn fetch_instruction(&self) -> u16 {
        (self.ram[self.pc] as u16) << 8 | (self.ram[self.pc + 1] as u16)
    }

    // process opcode
    fn process_opcode(&mut self, opcode: u16) {
        // common symbols
        let nnn = opcode & 0x0FFF;                  // 12-bit address
        let nn = (opcode & 0x00FF) as u8;           // 8-bit constant
        let n = opcode & 0x000F;                    // 4-bit constant
        let x = ((opcode & 0x0F00) >> 8) as usize;  // 4-bit register indentifier
        let y = ((opcode & 0x00F0) >> 4) as usize;  // 4-bit register indentifier

        // match
        let a = (opcode & 0xF000) >> 12;
        let b = (opcode & 0x0F00) >> 8;
        let c = (opcode & 0x00F0) >> 4;
        let d = opcode & 0x000F;
        match (a, b, c, d) {
            // 00E0 [disp] CLS
            (0x0, 0x0, 0xE, 0x0) => {
                self.vram = [0; VRAM_SIZE];
                self.vram_dirty = true;
            },

            // 00EE [flow] RET
            (0x0, 0x0, 0xE, 0xE) => {
                if self.sp == 0 { panic!("stack underflow"); }
                self.sp -= 1;
                self.pc = self.stack[self.sp];
            },

            // 1NNN [flow] JUMP NNN
            (0x1, _, _, _) => self.pc = (nnn - 2) as usize, // account for +2 in each execution cycle

            // 2NNN [flow] CALL NNN
            (0x2, _, _, _) => {
                if self.sp == STACK_SIZE { panic!("stack overflow"); }
                self.stack[self.sp] = self.pc;
                self.sp += 1;
                self.pc = (nnn - 2) as usize; // account for +2 in each execution cycle
            },

            // 3XNN [cond] SKIP VX == NN
            (0x3, _, _, _) => if self.v[x] == nn { self.pc += 2 },

            // 4XNN [cond] SKIP VX != NN
            (0x4, _, _, _) => if self.v[x] != nn { self.pc += 2 },

            // 5XY0 [cond] SKIP VX == VY
            (0x5, _, _, 0x0) => if self.v[x] == self.v[y] { self.pc += 2 },

            // 6XNN [const] LDI VX = NN
            (0x6, _, _, _) => self.v[x] = nn,

            // 7XNN [const] LDA VX += NN
            (0x7, _, _, _) => self.v[x] = self.v[x].wrapping_add(nn),

            // 8XY0 [assig] SET VX = VY
            (0x8, _, _, 0x0) => self.v[x] = self.v[y],

            // 8XY1 [bitop] OR VX = VX | VY
            (0x8, _, _, 0x1) => self.v[x] |= self.v[y],

            // 8XY2 [bitop] AND VX = VX & VY
            (0x8, _, _, 0x2) => self.v[x] &= self.v[y],

            // 8XY3 [bitop] XOR VX = VX ^ VY
            (0x8, _, _, 0x3) => self.v[x] ^= self.v[y],

            // 8XY4 [math] ADD VX += VY
            (0x8, _, _, 0x4) => {
                let (value, carry) = self.v[x].overflowing_add(self.v[y]);
                self.v[x] = value;
                self.v[0xF] = if carry { 1 } else { 0 };
            },

            // 8XY5 [math] SUB VX -= VY
            (0x8, _, _, 0x5) => {
                let (value, borrow) = self.v[x].overflowing_sub(self.v[y]);
                self.v[x] = value;
                self.v[0xF] = if borrow { 0 } else { 1 };
            },

            // 8XY6 [bitop] SHR VX >>= 1
            (0x8, _, _, 0x6) => {
                self.v[0xF] = self.v[x] & 0x01;
                self.v[x] >>= 1;
            },

            // 8XY7 [math] SUB VX = VY - VX
            (0x8, _, _, 0x7) => {
                self.v[0xF] = if self.v[y] < self.v[x] { 0 } else { 1 }; // borrow
                self.v[x] = self.v[y] - self.v[x];
            },

            // 8XYE [bitop] SHL VX <<= 1
            (0x8, _, _, 0xE) => {
                self.v[0xF] = self.v[x] & 0x80;
                self.v[x] <<= 1;
            },

            // 9XY0 [cond] SKIP VX != VY
            (0x9, _, _, 0x0) => if self.v[x] != self.v[y] { self.pc += 2 },

            // ANNN [mem] SET I = NNN
            (0xA, _, _, _) => self.i = nnn as usize,

            // BNNN [flow] JUMP V0 + NNN
            (0xB, _, _, _) => self.pc = (nnn + (self.v[0x0] as u16) - 2) as usize, // account for +2 in each execution cycle

            // CXNN [rand] RND VX = rand() & NN
            (0xC, _, _, _) => self.v[x] = rand::thread_rng().gen::<u8>() & nn,

            // DXYN [disp] DRAW at (VX, VY) with height N from memory location I
            (0xD, _, _, _) => {
                self.v[0xF] = 0; // reset flipped flag
                for y_offset in 0..n as usize {
                    let row = self.ram[self.i + y_offset];
                    for x_offset in 0..8 {
                        let pixel = (row >> (7 - x_offset)) & 0x1;
                        let pos = (y_offset + (self.v[y] as usize)) * SCREEN_WIDTH + x_offset + (self.v[x] as usize);
                        if self.vram[pos] == 1 && pixel == 1 {
                            self.v[0xF] = 1; // set flipped flag
                        }
                        self.vram[pos] ^= pixel;
                    }
                }
                self.vram_dirty = true;
            },

            // EX9E [keyop] SKIP key VX pressed
            (0xE, _, 0x9, 0xE) => if self.keypad[self.v[x] as usize] > 0 { self.pc += 2 },

            // EXA1 [keyop] SKIP key VX not pressed
            (0xE, _, 0xA, 0x1) => if self.keypad[self.v[x] as usize] == 0 { self.pc += 2 },

            // FX07 [timer] SET VX = delay timer
            (0xF, _, 0x0, 0x7) => self.v[x] = self.delay_timer,

            // FX0A [keyop] WAIT until key pressed, store in VX
            (0xF, _, 0x0, 0xA) => {
                let input = Keypad::wait_for_input();
                match input {
                    Input::Key(k) => self.v[x] = k,
                    Input::Quit => self.halt = true,
                }
            },

            // FX15 [timer] SET delay timer = VX
            (0xF, _, 0x1, 0x5) => self.delay_timer = self.v[x],

            // FX18 [sound] SET sound timer = VX
            (0xF, _, 0x1, 0x8) => self.sound_timer = self.v[x],

            // FX1E [mem] SET I += VX
            (0xF, _, 0x1, 0xE) => self.i = self.i.wrapping_add(self.v[x] as usize),

            // FX29 [mem] SET I = sprite(VX)
            (0xF, _, 0x2, 0x9) => self.i = FONTS_ADDR + (self.v[x] as usize) * 5,

            // FX33 [mem] BCD I = VX
            (0xF, _, 0x3, 0x3) => {
                let num = self.v[x];
                self.ram[self.i] = num / 100;
                self.ram[self.i + 1] = (num % 100) / 10;
                self.ram[self.i + 2] = num % 10;
            },

            // FX55 [mem] DREG dump V0 to VX to I
            (0xF, _, 0x5, 0x5) => {
                for reg in 0..=x {
                    self.ram[self.i + reg] = self.v[reg];
                }
            },

            // FX65 [mem] LREG load I into V0 to VX
            (0xF, _, 0x6, 0x5) => {
                for reg in 0..=x {
                    self.v[reg] = self.ram[self.i + reg];
                }
            },

            // [misc] UNSUPPORTED
            (_, _, _, _) => panic!("unsupported opcode at {:#05X}: {:#06X}", self.pc, opcode),
        }
    }
}
