mod accounts;
mod api;
mod block;
mod chain;
mod storage;

use chain::Chain;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    println!("-----------------------------------------------");
        println!("🚀 Starting Chain Node Server...");

            let chain = Arc::new(Mutex::new(Chain::new()));
                let app = api::router(chain);

                    let addr = "127.0.0.1:3000";
                        let listener = tokio::net::TcpListener::bind(addr)
                                .await
                                        .expect("Failed to bind TCP listener");

                                            println!("📍 Server listening on: http://{}", addr);
                                                println!("⚙️  Active Routes:");
                                                    println!("   • POST /jobs            -> Enqueue AI tasks");
                                                        println!("   • GET  /jobs/poll       -> Miners fetch tasks");
                                                            println!("   • POST /submit_receipt  -> Submit proof of work");
                                                                println!("   • GET  /blocks          -> Query blockchain state");
                                                                    println!("   • GET  /balance/:addr   -> Query miner balance");
                                                                        println!("-----------------------------------------------");

                                                                            if let Err(e) = axum::serve(listener, app).await {
                                                                                    eprintln!("❌ Server execution error: {}", e);
                                                                                        }
                                                                                        }