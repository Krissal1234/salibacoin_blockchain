use crypto::{digest::Digest, sha2::Sha256};
use log::info;
use hex::encode;
use merkletree::merkle::MerkleTree;
use merkletree::store::VecStore;
use merkletree::hash::{Algorithm, Hashable};
use serde::{Deserialize, Serialize};
use std::{alloc::System, io::Take, time::SystemTime, vec};
use uuid::Uuid;
pub type Result<T> = std::result::Result<T, failure::Error>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub transaction_id: String,
    pub transaction_timestamp: u128,
    pub transaction_details: String,
}

impl Transaction {
    pub fn new(details: String) -> Self {
        let transaction_id = Uuid::new_v4().to_string(); // Generate a unique transaction ID
        let transaction_timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis();

        Transaction {
            transaction_id,
            transaction_timestamp,
            transaction_details: details,
        }
    }
    // pub fn hash(&self) -> Vec<u8> {
    //     let json = serde_json::to_string(self).unwrap();
    //     let mut hasher = Sha256::new();
    //     hasher.input_str(&json);
    //     let mut result = vec![0u8; 32];
    //     hasher.result(&mut result);
    //     result
    // }
    pub fn hash(&self) -> [u8; 32] {
        let json = serde_json::to_string(self).unwrap();
        let mut hasher = Sha256::new();
        hasher.input_str(&json);
        let mut result = [0u8; 32];
        hasher.result(&mut result);
        result
    }

}
#[derive(Debug, Clone)]
pub struct Block {
    block_number: u64,
    block_timestamp: u128,
    transaction_list: Vec<Transaction>,
    merkle_tree_root: String,
    prev_block_hash: String,
    hash: String,
    height: usize, //the number of blocks that were confirmed the entire history of the blockchain network
    nonce: u64, // useulf or mining, when we want to mine the block we need to change the nonce to shuffle the hash to find the current hash becuase
}

const TARGET_HEX: usize = 4; //difficulty targket for mining,

impl Block {
    pub fn get_hash(&self) -> String {
        self.hash.clone()
    }

    pub fn new_genesis_block() -> Block {
        let first_transaction = Transaction::new(String::from("First Transaction"));
        Block::new_block(vec![first_transaction], 0, String::new(), 0).unwrap()
    }

    // fn get_merkle_tree_root(transactions: &Vec<Transaction>) -> Result<String> {
    //     let hashed_transactions: Vec<[u8; 32]> = transactions.iter().map(|tx| tx.hash()).collect();
    //     let merkle_tree = MerkleTree::new(hashed_transactions).unwrap();
    //     let root = merkle_tree.root();
    //     Ok(encode(root))
    // }

    pub fn new_block(
        transactions: Vec<Transaction>,
        prev_block_number: u64,
        prev_block_hash: String,
        height: usize,
    ) -> Result<Block> {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_millis();
        //write merkle tree implementation
        let merkle_root = String::from("Unimplemented");//Block::get_merkle_tree_root(transactions).unwrap();

        let mut block = Block {
            block_number: prev_block_number + 1,
            block_timestamp: timestamp,
            merkle_tree_root: merkle_root,
            transaction_list: transactions,
            prev_block_hash,
            hash: String::new(), //hash is filled when we mine a block
            height,
            nonce: 0, // nonce is 0 so when we mine a block, nonce will be changed
        };
        block.run_proof_of_work()?;
        Ok(block)
    }

    fn run_proof_of_work(&mut self) -> Result<()> {
        //Before completing the PoW, you need to first validate the hash, if it is not validated you increment the nonce
        info!("Mining the block");
        while !self.validate()? {
            self.nonce += 1;
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
            self.transaction_list.clone(),
            self.block_timestamp,
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

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_blockchain(){
//         let mut b = Blockchain::new();
//         b.add_block("data".to_string());
//         b.add_block("data2".to_string());
//         b.add_block("data3".to_string());
//         dbg!(b); // debug to print
//     }
// }
