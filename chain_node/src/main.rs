mod config;
mod lb_studio;
mod crypto;
mod chain_client;

use std::io::{self, Write};

#[tokio::main]
async fn main() {
    let cfg = config::load();

        loop {
                print!("Enter a prompt (or 'exit'): ");
                        io::stdout().flush().unwrap();

                                let mut prompt = String::new();
                                        io::stdin().read_line(&mut prompt).unwrap();
                                                let prompt = prompt.trim().to_string();

                                                        if prompt == "exit" {
                                                                    println!("Miner stopped.");
                                                                                break;
                                                                                        }

                                                                                                if prompt.is_empty() {
                                                                                                            println!("Please enter something.");
                                                                                                                        continue;
                                                                                                                                }

                                                                                                                                        // Run model
                                                                                                                                                let (output, tokens_used) = lb_studio::run(&cfg.lb_url, &prompt).await;

                                                                                                                                                        // Hash + sign
                                                                                                                                                                let hash = crypto::hash(&output);
                                                                                                                                                                        let _sig = crypto::sign(&hash, &cfg.keypair);

                                                                                                                                                                                // Submit receipt
                                                                                                                                                                                        chain_client::submit_receipt(
                                                                                                                                                                                                    &cfg.chain_url,
                                                                                                                                                                                                                "manual-job",
                                                                                                                                                                                                                            &cfg.miner_address,
                                                                                                                                                                                                                                        tokens_used,
                                                                                                                                                                                                                                                )
                                                                                                                                                                                                                                                        .await;

                                                                                                                                                                                                                                                                println!(
                                                                                                                                                                                                                                                                            "Manual job complete → {} tokens → receipt sent",
                                                                                                                                                                                                                                                                                        tokens_used
                                                                                                                                                                                                                                                                                                );
                                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                                    }