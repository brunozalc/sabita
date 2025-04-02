pub mod registers;
pub mod opcodes;
pub mod instructions;

use registers::Registers;
use opcodes::Opcodes;

pub struct CPU {
    pub registers: Registers,
    pub ime: bool,
    pub cycles: u64,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            registers: Registers::new(),
            ime: false,
            cycles: 0x0,
        } 
    }
    
    // fetch-decode-execute cycle
    pub fn fetch(&mut self) -> u8 {
        todo!()
    }
    
    pub fn decode(&mut self, opcode: u8) -> u8 {
        todo!()
    }
    
    pub fn execute(&mut self) -> u64 {
        todo!()
    }
    
}