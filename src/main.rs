use std::io;
use std::io::SeekFrom;
use std::io::prelude::*;
use std::fs::File;


trait Hex {
    fn down(&mut self);
    fn up(&mut self);
    fn read_hex(&mut self, file: File);
}

struct HexReader {
    index: u64,
    buffer: [u8; 160]
}

impl Hex for HexReader {
    fn down(&mut self) {
        self.index += 16;
    }

    fn up(&mut self) {
        self.index -= 16;
    }

    fn read_hex(&mut self, file: File) {
        file.seek(SeekFrom::Start(self.index));
        file.read(&mut self.buffer);

        println!("{:X?}", self.buffer[..100]);
    }
}

fn main() -> io::Result<()> {
    let mut file = File::open("file.txt")?;


    Ok(())
}
