#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use anyhow::*;
use chunk::Chunk;
use common::Opcode;
use compiler::compile;
use std::rc::Rc;
use vm::Vm;

pub mod chunk;
pub mod cli;
pub mod common;
pub mod compiler;
pub mod scanner;
pub mod value;
pub mod vm;

pub fn interpret2(line: String) -> Result<()> {
    let mut chunk = Chunk::default();
    let constant = chunk.add_constant(5.0);
    chunk.write_constant(constant, 123);
    let constant1 = chunk.add_constant(2.0);
    chunk.write_constant(constant1, 123);
    chunk.write_chunk(Opcode::Divide, 123);
    chunk.write_chunk(Opcode::Return, 123);
    Vm::interpret(chunk).context("Failed to interpret chunk")
}

pub fn interpret(source: String) -> Result<()> {
    let code = Rc::new(source);
    compile(code)?;
    Ok(())
}
