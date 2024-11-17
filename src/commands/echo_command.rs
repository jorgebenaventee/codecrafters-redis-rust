use crate::commands::Command;
use crate::utils::utils::to_resp_bulk_string;
use std::future::Future;
use std::pin::Pin;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

pub struct EchoCommand {
    args: Vec<String>,
}


impl EchoCommand {
    pub fn new(args: Vec<String>) -> EchoCommand {
        EchoCommand {
            args,
        }
    }
}


impl Command for EchoCommand {
    fn handle<'a>(&'a mut self, stream: &'a mut TcpStream) -> Pin<Box<dyn Future<Output=()> + Send + '_>> {
        Box::pin(async move {
            let resp = to_resp_bulk_string(self.args[0].clone());
            stream.write_all(resp.as_bytes()).await.unwrap();
        })
    }
}