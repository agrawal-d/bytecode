extern crate dirs;
extern crate log;
extern crate simplelog;
use anyhow::*;
use bytecode::cli;
use bytecode::interpret;
use simplelog::*;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::path::PathBuf;

fn init_logging() {
    // config with logging line and file
    let config = ConfigBuilder::new()
        .set_location_level(LevelFilter::Error)
        .set_time_level(LevelFilter::Off)
        .build();

    let log_file_path = dirs::state_dir()
        .expect("Could not get state dir")
        .join("bytecode.log");

    println!("Logging to {:?}", log_file_path);

    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Info,
            config.clone(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Debug,
            config.clone(),
            File::create(log_file_path).unwrap(),
        ),
    ])
    .unwrap();
}

fn repl() -> Result<()> {
    let stdin = io::stdin();
    println!("Bytecode interpreter REPL mode");
    print!(">>> ");
    io::stdout().flush()?;
    for line in stdin.lock().lines() {
        let line = line.context("Failed to read line")?;
        interpret(line).context("Failed to interpret code")?;
        print!(">>> ");
        io::stdout().flush()?;
    }
    println!();
    Ok(())
}

fn run_file(path: PathBuf) -> Result<()> {
    let file = File::open(path).context("Failed to open file")?;
    let mut buffer = String::new();
    BufReader::new(file)
        .read_to_string(&mut buffer)
        .context("Failed to read file")?;

    interpret(buffer).context("Failed to interpret code")?;
    Ok(())
}

fn main() -> Result<()> {
    init_logging();
    let args = cli::parse_args();
    if let Some(path) = args.file {
        run_file(path)?;
    } else {
        repl()?;
    }
    Ok(())
}
