use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::protocol::{RequestId, default_jsonrpc};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct JsonSuccess {
	#[serde(default = "default_jsonrpc")]
	pub jsonrpc: String,
	pub id: RequestId,
	pub result: Value
}