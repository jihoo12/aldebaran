use sha2::{Sha256, Digest};
use crate::transaction::Transaction;
use crate::hex_encode;

#[derive(Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: u128,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub hash: String,
    pub proposer: String,
    pub signature: String,
}

impl Block {
    pub fn new(
        index: u64,
        transactions: Vec<Transaction>,
        previous_hash: &str,
        proposer: &str,
    ) -> Self {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();

        let mut block = Block {
            index,
            timestamp,
            transactions,
            previous_hash: previous_hash.to_string(),
            hash: String::new(),
            proposer: proposer.to_string(),
            signature: String::new(),
        };

        block.hash = block.calculate_hash();
        block.signature = block.sign_block();
        block
    }

    pub fn genesis() -> Self {
        let tx = Transaction::new("genesis", "genesis", 0);
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();

        let mut block = Block {
            index: 0,
            timestamp,
            transactions: vec![tx],
            previous_hash: String::from("0"),
            hash: String::new(),
            proposer: String::from("genesis"),
            signature: String::new(),
        };

        block.hash = block.calculate_hash();
        block.signature = block.sign_block();
        block
    }

    fn calculate_hash(&self) -> String {
        let tx_data: String = self
            .transactions
            .iter()
            .map(|tx| &tx.hash)
            .cloned()
            .collect::<Vec<_>>()
            .join("");

        let content = format!(
            "{}{}{}{}{}",
            self.index, self.timestamp, tx_data, self.previous_hash, self.proposer
        );

        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        hex_encode(&hasher.finalize())
    }

    fn sign_block(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(format!("{}{}", self.hash, self.proposer).as_bytes());
        hex_encode(&hasher.finalize())
    }

    pub fn is_valid(&self, previous_hash: &str) -> bool {
        if self.previous_hash != previous_hash {
            return false;
        }
        if self.hash != self.calculate_hash() {
            return false;
        }
        true
    }
}
