use std::fmt::Display;

use crate::error::Result;
use crate::io::on_same_console_line;

pub trait ProgressEvent: Display {
    fn is_final_state(&self) -> bool;

    fn print_to_stdout(&self) -> Result<()> {
        on_same_console_line(|| {
            if self.is_final_state() {
                println!("{}", self)
            } else {
                print!("{}", self)
            }
        })
    }
}
