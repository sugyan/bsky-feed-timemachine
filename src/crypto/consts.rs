pub const BASE58_MULTIBASE_PREFIX: &str = "z";
pub const DID_KEY_PREFIX: &str = "did:key:";

#[derive(Debug)]
pub enum JwtAlg {
    P256,
    Secp256k1,
}
