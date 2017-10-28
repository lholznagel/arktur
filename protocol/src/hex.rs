const CHARS: &'static [u8] = b"0123456789ABCDEF";

pub fn to_hex(bytes: &[u8]) -> String {
    let mut v = Vec::with_capacity(bytes.len() * 2);

    for &byte in bytes.iter() {
        v.push(CHARS[(byte >> 4) as usize]);
        v.push(CHARS[(byte & 0xf) as usize]);
    }

    unsafe {
        String::from_utf8_unchecked(v)
    }
}

pub fn from_hex(content: &str) -> Vec<u8> {
    let mut b = Vec::with_capacity(content.len() / 2);
    let mut modulus = 0;
    let mut buf = 0;

    for (idx, byte) in content.bytes().enumerate() {
        buf <<= 4;

        match byte {
            b'A'...b'F' => buf |= byte - b'A' + 10,
            b'a'...b'f' => buf |= byte - b'a' + 10,
            b'0'...b'9' => buf |= byte - b'0',
            b' ' | b'\r' | b'\n' | b'\t' => {
                buf >>= 4;
                continue;
            }
            _ => {
                let ch = content[idx..].chars().next().unwrap();
                //return Err(InvalidHexCharacter(ch, idx));
                println!("Give me error handling, from_hex {:?}", ch);
            }
        }

        modulus += 1;
        if modulus == 2 {
            modulus = 0;
            b.push(buf);
        }
    }

    b.into_iter().collect()
}