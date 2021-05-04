use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::SeekFrom;

trait Hex {
    fn down(&mut self);
    fn up(&mut self);
    fn read_hex(&mut self, file: &mut File) -> io::Result<()>;
}

struct HexReader {
    index: u64,
    file_length: u64,
    buffer: Vec::<u8>,
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

    fn read_hex(&mut self, file: &mut File) -> io::Result<()> {
        file.seek(SeekFrom::Start(self.index))?;
        file.take(80).read_to_end(&mut self.buffer)?;
        println!("{:X?}", self.buffer);
        Ok(())
    }
}

fn main() -> io::Result<()> {
    let mut file = File::open("file.txt")?;

    let mut hex_reader = HexReader {
        index: 0,
        file_length: file.metadata().unwrap().len(),
        buffer: Vec::new(),
    };

    hex_reader.read_hex(&mut file)?;
    hex_reader.up();
    hex_reader.read_hex(&mut file)?;
    Ok(())
}
