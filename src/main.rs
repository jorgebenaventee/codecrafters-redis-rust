mod commands;
mod utils;

use crate::commands::{
    echo_command::EchoCommand,
    get_command::GetCommand,
    ping_command::PingCommand,
    set_command::SetCommand,
};
use commands::Command;
use lazy_static::lazy_static;
use std::collections::HashMap;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;

lazy_static! {
    static ref DB: Mutex<HashMap<String, DbValue>> = Mutex::new(HashMap::new());
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    println!("Listening for connections in port 6379");
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        tokio::spawn(async move { process(socket).await });
    }
}

async fn process(mut stream: TcpStream) {
    println!("Accepted new connection!");
    loop {
        let buf = &mut [0; 512];
        let n = match stream.read(buf).await {
            Ok(n) if n == 0 => return,    // Connection closed
            Ok(n) => n,
            Err(e) => {
                eprintln!("Failed to read from socket: {}", e);
                return;
            }
        };

        let read_command = String::from_utf8_lossy(&buf[..n]).to_string();
        let command = parse_command(read_command).await;
        if let Ok(mut command) = command {
            command.handle(&mut stream).await;
        } else {
            stream.write_all(b"-ERR unknown command\r\n").await.unwrap();
        }
    }
}

async fn parse_command(command: String) -> Result<Box<dyn Command>, String> {
    println!("Parsing command {}", command);
    let lines: Vec<&str> = command.split("\r\n").collect();
    if lines.len() < 4 {
        return Err("Invalid command".to_string());
    }

    let command = lines[2];
    let args = lines[3..]
        .iter()
        .enumerate()
        .skip(1)
        .filter(|&(i, s)| !s.is_empty() && i % 2 != 0)
        .map(|(_, s)| s.to_string())
        .collect::<Vec<String>>();

    println!("Received {} {:?}", command, args);
    match command.to_lowercase().as_str() {
        "ping" => Ok(Box::new(PingCommand::new())),
        "echo" => Ok(Box::new(EchoCommand::new(args))),
        "set" => Ok(Box::new(SetCommand::new(args, &DB))),
        "get" => Ok(Box::new(GetCommand::new(args, &DB))),
        _ => Err("Invalid command".to_string())
    }
}


struct DbValue {
    value: String,
    expires_at: Option<u128>,
}