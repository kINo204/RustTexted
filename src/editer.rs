use crossterm::{event::{self, Event::*, KeyCode::*, KeyEvent, KeyEventKind}, terminal};
use std::io::Result;

pub struct Editor {
    running: bool,
}

impl Default for Editor {
    fn default() -> Self {
        Self { running: true }
    }
}

impl Editor {
    pub fn run(&mut self) {
        terminal::enable_raw_mode().expect("Crossterm: terminal enter_raw_mode failed!");

        /* main loop */
        while self.running {
            self.poll_events().expect("Error occured in poll_events.");
        }
        
        terminal::disable_raw_mode().expect("Crossterm: terminal disable_raw_mode failed!");
    }

    fn terminate(&mut self) {
        self.running = false;
    }

    fn poll_events(&mut self) -> Result<()> {
        let event = event::read()?;
        match event {
            Key(key_event) => self.process_key_events(key_event),
            _ => Ok(())
        }
    }

    fn process_key_events(&mut self, key_event: KeyEvent) -> Result<()> {
        // Process key events on pressed down for now.
        match key_event.kind != KeyEventKind::Press {
            true => return Ok(()),
            false => (),
        }

        let key_code = key_event.code;
        match key_code {

            // plain charactors
            Char(ch) => {
                let bi = ch as u8;
                println!("Binary: {bi:08b} ASCII: {bi:#03} Character: {ch:#?}\r");
                if ch == 'q' {
                    self.terminate();
                }
                Ok(())
            }

            // modifiers
            Modifier(mkey_code) => {
                match mkey_code {
                    _ => Ok(())
                }
            }
            
            _ => Ok(())
        }
    }
}
