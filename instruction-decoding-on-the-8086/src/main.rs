use instruction_decoding_on_the_8086::{dw, mods, ops, reg};
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

    println!("\nbits 16\n");

    for bytes in contents.chunks(2) {
        let first = bytes[0];
        let second = bytes[1];

        let op = ops::get_operation(first);
        let (d, w) = dw::get_dw_bits(first);

        let mod_bits = mods::get_mod(second);
        match mod_bits {
            mods::ModType::RegisterToRegister => {
                let reg = reg::get_reg(second, &w);
                let rm = reg::get_rm(second, &w);

                let (dest, source) = reg::order_by_destination(reg, rm, &d);
                println!("{} {}, {}", op, dest, source);
            }
        }
    }
}
