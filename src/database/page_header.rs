use crate::util::{DecodeError, get_u16_from_bytes, get_u32_from_bytes};

#[derive(Debug, PartialEq, Clone)]
pub enum PageType {
    InteriorIndex,
    InteriorTable,
    LeafIndex,
    LeafTable,
}

pub enum PageTypeError {
    InvalidType(u8),
}

impl TryFrom<u8> for PageType {
    type Error = PageTypeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            2 => Ok(PageType::InteriorIndex),
            5 => Ok(PageType::InteriorTable),
            10 => Ok(PageType::LeafIndex),
            13 => Ok(PageType::LeafTable),
            _ => Err(PageTypeError::InvalidType(value)),
        }
    }
}

#[derive(Debug)]
pub struct PageHeader {
    page_type: PageType,
    first_page_offset: u16,
    number_of_cells: u16,
    cell_content_start: u16,
    num_fragmented_free_bytes: u8,
    // Only Interior pages contain a right-most pointer
    right_most_pointer: Option<u32>,
}

pub enum PageHeaderError {
    EmptyHeader,
    InvalidLength(PageType, usize),
    InvalidPageType(u8),
    DecodeError(DecodeError),
    Unknown(String),
}

impl From<DecodeError> for PageHeaderError {
    fn from(value: DecodeError) -> Self {
        Self::DecodeError(value)
    }
}

impl From<PageTypeError> for PageHeaderError {
    fn from(value: PageTypeError) -> Self {
        match value {
            PageTypeError::InvalidType(t) => Self::InvalidPageType(t),
            _ => Self::Unknown("Unknown page type found".to_owned()),
        }
    }
}

impl TryFrom<&[u8]> for PageHeader {
    type Error = PageHeaderError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(PageHeaderError::EmptyHeader);
        }
        // Value is non-empty, so we know it has at least one value
        let page_type: PageType = value[0].try_into()?;
        let right_most_pointer =
            if page_type == PageType::LeafTable || page_type == PageType::LeafIndex {
                if value.len() != 8 {
                    Err(PageHeaderError::InvalidLength(
                        page_type.clone(),
                        value.len(),
                    ))
                } else {
                    Ok(None)
                }
            } else if value.len() != 12 {
                Err(PageHeaderError::InvalidLength(
                    page_type.clone(),
                    value.len(),
                ))
            } else {
                Ok(Some(get_u32_from_bytes(&value[8..], "right_most_pointer")?))
            }?;
        let first_page_offset = get_u16_from_bytes(&value[1..3], "first_page_offset")?;
        let number_of_cells = get_u16_from_bytes(&value[3..5], "number_of_cells")?;
        let cell_content_start = get_u16_from_bytes(&value[5..7], "cell_content_start")?;
        let num_fragmented_free_bytes = value[7];
        Ok(PageHeader {
            page_type,
            first_page_offset,
            number_of_cells,
            cell_content_start,
            num_fragmented_free_bytes,
            right_most_pointer,
        })
    }
}
