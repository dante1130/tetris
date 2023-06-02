mod engine;
mod renderer;
mod tetris;
mod time;

use anyhow::Result;

pub fn run() -> Result<()> {
    engine::Engine::new()?.run()
}
