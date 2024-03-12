use crate::block::Block;
use crate::errors::Result;
#[derive(Debug)]
pub struct Blockchain{

    blocks: Vec<Block>,
    db: sled::Db,
}
impl Blockchain {
    pub fn new() -> Blockchain {
        Blockchain{
            blocks: vec![Block::new_genesis_block()]
        }
    }
    pub fn add_block(&mut self, data: String) -> Result<()> {
        let prev = self.blocks.last().unwrap();
        let new_block  = Block::new_block(data, prev.get_hash(), TARGET_HEX)?;
        self.blocks.push(new_block);
        Ok(())
}
}