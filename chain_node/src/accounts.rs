use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Accounts {
    pub balances: HashMap<String, u64>, // address → balance in units
    }

    impl Accounts {
        pub fn new() -> Self {
                Self {
                            balances: HashMap::new(),
                                    }
                                        }

                                            pub fn credit(&mut self, address: &str, amount: u64) {
                                                    let bal = self.balances.entry(address.to_string()).or_insert(0);
                                                            *bal += amount;
                                                                }

                                                                    pub fn get_balance(&self, address: &str) -> u64 {
                                                                            *self.balances.get(address).unwrap_or(&0)
                                                                                }
                                                                                }