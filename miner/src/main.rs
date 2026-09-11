mod config;
mod job;
mod lb_studio;
mod crypto;
mod chain_client;

#[tokio::main]
async fn main() {
    let cfg = config::load();

        loop {
                // Get next job (replace with your real job system)
                        let job = job::get_next_job().await;

                                // Run model + get tokens used
                                        let (output, tokens_used) = lb_studio::run(&cfg.lb_url, &job.prompt).await;

                                                // Hash + sign output
                                                        let hash = crypto::hash(&output);
                                                                let sig = crypto::sign(&hash, &cfg.keypair);

                                                                        // Submit receipt to chain node
                                                                                chain_client::submit_receipt(
                                                                                            &cfg.chain_url,
                                                                                                        &job.id,
                                                                                                                    &cfg.miner_address,
                                                                                                                                tokens_used,
                                                                                                                                        )
                                                                                                                                                .await;

                                                                                                                                                        println!(
                                                                                                                                                                    "Job {} complete → {} tokens → receipt sent",
                                                                                                                                                                                job.id, tokens_used
                                                                                                                                                                                        );
                                                                                                                                                                                            }
                                                                                                                                                                                            }