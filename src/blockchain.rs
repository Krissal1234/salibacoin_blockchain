use crate::block::Block;
use crate::errors::Result;
use sled;

#[derive(Debug)]
pub struct Blockchain{
    blocks: Vec<Block>,
    db: sled::Db,
}

const TARGET_HEX: usize = 4; //difficulty target for mining,

impl Blockchain {
    pub fn new() -> Blockchain {
        let block = Block::new_genesis_block();
        let new_db = sled::open("blockchain_db").unwrap();

        //add genisis block to db

        Blockchain{
            blocks: vec![block],
            db:new_db
        }
    }
    // pub fn add_block(&mut self, data: String) -> Result<()> {
    //     let prev = self.blocks.last().unwrap();
    //     let new_block  = Block::new_block(data, prev.get_hash(), TARGET_HEX)?;
    //     self.blocks.push(new_block);
    //     Ok(())
// }
}