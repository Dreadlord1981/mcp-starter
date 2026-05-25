use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::protocol::{RequestId, default_jsonrpc};

#[derive(Debug, Deserialize, Serialize, Default)]
struct JsonErrorObject {
    code: i64,
    message: String,
	#[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Value>
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct JsonError {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<RequestId>,
	#[serde(default = "default_jsonrpc")]
	pub jsonrpc: String,
	error: JsonErrorObject
}

impl JsonError {
    pub fn new(id: Option<RequestId>, code: i64, message: impl Into<String>) -> Self {
        Self {
            id,
            jsonrpc: default_jsonrpc(),
            error: JsonErrorObject {
                code,
                message: message.into(),
                data: None,
            },
        }
    }

    pub fn with_data(mut self, data: Value) -> Self {
        self.error.data = Some(data);
        self
    }
}
