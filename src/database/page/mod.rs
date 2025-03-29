use crate::util::{DecodeError, get_u16_from_bytes};
use thiserror::Error;

use header::{PageHeader, PageHeaderError, PageType, PageTypeError};

pub mod header;

#[derive(Debug)]
pub struct DatabasePage {
    page_header: PageHeader,
    cell_offsets: CellOffsets,
    bytes: Vec<u8>,
}

#[derive(Debug, Error)]
pub enum DatabasePageError {
    #[error("Encountered an error with the cell offsets:\n{0}")]
    CellOffsetError(CellOffsetError),
    #[error("Encountered an error calculating page type:\n{0}")]
    PageTypeError(PageTypeError),
    #[error("")]
    PageHeaderError(PageHeaderError),
    #[error("Unknown error")]
    Unknown,
}

#[derive(Debug)]
pub struct CellOffsets(Vec<u16>);

#[derive(Debug, Error)]
pub enum CellOffsetError {
    #[error("Encountered an error decoding cell offsets:\n{0}")]
    CellOffsetDecodeError(DecodeError),
}

impl TryFrom<&[u8]> for CellOffsets {
    type Error = CellOffsetError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let offsets: Result<Vec<u16>, CellOffsetError> = value
            .chunks(2)
            .map(|item| {
                get_u16_from_bytes(item, "cell_offsets")
                    .map_err(CellOffsetError::CellOffsetDecodeError)
            })
            .collect();
        offsets.map(CellOffsets)
    }
}

impl TryFrom<&[u8]> for DatabasePage {
    type Error = DatabasePageError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let page_type: PageType = value[0]
            .try_into()
            .map_err(DatabasePageError::PageTypeError)?;
        let mut next_byte: usize = 0;
        let page_header: Result<PageHeader, PageHeaderError> = match page_type {
            PageType::InteriorIndex | PageType::InteriorTable => {
                next_byte += 12;
                &value[0..12]
            }
            PageType::LeafIndex | PageType::LeafTable => {
                next_byte += 8;
                &value[0..8]
            }
        }
        .try_into();

        match page_header {
            Ok(header) => {
                let cell_offsets_len: usize = (header.get_number_of_cells() * 2).into();
                let cell_offsets: CellOffsets = value[next_byte..(next_byte + cell_offsets_len)]
                    .try_into()
                    .map_err(DatabasePageError::CellOffsetError)?;
                Ok(DatabasePage {
                    cell_offsets,
                    page_header: header,
                    bytes: value.to_vec(),
                })
            }
            Err(err) => Err(DatabasePageError::PageHeaderError(err)),
        }
    }
}
