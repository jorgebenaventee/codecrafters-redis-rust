use crate::commands::Command;
use crate::DB;
use std::future::Future;
use std::pin::Pin;
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
            db.insert(key, value);
            stream.write_all(OK).await.unwrap();
        })
    }
}