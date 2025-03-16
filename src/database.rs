use crate::{database_header::DatabaseHeader, database_page::DatabasePage};

pub struct Database {
    header: DatabaseHeader,
    pages: Vec<DatabasePage>,
}

impl Database {
    pub fn from_bytes(db_file: Vec<u8>) -> Self {
        todo!();
    }
}
