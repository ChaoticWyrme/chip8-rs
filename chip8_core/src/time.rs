use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct Timers {
    pub delay: usize,
    pub sound: usize,
    prev_tick: Instant,
    remainder: Duration,
    rate: usize,
}

impl Default for Timers {
    fn default() -> Self {
        Timers {
            delay: 0,
            sound: 0,
            prev_tick: Instant::now(),
            remainder: Duration::default(),
            rate: 60,
        }
    }
}

impl Timers {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_sound_on(&self) -> bool {
        self.sound > 0
    }

    /// Do n ticks where n = millis * 60
    pub fn do_ticks(&mut self) {
        // println!("Previous tick: {:?}", self.prev_tick);
        let now = Instant::now();
        // println!("Now: {:?}", now);
        let diff = now.duration_since(self.prev_tick) + self.remainder;
        // println!("Difference: {:?}", diff);
        // interpolate
        let tick_count = ((diff.as_millis() as f32) / 1000.0) * self.rate as f32;
        self.remainder = Duration::from_millis((tick_count.fract() * 1000.0) as u64);
        let tick_count = tick_count.trunc() as u32;
        // println!("Tick Count: {}", tick_count);
        for _ in 0..tick_count {
            self.single_tick();

            // short circuit if both timers are 0
            // since then we don't need to tick really
            if 0 == self.sound && self.delay == 0 {
                break;
            }
        }

        self.prev_tick = now;
    }

    fn single_tick(&mut self) {
        if self.delay > 0 {
            self.delay -= 1
        }
        if self.sound > 0 {
            self.sound -= 1;
        }
    }
}
