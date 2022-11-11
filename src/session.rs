use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Holds session keys for JSON-RPC. This struct is associated with the minichotan session's "accounts" field.
#[derive(Serialize, Deserialize, Default)]
pub struct AccountList {
    pub owner_id: Option<String>,
    pub rpc_session_keys: HashMap<String, String>,
}

impl AccountList {
    pub fn key_for(&self, id: &str) -> Option<String> {
        self.rpc_session_keys.get(id).cloned()
    }

    // Returns JSON-RPC session key for the owner.
    pub fn owner_key(&self) -> Option<String> {
        self.owner_id.as_ref().map(|id| self.key_for(id).unwrap())
    }
}
