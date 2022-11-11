use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct Post(serde_json::Value);
