pub mod consts;
pub mod did;
pub mod error;
pub mod multibase;
pub mod plugins;
pub mod utils;
pub mod verify;

use self::{consts::JwtAlg, error::Result};

pub trait DidKeyPlugin {
    fn jwt_alg(&self) -> JwtAlg;
    fn compress_pubkey(&self, uncompressed: &[u8]) -> Result<Vec<u8>>;
    fn decompress_pubkey(&self, compressed: &[u8]) -> Result<Vec<u8>>;
    fn verify_signature(&self, did: &str, msg: &[u8], sig: &[u8]) -> Result<()>;
}
