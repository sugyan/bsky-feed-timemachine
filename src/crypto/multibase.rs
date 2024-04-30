use super::error::{Error, Result};

pub fn multibase_to_bytes(mb: &str) -> Result<Vec<u8>> {
    match mb.chars().next() {
        Some('z') => Ok(bs58::decode(mb.as_bytes())
            .into_vec()
            .map_err(Error::Base58)?),
        _ => Err(Error::UnsupportedMultibase(mb.into())),
    }
}
