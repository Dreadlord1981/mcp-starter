use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::resource::ResourceDescriptor;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ListResourcesResult {
    pub resources: Vec<ResourceDescriptor>,
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Value>,
}
