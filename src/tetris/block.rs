use crate::renderer::Renderable;
use sdl2::{pixels::Color, rect::Rect, render::WindowCanvas};

use super::tetris::BLOCK_SIZE;

#[derive(Clone)]
pub struct Position(pub i32, pub i32);

pub struct Block {
    pub position: Position,
    pub shapes: [Vec<Position>; 4],
    pub shape_index: usize,
    pub color: Color,
}

impl Clone for Block {
    fn clone(&self) -> Self {
        Self {
            position: self.position.clone(),
            shapes: self.shapes.clone(),
            shape_index: self.shape_index.clone(),
            color: self.color.clone(),
        }
    }
}

impl Block {
    pub fn current_shape(&self) -> &Vec<Position> {
        &self.shapes[self.shape_index]
    }

    pub fn fall(&mut self) {
        self.position.1 += 1;
    }

    pub fn move_left(&mut self) {
        self.position.0 -= 1;
    }

    pub fn move_right(&mut self) {
        self.position.0 += 1;
    }

    pub fn rotate_clockwise(&mut self) {
        self.shape_index = (self.shape_index + 1) % 4;
    }

    pub fn rotate_counter_clockwise(&mut self) {
        self.shape_index = (self.shape_index + 3) % 4;
    }
}

impl Renderable for Block {
    fn render(&self, canvas: &mut WindowCanvas) {
        canvas.set_draw_color(self.color);

        for position in self.current_shape().iter() {
            let x = (self.position.0 + position.0) * BLOCK_SIZE as i32;
            let y = (self.position.1 + position.1) * BLOCK_SIZE as i32;
            canvas.fill_rect(Rect::new(x, y, BLOCK_SIZE, BLOCK_SIZE)).unwrap();
        }
    }
}
