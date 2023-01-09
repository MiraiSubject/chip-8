pub mod display;
pub mod input;
use display::Display;
use input::Input;

pub trait Frontend {
    type Display: Display;
    type Input: Input;
    fn display(&mut self) -> &mut Self::Display;
    fn input(&mut self) -> &mut Self::Input;
    fn new(render_scale: u32) -> Self;
}