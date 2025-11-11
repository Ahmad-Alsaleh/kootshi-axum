use hmac::{Hmac, Mac};
use rand::RngCore;
use sha2::Sha256;

pub struct SecretManager;
pub struct SecretDoesNotMatchTarget;

// TODO: use argon2 instad of HMAC + SHA256

impl SecretManager {
    pub fn generate_salt(salt: &mut [u8; 32]) {
        rand::rng().fill_bytes(salt);
    }

    pub fn hash_secret(plain_secret: &str, salt: &[u8], key: &[u8]) -> Vec<u8> {
        Hmac::<Sha256>::new_from_slice(key)
            .expect("HMAC can take a key of any size")
            .chain_update(salt)
            .chain_update(plain_secret)
            .finalize()
            .into_bytes()
            .to_vec()
    }

    pub fn verify_secret(
        plain_secret: String,
        salt: String,
        key: &[u8],
        hashed_target: &[u8],
    ) -> Result<(), SecretDoesNotMatchTarget> {
        Hmac::<Sha256>::new_from_slice(key)
            .expect("HMAC can take a key of any size")
            .chain_update(salt)
            .chain_update(plain_secret)
            .verify_slice(hashed_target)
            .map_err(|_| SecretDoesNotMatchTarget)
    }
}

// TODO: implement tests
