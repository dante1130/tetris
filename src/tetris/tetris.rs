use std::collections::VecDeque;

use super::{block::Block, blocks::spawn_block, grid::Grid};

pub struct Tetris {
    pub grid: Box<Grid>,
    pub current_block: Box<Block>,
    pub hold_block: Option<Box<Block>>,
    pub blocks_queue: VecDeque<Box<Block>>,
}

impl Tetris {
    pub fn new() -> Tetris {
        let mut blocks_queue = VecDeque::new();
        for _ in 0..5 {
            blocks_queue.push_back(Box::new(spawn_block()));
        }

        Tetris {
            grid: Box::new(Grid::new(10, 20)),
            current_block: blocks_queue.pop_front().unwrap(),
            hold_block: None,
            blocks_queue,
        }
    }
}
