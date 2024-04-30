use crate::{crypto, identity::did::did_resolver};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::future::Future;

#[derive(Debug, Deserialize)]
pub struct JwtPayload {
    pub iss: String,
    pub aud: String,
    pub exp: i64,
}

#[derive(Debug)]
pub enum JwtError {
    Bad,
    BadAudience,
    Expired,
}

#[derive(Debug)]
pub enum Error {
    AuthRequiredError(JwtError, String),
    Base64Decode(base64::DecodeError),
    DidResolver(did_resolver::Error),
    Crypto(crypto::error::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::AuthRequiredError(_, msg) => write!(f, "AuthRequiredError: {msg}"),
            Error::Base64Decode(err) => write!(f, "Base64Decode: {err}"),
            Error::DidResolver(err) => write!(f, "DidResolver: {err}"),
            Error::Crypto(err) => write!(f, "Crypto: {err}"),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;

pub trait SigningKeyProvider {
    fn get_signing_key(
        &self,
        iss: &str,
        force_refresh: bool,
    ) -> impl Future<Output = Result<String>>;
}

pub async fn verify_jwt<S: SigningKeyProvider>(
    jwt: &str,
    did: Option<&str>,
    signing_key_provider: S,
) -> Result<JwtPayload> {
    let parts = jwt.splitn(3, '.').collect::<Vec<_>>();
    if parts.len() != 3 {
        return Err(Error::AuthRequiredError(
            JwtError::Bad,
            String::from("poorly formatted jwt"),
        ));
    }
    let payload = parse_payload(parts[1])?;
    let sig = parts[2];

    if let Some(exp) = DateTime::from_timestamp(payload.exp, 0) {
        if exp < Utc::now() {
            return Err(Error::AuthRequiredError(
                JwtError::Expired,
                String::from("jwt expired"),
            ));
        }
    }
    if let Some(did) = did {
        if payload.aud != did {
            return Err(Error::AuthRequiredError(
                JwtError::BadAudience,
                String::from("jwt audience does not match service did"),
            ));
        }
    }
    let msg = [parts[0], parts[1]].join(".");
    let msg_bytes = msg.as_bytes();
    let sig_bytes = URL_SAFE_NO_PAD
        .decode(sig.as_bytes())
        .map_err(Error::Base64Decode)?;
    let signing_key = signing_key_provider
        .get_signing_key(&payload.iss, false)
        .await?;
    crypto::verify::verify_signature(&signing_key, msg_bytes, &sig_bytes).map_err(Error::Crypto)?;
    // TODO: get fresh signing key in case it failed due to a recent rotation
    Ok(payload)
}

fn parse_payload(b64: &str) -> Result<JwtPayload> {
    let decoded = URL_SAFE_NO_PAD
        .decode(b64.as_bytes())
        .map_err(Error::Base64Decode)?;
    println!("decoded: {:#?}", String::from_utf8_lossy(&decoded));
    serde_json::from_slice(&decoded)
        .map_err(|_| Error::AuthRequiredError(JwtError::Bad, String::from("poorly formatted jwt")))
}
