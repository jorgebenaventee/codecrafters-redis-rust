use std::{future::Future, pin::Pin};

use tokio::{io::AsyncWriteExt, net::TcpStream};

use crate::utils::utils::to_resp_array_string;

use super::Command;

pub struct ConfigCommand {
    args: Vec<String>,
    dir: String,
    db_file_name: String,
}

impl ConfigCommand {
    pub fn new(args: Vec<String>, dir: String, db_file_name: String) -> ConfigCommand {
        ConfigCommand {
            args,
            dir,
            db_file_name,
        }
    }
    fn get(&self, key: &str) -> Option<String> {
        match key {
            "dir" => Some(self.dir.clone()),
            "dbfilename" => Some(self.db_file_name.clone()),
            _ => None,
        }
    }
}

impl Command for ConfigCommand {
    fn handle<'a>(
        &'a mut self,
        stream: &'a mut TcpStream,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        Box::pin(async move {
            let is_get = self.args[0].to_lowercase() == "get";
            if is_get {
                let key = self.args[1].clone();
                let value = self.get(&key);
                if let Some(value) = value {
                    let response = to_resp_array_string(vec![key, value]);
                    let response_bytes = response.as_bytes();
                    let _ = stream.write_all(response_bytes).await;
                }
            }
        })
    }
}
