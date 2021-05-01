use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::SeekFrom;

trait Hex {
    fn down(&mut self);
    fn up(&mut self);
    fn read_hex(&mut self, file: &mut File);
}

struct HexReader {
    index: u64,
    buffer: Vec::<u8>,
}

impl Hex for HexReader {
    fn down(&mut self) {
        self.index += 16;
    }

    fn up(&mut self) {
        self.index -= 16;
    }

    fn read_hex(&mut self, file: &mut File) {
        file.seek(SeekFrom::Start(self.index));
        file.read(&mut self.buffer);

        println!("{:X?}", self.buffer);
    }
}

fn main() -> io::Result<()> {
    let mut file = File::open("file.txt")?;

    let mut hex_reader = HexReader {
        index: 0,
        buffer: Vec::new(),
    };

    hex_reader.read_hex(&mut file);

    Ok(())
}
