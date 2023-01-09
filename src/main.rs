use crate::frontend::display::Display;
extern crate sdl2;
extern crate imgui;

use frontend::input::{Chip8KeyCode, Input};
use sdl::SDL2Frontend;
use std::time::{Duration, Instant};

use std::fs::File;

pub mod cpu;
pub mod font;
mod sdl;
mod frontend;

use cpu::CPU;
use frontend::Frontend;

fn main() {
    let mut fr = SDL2Frontend::new_frontend(sdl::SDLDisplayRenderer::Software, 20);

    let file = File::open("sp_pong.ch8").expect("msg");
    let mut c = CPU::new(false);
    c.load_rom_in_ram(file);

    loop {
        let t0 = Instant::now();
        let (chip8_keycode, pressed) = fr.input().input_loop();
        // Received exit signal from input.
        match (&chip8_keycode, pressed) {
            (Some(Chip8KeyCode::Exit), true) => break,
            (None, false) | (None, true) => {}
            // Handle keycode in cpu
            (Some(_), _) => {
                let hex = <SDL2Frontend as Frontend>::Input::decode_input(chip8_keycode.unwrap());
                c.keypress(hex, pressed)
            }
        }
        let vram = c.get_vram();
        fr.display().draw(vram);
        
        for _ in 0..12 {
            c.cycle();
        }
        
        let sound_timer = c.cycle_timers();

        if sound_timer > 0 {
            fr.audio().start_beep();
        } else {
            fr.audio().stop_beep();
        }

        let elapsed = t0.elapsed().as_nanos();

        ::std::thread::sleep(Duration::new(0, ((1_000_000_000u128 - elapsed) / 60).try_into().unwrap()));
    }
}
