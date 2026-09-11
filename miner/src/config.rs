use dotenvy::dotenv;
use std::env;
use ed25519_dalek::{Keypair, SecretKey, PublicKey};

pub struct Config {
    pub lb_url: String,
        pub chain_url: String,
            pub miner_address: String,
                pub keypair: Keypair,
                }

                pub fn load() -> Config {
                    dotenv().ok();

                        let secret = SecretKey::from_bytes(&[1u8; 32]).unwrap();
                            let public = PublicKey::from(&secret);
                                let keypair = Keypair { secret, public };

                                    Config {
                                            lb_url: env::var("LB_STUDIO_URL").unwrap(),
                                                    chain_url: env::var("CHAIN_NODE_URL").unwrap(),
                                                            miner_address: env::var("MINER_ADDRESS").unwrap(),
                                                                    keypair,
                                                                        }
                                                                        }