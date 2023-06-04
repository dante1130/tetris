use std::collections::VecDeque;

use super::{
    block::{Block, Position},
    blocks::spawn_block,
    grid::{Collision, Grid},
};

pub const BLOCK_SIZE: u32 = 20;

pub struct Tetris {
    pub grid: Box<Grid>,
    pub current_block: Box<Block>,
    pub hold_block: Option<Box<Block>>,
    pub blocks_queue: VecDeque<Box<Block>>,
    pub spawn_position: Position,
}

impl Tetris {
    pub fn new(spawn_position: Position, grid_position: Position) -> Tetris {
        let mut blocks_queue = VecDeque::new();
        for _ in 0..5 {
            blocks_queue.push_back(Box::new(spawn_block(spawn_position.0, spawn_position.1)));
        }

        Tetris {
            grid: Box::new(Grid::new(
                Position(grid_position.0, grid_position.1),
                10,
                20,
            )),
            current_block: blocks_queue.pop_front().unwrap(),
            hold_block: None,
            blocks_queue,
            spawn_position,
        }
    }

    pub fn update(&mut self) {
        match self.grid.is_colliding(&self.current_block) {
            Collision::Left => self.current_block.move_right(),
            Collision::Right => self.current_block.move_left(),
            Collision::Bottom => {
                self.current_block.move_up();
                self.grid.lock_block(&self.current_block);
                self.renew_current_block();
            }
            _ => {}
        }

        self.grid.clear_full_rows();
    }

    pub fn fixed_update(&mut self) {
        self.current_block.soft_drop();
    }

    pub fn renew_current_block(&mut self) {
        self.current_block = self.blocks_queue.pop_front().unwrap();
        self.blocks_queue.push_back(Box::new(spawn_block(
            self.spawn_position.0,
            self.spawn_position.1,
        )));
    }
}
