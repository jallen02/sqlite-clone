mod database;

use clap::Parser;
use database::Database;
use std::fs;
use thiserror::Error;

fn main() {
    let args = Args::parse();
    let db = read_database(args.file_path);
    println!("{:?}", db);
}

#[derive(Error, Debug)]
enum FileReadError {
    #[error("File doesn't exist")]
    FileDoesNotExist,
    #[error("File doesn't conform to the SQLite file format")]
    NotSqlFormat,
}

fn read_database(file_path: String) -> Result<Database, FileReadError> {
    if let Ok(db_bytes) = fs::read(file_path) {
        return Database::from_bytes(db_bytes.to_vec()).map_err(|err| {
            println!("{:?}", err);
            FileReadError::NotSqlFormat
        });
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
