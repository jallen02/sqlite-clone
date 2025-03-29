use crate::util::{DecodeError, get_u16_from_bytes};
use thiserror::Error;

use header::{PageHeader, PageHeaderError, PageType};

pub mod header;

#[derive(Debug)]
pub struct DatabasePage {
    page_header: PageHeader,
    cell_offsets: Vec<u16>,
    bytes: Vec<u8>,
}

#[derive(Debug, Error)]
pub enum DatabasePageError {
    #[error("blah!")]
    Blah,
}

#[derive(Debug)]
pub struct CellOffsets(Vec<u16>);

pub enum CellOffsetError {
    CellOffsetDecodeError(DecodeError),
}

impl TryFrom<&[u8]> for CellOffsets {
    type Error = CellOffsetError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let offsets: Result<Vec<u16>, CellOffsetError> = value
            .chunks(2)
            .map(|item| {
                get_u16_from_bytes(item, "cell_offsets")
                    .map_err(|err| CellOffsetError::CellOffsetDecodeError(err))
            })
            .collect();
        offsets.map(CellOffsets)
    }
}

impl TryFrom<&[u8]> for DatabasePage {
    type Error = DatabasePageError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let page_type: PageType = value[0].try_into().map_err(|_| DatabasePageError::Blah)?;
        let mut next_byte = 0;
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

        if let Ok(header) = page_header {
            return Ok(DatabasePage {
                page_header: header,
                bytes: value.to_vec(),
            });
        } else {
            return Err(DatabasePageError::Blah);
        }
    }
}
