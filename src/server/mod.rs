mod initialize;
mod notification;
mod transport;

use crate::{
    protocol::{JsonError, JsonNotification, JsonRequest, JsonSuccess},
    tool::ToolRegistry,
};
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};

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

    pub async fn run_stdio(&self) -> io::Result<()> {
        let stdin = io::stdin();
        let stdout = io::stdout();
        let mut lines = BufReader::new(stdin).lines();
        let mut stdout = BufWriter::new(stdout);

        while let Some(line) = lines.next_line().await? {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            let message: serde_json::Value = match serde_json::from_str(line) {
                Ok(message) => message,
                Err(error) => {
                    eprintln!("invalid JSON from stdin: {error}");
                    continue;
                }
            };

            if message.get("id").is_none() {
                let notification: JsonNotification = match serde_json::from_value(message) {
                    Ok(notification) => notification,
                    Err(error) => {
                        eprintln!("invalid notification from stdin: {error}");
                        continue;
                    }
                };

                self.handle_notification(&notification).await;
                continue;
            }

            let request: JsonRequest = match serde_json::from_value(message) {
                Ok(request) => request,
                Err(error) => {
                    eprintln!("invalid request from stdin: {error}");
                    continue;
                }
            };

            let response = match self.handle_request(&request).await {
                Ok(result) => serde_json::to_string(&result),
                Err(error) => serde_json::to_string(&error),
            }
            .map_err(io::Error::other)?;

            stdout.write_all(response.as_bytes()).await?;
            stdout.write_all(b"\n").await?;
            stdout.flush().await?;
        }

        Ok(())
    }
}
