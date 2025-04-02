use clap::Parser;
use std::{fs, io};
use thiserror::Error;

use super::main_panel::start_ui;

pub fn start_cli() -> Result<(), CliError> {
    let args = Args::parse();
    start_ui(args);
    Ok(())
}

/// A file explorer to visualize a SQLite database.
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path to the SQLite database file.
    pub filepath: String,
}

#[derive(Error, Debug)]
pub enum CliError {
    #[error("Encountered an IO Error ")]
    IoError { filepath: String, err: io::Error },
}
