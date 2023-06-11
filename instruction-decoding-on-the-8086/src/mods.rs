#[derive(Debug)]
pub enum RegisterMode {
    NoDisplacement,
}

#[derive(Debug)]
pub enum MemoryMode {
    NoDisplacement,
    EightBitDisplacement,
    SixteenBitDisplacement,
}

#[derive(Debug)]
pub enum ModType {
    RegisterMode(RegisterMode),
    MemoryMode(MemoryMode),
}

pub fn get_mod(byte: u8) -> ModType {
    let mod_mask = 0b_11_000_000;
    let mod_bits = byte & mod_mask;
    match mod_bits >> 6 {
        0 => ModType::MemoryMode(MemoryMode::NoDisplacement),
        1 => ModType::MemoryMode(MemoryMode::EightBitDisplacement),
        2 => ModType::MemoryMode(MemoryMode::SixteenBitDisplacement),
        3 => ModType::RegisterMode(RegisterMode::NoDisplacement),
        _ => {
            println!("{:b}", mod_bits >> 6);
            panic!("Not handled. Should always be between registers.")
        }
    }
}
