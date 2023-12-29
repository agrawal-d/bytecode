extern crate dirs;
extern crate log;
extern crate simplelog;
use anyhow::Result;
use bytecode::cli;
use bytecode::dbg;
use simplelog::*;
use std::fs::File;

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

fn main() -> Result<()> {
    init_logging();
    let _args = cli::parse_args();
    dbg()?;
    Ok(())
}
