use crossterm::{
    cursor::MoveToColumn,
    execute,
    terminal::{Clear, ClearType},
};
use std::io::{stdout, Write};

use crate::error::{Error, Result};

pub fn on_same_console_line<F>(mut action: F) -> Result<()>
where
    F: FnMut(),
{
    let mut stdout = stdout();
    execute!(stdout, MoveToColumn(0), Clear(ClearType::CurrentLine))
        .map_err(|e| Error::Internal(e.to_string()))?;
    action();
    stdout.flush().map_err(|e| Error::Internal(e.to_string()))?;
    Ok(())
}
