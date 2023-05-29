use crate::renderer::Renderable;
use sdl2::{pixels::Color, rect::Rect, render::WindowCanvas};

#[derive(Clone)]
pub struct Position(pub i32, pub i32);

pub struct Shape {
    pub positions: Vec<Position>,
    pub anchor: Position,
}

#[derive(Clone)]
pub struct Block {
    pub shape: Shape,
    pub color: Color,
}

impl Clone for Shape {
    fn clone(&self) -> Self {
        Shape {
            positions: self.positions.clone(),
            anchor: self.anchor.clone(),
        }
    }
}

impl Block {
    pub fn fall(&mut self) {
        for position in &mut self.shape.positions {
            position.1 += 1;
        }
    }

    pub fn move_left(&mut self) {
        for position in &mut self.shape.positions {
            position.0 -= 1;
        }
    }

    pub fn move_right(&mut self) {
        for position in &mut self.shape.positions {
            position.0 += 1;
        }
    }
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
