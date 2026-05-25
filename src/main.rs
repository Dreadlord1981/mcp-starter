use mcp_starter::server::Server;
use tokio::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    Server::new().run_stdio().await
}
