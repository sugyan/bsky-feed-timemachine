use super::consts::{JwtAlg, BASE58_MULTIBASE_PREFIX, DID_KEY_PREFIX};
use super::error::{Error, Result};
use super::plugins::{p256::P256Plugin, secp256k1::Secp256k1Plugin};
use super::utils;
use super::DidKeyPlugin;

#[derive(Debug)]
pub struct ParsedMultikey {
    pub jwt_alg: JwtAlg,
    pub key: Vec<u8>,
}

fn find_plugin(prefix: &[u8]) -> Result<Box<dyn DidKeyPlugin>> {
    match &prefix[0..2] {
        [0x80, 0x24] => Ok(Box::new(P256Plugin)),
        [0xe7, 0x01] => Ok(Box::new(Secp256k1Plugin)),
        _ => Err(Error::UnsupportedKeyType),
    }
}

pub fn parse_multikey(multikey: &str) -> Result<ParsedMultikey> {
    let prefixed = utils::extract_prefixed_bytes(multikey)?;
    let plugin = find_plugin(&prefixed)?;
    Ok(ParsedMultikey {
        jwt_alg: plugin.jwt_alg(),
        key: plugin.decompress_pubkey(&prefixed[2..])?,
    })
}

pub fn format_multikey(jwt_alg: JwtAlg, key: &[u8]) -> Result<String> {
    let prefixed = match jwt_alg {
        JwtAlg::P256 => [vec![0x80, 0x24], P256Plugin.compress_pubkey(key)?].concat(),
        JwtAlg::Secp256k1 => [vec![0xe7, 0x01], Secp256k1Plugin.compress_pubkey(key)?].concat(),
    };
    Ok(BASE58_MULTIBASE_PREFIX.to_string() + &bs58::encode(&prefixed).into_string())
}

pub fn parse_did_key(did: &str) -> Result<ParsedMultikey> {
    let multikey = utils::extract_multikey(did)?;
    parse_multikey(multikey)
}

pub fn format_did_key(jwt_alg: JwtAlg, key: &[u8]) -> Result<String> {
    Ok(format!(
        "{DID_KEY_PREFIX}{}",
        format_multikey(jwt_alg, key)?
    ))
}
