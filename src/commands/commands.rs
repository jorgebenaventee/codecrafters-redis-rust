use std::future::Future;
use std::pin::Pin;
use tokio::net::TcpStream;
pub trait Command: Send {
    fn handle<'a>(&'a mut self, stream: &'a mut TcpStream) -> Pin<Box<dyn Future<Output=()> + Send + '_>>;
}
