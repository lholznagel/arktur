extern crate openssl;

use openssl::rsa::Rsa;

fn main() {
    let pripub = Rsa::generate(4096).unwrap();
    println!("{:?}", String::from_utf8(pripub.public_key_to_pem().unwrap()));
    println!("{:?}", String::from_utf8(pripub.private_key_to_pem().unwrap()));
}
