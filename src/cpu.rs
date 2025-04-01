pub mod registers;
pub mod opcodes;
pub mod instructions;

use registers::Registers;
use opcodes::Opcodes;

pub struct CPU {
    pub registers: Registers,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            registers: Registers::new(),
        } 
    }
    
    pub fn execute(&mut self, opcode: u8) {
       instructions::execute(self, opcode);
    }
}