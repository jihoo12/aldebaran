#[derive(Debug, Clone)]
pub struct Validator {
    pub address: String,
    pub stake: u64,
    pub rewards: u64,
}

impl Validator {
    pub fn new(address: &str, stake: u64) -> Self {
        Validator {
            address: address.to_string(),
            stake,
            rewards: 0,
        }
    }
}
