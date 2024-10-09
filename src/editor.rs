mod term;
mod events;

use term as ed_terminal;
use events as ed_events;

use std::io::Result;

pub struct Editor {
    running: bool,
    term: ed_terminal::Term,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            running: true,
            term: ed_terminal::Term::new(),
        }
    }

    pub fn run(&mut self) -> Result<()> {
        self.term.init()?;

        /* main loop */
        while self.running {
            ed_events::next(self)?;
        }

        self.term.exit()?;
        Ok(())
    }

    fn terminate(&mut self) {
        self.running = false;
    }
}