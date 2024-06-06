mod blockchain;
mod block;
mod errors;
mod merkle_tree;
mod transaction;
use blockchain::Blockchain;
use transaction::Transaction;
use crate::block::Block;
fn main() {
    let mut salibacoin_blockchain = Blockchain::new();


    let transactions = vec![Transaction::new(String::from("New Transaction")), Transaction::new(String::from("Another Transaction"))];

    salibacoin_blockchain.add_block(&transactions).unwrap();
    salibacoin_blockchain.add_block(&transactions).unwrap();
    salibacoin_blockchain.add_block(&transactions).unwrap();
    salibacoin_blockchain.print_blockchain();
}