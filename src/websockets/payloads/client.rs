use bon::Builder;
use serde::{Deserialize, Serialize};

use crate::websockets::payloads::general::AwaitableRequest;

#[derive(Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct CommandOutputPayload {
    pub stdout: String,
    pub stderr: String,
    pub status_code: Option<i32>,
    pub id: String,
}

#[derive(Serialize, Deserialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct ScreenshotPayload {
    pub id: String,
    #[serde(with = "serde_bytes")]
    pub image_data: Vec<u8>,
}

impl AwaitableRequest for CommandOutputPayload {
    fn id(&self) -> &str {
        &self.id
    }
}

impl AwaitableRequest for ScreenshotPayload {
    fn id(&self) -> &str {
        &self.id
    }
}
