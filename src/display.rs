use sdl2::{pixels::Color, Sdl};
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct Display {
    canvas: Canvas<Window>,
    render_scale: u32
}

use crate::cpu::{CHIP8_HEIGHT, CHIP8_WIDTH};

impl Display {
    pub fn new(sdl_context: &Sdl, render_scale: u32) -> Self {
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window(" Mirai's Chip-8 interpreter", CHIP8_WIDTH as u32 * render_scale, CHIP8_HEIGHT as u32 * render_scale)
            //.position_centered()
            .position(-100, 0)
            .build()
            .unwrap();

        let mut d = Display {
            canvas: window.into_canvas().build().unwrap(),
            render_scale
        };

        d.canvas.set_draw_color(Color::RGB(0, 0, 0));
        d.canvas.clear();
        d.canvas.present();
        d
    }

    pub fn draw(&mut self, vram: &[[u8; CHIP8_WIDTH]; CHIP8_HEIGHT]) {
        self.canvas.clear();

        for y in vram.iter().enumerate() {
            let y_coord = y.0 as u32 * self.render_scale;
            for (x, &row) in y.1.iter().enumerate() {
                let x_coord = x as u32 * self.render_scale;
                let mut output_color = Color::RGB(0, 0, 0);

                if row == 1 {
                    output_color = Color::RGB(255, 255, 255);
                }

                self.canvas.set_draw_color(output_color);

                let _ = self.canvas.fill_rect(Rect::new(x_coord as i32, y_coord as i32, self.render_scale, self.render_scale));
            }
        }

        self.canvas.present();
    }
}
