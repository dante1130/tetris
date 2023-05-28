use anyhow::{anyhow, Result};
use sdl2::{event::Event, keyboard::Keycode, pixels::Color, EventPump};
use std::time::Duration;

use crate::{renderer::Renderer, tetris::tetris::Tetris};

pub struct Engine {
    pub renderer: Renderer,
    event_pump: EventPump,
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

        let tetris = Tetris::new();

        let running = true;

        Ok(Self {
            renderer,
            event_pump,
            running,
            tetris,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        while self.running {
            self.handle_events();
            self.update();
            self.renderer.render();

            std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
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
                _ => {}
            }
        }
    }

    fn update(&mut self) {}
}
