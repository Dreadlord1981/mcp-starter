use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::tool::ToolDescriptor;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ListToolsResult {
    pub tools: Vec<ToolDescriptor>,
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Value>,
}
