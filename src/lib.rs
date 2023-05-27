mod engine;
mod renderer;
mod scene;

use anyhow::Result;

pub fn run() -> Result<()> {
    engine::Engine::new()?.run()
}
