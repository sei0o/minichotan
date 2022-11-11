use serde::{Deserialize, Serialize};

use crate::post::Post;

#[derive(Serialize, Deserialize)]
pub enum RpcObject {
    #[serde(rename = "result")]
    Result(RpcResponseResult),
    #[serde(rename = "error")]
    Error(RpcResponseError),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum RpcResponseResult {
    Plain {
        meta: RpcResponsePlainMeta,
        body: serde_json::Value,
    },
    HomeTimeline {
        meta: RpcResponsePlainMeta,
        body: Vec<Post>,
    },
    Status {
        version: String,
    },
    AccountList {
        user_ids: Vec<String>,
    },
    AccountAdd {
        user_id: String,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RpcResponsePlainMeta {
    pub api_calls_remaining: usize,
    pub api_calls_reset: usize, // in epoch sec
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RpcResponseError {
    pub code: isize,
    pub message: String,
    pub data: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize)]
pub struct RpcResponse {
    pub id: String,
    jsonrpc: String,
    #[serde(flatten)]
    pub object: RpcObject,
}
// TODO: handle error objects
