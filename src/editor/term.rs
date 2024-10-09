use std::io::stdout;
use std::io::Stdout;
use std::io::Result;

use crossterm::{terminal::{self, Clear, ClearType::All}, ExecutableCommand};

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
        self.o.execute(Clear(All))?;
        Ok(())
    }

    pub fn exit(&mut self) -> Result<()> {
        terminal::disable_raw_mode()?;
        Ok(())
    }
}