mod engine;
mod renderer;
mod scene;
pub mod tetris;

use anyhow::Result;

pub fn run() -> Result<()> {
    engine::Engine::new()?.run()
}
