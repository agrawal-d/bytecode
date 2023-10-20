use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = "Bytecode crafting interpreter")]
pub struct Cli {
    #[arg(name = "file", help = "Path to file to interpret")]
    pub file: Option<PathBuf>,
}

pub fn parse_args() -> Cli {
    Cli::parse()
}
