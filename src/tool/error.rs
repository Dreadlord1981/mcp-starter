use serde_json::Value;

#[derive(Debug)]
pub struct ToolError {
    pub message: String,
    pub data: Option<Value>,
}

impl ToolError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            data: None,
        }
    }
}
