mod commands;
mod persistence;
mod utils;

use crate::commands::{
    echo_command::EchoCommand, get_command::GetCommand, ping_command::PingCommand,
    set_command::SetCommand,
};
use clap::Parser;
use commands::config_command::ConfigCommand;
use commands::info_command::InfoCommand;
use commands::keys_command::KeysCommand;
use commands::Command;
use lazy_static::lazy_static;
use persistence::rdb_parser::RdbParser;
use std::collections::HashMap;
use std::fmt::Debug;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use tracing::Level;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long, default_value_t = String::from(""))]
    dir: String,
    #[arg(long, default_value_t = String::from(""))]
    dbfilename: String,
    #[arg(long, default_value_t = 6379)]
    port: u16,
    #[arg(long, default_value_t = String::from(""))]
    replicaof: String,
}

lazy_static! {
    static ref DB: Mutex<HashMap<String, DbValue>> = Mutex::new(HashMap::new());
}

#[tokio::main]
async fn main() {
    let _ = tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .init();
    let args: &Args = &Args::parse();
    tracing::debug!("{:?}", args);
    let listener = TcpListener::bind(format!("127.0.0.1:{}", args.port))
        .await
        .unwrap();
    if !args.dir.trim().is_empty() && !args.dbfilename.trim().is_empty() {
        tracing::info!("Parsing rdb file");
        let map = RdbParser::parse(args.dir.clone(), args.dbfilename.clone()).await;
        if let Ok(map) = map {
            *DB.lock().await = map;
        } else {
            let error = map.unwrap_err();
            println!("Error parsing rdb file: {}", error);
        }
    }

    tracing::info!("Listening for connections in port 6379");
    loop {
        let args = args.clone();
        let (socket, _) = listener.accept().await.unwrap();
        tokio::spawn(async move { process(socket, &args).await });
    }
}

async fn process(mut stream: TcpStream, args: &Args) {
    println!("Accepted new connection!");
    loop {
        let buf = &mut [0; 512];
        let n = match stream.read(buf).await {
            Ok(n) if n == 0 => return, // Connection closed
            Ok(n) => n,
            Err(e) => {
                eprintln!("Failed to read from socket: {}", e);
                return;
            }
        };

        let read_command = String::from_utf8_lossy(&buf[..n]).to_string();
        let command = parse_command(read_command, args).await;
        if let Ok(mut command) = command {
            command.handle(&mut stream).await;
        } else {
            stream.write_all(b"-ERR unknown command\r\n").await.unwrap();
        }
    }
}

async fn parse_command(command: String, args: &Args) -> Result<Box<dyn Command>, String> {
    println!("Parsing command {}", command);
    let lines: Vec<&str> = command.split("\r\n").collect();
    if lines.len() < 4 {
        return Err("Invalid command".to_string());
    }

    let command = lines[2];
    let command_args = lines[3..]
        .iter()
        .enumerate()
        .skip(1)
        .filter(|&(i, s)| !s.is_empty() && i % 2 != 0)
        .map(|(_, s)| s.to_string())
        .collect::<Vec<String>>();

    println!("Received {} {:?}", command, command_args);
    match command.to_lowercase().as_str() {
        "ping" => Ok(Box::new(PingCommand::new())),
        "echo" => Ok(Box::new(EchoCommand::new(command_args))),
        "set" => Ok(Box::new(SetCommand::new(command_args, &DB))),
        "get" => Ok(Box::new(GetCommand::new(command_args, &DB))),
        "config" => Ok(Box::new(ConfigCommand::new(
            command_args,
            args.dir.clone(),
            args.dbfilename.clone(),
        ))),
        "keys" => Ok(Box::new(KeysCommand::new(command_args, &DB))),
        "info" => Ok(Box::new(InfoCommand::new(
            command_args,
            args.replicaof.clone(),
        ))),
        _ => Err("Invalid command".to_string()),
    }
}

#[derive(Debug)]
struct DbValue {
    value: String,
    expires_at: Option<u128>,
}
