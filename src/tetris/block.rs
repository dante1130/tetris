use crate::renderer::Renderable;
use sdl2::{pixels::Color, rect::Rect, render::WindowCanvas};

pub enum Rotation {
    Deg0,
    Deg90,
    Deg180,
    Deg270,
}

pub struct Block {
    pub x: i32,
    pub y: i32,
    pub color: Color,
    pub shape: Vec<Vec<bool>>,
    pub rotation: Rotation,
}

impl Renderable for Block {
    fn render(&self, canvas: &mut WindowCanvas) {
        const BLOCK_SIZE: u32 = 20;

        for (y, row) in self.shape.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if *cell {
                    canvas.set_draw_color(self.color);
                    canvas.fill_rect(Rect::new(
                        (self.x + x as i32) * BLOCK_SIZE as i32,
                        (self.y + y as i32) * BLOCK_SIZE as i32,
                        BLOCK_SIZE,
                        BLOCK_SIZE,
                    )).unwrap();
                }
            }
        }
    }
}
