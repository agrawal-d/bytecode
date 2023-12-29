use chunk::Chunk;
use common::Opcode;

pub mod chunk;
pub mod cli;
pub mod common;
pub mod value;

pub fn dbg() -> anyhow::Result<()> {
    let mut chunk = Chunk::default();
    let constant = chunk.add_constant(1.2);
    chunk.write_chunk(Opcode::Return, 123);
    chunk.write_constant(constant, 123);
    chunk.disassemble("test chunk");
    Ok(())
}
