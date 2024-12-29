use std::{future::Future, pin::Pin};

use tokio::{io::AsyncWriteExt, net::TcpStream};

use crate::{commands::Command, utils::utils::to_resp_bulk_string};

pub struct InfoCommand {
    args: Vec<String>,
}

impl InfoCommand {
    pub fn new(args: Vec<String>) -> Self {
        InfoCommand { args }
    }
}
impl Command for InfoCommand {
    fn handle<'a>(
        &'a mut self,
        stream: &'a mut TcpStream,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        Box::pin(async move {
            let resp = to_resp_bulk_string(String::from("role:master"));
            stream.write(resp.as_bytes()).await.unwrap();
        })
    }
}
