use std::fs::File;
use std::io::prelude::*;
use std::io::{stdin, stdout, Result, SeekFrom, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

trait Hex {
    fn down(&mut self);
    fn up(&mut self);
    fn read_hex(&mut self, file: &mut File) -> Result<()>;
}

struct HexReader {
    index: u64,
    file_length: u64,
    buffer: Vec<u8>,
}

impl Hex for HexReader {
    fn down(&mut self) {
        if self.index < self.file_length - 16 {
            self.index += 16;
        }
    }

    fn up(&mut self) {
        if self.index > 16 {
            self.index -= 16;
        }
    }

    fn read_hex(&mut self, file: &mut File) -> Result<()> {
        self.buffer = Vec::new();
        file.seek(SeekFrom::Start(self.index))?;
        file.take(16).read_to_end(&mut self.buffer)?;
        println!("{:X?}", self.buffer);
        Ok(())
    }
}

fn main() -> Result<()> {
    let mut file = File::open("file.png")?;

    let mut hex_reader = HexReader {
        index: 0,
        file_length: file.metadata().unwrap().len(),
        buffer: Vec::new(),
    };

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    stdout.flush().unwrap();

    for c in stdin.keys() {
        write!(
            stdout,
            "{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::All
        )
        .unwrap();

        match c.unwrap() {
            Key::Char('j') => {
                hex_reader.down();
                hex_reader.read_hex(&mut file)?;
            }
            Key::Char('k') => {
                hex_reader.up();
                hex_reader.read_hex(&mut file)?;
            }
            Key::Char('q') => break,
            _ => continue,
        }
    }

    Ok(())
}
