extern crate crypto;
extern crate time;

mod block;
mod blockchain;

use block::Block;
use blockchain::Blockchain;

fn main() {
    let mut empty_prev = String::from("");
    for _ in 0..64 {
        empty_prev.push_str("0");
    }

    let first = Block::new(String::from("Some content"), empty_prev);
    let second = Block::new(String::from("Even more conent"), first.hash.clone());
    let third = Block::new(String::from("Nothing"), second.hash.clone());

    let blockchain = Blockchain::new(String::from("abcd"))
        .add_block(first)
        .add_block(second)
        .add_block(third);

    println!("{:?}", blockchain);
}