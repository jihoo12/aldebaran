mod block;
mod blockchain;
mod pos;
mod transaction;
mod validator;

use blockchain::Blockchain;

pub fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

struct SimpleRng(u64);

impl SimpleRng {
    fn seed_from_u64(seed: u64) -> Self {
        SimpleRng(seed)
    }

    fn gen_range(&mut self, range: std::ops::Range<u64>) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        range.start + self.0 % (range.end - range.start)
    }
}

fn main() {
    println!("=== Proof of Stake Blockchain Demo ===\n");

    let mut bc = Blockchain::new();
    bc.print_state();

    for round in 1..=5 {
        println!("--- Round {} ---", round);

        bc.create_transaction("alice", "bob", 10 + round);
        bc.create_transaction("bob", "carol", 5);
        bc.create_transaction("carol", "alice", 3);

        bc.propose_block(round as u64);
        println!();
    }

    bc.print_state();
}
