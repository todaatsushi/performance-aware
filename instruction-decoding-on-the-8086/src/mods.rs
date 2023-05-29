#[derive(Debug)]
pub enum ModType {
    RegisterToRegister,
}

pub fn get_mod(byte: u8) -> ModType {
    let mod_mask = 0b_11_000_000;
    let mod_bits = byte & mod_mask;
    match mod_bits >> 6 {
        3 => ModType::RegisterToRegister,
        _ => {
            println!("{:b}", mod_bits >> 6);
            panic!("Not handled. Should always be between registers.")
        }
    }
}
