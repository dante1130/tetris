use std::collections::VecDeque;

use super::{
    block::{Block, Position},
    blocks::spawn_block,
    grid::Grid,
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
}
