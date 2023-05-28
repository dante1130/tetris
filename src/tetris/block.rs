use crate::renderer::Renderable;
use sdl2::{pixels::Color, rect::Rect, render::WindowCanvas};

pub struct Position(pub i32, pub i32);

pub struct Shape {
    pub positions: Vec<Position>,
    pub anchor: Position,
}

pub struct Block {
    pub shape: Shape,
    pub color: Color,
}

impl Renderable for Block {
    fn render(&self, canvas: &mut WindowCanvas) {
        for position in &self.shape.positions {
            canvas.set_draw_color(self.color);
            canvas.fill_rect(Rect::new(
                position.0 * 20,
                position.1 * 20,
                20,
                20,
            )).unwrap();
        }
    }
}
