use failure::Error;
use nacl::Nacl;
use sodiumoxide::crypto::box_;
use sodiumoxide::crypto::box_::curve25519xsalsa20poly1305::{Nonce, PublicKey};

/// Tries to decrypt the given message
/// 
/// If successful it will return a new decrypted `Vec<u8>` without the nonce
pub fn decrypt(bytes: &[u8], nacl: &Nacl, public_key: &PublicKey) -> Result<Vec<u8>, Error> {
    let nonce = Nonce::from_slice(&bytes[0..24]).unwrap();
    match box_::open(&bytes[24..], &nonce, &public_key, &nacl.get_secret_key()) {
        Ok(val) => Ok(val),
        Err(_) => Err(format_err!("Error decrypting incoming message.")),
    }
}