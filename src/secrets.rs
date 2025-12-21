use crate::errors::error_impl;
use hmac::{Hmac, Mac};
use rand::RngCore;
use sha2::Sha256;

pub struct SecretManager;

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug)]
pub struct SecretDoesNotMatchTarget;

error_impl!(SecretDoesNotMatchTarget);

// TODO: use argon2 instad of HMAC + SHA256
// TODO: store hashed password as VARCHAR and use base64url

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
        secret_to_verify: &str,
        salt: &[u8],
        key: &[u8],
        target_secret_hash: &[u8],
    ) -> Result<(), SecretDoesNotMatchTarget> {
        Hmac::<Sha256>::new_from_slice(key)
            .expect("HMAC can take a key of any size")
            .chain_update(salt)
            .chain_update(secret_to_verify)
            .verify_slice(target_secret_hash)
            .map_err(|_| SecretDoesNotMatchTarget)
    }
}

#[cfg(test)]
mod tests {
    use crate::secrets::{SecretDoesNotMatchTarget, SecretManager};
    use rand::{
        RngCore,
        distr::{Alphanumeric, SampleString},
    };

    #[test]
    fn test_generate_salt_ok_buffer_filled() {
        let mut salt = [0; 32];
        SecretManager::generate_salt(&mut salt);
        assert_ne!(salt, [0; 32]);
    }

    #[test]
    fn test_generate_salt_ok_salt_is_random() {
        let mut salt1 = [0; 32];
        let mut salt2 = [0; 32];
        SecretManager::generate_salt(&mut salt1);
        SecretManager::generate_salt(&mut salt2);
        assert_ne!(salt1, salt2);
    }

    #[test]
    fn test_hash_secret_ok() {
        let plain_secret = "pass";
        let salt = b"salt";
        let key = b"key";
        let expected_hashed_secret = [
            218, 100, 229, 207, 122, 128, 254, 253, 76, 39, 3, 166, 163, 167, 41, 228, 246, 64,
            246, 255, 5, 179, 153, 161, 182, 179, 224, 243, 123, 218, 67, 226,
        ];

        let hashed_secret = SecretManager::hash_secret(plain_secret, salt, key);
        assert_eq!(hashed_secret, expected_hashed_secret);
    }

    #[test]
    fn test_verify_secret_ok() {
        let plain_secret = Alphanumeric.sample_string(&mut rand::rng(), 16);

        let mut key = [0; 10];
        rand::rng().fill_bytes(&mut key);

        let mut salt = [0; 10];
        rand::rng().fill_bytes(&mut salt);

        let hash = SecretManager::hash_secret(&plain_secret, &salt, &key);

        let result = SecretManager::verify_secret(&plain_secret, &salt, &key, &hash);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_verify_secret_err_secret_does_not_match_target() {
        let plain_secret = Alphanumeric.sample_string(&mut rand::rng(), 16);

        let mut key = [0; 10];
        rand::rng().fill_bytes(&mut key);

        let mut salt = [0; 10];
        rand::rng().fill_bytes(&mut salt);

        let mut hash = [0; 10];
        rand::rng().fill_bytes(&mut hash);

        let result = SecretManager::verify_secret(&plain_secret, &salt, &key, &hash);
        assert_eq!(result, Err(SecretDoesNotMatchTarget));
    }
}
