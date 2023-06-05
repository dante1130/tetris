use sdl2::{pixels::Color, rect::Rect, render::WindowCanvas};

use crate::renderer::Renderable;

use super::{
    block::{Block, Position},
    tetris::BLOCK_SIZE,
};

pub enum Collision {
    None,
    Left,
    Right,
    Top,
    Bottom,
}

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

    pub fn lock_block(&mut self, block: &Block) {
        for block_position in block.world_block_positions() {
            let x = block_position.0 - self.position.0;
            let y = block_position.1 - self.position.1;

            self.cells[y as usize][x as usize] = Some(block.color);
        }
    }

    pub fn clear_full_rows(&mut self) -> u32 {
        let mut cleared_rows = 0;

        for i in 0..self.cells.len() {
            if self.cells[i].iter().all(|&color| color != None) {
                self.cells.remove(i);
                self.cells.insert(0, vec![None; self.cells[0].len()]);
                cleared_rows += 1;
            }
        }

        cleared_rows
    }

    pub fn is_colliding(&self, block: &Block) -> Collision {
        for block_position in block.world_block_positions() {
            let diff_block_grid = Position(
                block_position.0 - self.position.0,
                block_position.1 - self.position.1,
            );

            if diff_block_grid.0 < 0 {
                return Collision::Left;
            }

            if diff_block_grid.0 >= self.cells[0].len() as i32 {
                return Collision::Right;
            }

            if diff_block_grid.1 < 0 {
                return Collision::Top;
            }

            if diff_block_grid.1 >= self.cells.len() as i32
                || self.cells[diff_block_grid.1 as usize][diff_block_grid.0 as usize] != None
            {
                return Collision::Bottom;
            }
        }

        Collision::None
    }

    pub fn is_touching_locked_blocks(&self, block: &Block) -> Collision {
        for block_position in block.world_block_positions() {
            for (y, row) in self.cells.iter().enumerate() {
                for (x, cell) in row.iter().enumerate() {
                    if *cell == None {
                        continue;
                    }

                    let cell_position =
                        Position(x as i32 + self.position.0, y as i32 + self.position.1);

                    if cell_position.1 != block_position.1 {
                        continue;
                    }

                    if cell_position.0 + 1 == block_position.0 {
                        return Collision::Left;
                    }

                    if cell_position.0 - 1 == block_position.0 {
                        return Collision::Right;
                    }
                }
            }
        }

        Collision::None
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
                let cell_position =
                    Position(x as i32 + self.position.0, y as i32 + self.position.1);

                if let Some(color) = color {
                    canvas.set_draw_color(*color);
                    canvas
                        .fill_rect(Rect::new(
                            cell_position.0 * BLOCK_SIZE as i32,
                            cell_position.1 * BLOCK_SIZE as i32,
                            BLOCK_SIZE,
                            BLOCK_SIZE,
                        ))
                        .unwrap();
                };

                canvas.set_draw_color(Color::WHITE);
                canvas
                    .draw_rect(Rect::new(
                        cell_position.0 * BLOCK_SIZE as i32,
                        cell_position.1 * BLOCK_SIZE as i32,
                        BLOCK_SIZE,
                        BLOCK_SIZE,
                    ))
                    .unwrap();
            }
        }
    }
}
