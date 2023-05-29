use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

pub struct Renderer<T: Renderable> {
    canvas: WindowCanvas,
    background: Color,
    renderables: Vec<T>,
}

pub trait Renderable {
    fn render(&self, canvas: &mut WindowCanvas);
}

impl<T> Renderer<T>
where
    T: Renderable,
{
    pub fn new(canvas: WindowCanvas, background: Color) -> Self {
        Self {
            canvas,
            background,
            renderables: Vec::new(),
        }
    }

    pub fn set_background(&mut self, color: Color) {
        self.background = color;
    }

    pub fn add_renderable(&mut self, renderable: T) {
        self.renderables.push(renderable);
    }

    pub fn clear_renderables(&mut self) {
        self.renderables.clear();
    }

    pub fn render(&mut self) {
        self.canvas.set_draw_color(self.background);
        self.canvas.clear();

        self.renderables.iter().for_each(|renderable| {
            renderable.render(&mut self.canvas);
        });

        self.canvas.present();
    }
}
