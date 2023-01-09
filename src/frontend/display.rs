#![allow(dead_code)]

use crate::cpu::{CHIP8_WIDTH, CHIP8_HEIGHT};

pub trait Display {
    fn draw(&mut self, vram: &[[u8; CHIP8_WIDTH]; CHIP8_HEIGHT]);
}

pub struct HeadlessDisplay;

impl Display for HeadlessDisplay {
    fn draw(&mut self, _vram: &[[u8; CHIP8_WIDTH]; CHIP8_HEIGHT]) {
    }
}

impl HeadlessDisplay {
    pub fn new() -> Self {
        Self
    }
}