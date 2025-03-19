use crate::{database_page::DatabasePage, database_header::DatabaseHeader};

#[derive(Debug)]
pub struct DatabasePageCollection {
    pages: Vec<DatabasePage>
}

impl DatabasePageCollection {
    pub fn from_bytes(bytes: Vec<u8>, header: &DatabaseHeader) -> DatabasePageCollection {
        // Split the bytes in the database file into chuncks of size `header.page_size`
        let all_pages: Vec<DatabasePage> = bytes
            .chunks(header.page_size.into())
            .enumerate()
            // Remove the first 100 bytes of the first page (the database header)
            .map(|(idx, item)| {
                if idx == 0 {
                    DatabasePage::try_from(&item[100..]).expect("couldn't convert bytes to database page on first page")
                } else {
                    DatabasePage::try_from(item).expect("couldn't convert bytes to database page")
                }
            })
            .collect();
        DatabasePageCollection {
            pages: all_pages
        }
    }
}
