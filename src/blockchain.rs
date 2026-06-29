use crate::block::Block;
use crate::transaction::Transaction;
use crate::pos::ProofOfStake;

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub pending_transactions: Vec<Transaction>,
    pub pos: ProofOfStake,
    pub block_reward: u64,
}

impl Blockchain {
    pub fn new() -> Self {
        let mut pos = ProofOfStake::new();
        pos.register_validator("alice", 100);
        pos.register_validator("bob", 50);
        pos.register_validator("carol", 30);

        let genesis = Block::genesis();
        Blockchain {
            chain: vec![genesis],
            pending_transactions: Vec::new(),
            pos,
            block_reward: 10,
        }
    }

    pub fn create_transaction(&mut self, sender: &str, receiver: &str, amount: u64) {
        let tx = Transaction::new(sender, receiver, amount);
        println!("  TX created: {} -> {} ({} units) [{}]", sender, receiver, amount, &tx.hash[..8]);
        self.pending_transactions.push(tx);
    }

    pub fn propose_block(&mut self, seed: u64) {
        if self.pending_transactions.is_empty() {
            println!("  No pending transactions to include.");
            return;
        }

        let proposer = match self.pos.select_proposer(seed) {
            Some(v) => v.address.clone(),
            None => {
                println!("  No validators available!");
                return;
            }
        };

        let last_block = self.chain.last().unwrap();
        let new_index = last_block.index + 1;

        let txs = self.pending_transactions.drain(..).collect();
        let block = Block::new(new_index, txs, &last_block.hash, &proposer);

        if block.is_valid(&last_block.hash) {
            println!(
                "  Block #{} proposed by {} (hash: {})",
                block.index,
                block.proposer,
                &block.hash[..12]
            );
            self.chain.push(block);
            self.pos.reward_validator(&proposer, self.block_reward);
            println!("  {} rewarded with {} tokens!", proposer, self.block_reward);
        } else {
            println!("  Invalid block! Rejected.");
            self.pending_transactions.clear();
        }
    }

    pub fn is_chain_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];
            if !current.is_valid(&previous.hash) {
                println!("  Block #{} is INVALID!", current.index);
                return false;
            }
        }
        true
    }

    pub fn print_state(&self) {
        println!("\n=== BLOCKCHAIN STATE ===");
        println!("Chain length: {}", self.chain.len());
        println!("Valid: {}", self.is_chain_valid());
        println!("\n--- Validators ---");
        println!("{}", self.pos.get_validator_info());
        println!("\n--- Blocks ---");
        for block in &self.chain {
            fn short(s: &str) -> &str {
                if s.len() > 12 { &s[..12] } else { s }
            }
            println!(
                "  Block #{} | proposer: {} | hash: {} | prev: {} | txs: {}",
                block.index,
                block.proposer,
                short(&block.hash),
                short(&block.previous_hash),
                block.transactions.len(),
            );
        }
        println!("===================\n");
    }
}
