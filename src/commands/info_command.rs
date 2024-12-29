use std::{future::Future, pin::Pin};

use tokio::{io::AsyncWriteExt, net::TcpStream};

use crate::{commands::Command, utils::utils::to_resp_bulk_string};

pub struct InfoCommand {
    args: Vec<String>,
    replicaof: String,
}

impl InfoCommand {
    pub fn new(args: Vec<String>, replicaof: String) -> Self {
        InfoCommand { args, replicaof }
    }
}
impl Command for InfoCommand {
    fn handle<'a>(
        &'a mut self,
        stream: &'a mut TcpStream,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        Box::pin(async move {
            let role = if self.replicaof.is_empty() {
                String::from("master")
            } else {
                String::from("slave")
            };
            let resp = to_resp_bulk_string(format!("role:{}", role));
            stream.write(resp.as_bytes()).await.unwrap();
        })
    }
}
