use clap::Parser;
use std::fs;

mod database_header;

fn main() {
    let args = Args::parse();
    let db = read_database(args.file_path);
    println!("{:?}", db);
}

#[derive(Debug)]
enum FileReadError {
    FileDoesNotExist,
    NotSqlFormat,
}

fn read_database(file_path: String) -> Result<Vec<u8>, FileReadError> {
    if let Ok(db_bytes) = fs::read(file_path) {
        return Ok(db_bytes);
    }
    Err(FileReadError::FileDoesNotExist)
}

/// A simple clone of SQLite written in Rust as a side project
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// File path to database
    #[arg(short, long)]
    file_path: String
}
