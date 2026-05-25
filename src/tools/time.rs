use async_trait::async_trait;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::tool::{ContentBlock, Tool, ToolDescriptor, ToolError, ToolResult, ToolSchema};

pub struct TimeTool;

impl TimeTool {
    pub fn new() -> Self {
        Self
    }
}

impl Default for TimeTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Tool for TimeTool {
    fn descriptor(&self) -> ToolDescriptor {
        ToolDescriptor {
            name: "time".to_string(),
            title: Some("Time".to_string()),
            description: Some("Returns the current Unix timestamp in seconds.".to_string()),
            input_schema: ToolSchema::default(),
            output_schema: None,
            annotations: None,
            meta: None,
        }
    }

    async fn call(&self, _arguments: Option<serde_json::Value>) -> Result<ToolResult, ToolError> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|error| ToolError::new(format!("system clock error: {error}")))?;

        Ok(ToolResult {
            content: vec![ContentBlock::Text {
                text: now.as_secs().to_string(),
            }],
            structured_content: Some(serde_json::json!({
                "unixSeconds": now.as_secs()
            })),
            is_error: Some(false),
            meta: None,
        })
    }
}
