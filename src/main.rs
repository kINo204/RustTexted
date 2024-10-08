use std::io::{stdin, Read, Result};
use crossterm::terminal::enable_raw_mode;

fn main() -> Result<()> {
    enable_raw_mode().expect("Crossterm: failed to init raw mode!");

    let reader = stdin();

    for b in reader.bytes() {
        let b = b.expect("Read: failed to read byte!");
        let c = b as char;

        if c.is_control() {
            println!("Binary: {0:08b} ASCII: {0:#03}", b);
        } else {
            println!("Binary: {0:08b} ASCII: {0:#03} Character: {1:#?}", b, c);
        }

        if c == 'q' {
            enable_raw_mode().expect("Crossterm: failed to exit raw mode!");
            break;
        }
    };

    Ok(())
}
