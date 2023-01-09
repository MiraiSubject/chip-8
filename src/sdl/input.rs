use sdl2::{Sdl, EventPump, event::Event, keyboard::{Keycode, Scancode}};

use crate::frontend::input::{Chip8KeyCode, Input};

pub struct SDLInput {
    event_pump: EventPump,
}

impl Input for SDLInput {
    fn new() -> Self {
        todo!()
    }

    fn input_loop(&mut self) -> (Option<Chip8KeyCode>, bool) {
        if let Some(event) = self.event_pump.poll_event() {
            return match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => (Some(Chip8KeyCode::Exit), true),
                Event::KeyDown { scancode: Some(Scancode::Num1), .. } => (Some(Chip8KeyCode::One), true),
                Event::KeyDown { scancode: Some(Scancode::Num2), .. } => (Some(Chip8KeyCode::Two), true),
                Event::KeyDown { scancode: Some(Scancode::Num3), .. } => (Some(Chip8KeyCode::Three), true),
                Event::KeyDown { scancode: Some(Scancode::Num4), .. } => (Some(Chip8KeyCode::C), true),
                Event::KeyDown { scancode: Some(Scancode::Q), .. } => (Some(Chip8KeyCode::Four), true),
                Event::KeyDown { scancode: Some(Scancode::W), .. } => (Some(Chip8KeyCode::Five), true),
                Event::KeyDown { scancode: Some(Scancode::E), .. } => (Some(Chip8KeyCode::Six), true),
                Event::KeyDown { scancode: Some(Scancode::R), .. } => (Some(Chip8KeyCode::D), true),
                Event::KeyDown { scancode: Some(Scancode::A), .. } => (Some(Chip8KeyCode::Seven), true),
                Event::KeyDown { scancode: Some(Scancode::S), .. } => (Some(Chip8KeyCode::Eight), true),
                Event::KeyDown { scancode: Some(Scancode::D), .. } => (Some(Chip8KeyCode::Nine), true),
                Event::KeyDown { scancode: Some(Scancode::F), .. } => (Some(Chip8KeyCode::E), true),
                Event::KeyDown { scancode: Some(Scancode::Z), .. } => (Some(Chip8KeyCode::A), true),
                Event::KeyDown { scancode: Some(Scancode::X), .. } => (Some(Chip8KeyCode::Zero), true),
                Event::KeyDown { scancode: Some(Scancode::C), .. } => (Some(Chip8KeyCode::B), true),
                Event::KeyDown { scancode: Some(Scancode::V), .. } => (Some(Chip8KeyCode::F), true),
                Event::KeyUp { keycode: Some(Keycode::Escape), .. } => (Some(Chip8KeyCode::Exit), false),
                Event::KeyUp { scancode: Some(Scancode::Num1), .. } => (Some(Chip8KeyCode::One), false),
                Event::KeyUp { scancode: Some(Scancode::Num2), .. } => (Some(Chip8KeyCode::Two), false),
                Event::KeyUp { scancode: Some(Scancode::Num3), .. } => (Some(Chip8KeyCode::Three), false),
                Event::KeyUp { scancode: Some(Scancode::Num4), .. } => (Some(Chip8KeyCode::C), false),
                Event::KeyUp { scancode: Some(Scancode::Q), .. } => (Some(Chip8KeyCode::Four), false),
                Event::KeyUp { scancode: Some(Scancode::W), .. } => (Some(Chip8KeyCode::Five), false),
                Event::KeyUp { scancode: Some(Scancode::E), .. } => (Some(Chip8KeyCode::Six), false),
                Event::KeyUp { scancode: Some(Scancode::R), .. } => (Some(Chip8KeyCode::D), false),
                Event::KeyUp { scancode: Some(Scancode::A), .. } => (Some(Chip8KeyCode::Seven), false),
                Event::KeyUp { scancode: Some(Scancode::S), .. } => (Some(Chip8KeyCode::Eight), false),
                Event::KeyUp { scancode: Some(Scancode::D), .. } => (Some(Chip8KeyCode::Nine), false),
                Event::KeyUp { scancode: Some(Scancode::F), .. } => (Some(Chip8KeyCode::E), false),
                Event::KeyUp { scancode: Some(Scancode::Z), .. } => (Some(Chip8KeyCode::A), false),
                Event::KeyUp { scancode: Some(Scancode::X), .. } => (Some(Chip8KeyCode::Zero), false),
                Event::KeyUp { scancode: Some(Scancode::C), .. } => (Some(Chip8KeyCode::B), false),
                Event::KeyUp { scancode: Some(Scancode::V), .. } => (Some(Chip8KeyCode::F), false),
                _ => (None, false)
            };
        }
        (None, false)
    }
}

impl SDLInput {
    pub fn from_context(sdl_context: &Sdl) -> Self {
        Self {
            event_pump: sdl_context.event_pump().unwrap(),
        }
    }
}