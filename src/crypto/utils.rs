use super::consts::{BASE58_MULTIBASE_PREFIX, DID_KEY_PREFIX};
use super::error::{Error, Result};

pub fn extract_multikey(did: &str) -> Result<&str> {
    if !did.starts_with(DID_KEY_PREFIX) {
        return Err(Error::IncorrectDidKeyPrefix(did.into()));
    }
    Ok(&did[DID_KEY_PREFIX.len()..])
}

pub fn extract_prefixed_bytes(multikey: &str) -> Result<Vec<u8>> {
    if !multikey.starts_with(BASE58_MULTIBASE_PREFIX) {
        return Err(Error::IncorrectMultikeyPrefix(multikey.into()));
    }
    bs58::decode(multikey[BASE58_MULTIBASE_PREFIX.len()..].as_bytes())
        .into_vec()
        .map_err(Error::Base58)
}
