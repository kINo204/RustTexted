use std::io::stdout;
use std::io::Stdout;
use std::io::Result;

use crossterm::cursor::MoveDown;
use crossterm::cursor::MoveTo;
use crossterm::cursor::MoveToColumn;
use crossterm::cursor::EnableBlinking;
use crossterm::cursor::{Hide, Show};
use crossterm::execute;
use crossterm::queue;
use crossterm::style::Print;
use crossterm::terminal::Clear;
use crossterm::terminal::ClearType::*;
use crossterm::terminal;

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

        queue!(self.o,
            Hide,
            MoveTo(0, 0),
        )?;

        let (_, nrows) = terminal::size()?;
        for _ in 0 .. nrows {
            queue!(self.o,
                Clear(CurrentLine),
                MoveToColumn(0),
                Print("~ "),
                MoveDown(1),
            )?;
        }

        queue!(self.o,
            MoveTo(2, 0),
            EnableBlinking,
            Show
        )?;

        execute!(self.o)?;
        Ok(())
    }

    pub fn exit(&mut self) -> Result<()> {
        execute!(self.o, Clear(All))?;
        terminal::disable_raw_mode()?;
        Ok(())
    }
}