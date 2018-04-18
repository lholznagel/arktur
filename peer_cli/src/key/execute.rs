use base64::{decode, encode};
use clap::ArgMatches;

use sodiumoxide::crypto::box_;
use sodiumoxide::crypto::box_::curve25519xsalsa20poly1305::SecretKey;

pub fn genkey(_: &ArgMatches) {
    let (_, secret_key) = box_::gen_keypair();
    println!("{}", encode(&secret_key.0));
}

pub fn pubkey(arg: &ArgMatches) {
    let decoded = decode(arg.value_of("secret key").unwrap()).unwrap();
    let secret_key = SecretKey::from_slice(&decoded).unwrap();
    let public_key = secret_key.public_key();
    println!("{}", encode(&public_key.0));
}