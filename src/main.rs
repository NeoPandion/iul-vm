mod class;
mod opt;
use opt::Opt;
use std::{
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
};
use structopt::StructOpt;

fn main() {
    let args = Opt::from_args();
    // println!("{:?}", args);

    let buffer = read_file(args.input);
    let mut c = 0;
    for i in buffer.iter() {
        print!("{:02X}", i);
        print!(" ");
        c += 1;
        if c % 16 == 0 {
            print!("\n");
            continue;
        }
    }
    // println!("\nEOF");
    // cafe babe
    if !(validate_file(&buffer)) {
        panic!("Not .class file")
    }

    println!("\n{:?}", (&buffer[0..4]));
    version(&buffer);
}

fn read_file(path: PathBuf) -> Vec<u8> {
    let f = match File::open(path) {
        Ok(a) => a,
        Err(e) => panic!("{e}"),
    };
    let mut reader = BufReader::new(f);
    let mut buffer = Vec::new();
    let _ = reader.read_to_end(&mut buffer);
    buffer
}
fn validate_file(buf: &Vec<u8>) -> bool {
    let magic = vec![202, 254, 186, 190];
    let abuf = buf[0..4].to_owned();
    for i in 0..4 {
        if magic[i] != abuf[i] {
            return false;
        }
    }
    true
}
fn version(buf: &Vec<u8>) {
    println!("min Version: {:?}", buf[4..6].to_owned());
    println!("max Version: {:?}", buf[6..8].to_owned());
}
