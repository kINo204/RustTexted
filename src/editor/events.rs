use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use std::io::Result;

use super::Editor;

pub fn next(ed: &mut Editor) -> Result<()> {
    let event = event::read()?;
    match event {
        Event::Key(key_event) => process_key_event(ed, key_event),
        _ => Ok(())
    }
}

fn process_key_event(ed: &mut Editor, key_event: KeyEvent) -> Result<()> {
    let KeyEvent {
        code,
        modifiers,
        kind,
        ..
    } = key_event;

    // Process key events on pressed down for now.
    match kind != KeyEventKind::Press {
        true => return Ok(()),
        false => (),
    }

    match code {
        // plain charactors
        KeyCode::Char(ch) => {
            if ch == 'q' && modifiers == KeyModifiers::CONTROL {
                ed.terminate();
                return Ok(());
            }
            let bi = ch as u8;
            println!("Binary: {bi:08b} ASCII: {bi:#03} Character: {ch:#?}\r");
            Ok(())
        }
        _ => Ok(())
    }
}