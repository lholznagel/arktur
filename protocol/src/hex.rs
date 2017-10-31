// Most of the code is taken from https://github.com/rust-lang-deprecated/rustc-serialize/blob/master/src/hex.rs
//
// Copyright 2013-2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this (being https://github.com/rust-lang-deprecated/rustc-serialize)
// distribution and at http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Helper for converting a hex string to byte vector

/// A trait for converting hexadecimal encoded values
pub trait FromHex {
    /// Converts the value of `self`, interpreted as hexadecimal encoded data,
    /// into an owned vector of bytes, returning the vector.
    fn from_hex(&self) -> Vec<u8>;
}

impl FromHex for str {
    /// Converts a byte array to a hex string
    ///
    /// # Return
    ///
    /// Vector containing the hex values to numbers
    ///
    /// # Example
    /// ```
    /// use blockchain_protocol::hex::FromHex;
    ///
    /// let content = "0102FFFF05390000";
    /// assert_eq!(content.from_hex(), &[1, 2, 255, 255, 5, 57, 0, 0]);
    /// ```
    fn from_hex(&self) -> Vec<u8> {
        let mut b = Vec::with_capacity(self.len() / 2);
        let mut modulus = 0;
        let mut buf = 0;

        for (idx, byte) in self.bytes().enumerate() {
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
                    let ch = self[idx..].chars().next().unwrap();
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
}