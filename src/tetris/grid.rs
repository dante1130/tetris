use sdl2::{pixels::Color, rect::Rect, render::WindowCanvas};

use crate::renderer::Renderable;

use super::{block::Position, tetris::BLOCK_SIZE};

pub struct Grid {
    pub position: Position,
    cells: Vec<Vec<Option<Color>>>,
}

impl Grid {
    pub fn new(position: Position, width: usize, height: usize) -> Grid {
        Grid {
            position,
            cells: vec![vec![None; width]; height],
        }
    }

    pub fn clear_full_rows(&mut self) -> u32 {
        let mut cleared_rows = 0;

        for i in 0..self.cells.len() {
            if self.cells[i].iter().all(|&color| color != None ) {
                self.cells.remove(i);
                self.cells.insert(0, vec![None; self.cells[0].len()]);
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

        for (y, row) in self.cells.iter().enumerate() {
            for (x, color) in row.iter().enumerate() {
                if let Some(color) = color {
                    canvas.set_draw_color(*color);
                    canvas
                        .fill_rect(Rect::new(
                            (x as i32 + self.position.0) * BLOCK_SIZE as i32,
                            (y as i32 + self.position.1) * BLOCK_SIZE as i32,
                            BLOCK_SIZE,
                            BLOCK_SIZE,
                        ))
                        .unwrap();
                };

                canvas.set_draw_color(Color::WHITE);
                canvas
                    .draw_rect(Rect::new(
                        (x as i32 + self.position.0) * BLOCK_SIZE as i32,
                        (y as i32 + self.position.1) * BLOCK_SIZE as i32,
                        BLOCK_SIZE,
                        BLOCK_SIZE,
                    ))
                    .unwrap();
            }
        }
    }
}
