use tokio::io::AsyncWriteExt;

use crate::DB;

use super::Command;
use crate::utils::utils::to_resp_array_string;

pub struct KeysCommand {
    args: Vec<String>,
    db: &'static DB,
}

impl KeysCommand {
    pub fn new(args: Vec<String>, db: &'static DB) -> Self {
        KeysCommand { args, db }
    }
}

impl Command for KeysCommand {
    fn handle<'a>(
        &'a mut self,
        stream: &'a mut tokio::net::TcpStream,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send + '_>> {
        Box::pin(async move {
            let db = self.db.lock().await;
            let pattern = self.args[0].clone();
            if pattern == "*" {
                let mut keys: Vec<String> = vec![];
                for key in db.keys() {
                    keys.push(key.to_string());
                }

                let resp = to_resp_array_string(keys);
                stream.write_all(resp.as_bytes()).await.unwrap();
            }
        })
    }
}
