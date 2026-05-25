use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::protocol::default_jsonrpc;

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct JsonNotification {
    #[serde(default = "default_jsonrpc")]
    pub jsonrpc: String,
    pub method: String,
	#[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Value>,
}