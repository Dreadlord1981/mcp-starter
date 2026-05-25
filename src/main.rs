use mcp_starter::{runtime::Runtime, server::Server};
use tokio::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    Runtime::new().run_with_stdio(&Server::new()).await
}
