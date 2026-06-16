use bon::Builder;
use serde::{Deserialize, Serialize};

use crate::websockets::payloads::general::AwaitableRequest;

#[derive(Serialize, Deserialize)]
#[derive(Builder)]
pub struct CommandOutputPayload{
    pub stdout: String,
    pub id: String,
}

impl AwaitableRequest for CommandOutputPayload {
    fn id(&self) -> &str {
        &self.id
    }
}
