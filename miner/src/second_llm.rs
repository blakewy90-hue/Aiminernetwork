use reqwest::Client;
use serde_json::json;

pub async fn run(url: &str, prompt: &str) -> String {
    let client = Client::new();

        client
                .post(url)
                        .json(&json!({ "prompt": prompt }))
                                .send()
                                        .await
                                                .unwrap()
                                                        .text()
                                                                .await
                                                                        .unwrap()
                                                                        }