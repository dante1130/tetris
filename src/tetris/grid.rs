use sdl2::{pixels::Color, rect::Rect, render::WindowCanvas};

use crate::renderer::Renderable;

use super::block::Position;

pub struct Grid {
    pub position: Position,
    pub cells: Vec<Vec<bool>>,
}

impl Grid {
    pub fn new(position: Position, width: usize, height: usize) -> Grid {
        Grid {
            position,
            cells: vec![vec![false; width]; height],
        }
    }

    pub fn clear_full_rows(&mut self) -> u32 {
        let mut cleared_rows = 0;

        for i in 0..self.cells.len() {
            if self.cells[i].iter().all(|&x| x) {
                self.cells.remove(i);
                self.cells.insert(0, vec![false; self.cells[0].len()]);
                cleared_rows += 1;
            }
        }

        cleared_rows
    }
}

impl Clone for Grid {
    fn clone(&self) -> Self {
        Self {
            position: self.position.clone(),
            cells: self.cells.clone(),
        }
    }
}

impl Renderable for Grid {
    fn render(&self, canvas: &mut WindowCanvas) {
        canvas.set_draw_color(Color::RGB(255, 255, 255));

        for (y, row) in self.cells.iter().enumerate() {
            for (x, _cell) in row.iter().enumerate() {
                canvas
                    .draw_rect(Rect::new(
                        (x as i32 + self.position.0) * 20,
                        (y as i32 + self.position.1) * 20,
                        20,
                        20,
                    ))
                    .unwrap();
            }
        }
    }
}
