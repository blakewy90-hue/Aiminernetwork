use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: i64,
    pub job_id: String,
    pub miner_address: String,

    // AI model metadata
    pub model: String,
    pub input: String,
    pub thinking: String,
    pub output: String,

    // Economics
    pub cost: u64,
    pub bounty: u64,
    pub total_reward: u64,

    // Chain linkage
    pub previous_hash: String,
    pub block_hash: String,
}
