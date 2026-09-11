use serde::{Deserialize, Serialize};
use reqwest::Client;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Job {
    pub id: String,
    pub prompt: String,
    pub status: String, // "Pending", "Processing", "Completed"
}

/// Poll the chain node for the next available job.
/// Returns `Some(Job)` if a job is available, otherwise `None`.
pub async fn get_next_job(client: &Client) -> Option<Job> {
    let response = client
        .get("http://127.0.0.1:3000/jobs/poll")
        .send()
        .await;

    if response.is_err() {
        println!("⚠️  Miner: Failed to poll job endpoint");
        return None;
    }

    let parsed = response.unwrap().json::<Option<Job>>().await;

    match parsed {
        Ok(job_opt) => job_opt,
        Err(_) => {
            println!("⚠️  Miner: Failed to parse job JSON");
            None
        }
    }
}
