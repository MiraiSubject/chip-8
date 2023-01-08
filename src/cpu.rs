#![allow(unused_variables)]

use rand::{thread_rng, Rng};
use std::{fs::File, io::Read};

use crate::{font::{FONT, SPRITE_WIDTH}};

use self::stack::Stack;
mod stack;

const RAM: usize = 4096;
pub const CHIP8_WIDTH: usize = 64;
pub const CHIP8_HEIGHT: usize = 32;
const KEY_COUNT: usize = 16;

#[derive(Debug)]
pub struct CPU {
    program_counter: usize,
    index_register: u16,
    vram: [[u8; CHIP8_WIDTH]; CHIP8_HEIGHT],
    stack: Stack<usize>,
    ram: [u8; RAM],
    delay_timer: u8,
    sound_timer: u8,
    var_registers: [u8; 16],
    keys: [bool; KEY_COUNT],
    super_chip: bool
}

impl CPU {
    pub fn new(super_chip: bool) -> Self {
        let mut cpu = CPU {
            program_counter: 0x200,
            index_register: 0,
            vram: [[0; CHIP8_WIDTH]; CHIP8_HEIGHT],
            stack: Stack::new(),
            ram: [0; RAM],
            delay_timer: 0,
            sound_timer: 0,
            var_registers: [0; 16],
            keys: [false; KEY_COUNT],
            super_chip
        };

        cpu.ram[..80].copy_from_slice(&FONT);
        cpu
    }

    pub fn get_vram(&self) -> &[[u8; CHIP8_WIDTH]; CHIP8_HEIGHT] {
        &self.vram
    }

    pub fn load_rom_in_ram(&mut self, mut file: File) {
        file.read(&mut self.ram[512..RAM]).unwrap();
    }

    pub fn keypress(&mut self, input: usize, pressed: bool) {
        println!("pressed: {input}, {pressed}");
        self.keys[input] = pressed;
    }

    pub fn fetch(&mut self) -> u16 {

        if self.program_counter >= RAM {
            panic!("PC jumped out of range of RAM: {0} >= {RAM}", self.program_counter);
        }

        let instruction_one = self.ram[self.program_counter];
        let instruction_two = self.ram[self.program_counter + 1];

        let final_instruction = ((instruction_one as u16) << 8) | instruction_two as u16;
        self.program_counter += 2;
        final_instruction
    }

    pub fn decode(&mut self, opcode: u16) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1
        }

        let instruction = (
            (0xF000 & opcode) >> 12,
            (0x0F00 & opcode) >> 8,
            (0x00F0 & opcode) >> 4,
            (0x000F & opcode),
        );

        // println!("Opcode {:X}", opcode);

        let nn: u8 = (0x00FF & opcode).try_into().unwrap();
        let nnn: u16 = 0x0FFF & opcode;

        let n: u8 = instruction.3.try_into().unwrap();
        let x: u8 = instruction.1.try_into().unwrap();
        let y: u8 = instruction.2.try_into().unwrap();

        match instruction {
            (0x0, 0x0, 0xe, 0x0) => self.op_00e0(),
            (0x1, _, _, _) => self.op_1nnn(nnn),
            (0x0, 0x0, 0xe, 0xe) => self.op_00ee(),
            (0x2, _, _, _) => self.op_2nnn(nnn),
            (0x3, _, _, _) => self.op_3xnn(x, nn),
            (0x4, _, _, _) => self.op_4xnn(x, nn),
            (0x5, _, _, 0) => self.op_5xy0(x, y),
            (0x6, _, _, _) => self.op_6xnn(x, nn),
            (0x7, _, _, _) => self.op_7xnn(x, nn),
            (0x9, _, _, 0) => self.op_9xy0(x, y),
            (0x8, _, _, 0) => self.op_8xy0(x, y),
            (0x8, _, _, 1) => self.op_8xy1(x, y),
            (0x8, _, _, 2) => self.op_8xy2(x, y),
            (0x8, _, _, 3) => self.op_8xy3(x, y),
            (0x8, _, _, 4) => self.op_8xy4(x, y),
            (0x8, _, _, 5) => self.op_8xy5(x, y),
            (0x8, _, _, 6) => self.op_8xy6(x, y),
            (0x8, _, _, 0xe) => self.op_8xye(x, y),
            (0x8, _, _, 7) => self.op_8xy7(x, y),
            (0xa, _, _, _) => self.op_annn(nnn),
            (0xb, _, _, _) => self.op_bnnn(x, nnn),
            (0xc, _, _, _) => self.op_cxnn(x, nn),
            (0xd, _, _, _) => self.op_dxyn(x, y, n),
            (0xf, _, 0, 7) => self.op_fx07(x),
            (0xf, _, 1, 5) => self.op_fx15(x),
            (0xf, _, 1, 8) => self.op_fx18(x),
            (0xe, _, 9, 0xe) => self.op_ex9e(x),
            (0xe, _, 0xa, 1) => self.op_exa1(x),
            (0xf, _, 0, 0xa) => self.op_fx0a(x),
            (0xf, _, 1, 0xe) => self.op_fx1e(x),
            (0xf, _, 2, 9) => self.op_fx29(x),
            (0xf, _, 3, 3) => self.op_fx33(x),
            (0xf, _, 5, 5) => self.op_fx55(x),
            (0xf, _, 6, 5) => self.op_fx65(x),
            _ => self.unknown_instruction(instruction),
        };

    }

    fn unknown_instruction(&mut self, instruction: (u16, u16, u16, u16)) {
        // println!(
        //     "Instruction: {:X}{:X}{:X}{:X} not implemented",
        //     instruction.0, instruction.1, instruction.2, instruction.3
        // );
    }

    // Clear screen
    fn op_00e0(&mut self) {
        for row in self.vram.iter_mut() {
            for col in row {
                *col = 0;
            }
        }
    }
    // jump
    fn op_1nnn(&mut self, nnn: u16) {
        self.program_counter = nnn as usize;
    }
    // Run subroutine
    fn op_2nnn(&mut self, nnn: u16) {
        self.stack.push(self.program_counter);
        self.program_counter = nnn as usize;
        // println!("nnn: {nnn}");
        // println!("In Ram here: {}", self.ram[nnn as usize]);
    }
    // Return from subroutine
    fn op_00ee(&mut self) {
        self.program_counter = self.stack.pop().unwrap();
    }
    // Skip
    fn op_3xnn(&mut self, x: u8, nn: u8) {
        let val = self.var_registers[x as usize];
        if val == nn {
            self.program_counter += 2;
        }
    }
    // Skip
    fn op_4xnn(&mut self, x: u8, nn: u8) {
        let val = self.var_registers[x as usize];
        if val != nn {
            self.program_counter += 2;
        }
    }
    // Skip
    fn op_5xy0(&mut self, x: u8, y: u8) {
        let val = self.var_registers[x as usize];
        let other = self.var_registers[y as usize];
        if val == other {
            self.program_counter += 2;
        }
    }
    // Skip
    fn op_9xy0(&mut self, x: u8, y: u8) {
        let val = self.var_registers[x as usize];
        let other = self.var_registers[y as usize];
        if val != other {
            self.program_counter += 2;
        }
    }
    // set register VX
    fn op_6xnn(&mut self, x: u8, nn: u8) {
        self.var_registers[x as usize] = nn;
    }
    // Add
    fn op_7xnn(&mut self, x: u8, nn: u8) {
        let (sum, overflow) = (self.var_registers[x as usize]).overflowing_add(nn);
        self.var_registers[x as usize] = sum;
        // Carry flag?
        // *self.var_registers.last_mut().unwrap() = u8::from(overflow);
    }

    // Set
    fn op_8xy0(&mut self, x: u8, y: u8) {
        self.var_registers[x as usize] = self.var_registers[y as usize];
    }

    // Bitwise OR VX
    fn op_8xy1(&mut self, x: u8, y: u8) {
        let val_vx = self.var_registers[x as usize];
        self.var_registers[x as usize] = val_vx | self.var_registers[y as usize];
    }

    fn op_8xy2(&mut self, x: u8, y: u8) {
        let val_vx = self.var_registers[x as usize];
        self.var_registers[x as usize] = val_vx & self.var_registers[y as usize];
    }

    fn op_8xy3(&mut self, x: u8, y: u8) {
        let val_vx = self.var_registers[x as usize];
        self.var_registers[x as usize] = val_vx ^ self.var_registers[y as usize];
    }

    // Add
    fn op_8xy4(&mut self, x: u8, y: u8) {
        let vy = self.var_registers[y as usize];
        let (sum, overflow) = self.var_registers[x as usize].overflowing_add(vy);
        self.var_registers[x as usize] = sum;
        if overflow {
            *self.var_registers.last_mut().unwrap() = 1;
        } else {
            *self.var_registers.last_mut().unwrap() = 0;
        }
    }

    fn op_8xy5(&mut self, x: u8, y: u8) {
        let vx = self.var_registers[x as usize];
        let vy = self.var_registers[y as usize];

        let (sum, underflow) = self.var_registers[x as usize].overflowing_sub(vy); // = vx - vy;
        self.var_registers[x as usize] = sum; 
        if underflow {
            *self.var_registers.last_mut().unwrap() = 0;
        } else {
            *self.var_registers.last_mut().unwrap() = 1;
        }
    }

    fn op_8xy6(&mut self, x: u8, y: u8) {
        if self.super_chip {
            self.var_registers[x as usize] = self.var_registers[y as usize];
        }
        let vx = self.var_registers[x as usize];
        let lsb = vx & 0b0000_0001;
        let shifted = vx >> 1;
        self.var_registers[x as usize] = shifted;

        *self.var_registers.last_mut().unwrap() = lsb;
    }

    fn op_8xye(&mut self, x: u8, y: u8) {
        if self.super_chip {
            self.var_registers[x as usize] = self.var_registers[y as usize];
        }
        let vx = self.var_registers[x as usize];
        let msb = vx & 0b1000_0000; // 1000 0000
        let shifted = vx << 1;
        self.var_registers[x as usize] = shifted;
        
        // If output is 1000 0000 MSB shift 7 times
        *self.var_registers.last_mut().unwrap() = msb >> 7;
    }

    fn op_8xy7(&mut self, x: u8, y: u8) {
        let vx = self.var_registers[x as usize];
        let vy = self.var_registers[y as usize];
        let (sub, underflow) = vy.overflowing_sub(vx);
        if underflow {
            *self.var_registers.last_mut().unwrap() = 0;
        } else {
            *self.var_registers.last_mut().unwrap() = 1;
        }
        self.var_registers[x as usize] = sub;
    }

    fn op_annn(&mut self, nnn: u16) {
        self.index_register = nnn as u16;
    }

    fn op_bnnn(&mut self, x: u8, nnn: u16) {
        let v0 = self.var_registers[0];
        let mut jump_loc = v0 + nnn as u8;
        
        if self.super_chip {
            jump_loc = self.var_registers[x as usize] + nnn as u8;
        }

        println!("Jump loc: {jump_loc}");
        self.program_counter = jump_loc as usize;
    }

    fn op_cxnn(&mut self, x: u8, nn: u8) {
        let mut rng = thread_rng();
        let number: u8 = rng.gen();
        let and_result = number & nn;
        self.var_registers[x as usize] = and_result;
    }

    fn op_fx07(&mut self, x: u8) {
        self.var_registers[x as usize] = self.delay_timer;
    }

    fn op_fx15(&mut self, x: u8) {
        self.delay_timer = self.var_registers[x as usize];
    }

    fn op_fx18(&mut self, x: u8) {
        self.sound_timer = self.var_registers[x as usize];
    }

    fn op_ex9e(&mut self, x: u8) {
        let vx = self.var_registers[x as usize];
        if self.keys[vx as usize] {
            self.program_counter +=2;
        }
    }

    fn op_exa1(&mut self, x: u8) {
        let vx = self.var_registers[x as usize];
        if !self.keys[vx as usize] {
            self.program_counter +=2;
        }
    }

    fn op_fx1e(&mut self, x: u8) {
        self.index_register += self.var_registers[x as usize] as u16;
        if self.index_register > 0x1000 {
            *self.var_registers.last_mut().unwrap() = 1;
        }
    }

    fn op_fx0a(&mut self, x: u8) {
        let mut key_pressed = false;
        for (key, pressed) in self.keys.iter().enumerate() {
            if *pressed {
                self.var_registers[x as usize] = key as u8;
                key_pressed = true;
                break;
            }
        }
        if !key_pressed {
            self.program_counter -= 2;
        }
    }

    fn op_fx29(&mut self, x: u8) {
        let character = self.var_registers[x as usize];
        // println!("Current char select {:X}", character);
        self.index_register = (SPRITE_WIDTH * character) as u16;

    }

    fn op_fx33(&mut self, x: u8) {
        let vx = self.var_registers[x as usize];
        let third = vx % 10;
        let second = (vx / 10) % 10;
        let first = vx / 100;

        let i = self.index_register as usize;
        self.ram[i] = first;
        self.ram[i+1] = second;
        self.ram[i+2] = third;
    }

    fn op_fx55(&mut self, x: u8) {
        let index = self.index_register;
        for i in 0..=x {
            let total = index + i as u16;
        //     println!("FX55: {} {} {}", total, val, i);
            self.ram[total as usize] = self.var_registers[i as usize] as u8;
        }
    }

    fn op_fx65(&mut self, x: u8) {
        let index = self.index_register;
        for i in 0..=x {
            let total = index + i as u16;
        //     println!("FX55: {} {} {}", total, val, i);
            self.var_registers[i as usize] = self.ram[total as usize] as u8;
        }
    }

    // fn op_dxyn(&mut self, x: u8, y: u8, n: u16) {
    //     let x_coord = self.var_registers[x as usize] % CHIP8_WIDTH as u16;
    //     let y_coord = self.var_registers[y as usize] % CHIP8_HEIGHT as u16;

    //     let vf_register = self.var_registers.last_mut().unwrap();
    //     *vf_register = 0;

    //     let mem_loc = self.index_register;

    //     for row in 0..n {
    //         let sprite_data = self.ram[(mem_loc + row) as usize];
    //         let current_y_coord = y_coord + row;
    //         for bit in 0..8 {
    //             let mask = 1 << bit;
    //             let sprite_bit = sprite_data & mask;
    //             if sprite_bit != 0 {
    //                 let current_x_coord = x_coord + bit;
    //                 if usize::from(current_x_coord) >= CHIP8_WIDTH {
    //                     break;
    //                 }
    //                 let screen_pixel = self.vram[current_x_coord as usize][current_y_coord as usize];
    //                 if screen_pixel && sprite_bit == 1 {
    //                     self.vram[current_x_coord as usize][current_y_coord as usize] = false;
    //                     *vf_register = 1;
    //                 } else if screen_pixel && sprite_bit == 1 {
    //                     self.vram[current_x_coord as usize][current_y_coord as usize] = true;
    //                 }
    //             }
    //         }
    //     }
    // }

    // fn op_dxyn(&mut self, x: u8, y: u8, n: u16) {
    //     println!("var_registers: {:?}", self.var_registers);
    //     let x_coord = self.var_registers[x as usize] % CHIP8_WIDTH as u16;
    //     let y_coord = self.var_registers[y as usize] % CHIP8_HEIGHT as u16;

    //     let vf_register = self.var_registers.last_mut().unwrap();
    //     *vf_register = 0;

    //     let mem_loc = self.index_register;

    //     for row in 0..n {
    //         let sprite_data = self.ram[(mem_loc + row) as usize];
    //         let current_y_coord = (y_coord + row) % CHIP8_HEIGHT as u16;
    //         for bit in 0..8 {
    //             let mask = 1 << bit;
    //             let sprite_bit = sprite_data & mask;
    //             if sprite_bit != 0 {
    //                 let current_x_coord = (x_coord + bit) % CHIP8_WIDTH as u16;
    //                 let screen_pixel = self.vram[current_x_coord as usize][current_y_coord as usize];
    //                 if screen_pixel && sprite_bit == 1 {
    //                     self.vram[current_x_coord as usize][current_y_coord as usize] = false;
    //                     *vf_register = 1;
    //                 } else if !screen_pixel && sprite_bit == 1 {
    //                     self.vram[current_x_coord as usize][current_y_coord as usize] = true;
    //                 }
    //             }
    //         }
    //     }
    // }

    fn op_dxyn(&mut self, x: u8, y: u8, n: u8) {
        *self.var_registers.last_mut().unwrap() = 0;
        for byte in 0..n {
            let y = ((self.var_registers[y as usize] + byte) as usize) % CHIP8_HEIGHT;
            for bit in 0..8 {
                let x = (self.var_registers[x as usize] as usize + bit) % CHIP8_WIDTH;
                let color =
                    (self.ram[(self.index_register + byte as u16) as usize] >> (7 - bit)) & 1;
                self.var_registers[0x0f] |= color & self.vram[y][x];
                self.vram[y][x] ^= color;
            }
        }
    }
}
