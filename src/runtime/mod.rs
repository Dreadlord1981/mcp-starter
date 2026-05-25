mod stdio;

use crate::server::Server;
use tokio::io;

#[derive(Debug, Default)]
pub enum RuntimeMode {
    #[default]
    Stdio,
}

#[derive(Default)]
pub struct Runtime;

impl Runtime {
    pub fn new() -> Self {
        Self
    }

    pub async fn run(&self, server: &Server, mode: RuntimeMode) -> io::Result<()> {
        match mode {
            RuntimeMode::Stdio => stdio::run(server).await,
        }
    }
}
