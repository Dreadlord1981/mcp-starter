use serde::{Serialize, Deserialize};
use serde_json::Value;

use crate::protocol::{JsonError, JsonErrorObject, JsonRequest, JsonSuccess};

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContentBlock {
    Text {
        text: String,
    },
    Image {
        data: String,
        #[serde(rename = "mimeType")]
        mime_type: String,
    },
    Audio {
        data: String,
        #[serde(rename = "mimeType")]
        mime_type: String,
    },
    ResourceLink {
        uri: String,
        name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[serde(rename = "mimeType", skip_serializing_if = "Option::is_none")]
        mime_type: Option<String>,
    },
    Resource {
        resource: Value,
    },
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ToolResult {
    content: Vec<ContentBlock>,
   #[serde(rename = "structuredContent", skip_serializing_if = "Option::is_none")]
    structured_content: Option<Value>,
    #[serde(rename = "isError", skip_serializing_if = "Option::is_none")]
    is_error: Option<bool>,
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    meta: Option<Value>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Method {
    Standard(StandardMethod),
    Other(String),
}

#[derive(Debug, Deserialize, Serialize)]
pub enum StandardMethod {
    #[serde(rename = "initialize")]
    Initialize,
    #[serde(rename = "tools/list")]
    ToolsList,
    #[serde(rename = "tools/call")]
    ToolsCall,
    #[serde(rename = "ping")]
    Ping,
}

pub fn handle_request(request: &JsonRequest) -> Result<JsonSuccess, JsonError> {
	let method = match serde_json::from_value::<Method>(serde_json::Value::String(request.method.clone())) {
		Ok(method) => method,
		Err(_) => {
			return  Err(JsonError {
				id: Some(request.id.clone()),
				error: JsonErrorObject {
					data: None,
					message: format!("invalid MCP method: {}", request.method),
					code: -100
				},
				..Default::default()
			});
		}
	};

	match method {
		Method::Standard(StandardMethod::Initialize) => Ok(JsonSuccess {
				jsonrpc: request.jsonrpc.clone(),
				id: request.id.clone(),
				result: ToolResult {
					content: vec![
						ContentBlock::Text {
							text: "MCP server initialized".to_string(),
						}
					],
					is_error: Some(false),
					structured_content: None,
					meta: None
				}
			}),
		Method::Standard(StandardMethod::ToolsList) => Ok(JsonSuccess {
			jsonrpc: request.jsonrpc.clone(),
			id: request.id.clone(),
			result: ToolResult {
				content: vec![
					ContentBlock::Text {
						text: "No tools registered yet".to_string(),
					}
				],
				is_error: Some(false),
				structured_content: None,
				meta: None,
			},
		}),
		Method::Standard(StandardMethod::ToolsCall) => Ok(JsonSuccess {
			jsonrpc: request.jsonrpc.clone(),
			id: request.id.clone(),
			result: ToolResult {
				content: vec![
					ContentBlock::Text {
						text: "Tool calling is not implemented yet".to_string(),
					}
				],
				is_error: Some(true),
				structured_content: None,
				meta: None,
			},
		}),
		Method::Standard(StandardMethod::Ping) => Ok(JsonSuccess {
			jsonrpc: request.jsonrpc.clone(),
			id: request.id.clone(),
			result: ToolResult {
				content: vec![
					ContentBlock::Text {
						text: "pong".to_string(),
					}
				],
				is_error: Some(false),
				structured_content: None,
				meta: None,
			},
		}),
		Method::Other(_) => {
			Err(JsonError {
				id: Some(request.id.clone()),
				error: JsonErrorObject {
					data: None,
					message: format!("unknown MCP method from stdin: {}", request.method),
					code: -100
				},
				..Default::default()
			})
		}
	}
}
