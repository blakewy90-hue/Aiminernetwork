mod chain_client;
mod config;
mod crypto;
mod lb_studio;

use std::io::{self, Write};

#[tokio::main]
async fn main() {
    let cfg = config::load();

    println!("===============================================");
    println!("⛏️  AI Miner Active");
    println!("📍 Chain Node:  {}", cfg.chain_url);
    println!("📍 LM Studio:   {}", cfg.lb_url);
    println!("👤 Miner Addr:  {}", cfg.miner_address);
    println!("===============================================");
    println!("⏳ Polling queue for work...");

    loop {
        if let Some(job) = chain_client::poll_job(&cfg.chain_url).await {
            println!("\n📥 [JOB FOUND] ID: {} | Prompt: \"{}\"", job.id, job.prompt);
            println!("⚙️  Running local AI inference...");

            // NEW: full inference output
            let (thinking, output, cost, model) =
                lb_studio::run(&cfg.lb_url, &job.prompt).await;

            println!("🧠 [INFERENCE READY] Compute cost: {} tokens", cost);

            // Hash + sign output (unchanged)
            let hash = crypto::hash(&output);
            let _sig = crypto::sign(&hash, &cfg.keypair);

            // NEW: full receipt submission
            chain_client::submit_receipt(
                &cfg.chain_url,
                &job.id,
                &cfg.miner_address,
                &model,
                &thinking,
                &output,
                cost,
            )
            .await;

            println!("✅ [RECEIPT ACCEPTED] Earned reward for {} tokens!", cost);
            print!("⏳ Waiting for new jobs...");
            io::stdout().flush().unwrap();
        } else {
            // Heartbeat
            print!(".");
            io::stdout().flush().unwrap();
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        }
    }
}
