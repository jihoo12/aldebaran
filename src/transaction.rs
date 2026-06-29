use sha2::{Sha256, Digest};
use crate::hex_encode;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: u64,
    pub hash: String,
}

impl Transaction {
    pub fn new(sender: &str, receiver: &str, amount: u64) -> Self {
        let content = format!("{}{}{}", sender, receiver, amount);
        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        let hash = hex_encode(&hasher.finalize());
        Transaction {
            sender: sender.to_string(),
            receiver: receiver.to_string(),
            amount,
            hash,
        }
    }
}
