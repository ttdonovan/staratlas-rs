use std::time::Duration;

#[derive(Debug, Default)]
pub struct Stopwatch {
    elapsed: Duration,
    paused: bool,
}

impl Stopwatch {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn elapsed(&self) -> Duration {
        self.elapsed
    }

    pub fn elapsed_secs(&self) -> f32 {
        self.elapsed().as_secs_f32()
    }

    pub fn elapsed_secs_f64(&self) -> f64 {
        self.elapsed().as_secs_f64()
    }

    pub fn set_elapsed(&mut self, time: Duration) {
        self.elapsed = time;
    }

    pub fn tick(&mut self, delta: Duration) {
        if !self.paused() {
            self.elapsed += delta;
        }
    }

    pub fn pause(&mut self) {
        self.paused = true;
    }

    pub fn unpause(&mut self) {
        self.paused = false;
    }

    pub fn paused(&self) -> bool {
        self.paused
    }

    pub fn reset(&mut self) {
        self.elapsed = Default::default();
    }
}

#[derive(Debug, Default)]
pub struct Timer {
    stopwatch: Stopwatch,
    duration: Duration,
    finished: bool,
}

impl Timer {
    pub fn new(duration: Duration) -> Self {
        Timer {
            duration,
            ..Default::default()
        }
    }

    pub fn from_seconds(duration: f32) -> Self {
        Timer {
            duration: Duration::from_secs_f32(duration),
            ..Default::default()
        }
    }

    pub fn finished(&self) -> bool {
        self.finished
    }

    pub fn elapsed(&self) -> Duration {
        self.stopwatch.elapsed()
    }

    pub fn set_elapsed(&mut self, elapsed: Duration) {
        self.stopwatch.set_elapsed(elapsed);
    }

    pub fn elapsed_secs(&self) -> f32 {
        self.stopwatch.elapsed_secs()
    }

    pub fn duration(&self) -> Duration {
        self.duration
    }

    pub fn set_duration(&mut self, duration: Duration) {
        self.duration = duration;
    }

    pub fn tick(&mut self, delta: Duration) -> &Self {
        if self.paused() || self.finished() {
            return self;
        }

        self.stopwatch.tick(delta);
        self.finished = self.elapsed() >= self.duration();

        if self.finished() {
            self.set_elapsed(self.duration());
        }

        return self;
    }

    pub fn pause(&mut self) {
        self.stopwatch.pause();
    }

    pub fn unpause(&mut self) {
        self.stopwatch.unpause();
    }

    pub fn paused(&self) -> bool {
        self.stopwatch.paused()
    }

    pub fn reset(&mut self) {
        self.stopwatch.reset();
        self.finished = false;
    }

    pub fn fraction(&self) -> f32 {
        if self.duration == Duration::ZERO {
            1.0
        } else {
            self.elapsed_secs() / self.duration().as_secs_f32()
        }
    }

    pub fn fraction_remaining(&self) -> f32 {
        1.0 - self.fraction()
    }

    pub fn remaining_secs(&self) -> f32 {
        (self.duration() - self.elapsed()).as_secs_f32()
    }
}

pub fn get_time() -> f64 {
    use std::time::SystemTime;

    let time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_else(|e| panic!("{}", e));

    time.as_secs_f64()
}
