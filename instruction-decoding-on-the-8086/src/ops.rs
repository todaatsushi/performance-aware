use std::fmt::Display;

pub enum Operation {
    MOV,
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::MOV => write!(f, "mov"),
        }
    }
}

pub fn get_operation(first: u8) -> Operation {
    let content = (first) >> 2;
    match content {
        34 => Operation::MOV,
        _ => panic!("Unhandled operation."),
    }
}
