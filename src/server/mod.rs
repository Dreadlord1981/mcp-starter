mod initialize;
mod notification;
mod transport;

use crate::{
    protocol::{JsonError, JsonNotification, JsonRequest, JsonSuccess},
    tool::ToolRegistry,
};

use self::transport::Transport;

#[derive(Default)]
pub struct Server {
    transport: Transport,
}

impl Server {
    pub fn new() -> Self {
        Self {
            transport: Transport::new(),
        }
    }

    pub fn registry(&self) -> &ToolRegistry {
        self.transport.registry()
    }

    pub fn registry_mut(&mut self) -> &mut ToolRegistry {
        self.transport.registry_mut()
    }

    pub async fn handle_request(&self, request: &JsonRequest) -> Result<JsonSuccess, JsonError> {
        self.transport.handle_request(request).await
    }

    pub async fn handle_notification(&self, notification: &JsonNotification) {
        self.transport.handle_notification(notification).await;
    }
}
