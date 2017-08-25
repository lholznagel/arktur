extern crate crypto;
extern crate time;

mod block;

use block::Block;

fn main() {
    let mut blocks = Vec::<Block>::new();

    let mut empty_prev = String::from("");
    for _ in 0..64 {
        empty_prev.push_str("0");
    }

    let first = Block::new(1, String::from("Some content"), empty_prev);
    let first = first.generate_hash();

    blocks.push(first);
    println!("{:?}", blocks);
}