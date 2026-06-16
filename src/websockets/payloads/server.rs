use bon::Builder;
use serde::{Deserialize, Serialize};

use crate::websockets::payloads::general::AwaitableRequest;



#[derive(Serialize, Deserialize)]
#[derive(Builder)]
pub struct CommandPayload{
    pub command: String,
    pub id: String
}

impl AwaitableRequest for CommandPayload {
    fn id(&self) -> &str {
        &self.id
    }
}

#[derive(Serialize, Deserialize)]
#[derive(Builder)]
#[serde(rename_all = "camelCase")]
pub struct ErrorPayload {
    #[serde(rename = "type")]
    pub err_type: String,
    pub msg: Option<String>
}
