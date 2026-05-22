use serde::{Deserialize, Serialize};
use::bon::Builder;

/// A basic payload containing a single string
#[derive(Serialize, Deserialize)]
#[derive(Builder)]
pub struct StringPayload {
    pub content: String,
}
