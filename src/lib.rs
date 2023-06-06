mod engine;
mod renderable;
mod tetris;
mod time;

use anyhow::Result;

pub fn run() -> Result<()> {
    engine::Engine::new()?.run()
}
