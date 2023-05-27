use super::block::Rotation;
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

pub fn spawn_block() -> Block {
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
        BlockType::I => new_i_block(),
        BlockType::J => new_j_block(),
        BlockType::L => new_l_block(),
        BlockType::O => new_o_block(),
        BlockType::S => new_s_block(),
        BlockType::T => new_t_block(),
        BlockType::Z => new_z_block(),
    }
}

fn new_i_block() -> Block {
    Block {
        x: 4,
        y: 0,
        color: Color::RGB(0, 255, 255),
        shape: vec![
            vec![false, false, false, false],
            vec![true, true, true, true],
            vec![false, false, false, false],
            vec![false, false, false, false],
        ],
        rotation: Rotation::Deg0,
    }
}

fn new_j_block() -> Block {
    Block {
        x: 4,
        y: 0,
        color: Color::RGB(0, 0, 255),
        shape: vec![
            vec![true, false, false],
            vec![true, true, true],
            vec![false, false, false],
        ],
        rotation: Rotation::Deg0,
    }
}

fn new_l_block() -> Block {
    Block {
        x: 4,
        y: 0,
        color: Color::RGB(255, 165, 0),
        shape: vec![
            vec![false, false, true],
            vec![true, true, true],
            vec![false, false, false],
        ],
        rotation: Rotation::Deg0,
    }
}

fn new_o_block() -> Block {
    Block {
        x: 4,
        y: 0,
        color: Color::RGB(255, 255, 0),
        shape: vec![vec![true, true], vec![true, true]],
        rotation: Rotation::Deg0,
    }
}

fn new_s_block() -> Block {
    Block {
        x: 4,
        y: 0,
        color: Color::RGB(0, 255, 0),
        shape: vec![
            vec![false, true, true],
            vec![true, true, false],
            vec![false, false, false],
        ],
        rotation: Rotation::Deg0,
    }
}

fn new_t_block() -> Block {
    Block {
        x: 4,
        y: 0,
        color: Color::RGB(128, 0, 128),
        shape: vec![
            vec![false, true, false],
            vec![true, true, true],
            vec![false, false, false],
        ],
        rotation: Rotation::Deg0,
    }
}

fn new_z_block() -> Block {
    Block {
        x: 4,
        y: 0,
        color: Color::RGB(255, 0, 0),
        shape: vec![
            vec![true, true, false],
            vec![false, true, true],
            vec![false, false, false],
        ],
        rotation: Rotation::Deg0,
    }
}
