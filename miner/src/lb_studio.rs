use reqwest::Client;
use serde_json::{json, Value};

pub async fn run(url: &str, prompt: &str) -> (String, u64) {
    let client = Client::new();

        let payload = json!({
                "model": "deepspeek-r1-0528-qwen3-8b",
                        "messages": [
                                    { "role": "user", "content": prompt }
                                            ]
                                                });

                                                    let res = client.post(url).json(&payload).send().await;

                                                        match res {
                                                                Ok(response) => {
                                                                            let json: Value = response.json().await.unwrap_or(Value::Null);

                                                                                        let text = json["choices"][0]["message"]["content"]
                                                                                                        .as_str()
                                                                                                                        .unwrap_or("")
                                                                                                                                        .to_string();

                                                                                                                                                    let tokens_used = json["usage"]["completion_tokens"]
                                                                                                                                                                    .as_u64()
                                                                                                                                                                                    .unwrap_or(0);

                                                                                                                                                                                                (text, tokens_used)
                                                                                                                                                                                                        }
                                                                                                                                                                                                                Err(err) => {
                                                                                                                                                                                                                            eprintln!("Model error: {}", err);
                                                                                                                                                                                                                                        ("error".into(), 0)
                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                    }