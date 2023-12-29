#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use anyhow::Context;
use chunk::Chunk;
use common::Opcode;
use vm::Vm;

pub mod chunk;
pub mod cli;
pub mod common;
pub mod value;
pub mod vm;

pub fn dbg() -> anyhow::Result<()> {
    let mut chunk = Chunk::default();
    let constant = chunk.add_constant(1.2);
    chunk.write_constant(constant, 123);
    chunk.write_chunk(Opcode::Return, 123);
    Vm::interpret(chunk).context("Failed to interpret chunk")
}
