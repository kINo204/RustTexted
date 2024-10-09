use crossterm::event::{self, Event::*, KeyCode::*, KeyEvent, KeyEventKind, KeyModifiers};
use term::Term;
use std::io::Result;

pub struct Editor {
    running: bool,
    term: term::Term,
}

mod term;

impl Editor {
    pub fn new() -> Self {
        Self {
            running: true,
            term: Term::new(),
        }
    }

    pub fn run(&mut self) -> Result<()> {
        self.term.init()?;

        /* main loop */
        while self.running {
            self.process_event()?;
        }

        self.term.exit()?;
        Ok(())
    }

    fn terminate(&mut self) {
        self.running = false;
    }

    fn process_event(&mut self) -> Result<()> {
        let event = event::read()?;
        match event {
            Key(key_event) => self.process_key_event(key_event),
            _ => Ok(())
        }
    }

    fn process_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        let KeyEvent { code, modifiers, kind, state } = key_event;

        // Process key events on pressed down for now.
        match kind != KeyEventKind::Press {
            true => return Ok(()),
            false => (),
        }

        match code {
            // plain charactors
            Char(ch) => {
                if ch == 'q' && modifiers == KeyModifiers::CONTROL {
                    self.terminate();
                    return Ok(());
                }
                let bi = ch as u8;
                println!("Binary: {bi:08b} ASCII: {bi:#03} Character: {ch:#?}\r");
                Ok(())
            }
            _ => Ok(())
        }
    }
}
