use anyhow::Result;

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
    /* server-side */
    Goodbye(general::StringPayload),
    Command(server::CommandPayload),
    Err(server::ErrorPayload)
}

#[derive(Serialize, Deserialize, Builder)]
pub struct SimccMessage {
    #[serde(flatten)]
    pub payload: Payload,
}

/// Convert [SimccMessage] into [Message]
impl TryInto<Message> for SimccMessage {
    fn try_into(self) -> Result<Message, Self::Error> {
        Ok(Message::Text(Utf8Bytes::from(serde_json::to_string(
            &self,
        )?)))
    }

    type Error = anyhow::Error;
}
