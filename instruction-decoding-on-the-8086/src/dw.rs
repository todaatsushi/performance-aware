#[derive(Debug)]
pub enum Bit {
    ON,
    OFF,
}

pub fn get_dw_bits(first: u8) -> (Bit, Bit) {
    let dw_mask = 0b_0000_00_11;
    let shifted = first & dw_mask;

    let d_bit = match shifted >> 1 {
        1 => Bit::ON,
        0 => Bit::OFF,
        _ => {
            println!("D BIT: {:b}", shifted >> 1);
            panic!("Bit has to be 0 or 1.")
        }
    };

    let w_mask = 0b_0000_00_01;
    let w_bit = match shifted & w_mask {
        1 => Bit::ON,
        0 => Bit::OFF,
        _ => {
            println!("W BIT: {:b}", shifted >> 1);
            panic!("Bit has to be 0 or 1.")
        }
    };
    (d_bit, w_bit)
}
