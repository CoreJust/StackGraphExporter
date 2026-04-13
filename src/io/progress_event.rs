use std::fmt::Display;
use std::time::Duration;

use crossterm::style::{style, Color, Stylize};
use indicatif::{ProgressBar, ProgressStyle};

use crate::error::Result;

pub struct Elapsed {
    pub elapsed: Duration,
}

pub struct ElapsedAndCount {
    pub current: usize,
    pub total: usize,
    pub elapsed: Duration,
}

#[derive(Debug, Clone, Copy)]
pub struct ProgressState {
    pub is_final: bool,
    pub elapsed: Duration,
    pub progress: f32,
    pub objects_handled: Option<(usize, usize)>,
}

pub trait ProgressEvent: Display {
    fn state(&self) -> ProgressState;
}

pub struct ProgressRenderer {
    bar: ProgressBar,
}

impl ProgressState {
    pub fn from_elapsed(elapsed: &Elapsed, is_final: bool) -> Self {
        Self {
            is_final,
            elapsed: elapsed.elapsed,
            progress: if is_final { 1.0 } else { 0.0 },
            objects_handled: None,
        }
    }

    pub fn from_elapsed_and_count(elapsed_and_count: &ElapsedAndCount, is_final: bool) -> Self {
        Self {
            is_final,
            elapsed: elapsed_and_count.elapsed,
            progress: if elapsed_and_count.total != 0 {
                elapsed_and_count.current as f32 / elapsed_and_count.total as f32
            } else {
                0.0
            },
            objects_handled: Some((elapsed_and_count.current, elapsed_and_count.total)),
        }
    }
}

impl ProgressRenderer {
    fn make_progress_bar() -> ProgressBar {
        let bar = ProgressBar::new(100);
        bar.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}% {msg}")
                .unwrap()
                .progress_chars("#>-"),
        );
        bar
    }

    pub fn new() -> Self {
        Self {
            bar: Self::make_progress_bar(),
        }
    }

    pub fn render<E: ProgressEvent>(&mut self, event: &E) -> Result<()> {
        let state = event.state();

        let elapsed_str = format!("[{}ms]", state.elapsed.as_millis());
        let elapsed_colored = style(elapsed_str).with(Color::Cyan);

        let objects_str = if let Some((cur, total)) = state.objects_handled {
            style(format!(" [{}/{}]", cur, total))
                .with(Color::Yellow)
                .to_string()
        } else {
            String::new()
        };

        let message = format!("{}{} {}", elapsed_colored, objects_str, event);
        let pos = (state.progress * 100.0) as u64;
        self.bar.set_position(pos);

        if state.is_final {
            self.bar.finish_with_message(message);
            self.bar = Self::make_progress_bar();
        } else {
            self.bar.set_message(message);
        }

        Ok(())
    }
}
