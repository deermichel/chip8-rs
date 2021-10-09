use crate::display::{SCREEN_HEIGHT, SCREEN_WIDTH};

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
    stack: [u16; STACK_SIZE],   // stack (16 12-bit addresses)
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

    // print memory dump
    pub fn print_memory_dump(&self) {
        for i in 0..RAM_SIZE {
            if i % 32 == 0 { print!("{:#05X}  |  ", i); }
            print!("{:02X}  ", self.ram[i]);
            if i % 32 == 31 { println!(); }
        }
    }
}
