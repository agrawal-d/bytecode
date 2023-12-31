#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use anyhow::*;
use chunk::Chunk;
use common::Opcode;
use compiler::Compiler;
use std::rc::Rc;
use vm::Vm;

pub mod chunk;
pub mod cli;
pub mod common;
pub mod compiler;
pub mod scanner;
pub mod value;
pub mod vm;

pub fn interpret(source: String) -> Result<()> {
    let chunk = Compiler::compile(Rc::from(source))?;
    let mut vm = Vm::new(chunk);
    vm.run()?;

    Ok(())
}
