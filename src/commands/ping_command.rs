use crate::commands::Command;
use std::future::Future;
use std::pin::Pin;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

pub struct PingCommand {}

impl PingCommand {
    pub fn new() -> PingCommand {
        PingCommand {}
    }
}

impl Command for PingCommand {
    fn handle<'a>(&'a mut self, stream: &'a mut TcpStream) -> Pin<Box<dyn Future<Output=()> + Send + '_>> {
        Box::pin(async move {
            stream.write_all(b"+PONG\r\n").await.unwrap();
        })
    }
}
