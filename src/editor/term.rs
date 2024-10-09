use std::io::stdout;
use std::io::Write;
use std::io::Stdout;
use std::io::Result;

use crossterm::cursor::MoveDown;
use crossterm::cursor::MoveTo;
use crossterm::cursor::MoveToColumn;
use crossterm::cursor::SetCursorStyle;
use crossterm::ExecutableCommand;
use crossterm::QueueableCommand;
use crossterm::terminal::{self, Clear, ClearType::All};

pub struct Term {
    o: Stdout,
}

impl Term {
    pub fn new() -> Self {
        Self {
            o: stdout(),
        }
    }

    pub fn init(&mut self) -> Result<()> {
        terminal::enable_raw_mode()?; // Now we're a VT-100 user!
        self.o
            .queue(Clear(All))?
            .queue(SetCursorStyle::BlinkingBlock)?
            .queue(MoveTo(0, 0))?;
        let (_, nrows) = terminal::size()?;
        write_row(self, "~ ")?;
        for _ in 1 .. nrows {
            self.o.execute(MoveDown(1))?;
            write_row(self, "~ ")?;
        }
        self.o.flush()?;
        Ok(())
    }

    pub fn exit(&mut self) -> Result<()> {
        terminal::disable_raw_mode()?;
        Ok(())
    }
}

// Drawing tools:
fn write_row(t: &mut Term, content: &str) -> Result<()> {
    let o = &mut t.o;
    o.execute(MoveToColumn(0))?;
    print!("{content}");
    Ok(())
}