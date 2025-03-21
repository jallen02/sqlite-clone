use core::str;
use thiserror::Error;

#[derive(Debug, PartialEq, Eq)]
enum FileFormatVersion {
    Legacy,
    Wal,
}

#[derive(Debug, Error, PartialEq, Eq)]
enum FileFormatVersionError {
    #[error("File format should be either 1 or 2, was {0}")]
    IncorrectVariant(u8),
}

impl TryFrom<u8> for FileFormatVersion {
    type Error = FileFormatVersionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Legacy),
            2 => Ok(Self::Wal),
            _ => Err(FileFormatVersionError::IncorrectVariant(value))
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum TextEncoding {
    Utf8,
    Utf16Le,
    Utf16Be,
}

#[derive(Debug, Error, PartialEq, Eq)]
enum TextEncodingError {
    #[error("File format should be either 1, 2, or 3, is {0}")]
    IncorrectVariant(u32),
}

impl TryFrom<u32> for TextEncoding {
    type Error = TextEncodingError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Utf8),
            2 => Ok(Self::Utf16Le),
            3 => Ok(Self::Utf16Be),
            _ => Err(TextEncodingError::IncorrectVariant(value))
        }
    }
}

// https://www.sqlite.org/fileformat.html
#[derive(Debug)]
pub struct DatabaseHeader {
    // The database page size in bytes.
    // Must be a power of two between 512 and 32768 inclusive, or the value 1 representing a page size of 65536.
    pub page_size: u16,
    // File format write version. 1 for legacy; 2 for WAL.
    pub file_format_write_version: FileFormatVersion,
    // File format read version. 1 for legacy; 2 for WAL.
    pub file_format_read_version: FileFormatVersion,
    // Bytes of unused "reserved" space at the end of each page. Usually 0.
    pub reserved_space: u8,
    // Maximum embedded payload fraction. Must be 64.
    pub maximum_embedded_payload_fraction: u8,
    // Minimum embedded payload fraction. Must be 32.
    pub minimum_embedded_payload_fraction: u8,
    // Leaf payload fraction. Must be 32.
    pub leaf_payload_fraction: u8,
    // File change counter.
    pub file_change_counter: u32,
    // Size of the database file in pages. The "in-header database size".
    pub database_size_in_pages: u32,
    // Page number of the first freelist trunk page.
    pub first_freelist: u32,
    // Total number of freelist pages.
    pub num_freelist: u32,
    // The schema cookie.
    pub schema_cookie: u32,
    // The schema format number. Supported schema formats are 1, 2, 3, and 4.
    pub schema_format_number: u32,
    // Default page cache size.
    pub default_page_cache_size: u32,
    // The page number of the largest root b-tree page when in auto-vacuum or incremental-vacuum modes, or zero otherwise.
    pub largest_root_page: u32,
    // The database text encoding. A value of 1 means UTF-8. A value of 2 means UTF-16le. A value of 3 means UTF-16be.
    pub text_encoding: TextEncoding,
    // The "user version" as read and set by the user_version pragma.
    pub user_version: u32,
    // True (non-zero) for incremental-vacuum mode. False (zero) otherwise.
    pub incremental_vaccuum_mode: bool,
    // The "Application ID" set by PRAGMA application_id.
    pub application_id: u32,
    pub version_valid_for: u32,
    pub sqlite_version_number: u32,
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum DatabaseHeaderError {
    #[error("Length should be 100. Was {0}")]
    IncorrectLength(usize),
    #[error("Header string should be SQLite format 3\\0, was {0}")]
    IncorrectHeaderString(String),
    #[error("DB Header problem encountered while computing {0}")]
    MalformedDatabaseHeader(String),
    #[error("Tried to parse value for {item_parsed} out of {num_bytes_recieved} bytes, but expected {num_bytes_expected}")]
    IncorrectNumberOfBytes {
        num_bytes_recieved: usize,
        num_bytes_expected: usize,
        item_parsed: String
    },
}

fn get_u16_from_bytes(bytes: &[u8], item: &str) -> Result<u16, DatabaseHeaderError> {
    Ok(u16::from_be_bytes(
        <[u8; 2]>::try_from(bytes)
            .map_err(|_| DatabaseHeaderError::IncorrectNumberOfBytes{
                num_bytes_expected: 2,
                num_bytes_recieved: bytes.len(),
                item_parsed: item.to_owned(),
            })?
    ))
}

fn get_u32_from_bytes(bytes: &[u8], item: &str) -> Result<u32, DatabaseHeaderError> {
    Ok(u32::from_be_bytes(
        <[u8; 4]>::try_from(bytes)
            .map_err(|_| DatabaseHeaderError::IncorrectNumberOfBytes{
                num_bytes_expected: 4,
                num_bytes_recieved: bytes.len(),
                item_parsed: item.to_owned(),
            })?
    ))
}

impl TryFrom<Vec<u8>> for DatabaseHeader {
    type Error = DatabaseHeaderError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        if value.len() != 100 {
            return Err(DatabaseHeaderError::IncorrectLength(value.len()));
        }

        let header_string_bytes = &value[0..16];

        let header_string = str::from_utf8(header_string_bytes).unwrap();
        if header_string != "SQLite format 3\0" {
            return Err(DatabaseHeaderError::IncorrectHeaderString(header_string.to_owned()));
        }

        let page_size = get_u16_from_bytes(&value[16..18], "page_size")?;
        let file_format_write_version = FileFormatVersion::try_from(value[18]).unwrap();
        let file_format_read_version = FileFormatVersion::try_from(value[19]).unwrap();
        let reserved_space = value[20];
        let maximum_embedded_payload_fraction = value[21];
        let minimum_embedded_payload_fraction = value[22];
        let leaf_payload_fraction = value[23];
        let file_change_counter = get_u32_from_bytes(&value[24..28], "file_change_counter")?;
        let database_size_in_pages = get_u32_from_bytes(&value[28..32], "database_size_in_pages")?;
        let first_freelist = get_u32_from_bytes(&value[32..36], "first_freelist")?;
        let num_freelist = get_u32_from_bytes(&value[36..40], "num_freelist")?;
        let schema_cookie = get_u32_from_bytes(&value[40..44], "schema_cookie")?;
        let schema_format_number = get_u32_from_bytes(&value[44..48], "schema_format_number")?;
        let default_page_cache_size = get_u32_from_bytes(&value[48..52], "default_page_cache_size")?;
        let largest_root_page = get_u32_from_bytes(&value[52..56], "largest_root_page")?;
        let text_encoding = TextEncoding::try_from(
            get_u32_from_bytes(&value[56..60], "text_encoding")?)
                .map_err(|_| DatabaseHeaderError::MalformedDatabaseHeader("text_encoding".to_owned()))?;
        let user_version = get_u32_from_bytes(&value[60..64], "user_version")?;
        let incremental_vaccuum_mode = get_u32_from_bytes(&value[64..68], "incremental_vaccuum_mode")? > 0;
        let application_id = get_u32_from_bytes(&value[68..72], "application_id")?;
        let version_valid_for = get_u32_from_bytes(&value[92..96], "version_valid_for")?;
        let sqlite_version_number = get_u32_from_bytes(&value[96..100], "sqlite_version_number")?;

        Ok(DatabaseHeader {
            page_size,
            file_format_write_version,
            file_format_read_version,
            reserved_space,
            maximum_embedded_payload_fraction,
            minimum_embedded_payload_fraction,
            leaf_payload_fraction,
            file_change_counter,
            database_size_in_pages,
            first_freelist,
            num_freelist,
            schema_cookie,
            schema_format_number,
            default_page_cache_size,
            largest_root_page,
            text_encoding,
            user_version,
            incremental_vaccuum_mode,
            application_id,
            version_valid_for,
            sqlite_version_number,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::database_header::{FileFormatVersion, FileFormatVersionError, TextEncoding, TextEncodingError, DatabaseHeaderError};

    #[test]
    fn file_format_version_conversion() {
        assert_eq!(FileFormatVersion::try_from(1), Ok(FileFormatVersion::Legacy));
        assert_eq!(FileFormatVersion::try_from(2), Ok(FileFormatVersion::Wal));
        assert_eq!(FileFormatVersion::try_from(3), Err(FileFormatVersionError::IncorrectVariant(3)));
    }

    #[test]
    fn text_encoding_conversion() {
        assert_eq!(TextEncoding::try_from(1), Ok(TextEncoding::Utf8));
        assert_eq!(TextEncoding::try_from(2), Ok(TextEncoding::Utf16Le));
        assert_eq!(TextEncoding::try_from(3), Ok(TextEncoding::Utf16Be));
        assert_eq!(TextEncoding::try_from(4), Err(TextEncodingError::IncorrectVariant(4)));
    }

    #[test]
    fn get_u16_from_bytes() {
        assert_eq!(super::get_u16_from_bytes(&[0, 2], "test"), Ok(2));
        assert_eq!(super::get_u16_from_bytes(&[0, 10], "test"), Ok(10));
        assert_eq!(super::get_u16_from_bytes(&[1, 0], "test"), Ok(256));
        assert_eq!(
            super::get_u16_from_bytes(&[1, 0, 0], "test"),
            Err(DatabaseHeaderError::IncorrectNumberOfBytes {
                num_bytes_recieved: 3,
                num_bytes_expected: 2,
                item_parsed: "test".to_owned(),
            }
        ));
    }

    #[test]
    fn get_u32_from_bytes() {
        assert_eq!(super::get_u32_from_bytes(&[0, 0, 0, 2], "test"), Ok(2));
        assert_eq!(super::get_u32_from_bytes(&[0, 0, 0, 10], "test"), Ok(10));
        assert_eq!(super::get_u32_from_bytes(&[0, 0, 1, 0], "test"), Ok(256));
        assert_eq!(super::get_u32_from_bytes(&[0, 1, 0, 0], "test"), Ok(65536));
        assert_eq!(super::get_u32_from_bytes(&[1, 0, 0, 0], "test"), Ok(16777216));
        assert_eq!(
            super::get_u32_from_bytes(&[1, 0, 0], "test"),
            Err(DatabaseHeaderError::IncorrectNumberOfBytes {
                num_bytes_recieved: 3,
                num_bytes_expected: 4,
                item_parsed: "test".to_owned(),
            }
        ));
    }
}
