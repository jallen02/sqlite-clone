mod database_header;
mod database_page;
mod database;

use clap::Parser;
use std::fs;
use database_header::DatabaseHeader;
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

fn read_database(file_path: String) -> Result<DatabaseHeader, FileReadError> {
    if let Ok(db_bytes) = fs::read(file_path) {
        return DatabaseHeader::try_from(db_bytes[..100].to_vec()).map_err(|err| {
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
