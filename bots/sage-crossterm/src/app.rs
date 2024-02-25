use color_eyre::Result;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};

use std::time::{Duration, Instant};

use crate::{term, time};

pub struct App {
    mode: Mode,
    stopwatch: time::Stopwatch,
    timer: time::Timer,
    last_time: Instant,
}

#[derive(Default, PartialEq)]
enum Mode {
    #[default]
    Running,
    Quit,
}

pub fn run() -> Result<()> {
    App::new().run()
}

impl App {
    pub fn new() -> Self {
        let mut timer = time::Timer::from_seconds(15.0);
        timer.pause();

        App {
            mode: Mode::default(),
            stopwatch: time::Stopwatch::default(),
            timer: timer,
            last_time: Instant::now(),
        }
    }

    pub fn run(&mut self) -> Result<()> {
        let mut stdout = std::io::stdout();

        while self.is_running() {
            term::clear(&mut stdout)?;
            self.update()?;
            self.draw()?;
            self.handle_events()?;
            term::flush(&mut stdout)?;
        }

        Ok(())
    }

    fn is_running(&self) -> bool {
        self.mode != Mode::Quit
    }

    fn handle_events(&mut self) -> Result<()> {
        let timeout = Duration::from_secs_f64(1.0 / 100.0);

        match term::next_event(timeout)? {
            Some(Event::Key(key)) if key.kind == KeyEventKind::Press => self.handle_key_press(key),
            _ => Ok(()),
        }
    }

    fn handle_key_press(&mut self, key: KeyEvent) -> Result<()> {
        use KeyCode::*;

        match key.code {
            Char('q') | Esc => self.quit(),
            Char('r') => self.timer.reset(),
            Char('p') => self.timer.pause(),
            Char('u') => self.timer.unpause(),
            _ => {}
        };

        Ok(())
    }

    fn update(&mut self) -> Result<()> {
        let now = Instant::now();
        let dt = now.duration_since(self.last_time);
        self.last_time = now;

        // dbg!(dt);

        self.stopwatch.tick(dt);
        self.timer.tick(dt);
        Ok(())
    }

    fn draw(&mut self) -> Result<()> {
        println!("Welcome to Sage! Press 'q' to quit.");
        println!("Stopwatch: {:?}", self.stopwatch);
        println!("Elapsed time: {:.2}", self.stopwatch.elapsed_secs_f64());
        println!("Timer: {:?}", self.timer);
        println!("Finished: {:?}", self.timer.finished());
        println!("Fraction: {:.2}", self.timer.fraction());
        println!("Fraction Remaining: {:.2}", self.timer.fraction_remaining());
        Ok(())
    }

    fn quit(&mut self) {
        self.mode = Mode::Quit;
    }
}
