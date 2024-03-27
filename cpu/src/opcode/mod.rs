mod cb_prefixed;
mod unprefixed;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Flag {
    Set,
    Unset,
    Dependent,
    Independent,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Flags {
    zero: Flag,
    subtraction: Flag,
    half_carry: Flag,
    carry: Flag,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Opcode {
    value: u8,
    name: String,
    tcycles: (u8, u8), //without, with branching
    mcycles: (u8, u8), //without, with branching
    length: u8,
    flags: Flags,
}

impl Opcode {
    pub fn new(
        value: u8,
        name: String,
        tcycles: (u8, u8),
        mcycles: (u8, u8),
        length: u8,
        flags: Flags,
    ) -> Self {
        Opcode {
            value,
            name,
            tcycles,
            mcycles,
            length,
            flags,
        }
    }
}
