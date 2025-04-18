use thiserror::Error;

use crate::{
    database::header::{DatabaseHeader, DatabaseHeaderError},
    database::page_collection::PageCollection,
};

pub mod header;
pub mod page;
pub mod page_collection;

#[derive(Debug)]
pub struct Database {
    pub header: DatabaseHeader,
    pages: PageCollection,
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum DatabaseReadError {
    #[error("Database file has a malformed header: {0:?}")]
    InvalidHeader(DatabaseHeaderError),
}

impl Database {
    pub fn from_bytes(db_file: Vec<u8>) -> Result<Self, DatabaseReadError> {
        let header_bytes = db_file[..100].to_vec();
        let header =
            DatabaseHeader::try_from(header_bytes).map_err(DatabaseReadError::InvalidHeader)?;
        let pages = PageCollection::from_bytes(db_file, &header);
        Ok(Database { header, pages })
    }
}
