use thiserror::Error;

use crate::{database_header::{DatabaseHeader, DatabaseHeaderError}, database_page_collection::DatabasePageCollection};

#[derive(Debug)]
pub struct Database {
    header: DatabaseHeader,
    pages: DatabasePageCollection,
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum DatabaseReadError {
    #[error("Database file has a malformed header: {0:?}")]
    InvalidHeader(DatabaseHeaderError),
}

impl Database {
    pub fn from_bytes(db_file: Vec<u8>) -> Result<Self, DatabaseReadError> {
        let header_bytes = db_file[..100].to_vec();
        let header = DatabaseHeader::try_from(header_bytes).map_err(DatabaseReadError::InvalidHeader)?;
        println!("{header:?}");
        let pages = DatabasePageCollection::from_bytes(db_file, &header);
        Ok(Database {
            header,
            pages,
        })
    }
}
