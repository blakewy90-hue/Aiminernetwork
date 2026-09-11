use serde::{Deserialize, Serialize};
use reqwest::Client;
use serde_json::json;

#[derive(Deserialize, Debug)]
pub struct Job {
    pub id: String,
    pub prompt: String,
    pub status: String,
}

/// Poll the chain node for the next available job
pub async fn poll_job(chain_url: &str) -> Option<Job> {
    let client = Client::new();
    let url = format!("{}/jobs/poll", chain_url.trim_end_matches('/'));

    match client.get(&url).send().await {
        Ok(res) => res.json::<Option<Job>>().await.unwrap_or(None),
        Err(_) => None,
    }
}

/// Submit a full receipt to the chain node
pub async fn submit_receipt(
    chain_url: &str,
    job_id: &str,
    miner_address: &str,
    model: &str,
    thinking: &str,
    output: &str,
    cost: u64,
) {
    let client = Client::new();
    let url = format!("{}/submit_receipt", chain_url.trim_end_matches('/'));

    let payload = json!({
        "job_id": job_id,
        "miner_address": miner_address,
        "model": model,
        "thinking": thinking,
        "output": output,
        "cost": cost
    });

    let res = client.post(&url).json(&payload).send().await;

    match res {
        Ok(_) => println!("📤 [RECEIPT SENT] Job {} | Cost {} tokens", job_id, cost),
        Err(e) => println!("❌ [RECEIPT FAILED] {}", e),
    }
}
