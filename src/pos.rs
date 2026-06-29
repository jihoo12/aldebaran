use crate::validator::Validator;

pub struct ProofOfStake {
    pub validators: Vec<Validator>,
    pub total_stake: u64,
}

impl ProofOfStake {
    pub fn new() -> Self {
        ProofOfStake {
            validators: Vec::new(),
            total_stake: 0,
        }
    }

    pub fn register_validator(&mut self, address: &str, stake: u64) {
        if stake == 0 {
            return;
        }
        self.validators.push(Validator::new(address, stake));
        self.total_stake += stake;
    }

    pub fn select_proposer(&self, seed: u64) -> Option<&Validator> {
        if self.validators.is_empty() || self.total_stake == 0 {
            return None;
        }

        let mut rng = crate::SimpleRng::seed_from_u64(seed);
        let mut roll = rng.gen_range(0..self.total_stake);

        for validator in &self.validators {
            if roll < validator.stake {
                return Some(validator);
            }
            roll -= validator.stake;
        }

        self.validators.last()
    }

    pub fn reward_validator(&mut self, address: &str, reward: u64) {
        if let Some(v) = self.validators.iter_mut().find(|v| v.address == address) {
            v.rewards += reward;
            v.stake += reward;
            self.total_stake += reward;
        }
    }

    pub fn get_validator_info(&self) -> String {
        self.validators
            .iter()
            .map(|v| {
                let pct = if self.total_stake > 0 {
                    (v.stake as f64 / self.total_stake as f64) * 100.0
                } else {
                    0.0
                };
                format!(
                    "  {} | stake: {} ({:.1}%) | rewards: {}",
                    v.address, v.stake, pct, v.rewards
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}
