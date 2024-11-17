mod commands;
mod utils;

use crate::commands::echo_command::EchoCommand;
use crate::commands::ping_command::PingCommand;
use commands::Command;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

const PONG: &[u8] = b"+PONG\r\n";
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
        stream.read(buf).await.unwrap();
        let read_command = String::from_utf8(buf.to_vec()).unwrap();
        let command = parse_command(read_command);
        if let Ok(mut command) = command {
            command.handle(&mut stream).await;
        } else {
            stream.write_all(b"-ERR unknown command\r\n").await.unwrap();
        }
    }
}

fn parse_command(command: String) -> Result<Box<dyn Command>, String> {
    println!("Parsing command {}", command);
    let lines: Vec<&str> = command.split("\r\n").collect();
    if lines.len() < 4 {
        return Err("Invalid command".to_string());
    }

    let command = lines[2];
    let args = lines[3..]
        .iter()
        .skip(1)
        .filter(|&s| !s.is_empty())
        .copied()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    println!("Received {} {:?}", command, args);
    match command.to_lowercase().as_str() {
        "ping" => Ok(Box::new(PingCommand::new())),
        "echo" => Ok(Box::new(EchoCommand::new(args))),
        _ => Err("Invalid command".to_string())
    }
}
