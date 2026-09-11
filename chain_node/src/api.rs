use axum::{
        routing::{post, get},
            Json, Router,
                extract::{Path, State},
                };
                use serde::Deserialize;
                use std::sync::{Arc, Mutex};

                use crate::{chain::Chain, storage};

                #[derive(Deserialize)]
                pub struct Receipt {
                    pub job_id: String,
                        pub miner_address: String,
                            pub tokens_used: u64,
                            }

                            pub type SharedChain = Arc<Mutex<Chain>>;

                            pub fn router(chain: SharedChain) -> Router {
                                Router::new()
                                        .route("/submit_receipt", post(submit_receipt))
                                                .route("/blocks", get(get_blocks))
                                                        .route("/balance/:address", get(get_balance))
                                                                .route("/job_count", get(get_job_count))
                                                                        .with_state(chain)
                                                                        }

                                                                        async fn submit_receipt(
                                                                            State(chain): State<SharedChain>,
                                                                                Json(receipt): Json<Receipt>,
                                                                                ) -> &'static str {
                                                                                    let mut chain = chain.lock().unwrap();

                                                                                        let block = chain.create_block(
                                                                                                receipt.job_id,
                                                                                                        receipt.miner_address,
                                                                                                                receipt.tokens_used,
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
                                                                                                                                                    ) -> Json<u128> {
                                                                                                                                                        let chain = chain.lock().unwrap();
                                                                                                                                                            let bal = chain.accounts.get_balance(&address);
                                                                                                                                                                Json(bal)
                                                                                                                                                                }

                                                                                                                                                                async fn get_job_count(
                                                                                                                                                                    State(chain): State<SharedChain>,
                                                                                                                                                                    ) -> Json<u64> {
                                                                                                                                                                        let chain = chain.lock().unwrap();
                                                                                                                                                                            Json(chain.blocks.len() as u64)
                                                                                                                                                                            }
}