use std::io::Read;
use std::{env, fs};

fn read_binary() -> Vec<u8> {
    let args: Vec<String> = env::args().collect();
    let file: &str = args[1].as_str();

    let mut input = fs::File::open(file).expect("Couldn't open file");
    let mut buffer = Vec::new();

    let _ = input.read_to_end(&mut buffer);
    buffer
}

fn main() {
    let contents = read_binary();
    for v in contents {
        println!("{:08b}", v);
    }
}
