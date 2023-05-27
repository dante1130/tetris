use crate::renderer::Renderable;
use sdl2::pixels::Color;

pub struct Scene {
    pub background: Color,
    pub renderables: Vec<Box<dyn Renderable>>,
}
