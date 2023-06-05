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

pub enum LockResult {
    Locked,
    GameOver,
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

    pub fn lock_block(&mut self, block: &Block) -> LockResult {
        for block_position in block.world_block_positions() {
            let x = block_position.0 - self.position.0;
            let y = block_position.1 - self.position.1;

            if y < 0 {
                return LockResult::GameOver;
            }

            self.cells[y as usize][x as usize] = Some(block.color);
        }

        LockResult::Locked
    }

    pub fn clear_full_rows(&mut self) -> u32 {
        let mut cleared_rows = 0;

        for i in 0..self.cells.len() {
            if self.cells[i].iter().all(|&color| color.is_some()) {
                self.cells.remove(i);
                self.cells.insert(0, vec![None; self.cells[0].len()]);
                cleared_rows += 1;
            }
        }

        cleared_rows
    }

    pub fn is_colliding(&self, block: &Block) -> Collision {
        for block_position in block.world_block_positions() {
            let x = block_position.0 - self.position.0;
            let y = block_position.1 - self.position.1;

            if x < 0 {
                return Collision::Left;
            }

            if x >= self.cells[0].len() as i32 {
                return Collision::Right;
            }

            if y < 0 {
                return Collision::Top;
            }

            if y >= self.cells.len() as i32 || self.cells[y as usize][x as usize].is_some() {
                return Collision::Bottom;
            }
        }

        Collision::None
    }

    pub fn is_touching_locked_blocks(&self, block: &Block) -> (bool, bool) {
        let mut touching_dir_tuple = (false, false);

        for block_position in block.world_block_positions() {
            let x = block_position.0 - self.position.0;
            let y = block_position.1 - self.position.1;

            if y < 0 || y >= self.cells.len() as i32 {
                continue;
            }

            let left_x = x - 1;
            let right_x = x + 1;

            let row = &self.cells[y as usize];

            if left_x >= 0 && row[left_x as usize].is_some() {
                touching_dir_tuple.0 = true;
            }

            if right_x < row.len() as i32 && row[right_x as usize].is_some() {
                touching_dir_tuple.1 = true;
            }

            if touching_dir_tuple.0 && touching_dir_tuple.1 {
                break;
            }
        }

        touching_dir_tuple
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
