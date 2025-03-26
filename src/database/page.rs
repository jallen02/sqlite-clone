use thiserror::Error;

use crate::database::page_header::{PageHeader, PageType};

#[derive(Debug)]
pub struct DatabasePage {
    page_header: PageHeader,
    bytes: Vec<u8>,
}

#[derive(Debug, Error)]
pub enum DatabasePageError {
    #[error("blah!")]
    Blah,
}

impl TryFrom<&[u8]> for DatabasePage {
    type Error = DatabasePageError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let page_type: PageType = value[0].try_into().map_err(|_| DatabasePageError::Blah)?;
        let page_header = match page_type {
            PageType::InteriorIndex => &value[0..12],
            PageType::InteriorTable => &value[0..12],
            PageType::LeafIndex => &value[0..8],
            PageType::LeafTable => &value[0..8],
        }
        .try_into()
        .map_err(|_| DatabasePageError::Blah)?;
        Ok(DatabasePage {
            page_header,
            bytes: value.to_vec(),
        })
    }
}
