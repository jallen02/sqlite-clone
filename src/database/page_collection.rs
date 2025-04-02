use crate::{database::header::DatabaseHeader, database::page::Page};

#[derive(Debug)]
pub struct PageCollection {
    pages: Vec<Page>,
}

impl PageCollection {
    pub fn from_bytes(bytes: Vec<u8>, header: &DatabaseHeader) -> Self {
        // Split the bytes in the database file into chuncks of size `header.page_size`
        let all_pages: Vec<Page> = bytes
            .chunks(header.page_size.into())
            .enumerate()
            // Remove the first 100 bytes of the first page (the database header)
            .map(|(idx, item)| {
                if idx == 0 {
                    Page::try_from(&item[100..])
                        .expect("couldn't convert bytes to database page on first page")
                } else {
                    Page::try_from(item).expect("couldn't convert bytes to database page")
                }
            })
            .collect();
        Self { pages: all_pages }
    }

    pub fn len(&self) -> usize {
        self.pages.len()
    }
}
