use super::consts::JwtAlg;
use super::did;
use super::error::Result;
use super::plugins::{p256::P256Plugin, secp256k1::Secp256k1Plugin};
use super::DidKeyPlugin;

pub fn verify_signature(did_key: &str, msg: &[u8], sig: &[u8]) -> Result<()> {
    let parsed = did::parse_did_key(did_key)?;
    match parsed.jwt_alg {
        JwtAlg::P256 => P256Plugin.verify_signature(did_key, msg, sig),
        JwtAlg::Secp256k1 => Secp256k1Plugin.verify_signature(did_key, msg, sig),
    }
}
