use std::time::{Instant, Duration};

pub struct Time {
    pub now: Instant,
    pub prev: Instant,
    pub delta: Duration,
    pub accumulator: Duration,
    pub time_step: Duration,
}

impl Time {
    pub fn new(time_step: Duration) -> Self {
        Self {
            now: Instant::now(),
            prev: Instant::now(),
            delta: Duration::new(0, 0), 
            accumulator: Duration::new(0, 0),
            time_step,
        }
    }

    pub fn tick(&mut self) {
        self.prev = self.now;
        self.now = Instant::now();
        self.delta = self.now - self.prev;
        self.accumulator += self.delta;
    }

    pub fn should_update(&self) -> bool {
        self.accumulator >= self.time_step
    }

    pub fn update(&mut self) {
        self.accumulator -= self.time_step;
    }
}



