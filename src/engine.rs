use anyhow::{anyhow, Result};
use sdl2::{event::Event, keyboard::Keycode, pixels::Color, EventPump};
use std::time::Duration;

use crate::{renderer::Renderer, tetris::{tetris::Tetris, block::Position}, time::Time};

pub struct Engine {
    pub renderer: Renderer,
    event_pump: EventPump,
    engine_time: Time,
    running: bool,

    tetris: Tetris,
}

impl Engine {
    pub fn new() -> Result<Self> {
        let sdl_context = match sdl2::init() {
            Ok(sdl) => sdl,
            Err(err) => return Err(anyhow!(err)),
        };
        let video_subsystem = match sdl_context.video() {
            Ok(video) => video,
            Err(err) => return Err(anyhow!(err)),
        };
        let window = video_subsystem
            .window("Tetris", 800, 600)
            .position_centered()
            .build()?;
        let canvas = window.into_canvas().build()?;
        let event_pump = match sdl_context.event_pump() {
            Ok(event) => event,
            Err(err) => return Err(anyhow!(err)),
        };

        let renderer = Renderer::new(canvas, Color::RGB(0, 0, 0));

        let tetris = Tetris::new(Position(11, 0), Position(8, 0));

        let engine_time = Time::new(Duration::from_secs(1));

        let running = true;

        Ok(Self {
            renderer,
            event_pump,
            engine_time,
            running,
            tetris,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        self.renderer.set_scale(1.5);

        while self.running {
            self.engine_time.tick();
            self.handle_events();
            self.update();
            self.fixed_update();
            self.render();
        }

        Ok(())
    }

    fn handle_events(&mut self) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    self.running = false;
                }

                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    self.tetris.current_block.move_left();
                }

                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    self.tetris.current_block.move_right();
                }

                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    self.tetris.current_block.fall();
                }
                _ => {}
            }
        }
    }

    fn update(&mut self) {

    }

    fn fixed_update(&mut self) {
        while self.engine_time.should_update() {
            self.tetris.current_block.fall();
            self.engine_time.update();
        }
    }

    fn render(&mut self) {
        self.renderer.clear_renderables();

        self.renderer.add_renderable(self.tetris.current_block.clone());
        self.renderer.add_renderable(self.tetris.grid.clone());

        self.renderer.render();
    }
}
