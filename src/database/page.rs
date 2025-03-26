use thiserror::Error;

#[derive(Debug)]
pub struct DatabasePage {
    bytes: Vec<u8>,
}

#[derive(Debug, Error)]
pub enum DatabasePageError {
    #[error("blah!")]
    Blah,
}

impl DatabasePage {
    fn from_bytes(page_bytes: Vec<u8>) -> Self {
        todo!();
    }
}

impl TryFrom<&[u8]> for DatabasePage {
    type Error = DatabasePageError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(DatabasePage {
            bytes: value.to_vec(),
        })
    }
}
