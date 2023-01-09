#![allow(dead_code)]

mod software;

mod input;
mod audio;
use core::panic;

use input::SDLInput;

use crate::frontend::Frontend;

use self::{software::SDL2SoftwareDisplay, audio::AudioBackend};

#[derive(Debug)]
pub enum SDLDisplayRenderer {
    Software,
    Vulkan,
    OpenGL,
    Metal
}

pub struct SDL2Frontend {
    display: SDL2SoftwareDisplay,
    input: SDLInput,
    audio: AudioBackend
}

impl Frontend for SDL2Frontend {
    type Display = SDL2SoftwareDisplay;
    type Input = SDLInput;

    fn display(&mut self) -> &mut Self::Display {
        &mut self.display
    }

    fn new(_render_scale: u32) -> Self {
        panic!("Use the new_frontend() method");
    }

    fn input(&mut self) -> &mut Self::Input {
        &mut self.input
    }
}

impl SDL2Frontend {
    pub fn new_frontend(renderer: SDLDisplayRenderer, render_scale: u32) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let input = SDLInput::from_context(&sdl_context);
        let audio = AudioBackend::new(&sdl_context);

        let display = match renderer {
            SDLDisplayRenderer::Software => SDL2SoftwareDisplay::from_context(&sdl_context, render_scale),
            _ => {
                println!("{renderer:?} not available. Falling back to software rendering");
                SDL2SoftwareDisplay::from_context(&sdl_context, render_scale)
            } 
        };
        Self {
            display,
            input,
            audio
        }
    }

    pub fn audio(&self) -> &AudioBackend {
        &self.audio
    }
}
