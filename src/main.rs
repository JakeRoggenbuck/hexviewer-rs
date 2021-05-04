use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{stdin, stdout, Result, SeekFrom, Stdout, Write};
use std::process;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;

trait Hex {
    fn down(&mut self);
    fn up(&mut self);
    fn read_hex(&mut self, file: &mut File) -> Result<()>;
    fn print(&mut self);
    fn clear(&mut self);
}

struct HexReader {
    index: u64,
    file_length: u64,
    buffer: Vec<u8>,
    stdout: RawTerminal<Stdout>,
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
        file.take(400).read_to_end(&mut self.buffer)?;
        self.print();
        Ok(())
    }

    fn clear(&mut self) {
        write!(
            self.stdout,
            "{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::All
        )
        .unwrap();
    }

    fn print(&mut self) {
        let mut index = 1;
        let mut line = 0;
        for x in &self.buffer {
            let mut printable_hex: String = format!("{:X?}", x);
            if printable_hex.len() == 1 {
                printable_hex = String::from(printable_hex + " ");
            }
            if index % 8 == 0 {
                write!(
                    self.stdout,
                    "{}{} ",
                    printable_hex,
                    termion::cursor::Goto(1, line)
                )
                .unwrap();
                line += 1;
            } else {
                write!(self.stdout, "{} ", printable_hex).unwrap();
            }
            index += 1;
            stdout().flush().unwrap();
        }
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        eprintln!("Error: Please include a file");
        process::exit(0);
    }

    if &args[1] == "--help" || &args[1] == "-h" {
        println!("
        HexViewer -- 0.1
        ----------------

        Usage: hexv [OPTION]... [FILE]...

        -h, --help      Display this page and exit
        -v, --version   Display the version and exit
        ");
        process::exit(0);
    }

    let filename = &args[1];

    let mut file = File::open(filename)?;

    let stdin = stdin();
    let stdout = stdout().into_raw_mode().unwrap();

    let mut hex_reader = HexReader {
        index: 0,
        file_length: file.metadata().unwrap().len(),
        buffer: Vec::new(),
        stdout: stdout,
    };

    hex_reader.stdout.flush().unwrap();
    hex_reader.clear();
    hex_reader.read_hex(&mut file)?;

    for c in stdin.keys() {
        hex_reader.clear();

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
