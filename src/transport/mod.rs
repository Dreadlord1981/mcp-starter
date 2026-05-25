mod initialize;

use serde::{Deserialize, Serialize};

use crate::{
    protocol::{JsonError, JsonErrorObject, JsonRequest, JsonSuccess},
    tool::{ContentBlock, ListToolsResult, ToolRegistry, ToolResult},
};

pub use initialize::{Implementation, InitializeResult, ServerCapabilities, ToolsCapability};

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

fn to_json_value(input: impl serde::Serialize) -> serde_json::Value {
    serde_json::to_value(input).expect("serializing response payload should not fail")
}

pub async fn handle_request(
    request: &JsonRequest,
    registry: &ToolRegistry,
) -> Result<JsonSuccess, JsonError> {
    let method =
        match serde_json::from_value::<Method>(serde_json::Value::String(request.method.clone())) {
            Ok(method) => method,
            Err(_) => {
                return Err(JsonError {
                    id: Some(request.id.clone()),
                    error: JsonErrorObject {
                        data: None,
                        message: format!("invalid MCP method: {}", request.method),
                        code: -100,
                    },
                    ..Default::default()
                });
            }
        };

    match method {
        Method::Standard(StandardMethod::Initialize) => Ok(JsonSuccess {
            jsonrpc: request.jsonrpc.clone(),
            id: request.id.clone(),
            result: to_json_value(InitializeResult {
				protocol_version: "2025-06-18".to_string(),
				capabilities: ServerCapabilities {
					tools: Some(ToolsCapability {
						list_changed: Some(false),
					}),
				},
				server_info: Implementation {
					name: "mcp-starter".to_string(),
					version: env!("CARGO_PKG_VERSION").to_string(),
				},
				instructions: None,
				meta: None,
			}),
        }),
        Method::Standard(StandardMethod::ToolsList) => Ok(JsonSuccess {
            jsonrpc: request.jsonrpc.clone(),
            id: request.id.clone(),
            result: to_json_value(ListToolsResult {
                tools: registry.list(),
                meta: None,
            }),
        }),
        Method::Standard(StandardMethod::ToolsCall) => Ok(JsonSuccess {
            jsonrpc: request.jsonrpc.clone(),
            id: request.id.clone(),
            result: to_json_value(ToolResult {
                content: vec![ContentBlock::Text {
                    text: "Tool calling is not implemented yet".to_string(),
                }],
                is_error: Some(true),
                structured_content: None,
                meta: None,
            }),
        }),
        Method::Standard(StandardMethod::Ping) => Ok(JsonSuccess {
            jsonrpc: request.jsonrpc.clone(),
            id: request.id.clone(),
            result: to_json_value(ToolResult {
                content: vec![ContentBlock::Text {
                    text: "pong".to_string(),
                }],
                is_error: Some(false),
                structured_content: None,
                meta: None,
            }),
        }),
        Method::Other(_) => Err(JsonError {
            id: Some(request.id.clone()),
            error: JsonErrorObject {
                data: None,
                message: format!("unknown MCP method from stdin: {}", request.method),
                code: -100,
            },
            ..Default::default()
        }),
    }
}
