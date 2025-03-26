use thiserror::Error;

#[derive(PartialEq, Eq, Debug, Error)]
pub enum DecodeError {
    #[error(
        "Tried to parse value for {item_parsed} out of {num_bytes_recieved} bytes, but expected {num_bytes_expected}"
    )]
    IncorrectNumberOfBytes {
        num_bytes_expected: usize,
        num_bytes_recieved: usize,
        item_parsed: String,
    },
}

pub(crate) fn get_u16_from_bytes(bytes: &[u8], item: &str) -> Result<u16, DecodeError> {
    Ok(u16::from_be_bytes(<[u8; 2]>::try_from(bytes).map_err(
        |_| DecodeError::IncorrectNumberOfBytes {
            num_bytes_expected: 2,
            num_bytes_recieved: bytes.len(),
            item_parsed: item.to_owned(),
        },
    )?))
}

pub(crate) fn get_u32_from_bytes(bytes: &[u8], item: &str) -> Result<u32, DecodeError> {
    Ok(u32::from_be_bytes(<[u8; 4]>::try_from(bytes).map_err(
        |_| DecodeError::IncorrectNumberOfBytes {
            num_bytes_expected: 4,
            num_bytes_recieved: bytes.len(),
            item_parsed: item.to_owned(),
        },
    )?))
}

#[cfg(test)]
mod test {
    use crate::util::DecodeError;

    #[test]
    fn get_u16_from_bytes() {
        assert_eq!(super::get_u16_from_bytes(&[0, 2], "test"), Ok(2));
        assert_eq!(super::get_u16_from_bytes(&[0, 10], "test"), Ok(10));
        assert_eq!(super::get_u16_from_bytes(&[1, 0], "test"), Ok(256));
        assert_eq!(
            super::get_u16_from_bytes(&[1, 0, 0], "test"),
            Err(DecodeError::IncorrectNumberOfBytes {
                num_bytes_recieved: 3,
                num_bytes_expected: 2,
                item_parsed: "test".to_owned(),
            })
        );
    }

    #[test]
    fn get_u32_from_bytes() {
        assert_eq!(super::get_u32_from_bytes(&[0, 0, 0, 2], "test"), Ok(2));
        assert_eq!(super::get_u32_from_bytes(&[0, 0, 0, 10], "test"), Ok(10));
        assert_eq!(super::get_u32_from_bytes(&[0, 0, 1, 0], "test"), Ok(256));
        assert_eq!(super::get_u32_from_bytes(&[0, 1, 0, 0], "test"), Ok(65536));
        assert_eq!(
            super::get_u32_from_bytes(&[1, 0, 0, 0], "test"),
            Ok(16777216)
        );
        assert_eq!(
            super::get_u32_from_bytes(&[1, 0, 0], "test"),
            Err(DecodeError::IncorrectNumberOfBytes {
                num_bytes_recieved: 3,
                num_bytes_expected: 4,
                item_parsed: "test".to_owned(),
            })
        );
    }
}
