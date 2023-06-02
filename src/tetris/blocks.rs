use super::block::{Position, Shape};
use crate::tetris::block::Block;
use rand::Rng;
use sdl2::pixels::Color;

pub enum BlockType {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

pub fn spawn_block(x: i32, y: i32) -> Block {
    let mut rng = rand::thread_rng();
    let block_types: [BlockType; 7] = [
        BlockType::I,
        BlockType::J,
        BlockType::L,
        BlockType::O,
        BlockType::S,
        BlockType::T,
        BlockType::Z,
    ];

    match block_types[rng.gen_range(0..7)] {
        BlockType::I => new_i_block(x, y),
        BlockType::J => new_j_block(x, y),
        BlockType::L => new_l_block(x, y),
        BlockType::O => new_o_block(x, y),
        BlockType::S => new_s_block(x, y),
        BlockType::T => new_t_block(x, y),
        BlockType::Z => new_z_block(x, y),
    }
}

fn new_i_block(x: i32, y: i32) -> Block {
    Block {
        shape: Shape {
            positions: vec![
                Position(0 + x, 0 + y),
                Position(1 + x, 0 + y),
                Position(2 + x, 0 + y),
                Position(3 + x, 0 + y),
            ],
            anchor: Position(1, 0),
        },
        color: Color::RGB(0, 255, 255),
    }
}

fn new_j_block(x: i32, y: i32) -> Block {
    Block {
        shape: Shape {
            positions: vec![
                Position(0 + x, 0 + y),
                Position(0 + x, 1 + y),
                Position(1 + x, 1 + y),
                Position(2 + x, 1 + y),
            ],
            anchor: Position(1, 1),
        },
        color: Color::RGB(0, 0, 255),
    }
}

fn new_l_block(x: i32, y: i32) -> Block {
    Block {
        shape: Shape {
            positions: vec![
                Position(0 + x, 1 + y),
                Position(1 + x, 1 + y),
                Position(2 + x, 1 + y),
                Position(2 + x, 0 + y),
            ],
            anchor: Position(1, 1),
        },
        color: Color::RGB(255, 165, 0),
    }
}

fn new_o_block(x: i32, y: i32) -> Block {
    Block {
        shape: Shape {
            positions: vec![
                Position(0 + x, 0 + y),
                Position(1 + x, 0 + y),
                Position(0 + x, 1 + y),
                Position(1 + x, 1 + y),
            ],
            anchor: Position(0, 0),
        },
        color: Color::RGB(255, 255, 0),
    }
}

fn new_s_block(x: i32, y: i32) -> Block {
    Block {
        shape: Shape {
            positions: vec![
                Position(0 + x, 1 + y),
                Position(1 + x, 1 + y),
                Position(1 + x, 0 + y),
                Position(2 + x, 0 + y),
            ],
            anchor: Position(1, 1),
        },
        color: Color::RGB(0, 255, 0),
    }
}

fn new_t_block(x: i32, y: i32) -> Block {
    Block {
        shape: Shape {
            positions: vec![
                Position(0 + x, 1 + y),
                Position(1 + x, 1 + y),
                Position(2 + x, 1 + y),
                Position(1 + x, 0 + y),
            ],
            anchor: Position(1, 1),
        },
        color: Color::RGB(128, 0, 128),
    }
}

fn new_z_block(x: i32, y: i32) -> Block {
    Block {
        shape: Shape {
            positions: vec![
                Position(0 + x, 0 + y),
                Position(1 + x, 0 + y),
                Position(1 + x, 1 + y),
                Position(2 + x, 1 + y),
            ],
            anchor: Position(1, 1),
        },
        color: Color::RGB(255, 0, 0),
    }
}

