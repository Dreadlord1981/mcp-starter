use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
pub struct ToolSchema {
    #[serde(rename = "$schema", skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    #[serde(rename = "type")]
    pub schema_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<Value>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub required: Vec<String>,
}

impl Default for ToolSchema {
    fn default() -> Self {
        Self {
            schema: None,
            schema_type: "object".to_string(),
            properties: None,
            required: Vec::new(),
        }
    }
}

impl ToolSchema {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_schema(mut self, schema: impl Into<String>) -> Self {
        self.schema = Some(schema.into());
        self
    }

    pub fn with_properties(mut self, properties: Value) -> Self {
        self.properties = Some(properties);
        self
    }

    pub fn with_required(mut self, required: Vec<String>) -> Self {
        self.required = required;
        self
    }
}
