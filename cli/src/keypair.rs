use openssl::rsa::Rsa;
use file;

pub fn generate_key_pair(filename: String, dir: &String) {
  let privpub = Rsa::generate(4096).unwrap();
  
  let public = String::from_utf8(privpub.public_key_to_pem().unwrap());
  let private = String::from_utf8(privpub.private_key_to_pem().unwrap());

  file::write_file(format!("{}/{}.pub", dir, filename), public.unwrap());
  file::write_file(format!("{}/{}.priv", dir, filename), private.unwrap());
}