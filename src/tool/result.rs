use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::tool::ContentBlock;

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ToolResult {
    pub content: Vec<ContentBlock>,
    #[serde(rename = "structuredContent", skip_serializing_if = "Option::is_none")]
    pub structured_content: Option<Value>,
    #[serde(rename = "isError", skip_serializing_if = "Option::is_none")]
    pub is_error: Option<bool>,
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Value>,
}