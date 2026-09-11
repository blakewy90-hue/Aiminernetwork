use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Block {
    pub index: u64,
        pub timestamp: i64,
            pub job_id: String,
                pub miner_address: String,
                    pub tokens_used: u64,
                        pub reward_units: u64,
                            pub previous_hash: String,
                                pub block_hash: String,
                                }