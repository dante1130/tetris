use sdl2::{pixels::Color, rect::Rect, render::WindowCanvas};

use crate::renderer::Renderable;

pub struct Grid {
    pub cells: Vec<Vec<bool>>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Grid {
        Grid {
            cells: vec![vec![false; width]; height],
        }
    }
}

impl Clone for Grid {
    fn clone(&self) -> Self {
        Self {
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
                    .draw_rect(Rect::new((x * 20) as i32, (y * 20) as i32, 20, 20))
                    .unwrap();
            }
        }
    }
}
