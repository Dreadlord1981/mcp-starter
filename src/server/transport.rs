use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    protocol::{JsonError, JsonNotification, JsonRequest, JsonSuccess},
    resource::{ListResourcesResult, ListResourcesTemplateResult},
    tool::{ContentBlock, ListToolsResult, ToolRegistry, ToolResult},
    tools::{EchoTool, TimeTool},
};

use super::{
    initialize::{
        Implementation, InitializeResult, ResourcesCapability, ServerCapabilities,
        ToolsCapability,
    },
    notification::{NotificationMethod, StandardNotificationMethod},
};

#[derive(Default)]
pub struct Transport {
    registry: ToolRegistry,
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
    #[serde(rename = "resources/list")]
    ResourcesList,
    #[serde(rename = "resources/templates/list")]
    ResourcesTemplatesList,
    #[serde(rename = "tools/list")]
    ToolsList,
    #[serde(rename = "tools/call")]
    ToolsCall,
    #[serde(rename = "ping")]
    Ping,
}

#[derive(Debug, Deserialize)]
struct ToolCallParams {
    name: String,
    #[serde(default)]
    arguments: Option<Value>,
}

impl Transport {
    pub fn new() -> Self {
        let mut transport = Self::default();
        transport.initialize();
        transport
    }

    pub fn registry(&self) -> &ToolRegistry {
        &self.registry
    }

    pub fn registry_mut(&mut self) -> &mut ToolRegistry {
        &mut self.registry
    }

    fn initialize(&mut self) {
        self.register_default_tools();
    }

    fn register_default_tools(&mut self) {
        self.registry.register(EchoTool::new());
        self.registry.register(TimeTool::new());
    }

    pub async fn handle_request(&self, request: &JsonRequest) -> Result<JsonSuccess, JsonError> {
        let method = match serde_json::from_value::<Method>(serde_json::Value::String(
            request.method.clone(),
        )) {
            Ok(method) => method,
            Err(_) => {
                return Err(JsonError::new(
                    Some(request.id.clone()),
                    -100,
                    format!("invalid MCP method: {}", request.method),
                ));
            }
        };

        match method {
            Method::Standard(StandardMethod::Initialize) => Ok(JsonSuccess {
                jsonrpc: request.jsonrpc.clone(),
                id: request.id.clone(),
                result: to_json_value(InitializeResult {
                    protocol_version: "2025-06-18".to_string(),
                    capabilities: ServerCapabilities {
                        resources: Some(ResourcesCapability {
                            list_changed: Some(false),
                        }),
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
            Method::Standard(StandardMethod::ResourcesList) => Ok(JsonSuccess {
                jsonrpc: request.jsonrpc.clone(),
                id: request.id.clone(),
                result: to_json_value(ListResourcesResult {
                    resources: Vec::new(),
                    meta: None,
                }),
            }),
            Method::Standard(StandardMethod::ResourcesTemplatesList) => Ok(JsonSuccess {
                jsonrpc: request.jsonrpc.clone(),
                id: request.id.clone(),
                result: to_json_value(ListResourcesTemplateResult {
                    resource_templates: Vec::new(),
                    meta: None,
                }),
            }),
            Method::Standard(StandardMethod::ToolsList) => Ok(JsonSuccess {
                jsonrpc: request.jsonrpc.clone(),
                id: request.id.clone(),
                result: to_json_value(ListToolsResult {
                    tools: self.registry.list(),
                    meta: None,
                }),
            }),
            Method::Standard(StandardMethod::ToolsCall) => Ok(JsonSuccess {
                jsonrpc: request.jsonrpc.clone(),
                id: request.id.clone(),
                result: to_json_value(self.handle_tool_call(request).await?),
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
            Method::Other(_) => Err(JsonError::new(
                Some(request.id.clone()),
                -100,
                format!("unknown MCP method from stdin: {}", request.method),
            )),
        }
    }

    pub async fn handle_notification(&self, notification: &JsonNotification) {
        let method = match serde_json::from_value::<NotificationMethod>(
            serde_json::Value::String(notification.method.clone()),
        ) {
            Ok(method) => method,
            Err(_) => return,
        };

        match method {
            NotificationMethod::Standard(StandardNotificationMethod::Initialized) => {
                self.handle_initialized_notification(notification).await;
            }
            NotificationMethod::Other(_) => {}
        }
    }

    async fn handle_tool_call(&self, request: &JsonRequest) -> Result<ToolResult, JsonError> {
        let params = serde_json::from_value::<ToolCallParams>(
            request.params.clone().unwrap_or(Value::Null),
        )
        .map_err(|error| {
            JsonError::new(
                Some(request.id.clone()),
                -32602,
                format!("invalid tools/call params: {error}"),
            )
        })?;

        let tool = self.registry.get(&params.name).ok_or_else(|| {
            JsonError::new(
                Some(request.id.clone()),
                -32601,
                format!("unknown tool: {}", params.name),
            )
        })?;

        tool.call(params.arguments).await.map_err(|error| {
            let mut protocol_error =
                JsonError::new(Some(request.id.clone()), -32000, error.message);

            if let Some(data) = error.data {
                protocol_error = protocol_error.with_data(data);
            }

            protocol_error
        })
    }

    async fn handle_initialized_notification(&self, _notification: &JsonNotification) {
        // Placeholder for post-initialize session state if needed later.
    }
}

fn to_json_value(input: impl serde::Serialize) -> serde_json::Value {
    serde_json::to_value(input).expect("serializing response payload should not fail")
}
