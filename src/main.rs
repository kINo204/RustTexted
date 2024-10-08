use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::event::KeyEventKind;
use crossterm::terminal;
use crossterm::event;

fn main() {
    terminal::enable_raw_mode().unwrap();
    loop {
        let e = event::read().unwrap();
        if let Event::Key(key_event) = e {
            if key_event.kind == KeyEventKind::Press {
                if let KeyCode::Char(c) = key_event.code {
                    let b = c as u8;
                    println!("Binary: {0:08b} ASCII: {0:#03} Character: {1:#?}\r", b, c);
                    if c == 'q' {
                        break;
                    }
                }
            }
        }
    }
    terminal::disable_raw_mode().unwrap();
}