use crate::dw::Bit;
use std::{fmt::Display, usize};

#[derive(Clone, Copy, Debug)]
pub enum Reg {
    AL,
    AX,
    CL,
    CX,
    DL,
    DX,
    BL,
    BX,
    AH,
    SP,
    CH,
    BP,
    DH,
    SI,
    BH,
    DI,
}

impl Display for Reg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

const W_BIT_ON_REG: [Reg; 8] = [
    Reg::AX,
    Reg::CX,
    Reg::DX,
    Reg::BX,
    Reg::SP,
    Reg::BP,
    Reg::SI,
    Reg::DI,
];
const W_BIT_OFF_REG: [Reg; 8] = [
    Reg::AL,
    Reg::CL,
    Reg::DL,
    Reg::BL,
    Reg::AH,
    Reg::CH,
    Reg::BH,
    Reg::DH,
];

pub fn get_reg(byte: u8, w_bit: &Bit) -> Reg {
    let reg_mask = 0b_00_111_000;
    let reg: usize = ((byte & reg_mask) >> 3).into();

    let reg_match = match w_bit {
        Bit::ON => W_BIT_ON_REG,
        Bit::OFF => W_BIT_OFF_REG,
    };

    reg_match[reg]
}

pub fn get_rm(byte: u8, w_bit: &Bit) -> Reg {
    let rm_mask = 0b_00_000_111;
    let reg: usize = (byte & rm_mask).into();

    let reg_match = match w_bit {
        Bit::ON => W_BIT_ON_REG,
        Bit::OFF => W_BIT_OFF_REG,
    };

    reg_match[reg]
}

pub fn order_by_destination(reg: Reg, rm: Reg, d_bit: &Bit) -> (Reg, Reg) {
    // (dest, source)
    match d_bit {
        Bit::ON => (reg, rm),
        Bit::OFF => (rm, reg),
    }
}
