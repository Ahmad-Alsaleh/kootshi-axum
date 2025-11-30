use hmac::{Hmac, Mac};
use rand::RngCore;
use sha2::Sha256;

fn main() {
    let plain_secret = std::env::args().nth(1).expect("please pass plain_secret");
    let key = std::env::var("PASSWORD_KEY").unwrap().into_bytes();

    let mut salt = [0; 32];
    generate_salt(&mut salt);
    let secret = hash_secret(&plain_secret, &salt, &key);

    println!("salt:");
    print_bytes_as_hex(&salt);

    println!("\nsecret:");
    print_bytes_as_hex(&secret);
}

fn generate_salt(salt: &mut [u8; 32]) {
    rand::rng().fill_bytes(salt);
}

fn hash_secret(plain_secret: &str, salt: &[u8], key: &[u8]) -> Vec<u8> {
    Hmac::<Sha256>::new_from_slice(key)
        .expect("HMAC can take a key of any size")
        .chain_update(salt)
        .chain_update(plain_secret)
        .finalize()
        .into_bytes()
        .to_vec()
}

fn print_bytes_as_hex(bytes: &[u8]) {
    print!("\\x");
    for byte in bytes {
        print!("{byte:02x}");
    }
    println!();
}
