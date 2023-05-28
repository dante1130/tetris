mod engine;
mod renderer;
mod tetris;

use anyhow::Result;

pub fn run() -> Result<()> {
    engine::Engine::new()?.run()
}
