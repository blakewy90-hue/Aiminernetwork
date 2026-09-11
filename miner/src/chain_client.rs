use reqwest::Client;
use serde_json::json;

pub async fn submit_receipt(
    chain_url: &str,
        job_id: &str,
            miner_address: &str,
                tokens_used: u64,
                ) {
                    let client = Client::new();

                        let payload = json!({
                                "job_id": job_id,
                                        "miner_address": miner_address,
                                                "tokens_used": tokens_used
                                                    });

                                                        let res = client.post(chain_url).json(&payload).send().await;

                                                            match res {
                                                                    Ok(response) => {
                                                                                if !response.status().is_success() {
                                                                                                eprintln!("Chain node error: {}", response.status());
                                                                                                            }
                                                                                                                    }
                                                                                                                            Err(err) => {
                                                                                                                                        eprintln!("Failed to submit receipt: {}", err);
                                                                                                                                                }
                                                                                                                                                    }
                                                                                                                                                    }