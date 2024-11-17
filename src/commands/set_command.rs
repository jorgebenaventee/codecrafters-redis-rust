use crate::commands::Command;
use crate::{DbValue, DB};
use std::future::Future;
use std::pin::Pin;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

pub struct SetCommand {
    args: Vec<String>,
    db: &'static DB,
}


impl SetCommand {
    pub fn new(args: Vec<String>, db: &'static DB) -> Self {
        SetCommand {
            args,
            db,
        }
    }
}

const OK: &[u8; 5] = b"+OK\r\n";

impl Command for SetCommand {
    fn handle<'a>(&'a mut self, stream: &'a mut TcpStream) -> Pin<Box<dyn Future<Output=()> + Send + '_>> {
        Box::pin(async move {
            let key = self.args[0].clone();
            let mut db = self.db.lock().await;
            let value = self.args[1].clone();
            let has_expires = self.args.len() == 4 && self.args[2].to_lowercase() == "px";
            let expires_at = if has_expires {
                let expires_in = self.args[3].parse::<u64>().unwrap();
                Some(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() + expires_in as u128)
            } else {
                None
            };
            db.insert(key, DbValue { value, expires_at });
            stream.write_all(OK).await.unwrap();
        })
    }
}