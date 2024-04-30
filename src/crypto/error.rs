#[derive(Debug)]
pub enum Error {
    IncorrectDidKeyPrefix(String),
    IncorrectMultikeyPrefix(String),
    UnsupportedMultibase(String),
    UnsupportedKeyType,
    Base58(bs58::decode::Error),
    ECDSA(ecdsa::elliptic_curve::Error),
    Signature(ecdsa::signature::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::IncorrectDidKeyPrefix(did) => write!(f, "Incorrect prefix for did:key: {did}"),
            Error::IncorrectMultikeyPrefix(multikey) => {
                write!(f, "Incorrect prefix for multikey: {multikey}")
            }
            Error::UnsupportedMultibase(mb) => write!(f, "Unsupported multibase: {mb}"),
            Error::UnsupportedKeyType => write!(f, "Unsupported key type"),
            Error::Base58(err) => write!(f, "Base58 decoding error: {err}"),
            Error::ECDSA(err) => write!(f, "ECDSA elliptic_curve error: {err}"),
            Error::Signature(err) => write!(f, "ECDSA signature error: {err}"),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;
