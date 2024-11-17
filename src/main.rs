use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;

const PONG: &[u8] = b"+PONG\r\n";
fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        thread::spawn(
            || {
                match stream {
                    Ok(mut stream) => {
                        println!("Accepted new connection!");
                        let mut read_command = [0; 512];
                        loop {
                            let result = stream.read(&mut read_command);
                            let _ = stream.write(PONG);
                        }
                    }
                    Err(e) => {
                        println!("error: {}", e);
                    }
                }
            }
        );
    }
}
