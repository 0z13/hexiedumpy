use clap::Clap;
use std::fs;
use std::io::prelude::*;
#[derive(Clap, Debug)]
#[clap(name = "Hexiedumpie")]
struct Opts {
    /// Name of the person to greet
    #[clap(short, long)]
    name: String
}




fn main() {
    let opts = Opts::parse();
    const BYTES_PER_LINE:usize = 16;
    let mut f = fs::File::open(&opts.name).expect("Unknown file.");
    let mut buffer = [0; BYTES_PER_LINE];
    let mut pos = 0;

    while let Ok(_) = f.read_exact(&mut buffer) {
        print!("[0x{:08x}] ", pos);
        for b in &buffer {
            match b {
                0x00 => print!(".  "), 
                0xff => print!("## "),
                _    => print!("{:2x} ", b),
            }
        }
        pos += 16;
        println!("")
    }
}


