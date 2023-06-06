use sdl2::{pixels::Color, rect::Rect, render::WindowCanvas};

use crate::renderable::Renderable;

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
            shape_index: self.shape_index,
            color: self.color,
        }
    }
}

impl Block {
    pub fn current_shape(&self) -> &Vec<Position> {
        &self.shapes[self.shape_index]
    }

    pub fn world_block_positions(&self) -> Vec<Position> {
        self.current_shape()
            .iter()
            .map(|position| Position(self.position.0 + position.0, self.position.1 + position.1))
            .collect()
    }

    pub fn soft_drop(&mut self) {
        self.position.1 += 1;
    }

    pub fn move_up(&mut self) {
        self.position.1 -= 1;
    }

    pub fn move_left(&mut self) {
        self.position.0 -= 1;
    }

    pub fn move_right(&mut self) {
        self.position.0 += 1;
    }

    pub fn rotate_clockwise(&mut self) {
        if self.shape_index == self.shapes.len() - 1 {
            self.shape_index = 0;
        } else {
            self.shape_index += 1;
        }
    }

    pub fn rotate_counter_clockwise(&mut self) {
        if self.shape_index == 0 {
            self.shape_index = self.shapes.len() - 1;
        } else {
            self.shape_index -= 1;
        }
    }
}

impl Renderable for Block {
    fn render(&self, canvas: &mut WindowCanvas) {
        canvas.set_draw_color(self.color);

        for position in self.current_shape().iter() {
            let x = (self.position.0 + position.0) * BLOCK_SIZE as i32;
            let y = (self.position.1 + position.1) * BLOCK_SIZE as i32;
            canvas
                .fill_rect(Rect::new(x, y, BLOCK_SIZE, BLOCK_SIZE))
                .unwrap();
        }
    }
}
