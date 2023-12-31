use log::*;
use std::collections::HashMap;

use crate::{
    common::*,
    value::{Value, ValueArray},
};

#[derive(Default)]
pub struct Chunk {
    pub code: Vec<u8>,
    pub lines: HashMap<usize, usize>,
    pub constants: ValueArray,
}

impl Chunk {
    pub fn write_opcode(&mut self, opcode: Opcode, line: usize) {
        self.write_byte(opcode as u8, line);
    }

    pub fn write_byte(&mut self, byte: u8, line: usize) {
        self.lines.insert(self.code.len(), line);
        self.code.push(byte);
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }

    pub(crate) fn write_constant(&mut self, constant_index: usize, line: usize) {
        self.write_opcode(Opcode::Constant, line);
        self.code.push(constant_index.try_into().unwrap());
    }
}

// Disassemble related methods
impl Chunk {
    pub fn disassemble(&self, name: &str) {
        println!("== {name} ==");

        let mut offset = 0;
        while offset < self.code.len() {
            offset = self.disassemble_instruction(offset);
        }

        println!("====");
    }

    #[cfg(feature = "tracing")]
    pub fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{offset:04} ");
        print!("{:4} ", self.lines[&offset]);

        let instruction = Opcode::try_from(self.code[offset]);
        let Ok(instruction) = instruction else {
            print!("Invalid opcode {:04}", self.code[offset],);
            return offset + 1;
        };

        let ret: usize = match instruction {
            Opcode::Return | Opcode::Negate => self.simple_instruction(instruction, offset),
            Opcode::Constant => self.constant_instruction(instruction, offset),
            Opcode::Add | Opcode::Subtract | Opcode::Multiply | Opcode::Divide => {
                self.simple_instruction(instruction, offset)
            }
        };

        println!();

        ret
    }

    #[cfg(not(feature = "tracing"))]
    pub fn disassemble_instruction(&self, offset: usize) -> usize {
        self.code.len()
    }

    fn simple_instruction(&self, instruction: Opcode, offset: usize) -> usize {
        print!("{instruction}");

        offset + 1
    }

    fn constant_instruction(&self, instruction: Opcode, offset: usize) -> usize {
        let Ok(constant_idx): Result<usize, _> = self.code[offset + 1].try_into() else {
            print!(
                "Failed to convert data {} at offset {} into constant index",
                self.code[offset + 1],
                offset + 1
            );
            return offset + 2;
        };
        print!("{instruction} Idx {constant_idx} ");
        self.print_value(self.constants[constant_idx]);

        offset + 2
    }

    #[cfg(feature = "tracing")]
    pub fn print_value(&self, value: Value) {
        print!("Value {value}");
    }

    #[cfg(not(feature = "tracing"))]
    pub fn print_value(&self, value: Value) {
        print!("Value {value}");
    }

    #[cfg(feature = "tracing")]
    pub fn line() {
        println!();
    }

    #[cfg(not(feature = "tracing"))]
    pub fn line() {}
}
