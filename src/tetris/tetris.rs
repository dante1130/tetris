use std::collections::VecDeque;

use super::{block::Block, blocks::spawn_block};

pub struct Tetris {
    pub board: [[i32; 10]; 22],
    pub current_block: Box<Block>,
    pub hold_block: Option<Box<Block>>,
    pub blocks_queue: VecDeque<Box<Block>>,
}

impl Tetris {
    pub fn new() -> Tetris {
        let mut blocks_queue = VecDeque::new();
        for _ in 0..5 {
            blocks_queue.push_back(spawn_block());
        }

        Tetris {
            board: [[0; 10]; 22],
            current_block: blocks_queue.pop_front().unwrap(),
            hold_block: None,
            blocks_queue,
        }
    }
}
