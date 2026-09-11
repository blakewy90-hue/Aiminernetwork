use reqwest::Client;
use serde_json::{json, Value};

/// Run inference on the model and return:
/// thinking, output, cost, model_name
pub async fn run(url: &str, prompt: &str) -> (String, String, u64, String) {
    let client = Client::new();

    let model_name = "deepspeek-r1-0528-qwen3-8b";

    let payload = json!({
        "model": model_name,
        "messages": [
            { "role": "user", "content": prompt }
        ]
    });

    let res = client.post(url).json(&payload).send().await;

    match res {
        Ok(response) => {
            let json: Value = response.json().await.unwrap_or(Value::Null);

            // Extract output
            let output = json["choices"][0]["message"]["content"]
                .as_str()
                .unwrap_or("")
                .to_string();

            // Extract DeepSeek reasoning
            let thinking = json["choices"][0]["message"]["reasoning_content"]
                .as_str()
                .unwrap_or("")
                .to_string();

            // Extract cost
            let cost = json["usage"]["completion_tokens"]
                .as_u64()
                .unwrap_or(0);

            (thinking, output, cost, model_name.to_string())
        }
        Err(err) => {
            eprintln!("Model error: {}", err);
            ("error".into(), "error".into(), 0, model_name.to_string())
        }
    }
}
