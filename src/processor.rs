use crate::display::{SCREEN_HEIGHT, SCREEN_WIDTH};
use std::panic;

// processor constants
const BASE_ADDR: usize = 0x200;
const NUM_REGISTERS: usize = 16;
const RAM_SIZE: usize = 4096;
const STACK_SIZE: usize = 16;
const VRAM_SIZE: usize = SCREEN_WIDTH * SCREEN_HEIGHT;

// chip8 processor
pub struct Processor {
    delay_timer: u8,            // delay timer
    i: u16,                     // 12-bit index register
    pc: usize,                  // program counter
    ram: [u8; RAM_SIZE],        // main memory (4096 bytes)
    stack: [usize; STACK_SIZE], // stack (16 12-bit addresses)
    sp: usize,                  // stack pointer
    v: [u8; NUM_REGISTERS],     // 16 8-bit registers
    vram: [u8; VRAM_SIZE],      // framebuffer (64x32)
}

// processor impl
impl Processor {
    // constructor
    pub fn new() -> Self {
        Self {
            delay_timer: 0,
            i: 0,
            pc: BASE_ADDR,
            ram: [0; RAM_SIZE],
            stack: [0; STACK_SIZE],
            sp: 0,
            v: [0; NUM_REGISTERS],
            vram: [0; VRAM_SIZE],
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
        while self.pc < RAM_SIZE - 1 {
            let opcode = self.fetch_instruction();
            self.process_opcode(opcode);
            self.pc += 2;
        }
    }

    // print memory dump
    pub fn print_memory_dump(&self) {
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
            (0, 0, 0xE, 0) => self.vram = [0; VRAM_SIZE],

            // 00EE [flow] RET
            (0, 0, 0xE, 0xE) => {
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

            // 3XNN [cond] SKIP if Vx == NN
            (0x3, _, _, _) => if self.v[x] == nn { self.pc += 2 },

            // 4XNN [cond] SKIP if Vx != NN
            (0x4, _, _, _) => if self.v[x] != nn { self.pc += 2 },

            // 5XY0 [cond] SKIP if Vx == Vy
            (0x5, _, _, 0) => if self.v[x] == self.v[y] { self.pc += 2 },

            // 6XNN [const] SET Vx = NN
            (0x6, _, _, _) => self.v[x] = nn,

            // 7XNN [const] ADD Vx += NN
            (0x7, _, _, _) => self.v[x] += nn,

            // 8XY0 [assig] SET Vx = Vy
            (0x8, _, _, 0) => self.v[x] = self.v[y],

            // [misc] UNSUPPORTED
            (_, _, _, _) => panic!("unsupported opcode at {:#05X}: {:02X}", self.pc, opcode),
        }
    }
}
