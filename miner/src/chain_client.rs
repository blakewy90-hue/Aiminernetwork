use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct Job {
    pub id: String,
        pub prompt: String,
        }

        #[derive(Serialize)]
        struct ReceiptPayload<'a> {
            job_id: &'a str,
                miner_address: &'a str,
                    tokens_used: u64,
                    }

                    pub async fn poll_job(chain_url: &str) -> Option<Job> {
                        let client = reqwest::Client::new();
                            let url = format!("{}/jobs/poll", chain_url.trim_end_matches('/'));

                                match client.get(&url).send().await {
                                        Ok(res) => res.json::<Option<Job>>().await.unwrap_or(None),
                                                Err(_) => None,
                                                    }
                                                    }

                                                    pub async fn submit_receipt(
                                                        chain_url: &str,
                                                            job_id: &str,
                                                                miner_address: &str,
                                                                    tokens_used: u64,
                                                                    ) {
                                                                        let client = reqwest::Client::new();
                                                                            let url = format!("{}/submit_receipt", chain_url.trim_end_matches('/'));

                                                                                let payload = ReceiptPayload {
                                                                                        job_id,
                                                                                                miner_address,
                                                                                                        tokens_used,
                                                                                                            };

                                                                                                                let _ = client.post(&url).json(&payload).send().await;
                                                                                                                }