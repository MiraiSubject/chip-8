extern crate sdl2;

use display::Display;
use input::{Chip8KeyCode, Input};
use std::time::{Duration, Instant};

use std::fs::File;

pub mod cpu;
mod display;
pub mod font;
mod input;

use cpu::CPU;

fn main() {

    let sdl_context = sdl2::init().unwrap();

    let mut display = Display::new(&sdl_context, 20);
    let mut input = Input::new(&sdl_context);

    let file = File::open("Keypad Test [Hap, 2006].ch8").expect("msg");
    let mut c = CPU::new(false);
    c.load_rom_in_ram(file);

    loop {
        let t0 = Instant::now();
        let (chip8_keycode, pressed) = input.input_loop();
        // Received exit signal from input.
        match (&chip8_keycode, pressed) {
            (Some(Chip8KeyCode::Exit), true) => break,
            (None, false) | (None, true) => {}
            // Handle keycode in cpu
            (Some(_), _) => {
                let hex = Input::decode_input(chip8_keycode.unwrap());
                c.keypress(hex, pressed)
            }
        }
        let vram = c.get_vram();
        display.draw(vram);
        
        for _ in 0..12 {
            c.cycle();
        }
        
        c.cycle_timers();

        let elapsed = t0.elapsed().as_nanos();

        ::std::thread::sleep(Duration::new(0, ((1_000_000_000u128 - elapsed) / 60).try_into().unwrap()));
    }
}
