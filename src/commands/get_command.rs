use crate::commands::Command;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use crate::DB;
use crate::utils::utils::to_resp_bulk_string;

pub struct GetCommand {
    args: Vec<String>,
    db: &'static DB,
}

impl GetCommand {
    pub fn new(args: Vec<String>, db: &'static DB) -> Self {
        GetCommand {
            args,
            db,
        }
    }
}
const NOT_FOUND: &[u8; 5] = b"$-1\r\n";
impl Command for GetCommand {
    fn handle<'a>(&'a mut self, stream: &'a mut TcpStream) -> Pin<Box<dyn Future<Output=()> + Send + '_>> {
        Box::pin(async move {
            let key = self.args[0].clone();
            let mut db = self.db.lock().await;
            let value = db.get(&key);
            if let Some(value) = value {
                let resp = to_resp_bulk_string(value.to_string());
                stream.write_all(resp.as_bytes()).await.unwrap();
            } else {
                stream.write_all(NOT_FOUND).await.unwrap();
            }
        })
    }
}