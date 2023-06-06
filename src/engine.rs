use anyhow::{anyhow, Result};
use sdl2::{event::Event, keyboard::Keycode, EventPump, render::WindowCanvas, pixels::Color};
use std::time::Duration;

use crate::{
    tetris::{
        block::Position,
        grid::{Collision, LockResult},
        tetris::Tetris,
    },
    time::Time, renderable::Renderable,
};

pub struct Engine {
    pub canvas: WindowCanvas,
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

        let tetris = Tetris::new(Position(11, 0), Position(8, 0));

        let engine_time = Time::new(Duration::from_secs(1));

        let running = true;

        Ok(Self {
            canvas,
            event_pump,
            engine_time,
            running,
            tetris,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        match self.canvas.set_scale(1.5, 1.5) {
            Ok(it) => it,
            Err(err) => return Err(anyhow!(err)),
        };

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
        let touching_locked_blocks_dir = self
            .tetris
            .grid
            .is_touching_locked_blocks(&self.tetris.current_block);

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
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    if !touching_locked_blocks_dir.0 {
                        self.tetris.current_block.move_left();
                    }
                }

                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    if !touching_locked_blocks_dir.1 {
                        self.tetris.current_block.move_right();
                    }
                }

                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    self.tetris.current_block.soft_drop();
                }

                Event::KeyDown {
                    keycode: Some(Keycode::Z),
                    ..
                } => {
                    self.tetris.current_block.rotate_clockwise();
                }

                Event::KeyDown {
                    keycode: Some(Keycode::X),
                    ..
                } => {
                    self.tetris.current_block.rotate_counter_clockwise();
                }

                _ => {}
            }
        }
    }

    fn update(&mut self) {
        match self.tetris.grid.is_colliding(&self.tetris.current_block) {
            Collision::Left => self.tetris.current_block.move_right(),
            Collision::Right => self.tetris.current_block.move_left(),
            Collision::Bottom => {
                self.tetris.current_block.move_up();
                match self.tetris.grid.lock_block(&self.tetris.current_block) {
                    LockResult::GameOver => self.running = false,
                    LockResult::Locked => self.tetris.renew_current_block(),
                }
            }
            _ => {}
        }

        self.tetris.grid.clear_full_rows();
    }

    fn fixed_update(&mut self) {
        while self.engine_time.is_time_step_passed() {
            self.tetris.current_block.soft_drop();
            self.engine_time.update_accumulator();
        }
    }

    fn render(&mut self) {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();

        self.tetris.current_block.render(&mut self.canvas);
        self.tetris.grid.render(&mut self.canvas);

        self.canvas.present();
    }
}
