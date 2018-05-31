use sodiumoxide::crypto::box_;
use sodiumoxide::crypto::box_::curve25519xsalsa20poly1305::{Nonce, SecretKey};

/// Struct that holds the given secret key and the current nonce
///
/// Used to encrypt the message
/// The secret key can only be obtained by this crate
///
/// When creating a new instance of this struct, a random nonce
/// is generated
/// When a new message is constructed the nonce incremented and returned.
/// That way there is a different nonce for each message and two peers
/// won´t start with the same nonce
/// 
/// ```
/// extern crate carina_core_protocol;
/// extern crate sodiumoxide;
/// 
/// use carina_core_protocol::Nacl;
/// use sodiumoxide::crypto::box_;
/// 
/// fn main() {
///     let (_, secret_key) = box_::gen_keypair();
///     let mut nacl = Nacl::new(secret_key);
/// }
/// ```
#[derive(Clone, Debug)]
pub struct Nacl {
    secret_key: SecretKey,
    nonce: Nonce,
}

impl Nacl {
    /// Creates a new instance with the given secret key
    pub fn new(secret_key: SecretKey) -> Self {
        let nonce = box_::gen_nonce();
        Self { secret_key, nonce }
    }

    /// Increments the nonce and returns its new value
    pub fn get_nonce(&mut self) -> Nonce {
        self.nonce.increment_le()
    }

    /// Gets the secret key
    ///
    /// Only available for this crate
    pub(crate) fn get_secret_key(&self) -> SecretKey {
        self.secret_key.clone()
    }
}

impl Default for Nacl {
    fn default() -> Self {
        let (_, secret_key) = box_::gen_keypair();
        let nonce = box_::gen_nonce();

        Self { secret_key, nonce }
    }
}
