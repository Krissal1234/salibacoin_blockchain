use crate::transaction::Transaction;
use crate::merkle_tree::{TestItem, TestSha256Hasher};
use hex::encode;
use merkletree::merkle::{Element as _, MerkleTree};
use merkletree::store::VecStore;
use std::time::SystemTime;
use std::result::Result;
use failure::Error;
use crypto::digest::Digest;
use crypto::sha2::Sha256; // Add this line to import Sha256 correctly

#[derive(Debug, Clone)]
pub struct Block {
    block_id: u64,
    block_timestamp: u128,
    transaction_list: Vec<Transaction>,
    merkle_tree_root: String,
    main_tree: bool,
    prev_block_hash: String,
    hash: String,
    height: usize,
    nonce: u64,
}

//To determine difficult, defines the number of leading zeros
const TARGET_HEX: usize = 4;

impl Block {
    pub fn get_hash(&self) -> String {
        self.hash.clone()
    }
    pub fn get_block_id(&self) -> u64 {
        self.block_id.clone()
    }

    pub fn new_genesis_block() -> Block {
        let first_transaction = Transaction::new(String::from("First Transaction"));
        let second_transaction = Transaction::new(String::from("Second Transaction"));
        let transactions =vec![first_transaction, second_transaction];
        Block::new_block(&transactions, 0, String::new(), 1).unwrap()
    }

    fn get_merkle_tree_root(transactions: &[Transaction]) -> Result<String, Error> {
        let hashed_transactions: Vec<TestItem> = transactions.iter().map(|tx| {
            let hash = tx.hash();
            TestItem::from_slice(&hash)
        }).collect();

        let merkle_tree: MerkleTree<TestItem, TestSha256Hasher, VecStore<TestItem>> =
            MerkleTree::new(hashed_transactions).expect("Failed to create Merkle Tree");

        let root = merkle_tree.root();
        Ok(encode(root.as_ref()))
    }

    pub fn new_block(
        transactions: &[Transaction],
        prev_block_id: u64,
        prev_block_hash: String,
        height: usize,
    ) -> Result<Block, Error> {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_millis();
        let merkle_root = Block::get_merkle_tree_root(transactions)?;

        let mut block = Block {
            block_id: prev_block_id+ 1,
            block_timestamp: timestamp,
            merkle_tree_root: merkle_root,
            transaction_list: transactions.to_vec(),
            main_tree: true, //this will have to be used when a fork happens, when two miners upload at the same time
            prev_block_hash,
            hash: String::new(),
            height,
            nonce: 0,
        };
        block.run_proof_of_work()?;
        Ok(block)
    }

    fn run_proof_of_work(&mut self) -> Result<(), Error> {
        println!("Mining the block");
        while !self.validate()? {
            self.nonce += 1;
        }
        let data = self.prepare_hash_data()?;
        let mut hasher = Sha256::new();
        hasher.input(&data[..]);
        self.hash = hasher.result_str();
        Ok(())
    }

    fn prepare_hash_data(&self) -> Result<Vec<u8>, Error> {
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

    fn validate(&self) -> Result<bool, Error> {
        let data = self.prepare_hash_data()?;
        let mut hasher = Sha256::new();
        hasher.input(&data[..]);
        let mut vec1: Vec<u8> = vec![];
        vec1.resize(TARGET_HEX, '0' as u8);
        Ok(&hasher.result_str()[0..TARGET_HEX] == String::from_utf8(vec1)?)
    }
}
