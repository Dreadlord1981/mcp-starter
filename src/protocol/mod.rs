use serde::{Deserialize, Serialize};
use serde_json::{Value};

use crate::tool::ToolResult;

fn default_jsonrpc() -> String {
	"2.0".into()
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum RequestId {
    String(String),
    Number(i64),
}

impl Default for RequestId {
	fn default() -> Self {
		Self::Number(0)
	}
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JsonRequest {
	#[serde(default = "default_jsonrpc")]
	pub jsonrpc: String,
	pub id: RequestId,
	pub method: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub params: Option<Value>
}


#[derive(Debug, Deserialize, Serialize, Default)]
pub struct JsonSuccess {
	#[serde(default = "default_jsonrpc")]
	pub jsonrpc: String,
	pub id: RequestId,
	pub result: ToolResult
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct JsonErrorObject {
    pub code: i64,
    pub message: String,
	#[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct JsonError {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<RequestId>,
	#[serde(default = "default_jsonrpc")]
	pub jsonrpc: String,
	pub error: JsonErrorObject
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct JsonNotification {
    #[serde(default = "default_jsonrpc")]
    pub jsonrpc: String,
    pub method: String,
	#[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Value>,
}
