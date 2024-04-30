use crate::common_web::did_doc::DidDocument;
use crate::crypto::consts::JwtAlg;
use crate::crypto::{did, error, multibase};

#[derive(Debug)]
pub enum Error {
    SigningKeyNotFound(DidDocument),
    Crypto(error::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::SigningKeyNotFound(did_doc) => {
                write!(f, "Could not parse signingKey from doc: {did_doc:?}")
            }
            Error::Crypto(err) => write!(f, "Crypto error: {err}"),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;

fn get_key(did_doc: &DidDocument) -> Result<Option<String>> {
    if let Some(key) = did_doc.get_signing_key() {
        Ok(Some(get_did_key_from_multibase(key)?))
    } else {
        Ok(None)
    }
}

fn get_did_key_from_multibase((r#type, public_key_multibase): (String, String)) -> Result<String> {
    match r#type.as_str() {
        "EcdsaSecp256r1VerificationKey2019" => Ok(did::format_did_key(
            JwtAlg::P256,
            &multibase::multibase_to_bytes(&public_key_multibase).map_err(Error::Crypto)?,
        )
        .map_err(Error::Crypto)?),
        "EcdsaSecp256k1VerificationKey2019" => Ok(did::format_did_key(
            JwtAlg::Secp256k1,
            &multibase::multibase_to_bytes(&public_key_multibase).map_err(Error::Crypto)?,
        )
        .map_err(Error::Crypto)?),
        "Multikey" => {
            let parsed = did::parse_multikey(&public_key_multibase).map_err(Error::Crypto)?;
            Ok(did::format_did_key(parsed.jwt_alg, &parsed.key).map_err(Error::Crypto)?)
        }
        _ => unimplemented!(),
    }
}

pub fn ensure_atproto_key(did_doc: &DidDocument) -> Result<String> {
    get_key(did_doc)?.ok_or(Error::SigningKeyNotFound(did_doc.clone()))
}
