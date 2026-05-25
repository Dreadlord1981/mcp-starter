use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::protocol::{RequestId, default_jsonrpc};

#[derive(Debug, Deserialize, Serialize)]
pub struct JsonRequest {
	#[serde(default = "default_jsonrpc")]
	pub jsonrpc: String,
	pub id: RequestId,
	pub method: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub params: Option<Value>
}