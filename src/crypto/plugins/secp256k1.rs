use super::super::error::{Error, Result};
use super::super::utils;
use super::super::{consts::JwtAlg, DidKeyPlugin};
use ecdsa::elliptic_curve::PublicKey;
use ecdsa::signature::Verifier;
use ecdsa::{Signature, VerifyingKey};
use k256::elliptic_curve::sec1::ToEncodedPoint;

pub struct Secp256k1Plugin;

impl DidKeyPlugin for Secp256k1Plugin {
    fn jwt_alg(&self) -> JwtAlg {
        JwtAlg::Secp256k1
    }
    fn compress_pubkey(&self, uncompressed: &[u8]) -> Result<Vec<u8>> {
        let point =
            PublicKey::<k256::Secp256k1>::from_sec1_bytes(uncompressed).map_err(Error::ECDSA)?;
        Ok(point.to_encoded_point(true).as_bytes().to_vec())
    }
    fn decompress_pubkey(&self, compressed: &[u8]) -> Result<Vec<u8>> {
        let point =
            PublicKey::<k256::Secp256k1>::from_sec1_bytes(compressed).map_err(Error::ECDSA)?;
        Ok(point.to_encoded_point(false).as_bytes().to_vec())
    }
    fn verify_signature(&self, did: &str, msg: &[u8], sig: &[u8]) -> Result<()> {
        let prefix = utils::extract_prefixed_bytes(utils::extract_multikey(did)?)?;
        VerifyingKey::from_sec1_bytes(&prefix[2..])
            .map_err(Error::Signature)?
            .verify(
                msg,
                &Signature::<k256::Secp256k1>::from_slice(sig).map_err(Error::Signature)?,
            )
            .map_err(Error::Signature)
    }
}
