use mcp_starter::{
    runtime::{Runtime, RuntimeMode},
    server::Server,
};
use tokio::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    Runtime::new().run(&Server::new(), RuntimeMode::Stdio).await
}
