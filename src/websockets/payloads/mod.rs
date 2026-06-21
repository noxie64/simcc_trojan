use anyhow::Result;

use base64::{Engine as _, engine::general_purpose};
use bon::Builder;
use serde::{Deserialize, Serialize};
use tokio_tungstenite::tungstenite::{Message, Utf8Bytes};
pub mod client;
pub mod general;
pub mod server;

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", content = "payload", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Payload {
    /* client-side */
    Hello(general::StringPayload),
    CommandOutput(client::CommandOutputPayload),
    ScreenshotResponse(client::ScreenshotPayload),
    /* server-side */
    Goodbye(general::StringPayload),
    Command(server::CommandPayload),
    Err(server::ErrorPayload),
    ScreenshotRequest(server::ScreenshotRequestPayload),
}

#[derive(Serialize, Deserialize, Builder)]
pub struct SimccMessage {
    #[serde(flatten)]
    pub payload: Payload,
}

pub struct SimccBinaryMessage {
    pub payload: Payload,
}

/// Convert [SimccMessage] into [Message]
impl SimccMessage {
    /// Serialize as JSON text
    pub fn to_text_message(&self) -> Result<Message, anyhow::Error> {
        Ok(Message::Text(Utf8Bytes::from(serde_json::to_string(self)?)))
    }

    /// Serialize as binary
    pub fn to_binary_message(&self) -> Result<Message, anyhow::Error> {
        let bytes = rmp_serde::to_vec_named(self)?;

        let encoded = general_purpose::STANDARD.encode(bytes.clone());
        println!("{}", encoded);
        Ok(Message::Binary(bytes.into()))
    }
}
