extern crate blockchain_lib;

use blockchain_lib::{Block, Blockchain};

fn main() {
    let mut empty_prev = String::from("");
    for _ in 0..64 {
        empty_prev.push_str("0");
    }

    let first = Block::new(String::from("Some content"), empty_prev);
    let second = Block::new(String::from("Even more conent"), first.hash.clone());
    let third = Block::new(String::from("Nothing"), second.hash.clone());

    let blockchain = Blockchain::new(String::from("abcdef"))
        .add_block(first)
        .add_block(second)
        .add_block(third);

    println!("{:?}", blockchain);
}