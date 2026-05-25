use async_trait::async_trait;
use serde_json::Value;

mod descriptor;
mod schema;
mod annotation;
mod result;
mod content;
mod list;
mod error;
mod registry;

pub use descriptor::ToolDescriptor;
pub use schema::ToolSchema;
pub use result::ToolResult;
pub use annotation::ToolAnnotations;
pub use content::ContentBlock;
pub use list::ListToolsResult;
pub use error::ToolError;
pub use registry::ToolRegistry;

#[async_trait]
pub trait Tool: Send + Sync {
    fn descriptor(&self) -> ToolDescriptor;
    async fn call(&self, arguments: Option<Value>) -> Result<ToolResult, ToolError>;
}
