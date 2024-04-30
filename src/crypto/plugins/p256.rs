use super::super::{consts::JwtAlg, error::Result, DidKeyPlugin};

pub struct P256Plugin;

impl DidKeyPlugin for P256Plugin {
    fn jwt_alg(&self) -> JwtAlg {
        JwtAlg::P256
    }
    fn compress_pubkey(&self, uncompressed: &[u8]) -> Result<Vec<u8>> {
        unimplemented!()
    }
    fn decompress_pubkey(&self, compressed: &[u8]) -> Result<Vec<u8>> {
        unimplemented!()
    }
    fn verify_signature(&self, did: &str, msg: &[u8], sig: &[u8]) -> Result<()> {
        unimplemented!()
    }
}
