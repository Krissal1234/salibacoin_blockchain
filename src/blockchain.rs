use crate::block::Block;
use crate::errors::Result;
use crate::transaction::Transaction;
use sled::{self, transaction};

#[derive(Debug)]
pub struct Blockchain {
    blocks: Vec<Block>,
    db: sled::Db,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        let block = Block::new_genesis_block();
        let new_db = sled::open("blockchain_db").unwrap();

        Blockchain {
            blocks: vec![block],
            db: new_db,
        }
    }
    pub fn add_block(&mut self, transactions: &[Transaction]) -> Result<()> {
        //getting last block
        let prev = self.blocks.last().unwrap();
        let new_block = Block::new_block(
            transactions,
            prev.get_block_id(),
            prev.get_hash(),
            self.blocks.len(),
        )?;
        //At this point we have to somehow check if someon has beaten us I assume
        self.blocks.push(new_block);
        //reward SalibaCoin
        Ok(())
    }
    pub fn print_blockchain(&self){
        for block in &self.blocks {
            println!("{:#?}", block);
        }
    }
}
