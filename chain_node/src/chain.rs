use crate::{accounts::Accounts, block::Block, storage};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::VecDeque;

// 1 full network token = 100,000,000 base units (8 decimal places)
// 1 base unit = 0.00000001 network tokens
pub const DECIMALS: u64 = 100_000_000;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum JobStatus {
    Pending,
    Processing,
    Completed,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Job {
    pub id: String,
    pub prompt: String,
    pub status: JobStatus,
}

pub struct Chain {
    pub blocks: Vec<Block>,
    pub accounts: Accounts,
    pub job_queue: VecDeque<Job>,
}

impl Chain {
    pub fn new() -> Self {
        if let Some(chain_file) = storage::load_chain() {
            println!("💾 Loaded existing state from chain_state.json");
            Self {
                blocks: chain_file.blocks,
                accounts: chain_file.accounts,
                job_queue: VecDeque::new(),
            }
        } else {
            let genesis = Self::genesis_block();
            Self {
                blocks: vec![genesis],
                accounts: Accounts::new(),
                job_queue: VecDeque::new(),
            }
        }
    }

    fn genesis_block() -> Block {
        let timestamp = Utc::now().timestamp();
        let mut hasher = Sha256::new();
        hasher.update(b"genesis");
        let hash = format!("{:x}", hasher.finalize());

        Block {
            index: 0,
            timestamp,
            job_id: "genesis".into(),
            miner_address: "system".into(),
            model: "none".into(),
            input: "Genesis block initialized".into(),
            thinking: "".into(),
            output: "".into(),
            cost: 0,
            bounty: 0,
            total_reward: 0,
            previous_hash: String::new(),
            block_hash: hash,
        }
    }

    pub fn last_hash(&self) -> String {
        self.blocks.last().unwrap().block_hash.clone()
    }

    pub fn next_index(&self) -> u64 {
        self.blocks.len() as u64
    }

    pub fn add_job(&mut self, prompt: String) -> Job {
        let job = Job {
            id: format!("job-{}", Utc::now().timestamp_nanos_opt().unwrap_or(0)),
            prompt,
            status: JobStatus::Pending,
        };
        self.job_queue.push_back(job.clone());
        job
    }

    pub fn fetch_next_job(&mut self) -> Option<Job> {
        if let Some(job) = self.job_queue.iter_mut().find(|j| j.status == JobStatus::Pending) {
            job.status = JobStatus::Processing;
            return Some(job.clone());
        }
        None
    }

    pub fn create_block(
        &mut self,
        job_id: String,
        miner_address: String,
        model: String,
        thinking: String,
        output: String,
        cost: u64,
    ) -> Block {
        // Protocol Economics: Compute cost + fixed 1,000 unit data bounty
        let bounty: u64 = 1000; 
        let total_reward = cost + bounty;
        
        self.accounts.credit(&miner_address, total_reward);

        // Retrieve the original prompt (input) from the job queue
        let mut input = String::from("unknown prompt");
        if let Some(job) = self.job_queue.iter_mut().find(|j| j.id == job_id) {
            job.status = JobStatus::Completed;
            input = job.prompt.clone();
        }

        let index = self.blocks.len() as u64;
        let timestamp = Utc::now().timestamp();
        let previous_hash = self.last_hash();

        // Cryptographically hash all data fields
        let mut hasher = Sha256::new();
        hasher.update(index.to_be_bytes());
        hasher.update(timestamp.to_be_bytes());
        hasher.update(job_id.as_bytes());
        hasher.update(miner_address.as_bytes());
        hasher.update(model.as_bytes());
        hasher.update(input.as_bytes());
        hasher.update(thinking.as_bytes());
        hasher.update(output.as_bytes());
        hasher.update(cost.to_be_bytes());
        hasher.update(bounty.to_be_bytes());
        hasher.update(total_reward.to_be_bytes());
        hasher.update(previous_hash.as_bytes());
        let block_hash = format!("{:x}", hasher.finalize());

        Block {
            index,
            timestamp,
            job_id,
            miner_address,
            model,
            input,
            thinking,
            output,
            cost,
            bounty,
            total_reward,
            previous_hash,
            block_hash,
        }
    }
}
