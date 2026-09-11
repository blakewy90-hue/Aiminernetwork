mod api;
mod block;
mod chain;
mod storage;
mod accounts;

use api::{router, SharedChain};
use chain::Chain;
use std::sync::{Arc, Mutex};
use tokio::signal;
use axum::serve;
use tokio::net::TcpListener;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Load chain from disk or create new
        let chain = if let Some(file) = storage::load_chain() {
                let mut c = Chain::new();
                        c.blocks = file.blocks;
                                c.accounts = file.accounts;
                                        c
                                            } else {
                                                    Chain::new()
                                                        };

                                                            let shared: SharedChain = Arc::new(Mutex::new(chain));

                                                                let app = router(shared);

                                                                    let addr = SocketAddr::from(([127, 0, 0, 1], 7000));
                                                                        println!("Chain node listening on http://{}", addr);

                                                                            let listener = TcpListener::bind(addr).await.unwrap();

                                                                                serve(listener, app)
                                                                                        .with_graceful_shutdown(shutdown_signal())
                                                                                                .await
                                                                                                        .unwrap();
                                                                                                        }

                                                                                                        async fn shutdown_signal() {
                                                                                                            let _ = signal::ctrl_c().await;
                                                                                                            }