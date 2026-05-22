use serde::{Deserialize, Serialize};
use::bon::Builder;


#[derive(Serialize, Deserialize)]
#[derive(Builder)]
pub struct StringPayload {
    pub content: String,
}
