mod stdio;

use crate::server::Server;
use tokio::io;

#[derive(Default)]
pub struct Runtime;

impl Runtime {
    pub fn new() -> Self {
        Self
    }

    pub async fn run_with_stdio(&self, server: &Server) -> io::Result<()> {
        stdio::run(server).await
    }
}
