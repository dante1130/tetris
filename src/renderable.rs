use sdl2::render::WindowCanvas;

pub trait Renderable {
    fn render(&self, canvas: &mut WindowCanvas);
}
