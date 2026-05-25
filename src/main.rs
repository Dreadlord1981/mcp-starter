use mcp_starter::{protocol::JsonRequest, tool::ToolRegistry, transport::handle_request};
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};

#[tokio::main]
async fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let registry = ToolRegistry::new();
    let mut lines = BufReader::new(stdin).lines();
    let mut stdout = BufWriter::new(stdout);

    while let Some(line) = lines.next_line().await? {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let request: JsonRequest = match serde_json::from_str(line) {
            Ok(request) => request,
            Err(error) => {
                eprintln!("invalid JSON from stdin: {error}");
                continue;
            }
        };

        let response = match handle_request(&request, &registry).await {
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
