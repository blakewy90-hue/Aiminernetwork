use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

use crate::{
    chain::{Chain, Job, DECIMALS},
    storage,
};

#[derive(Deserialize)]
pub struct Receipt {
    pub job_id: String,
    pub miner_address: String,
    pub model: String,
    pub thinking: String,
    pub output: String,
    pub cost: u64,
}

#[derive(Deserialize)]
pub struct CreateJobRequest {
    pub prompt: String,
}

#[derive(Serialize)]
pub struct BalanceResponse {
    pub address: String,
    pub raw_base_units: u64,
    pub formatted_balance: String,
}

pub type SharedChain = Arc<Mutex<Chain>>;

pub fn router(chain: SharedChain) -> Router {
    Router::new()
        .route("/submit_receipt", post(submit_receipt))
        .route("/blocks", get(get_blocks))
        .route("/balance/:address", get(get_balance))
        .route("/job_count", get(get_job_count))
        .route("/jobs", post(create_job))
        .route("/jobs/poll", get(poll_job))
        .with_state(chain)
}

async fn create_job(
    State(chain): State<SharedChain>,
    Json(payload): Json<CreateJobRequest>,
) -> Json<Job> {
    let mut chain = chain.lock().unwrap();
    let job = chain.add_job(payload.prompt);

    println!("➕ [JOB QUEUED] ID: {} | Prompt: \"{}\"", job.id, job.prompt);

    Json(job)
}

async fn poll_job(
    State(chain): State<SharedChain>,
) -> Json<Option<Job>> {
    let mut chain = chain.lock().unwrap();
    let job = chain.fetch_next_job();

    if let Some(ref j) = job {
        println!("🔄 [JOB ASSIGNED] Dispatching {} to miner", j.id);
    }

    Json(job)
}

async fn submit_receipt(
    State(chain): State<SharedChain>,
    Json(receipt): Json<Receipt>,
) -> &'static str {
    let mut chain = chain.lock().unwrap();

    println!(
        "📥 [RECEIPT] Job: {} | Miner: {} | Model: {}",
        receipt.job_id, receipt.miner_address, receipt.model
    );

    let block = chain.create_block(
        receipt.job_id,
        receipt.miner_address,
        receipt.model,
        receipt.thinking,
        receipt.output,
        receipt.cost,
    );

    println!(
        "📦 [BLOCK MINED] Block #{} | Cost: {} | Bounty: {} | Total Payout: {} base units | Hash: {}...",
        block.index,
        block.cost,
        block.bounty,
        block.total_reward,
        &block.block_hash[..10]
    );

    chain.blocks.push(block);
    storage::save_chain(&chain.blocks, &chain.accounts);

    "ok"
}

async fn get_blocks(
    State(chain): State<SharedChain>,
) -> Json<Vec<crate::block::Block>> {
    let chain = chain.lock().unwrap();
    Json(chain.blocks.clone())
}

async fn get_balance(
    Path(address): Path<String>,
    State(chain): State<SharedChain>,
) -> Json<BalanceResponse> {
    let chain = chain.lock().unwrap();
    let raw_bal = chain.accounts.get_balance(&address);
    let formatted = format!("{:.8}", raw_bal as f64 / DECIMALS as f64);

    Json(BalanceResponse {
        address,
        raw_base_units: raw_bal,
        formatted_balance: formatted,
    })
}

async fn get_job_count(
    State(chain): State<SharedChain>,
) -> Json<u64> {
    let chain = chain.lock().unwrap();
    Json(chain.blocks.len() as u64)
}
