use block::Block;

#[derive(Debug)]
pub struct Blockchain {
    blocks: Vec<Block>,
    signkey: String
}

impl Blockchain {
    pub fn new(signkey: String) -> Self {
        Blockchain {
            blocks: Vec::new(),
            signkey: signkey
        }
    }

    pub fn add_block(mut self, block: Block) -> Self {
        let block = block.set_index(self.blocks.len()).generate_hash(self.signkey.clone());
        self.blocks.push(block);
        self
    }
}