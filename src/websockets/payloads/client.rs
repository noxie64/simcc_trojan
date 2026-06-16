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

impl AwaitableRequest for CommandOutputPayload {
    fn id(&self) -> &str {
        &self.id
    }
}
