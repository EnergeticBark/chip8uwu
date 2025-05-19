use std::error::Error;

use rand::random;

use super::op::Op;
use crate::chip8::font::Font;

pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;

const ROM_START: u16 = 0x200;
const SCREEN_START: usize = 0xf00;
const FONT_START: u16 = 0x000;

pub struct State {
    pub rom_loaded: bool,
    pub v: [u8; 16],
    i: u16,
    sp: u8,
    pub pc: u16,
    pub delay: u8,
    sound: u8,
    pub memory: [u8; 4096],
    stack: [u16; 16],
    pub keyboard: [bool; 16],
}

impl State {
    pub fn new() -> Self {
        State {
            rom_loaded: false,
            v: [0x00; 16],
            i: 0x00,
            sp: 0x00,
            pc: ROM_START,
            delay: 0x00,
            sound: 0x00,
            memory: [0x00; 4096],
            stack: [0x00; 16],
            keyboard: [false; 16],
        }
    }

    fn load_font(&mut self, font: Font) {
        let mut mem_index = FONT_START as usize;
        for char in font.0 {
            for line in char {
                self.memory[mem_index] = line;
                mem_index += 1;
            }
        }
    }

    // initializes state and loads rom
    pub fn load_rom(&mut self, rom: &[u8]) {
        *self = Self::new();
        self.load_font(Font::new());

        for (i, byte) in rom.iter().enumerate() {
            self.memory[ROM_START as usize + i] = *byte;
        }
        self.rom_loaded = true;
    }

    fn xor_pixel(&mut self, x: usize, y: usize) -> bool {
        // bounds check
        let x = x % 64;
        let y = y % 32;

        let mut flipped = false;

        // get the address of the byte where the pixel should be drawn
        let address = SCREEN_START + ((y * 64) + x) / 8;
        // get the amount of bits we need to shift to the right
        let pixel_bit_offset = ((y * 64) + x) % 8;
        let pixel_byte = &mut self.memory[address];

        if (*pixel_byte << pixel_bit_offset) & 0b1000_0000 != 0 {
            flipped = true;
        }
        // shift the bit into position and then XOR it to the byte
        *pixel_byte ^= 0b1000_0000 >> pixel_bit_offset;

        flipped
    }

    pub fn emulate(&mut self) -> Result<(), Box<dyn Error>> {
        let op_byte1 = self.memory[self.pc as usize];
        let op_byte2 = self.memory[self.pc as usize + 1];
        let op = Op::new(op_byte1, op_byte2)?;

        match op {
            Op::Cls => {
                for (i, byte) in self.memory[SCREEN_START..].iter_mut().enumerate() {
                    if i >= 256 {
                        break;
                    }
                    *byte = 0x00;
                }
                self.pc += 2;
            }
            Op::Rts => {
                self.sp -= 1; // Decrement stack pointer.
                self.pc = self.stack[self.sp as usize];
            }
            Op::Jump(address) => self.pc = address,
            Op::Call(address) => {
                self.pc += 2; // Increment program counter to get next instruction.
                self.stack[self.sp as usize] = self.pc; // Push next instruction to the stack.
                self.sp += 1; // Increment stack pointer.

                self.pc = address;
            }
            Op::SkipEqLit { v, lit } => {
                if self.v[v as usize] == lit {
                    self.pc += 2;
                }
                self.pc += 2;
            }
            Op::SkipNeLit { v, lit } => {
                if self.v[v as usize] != lit {
                    self.pc += 2;
                }
                self.pc += 2;
            }
            Op::SkipEq { v, v2 } => {
                if self.v[v as usize] == self.v[v2 as usize] {
                    self.pc += 2;
                }
                self.pc += 2;
            }
            Op::MviLit { v, lit } => {
                self.v[v as usize] = lit;
                self.pc += 2;
            }
            Op::AdiLit { v, lit } => {
                self.v[v as usize] = self.v[v as usize].wrapping_add(lit);
                self.pc += 2;
            }
            Op::Mov { v, v2 } => {
                self.v[v as usize] = self.v[v2 as usize];
                self.pc += 2;
            }
            Op::Or { v, v2 } => {
                let value = self.v[v as usize];
                let value2 = self.v[v2 as usize];
                self.v[v as usize] = value | value2;
                self.pc += 2;
            }
            Op::And { v, v2 } => {
                let value = self.v[v as usize];
                let value2 = self.v[v2 as usize];
                self.v[v as usize] = value & value2;
                self.pc += 2;
            }
            Op::Xor { v, v2 } => {
                let value = self.v[v as usize];
                let value2 = self.v[v2 as usize];
                self.v[v as usize] = value ^ value2;
                self.pc += 2;
            }
            Op::Add { v, v2 } => {
                let value = self.v[v as usize];
                let value2 = self.v[v2 as usize];
                let (result, overflow) = value.overflowing_add(value2);
                self.v[v as usize] = result;
                self.v[0xF] = u8::from(overflow);
                self.pc += 2;
            }
            Op::Sub { v, v2 } => {
                let value = self.v[v as usize];
                let value2 = self.v[v2 as usize];
                let (result, underflow) = value.overflowing_sub(value2);
                self.v[v as usize] = result;
                self.v[0xF] = u8::from(!underflow);
                self.pc += 2;
            }
            Op::Shr(v) => {
                let value = self.v[v as usize];
                self.v[v as usize] = value >> 1;
                self.v[0xF] = value & 0b0000_0001;
                self.pc += 2;
            }
            Op::Subb { v, v2 } => {
                let value = self.v[v as usize];
                let value2 = self.v[v2 as usize];
                let (result, underflow) = value2.overflowing_sub(value);
                self.v[v as usize] = result;
                self.v[0xF] = u8::from(!underflow);
                self.pc += 2;
            }
            Op::Shl(v) => {
                let value = self.v[v as usize];
                self.v[v as usize] = value << 1;
                self.v[0xF] = (value & 0b1000_0000) >> 7;
                self.pc += 2;
            }
            Op::SkipNe { v, v2 } => {
                if self.v[v as usize] != self.v[v2 as usize] {
                    self.pc += 2;
                }
                self.pc += 2;
            }
            Op::SetI(address) => {
                self.i = address;
                self.pc += 2;
            }
            Op::JumpPlusV0(address) => {
                self.pc = address + u16::from(self.v[0x0]);
            }
            Op::Rand { v, lit } => {
                let random_byte = random::<u8>();
                self.v[v as usize] = lit & random_byte;
                self.pc += 2;
            }
            Op::Draw { v, v2, lit } => {
                let mut flipped = false;
                for row in 0..lit {
                    let line = self.memory[self.i as usize + row as usize];
                    for column in 0..8 {
                        let pixel = line << column & 0b1000_0000;
                        if pixel != 0 {
                            let x = self.v[v as usize] as usize + column;
                            let y = self.v[v2 as usize] as usize + row as usize;

                            if self.xor_pixel(x, y) {
                                flipped = true;
                            }
                        }
                    }
                }
                self.v[0xF] = u8::from(flipped);
                self.pc += 2;
            }
            Op::SkipKey(v) => {
                let key = self.v[v as usize];
                if self.keyboard[key as usize] {
                    self.pc += 2;
                }
                self.pc += 2;
            }
            Op::SkipNoKey(v) => {
                let key = self.v[v as usize];
                if !self.keyboard[key as usize] {
                    self.pc += 2;
                }
                self.pc += 2;
            }
            Op::GetDelay(v) => {
                self.v[v as usize] = self.delay;
                self.pc += 2;
            }
            Op::GetKey(v) => {
                for (key, pressed) in (0..).zip(self.keyboard.iter()) {
                    if *pressed {
                        self.v[v as usize] = key;
                        self.pc += 2;
                        break;
                    }
                }
            }
            Op::Delay(v) => {
                self.delay = self.v[v as usize];
                self.pc += 2;
            }
            Op::Sound(v) => {
                self.sound = self.v[v as usize];
                self.pc += 2;
            }
            Op::AddI(v) => {
                let value = u16::from(self.v[v as usize]);
                self.i = self.i.wrapping_add(value);
                self.pc += 2;
            }
            Op::SpriteChar(v) => {
                // get the value of v[v]
                // each of the characters in our font are made up of 5 bytes,
                // so we multiply this value by 5 and add FONT_START
                // this is where our character's font lies in memory
                self.i = FONT_START + u16::from(self.v[v as usize]) * 5;
                self.pc += 2;
            }
            Op::MovBcd(v) => {
                let value = self.v[v as usize];
                let ones = value % 10;
                let tens = value / 10 % 10;
                let hundreds = value / 10 / 10 % 10;

                self.memory[self.i as usize] = hundreds;
                self.memory[self.i as usize + 1] = tens;
                self.memory[self.i as usize + 2] = ones;
                self.pc += 2;
            }
            Op::RegDump(vx) => {
                for v in 0..=vx {
                    self.memory[self.i as usize + v as usize] = self.v[v as usize];
                }
                self.pc += 2;
            }
            Op::RegLoad(vx) => {
                for v in 0..=vx {
                    self.v[v as usize] = self.memory[self.i as usize + v as usize];
                }
                self.pc += 2;
            }
        }

        Ok(())
    }

    pub fn frame_grayscale(&self) -> Vec<u8> {
        let mut frame = Vec::new();
        for i in 0..WIDTH * HEIGHT {
            let screen_bit = {
                let byte = SCREEN_START + i / 8;
                let bit_offset = i % 8;
                self.memory[byte] << bit_offset
            };

            if screen_bit & 0b1000_0000 != 0 {
                // draw white
                frame.push(0xff);
            } else {
                // draw black
                frame.push(0x00);
            }
        }
        frame
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn emu_set_i() {
        let rom = vec![0xa1, 0x23]; // MVI I,#$123
        let mut state = State::new();
        state.load_rom(&rom);

        state.emulate().unwrap();
        assert_eq!(0x0123, state.i);
    }

    #[test]
    fn emu_mov_bcd() {
        let rom = vec![
            0xa3, 0x00, // MVI    I,#$300
            0x61, 0x7b, // MVI    V1,#$7b
            0xf1, 0x33, // MOVBCD V1
        ];
        let mut state = State::new();
        state.load_rom(&rom);

        state.emulate().unwrap();
        state.emulate().unwrap();
        state.emulate().unwrap(); // emulate all 3 instructions
        let hundreds = state.memory[state.i as usize];
        let tens = state.memory[state.i as usize + 1];
        let ones = state.memory[state.i as usize + 2];
        assert_eq!((0x01, 0x02, 0x03), (hundreds, tens, ones));
    }
}
