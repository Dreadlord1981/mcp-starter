use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{Value, json};

use crate::tool::{ContentBlock, Tool, ToolDescriptor, ToolError, ToolResult, ToolSchema};

pub struct EchoTool;

#[derive(Debug, Deserialize)]
struct EchoArguments {
    text: String,
}

impl EchoTool {
    pub fn new() -> Self {
        Self
    }
}

impl Default for EchoTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Tool for EchoTool {
    fn descriptor(&self) -> ToolDescriptor {
        ToolDescriptor {
            name: "echo".to_string(),
            title: Some("Echo".to_string()),
            description: Some("Echoes the provided text back to the caller.".to_string()),
            input_schema: ToolSchema {
                properties: Some(json!({
                    "text": {
                        "type": "string",
                        "description": "The text to echo back."
                    }
                })),
                required: Some(vec!["text".to_string()]),
                ..Default::default()
            },
            output_schema: None,
            annotations: None,
            meta: None,
        }
    }

    async fn call(&self, arguments: Option<Value>) -> Result<ToolResult, ToolError> {
        let args = serde_json::from_value::<EchoArguments>(arguments.unwrap_or(Value::Null))
            .map_err(|error| ToolError::new(format!("invalid echo arguments: {error}")))?;

        Ok(ToolResult {
            content: vec![ContentBlock::Text { text: args.text }],
            structured_content: None,
            is_error: Some(false),
            meta: None,
        })
    }
}
