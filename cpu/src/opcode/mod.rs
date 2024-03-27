mod cb_prefixed;
mod unprefixed;

use self::cb_prefixed::CB_PREFIXED;
use self::unprefixed::UNPREFIXED;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

impl Flags {
    pub fn new(zero: Flag, subtraction: Flag, half_carry: Flag, carry: Flag) -> Self {
        Flags {
            zero,
            subtraction,
            half_carry,
            carry,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpCode {
    value: u8,
    name: String,
    tcycles: (u8, u8), //without, with branching
    mcycles: (u8, u8), //without, with branching
    length: u8,
    flags: Flags,
}

impl OpCode {
    pub fn new(
        value: u8,
        name: String,
        tcycles: (u8, u8),
        mcycles: (u8, u8),
        length: u8,
        flags: Flags,
    ) -> Self {
        OpCode {
            value,
            name,
            tcycles,
            mcycles,
            length,
            flags,
        }
    }
}

lazy_static! {
    pub static ref UNPREFIXED_OPCODE_MAP: HashMap<u8, OpCode> =
        serde_json::from_str(UNPREFIXED).expect("Unable to serialze json");
    pub static ref CB_PREFIXED_OPCODE_MAP: HashMap<u8, OpCode> =
        serde_json::from_str(CB_PREFIXED).expect("Unable to serialze json");
}
