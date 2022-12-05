use std::io::Write;

use anyhow::{anyhow, Context};
use rand::{
    thread_rng, Rng,
};
use serde::Deserialize;

use super::app_config::AppConfig;


#[derive(Debug, Deserialize)]
struct SeedResponse {
    seed: String,
}

pub async fn get_trustee_seed(config: &AppConfig) -> anyhow::Result<String> {
    if let Some(ledger_url) = config.ledger_url() {
        let url = format!("{}/register", ledger_url);
        let client = reqwest::Client::new();
        let body = json!({
            "role": "TRUST_ANCHOR",
            "seed": format!("my_seed_000000000000000000{}", thread_rng().gen_range(100000..1000000))
        })
        .to_string();
        Ok(client
            .post(&url)
            .body(body)
            .send()
            .await
            .context("Failed to send message")?
            .json::<SeedResponse>()
            .await
            .context("Failed to deserialize response")?
            .seed)
    } else {
        Ok("000000000000000000000000Trustee1".to_string())
    }
}

pub async fn download_genesis_file(config: &AppConfig) -> anyhow::Result<String> {
    match config.genesis_file() {
        Some(genesis_file) => {
            if !std::path::Path::new(&genesis_file).exists() {
                Err(anyhow!("The file {} does not exist", genesis_file))
            } else {
                info!("Using genesis file {}", genesis_file);
                Ok(genesis_file.to_string())
            }
        }
        None => match config.ledger_url() {
            Some(ledger_url) => {
                info!("Downloading genesis file from {}", ledger_url);
                let genesis_url = format!("{}/genesis", ledger_url);
                let body = reqwest::get(&genesis_url)
                    .await
                    .context("Failed to get genesis file from ledger")?
                    .text()
                    .await
                    .context("Failed to get the response text")?;
                let path = std::env::current_dir()
                    .context("Failed to obtain the current directory path")?
                    .join("resource")
                    .join("genesis_file.txn");
                info!("Storing genesis file to {:?}", path);
                let mut f = std::fs::OpenOptions::new()
                    .write(true)
                    .create(true)
                    .open(path.clone())
                    .context("Unable to open file")?;
                f.write_all(body.as_bytes()).context("Unable to write data")?;
                debug!("Genesis file downloaded and saved to {:?}", path);
                path.to_str()
                    .map(|s| s.to_string())
                    .ok_or(anyhow!("Failed to convert genesis file path to string".to_string()))
            }
            None => std::env::current_dir()
                .context("Failed to obtain the current directory path")?
                .join("resource")
                .join("indypool.txn")
                .to_str()
                .map(|s| s.to_string())
                .ok_or(anyhow!("Failed to convert genesis file path to string".to_string())),
        },
    }
}
