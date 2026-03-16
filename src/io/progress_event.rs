use std::fmt::Display;
use std::time::Duration;

use crossterm::style::{style, Color, Stylize};
use indicatif::{ProgressBar, ProgressStyle};

use crate::error::Result;

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

impl ProgressRenderer {
    pub fn new() -> Self {
        let bar = ProgressBar::new(100);
        bar.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}% {msg}")
                .unwrap()
                .progress_chars("#>-"),
        );
        Self { bar }
    }

    pub fn render<E: ProgressEvent>(&mut self, event: &E) -> Result<()> {
        let state = event.state();

        let elapsed_str = format!(
            "[{:02}:{:02}]",
            state.elapsed.as_secs() / 60,
            state.elapsed.as_secs() % 60
        );
        let elapsed_colored = style(elapsed_str).with(Color::Cyan);

        let objects_str = if let Some((cur, total)) = state.objects_handled {
            style(format!("[{}/{}]", cur, total))
                .with(Color::Yellow)
                .to_string()
        } else {
            String::new()
        };

        let message = format!("{} {} {}", elapsed_colored, objects_str, event);
        let pos = (state.progress * 100.0) as u64;
        self.bar.set_position(pos);

        if state.is_final {
            self.bar.finish_with_message(message);
        } else {
            self.bar.set_message(message);
        }

        Ok(())
    }
}
