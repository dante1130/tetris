use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

pub struct Renderer {
    canvas: WindowCanvas,
    background: Color,
    renderables: Vec<Box<dyn Renderable>>,
}

pub trait Renderable {
    fn render(&self, canvas: &mut WindowCanvas);
}

impl Renderer {
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

    pub fn add_renderable(&mut self, renderable: Box<dyn Renderable>) {
        self.renderables.push(renderable);
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
