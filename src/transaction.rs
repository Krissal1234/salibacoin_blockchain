use crypto::{digest::Digest, sha2::Sha256};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use uuid::Uuid;

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

    pub fn hash(&self) -> [u8; 32] {
        let json = serde_json::to_string(self).unwrap();
        let mut hasher = Sha256::new();
        hasher.input_str(&json);
        let mut result = [0u8; 32];
        hasher.result(&mut result);

        result
    }
}
