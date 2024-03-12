use std::{alloc::System, io::Take, time::SystemTime, vec};

use crypto::{digest::Digest, sha2::Sha256};
use log::info;
pub type Result<T> = std::result::Result<T, failure::Error>;

#[derive(Debug,Clone)]
pub struct Block {
timestamp:u128,
transactions: String,
prev_block_hash: String,
hash: String,
height:usize, //the number of blocks that were confirmed the entire history of the blockchain network
nonce: i32, // useulf or mining, when we want to mine the block we need to change the nonce to shuffle the hash to find the current hash becuase 
}

#[derive(Debug)]
pub struct Blockchain{
    blocks: Vec<Block>
}
const TARGET_HEX: usize = 4; //difficulty target for mining,

impl Block {
    pub fn get_hash(&self) -> String {
        self.hash.clone()
    }

    pub fn new_genesis_block() -> Block {
        Block::new_block(String::from("Genesis Block"), String::new(), 0).unwrap()
    }
    pub fn new_block(data:String, prev_block_hash: String, height:usize) -> Result<Block>{
        let timestamp = SystemTime::now().
        duration_since(SystemTime::UNIX_EPOCH)?.as_millis();

        let mut block = Block {
            timestamp: timestamp,
            transactions: data,
            prev_block_hash,
            hash: String::new(), //hash is filled when we mine a block
            height,
            nonce:0, // nonce is 0 so when we mine a block, nonce will be changed
        };
        block.run_proof_of_work()?;
        Ok(block)
    }

    fn run_proof_of_work(&mut self) ->Result<()> {
        //Before completing the PoW, you need to first validate the hash, if it is not validated you increment the nonce
        info!("Mining the block");
        while !self.validate()? {
            self.nonce +=1;
        }
        let data = self.prepare_hash_data()?;
        let mut hasher = Sha256::new();
        hasher.input(&data[..]);
        self.hash = hasher.result_str();
        Ok(())
    }

    fn prepare_hash_data(&self) -> Result<Vec<u8>> {
        let content = (
            self.prev_block_hash.clone(),
            self.transactions.clone(),
            self.timestamp,
            TARGET_HEX,
            self.nonce,
        );

        let bytes = bincode::serialize(&content)?;
        Ok(bytes)
    }

    //this is part of the proof of work mechanism, Target_HEX defines the difficulty of the hashing puzzle
    //A block is considered valid if its hash has a certain number of leading zeros
    //
    //The nonce is a variable that miners adjust to change the block's hash output The validate function checks if, 
    //for the current nonce value, the hash of the block has the required number of leading zeros. 
    //If not, the mining process (run_proof_of_work) will increment the nonce and try again.
    fn validate(&self) -> Result<bool> {
        let data = self.prepare_hash_data()?;
        let mut hasher = Sha256::new();

        hasher.input(&data[..]); //Vec<u8> to [u8] , it creates a slice that ocvers the entire range of the vector
        let mut vec1: Vec<u8> = vec![];

        vec1.resize(TARGET_HEX, '0' as u8);
        Ok(&hasher.result_str()[0..TARGET_HEX] == String::from_utf8(vec1)?) //compar
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blockchain(){
        let mut b = Blockchain::new();
        b.add_block("data".to_string());
        b.add_block("data2".to_string());
        b.add_block("data3".to_string());
        dbg!(b); // debug to print
    }
}