use crate::{
    protocol::{JsonNotification, JsonRequest},
    server::Server,
};
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};

pub async fn run(server: &Server) -> io::Result<()> {
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

            server.handle_notification(&notification).await;
            continue;
        }

        let request: JsonRequest = match serde_json::from_value(message) {
            Ok(request) => request,
            Err(error) => {
                eprintln!("invalid request from stdin: {error}");
                continue;
            }
        };

        let response = match server.handle_request(&request).await {
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
