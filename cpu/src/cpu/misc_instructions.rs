use crate::opcodes::OpCode;

use super::{ImeState, Instructions};

pub struct MiscInstructions {}

impl Instructions for MiscInstructions {}

impl MiscInstructions {
    pub fn new() -> Self {
        MiscInstructions {}
    }

    pub fn nop(&mut self, opcode: &OpCode) -> u8 {
        opcode.tcycles.0
    }

    pub fn stop(&mut self, opcode: &OpCode) -> u8 {
        todo!();
        opcode.tcycles.0
    }

    pub fn halt(&mut self, opcode: &OpCode, halted: &mut bool) -> u8 {
        *halted = true;
        opcode.tcycles.0
    }

    pub fn di(&mut self, opcode: &OpCode, di: &mut ImeState) -> u8 {
        *di = ImeState::Staged;
        opcode.tcycles.0
    }

    pub fn ei(&mut self, opcode: &OpCode, ei: &mut ImeState) -> u8 {
        *ei = ImeState::Staged;
        opcode.tcycles.0
    }
}
