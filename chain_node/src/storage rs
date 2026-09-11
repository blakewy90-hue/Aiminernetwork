use crate::{accounts::Accounts, block::Block};
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

#[derive(Serialize, Deserialize)]
pub struct ChainFile {
    pub blocks: Vec<Block>,
        pub accounts: Accounts,
        }

        const CHAIN_PATH: &str = "chain.json";

        pub fn load_chain() -> Option<ChainFile> {
            if !std::path::Path::new(CHAIN_PATH).exists() {
                    return None;
                        }

                            let mut file = File::open(CHAIN_PATH).unwrap();
                                let mut buf = String::new();
                                    file.read_to_string(&mut buf).unwrap();

                                        serde_json::from_str(&buf).ok()
                                        }

                                        pub fn save_chain(blocks: &[Block], accounts: &Accounts) {
                                            let data = ChainFile {
                                                    blocks: blocks.to_vec(),
                                                            accounts: accounts.clone(),
                                                                };

                                                                    let json = serde_json::to_string_pretty(&data).unwrap();

                                                                        let mut file = OpenOptions::new()
                                                                                .create(true)
                                                                                        .write(true)
                                                                                                .truncate(true)
                                                                                                        .open(CHAIN_PATH)
                                                                                                                .unwrap();

                                                                                                                    file.write_all(json.as_bytes()).unwrap();
                                                                                                                    }