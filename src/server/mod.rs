use std::collections::HashMap;

use parse::Frame;
use tokio::net::TcpStream;

mod parse;

pub const HOST: &str = "127.0.0.1:6173";

pub struct Db<K, V> {
    records: HashMap<K, V>,
}

impl Db<String, String> {
    pub fn new() -> Self {
        Self {
            records: HashMap::new(),
        }
    }
    
    pub async fn process(&self, socket: TcpStream) {
        let mut data = Frame::new(socket);

        data.parse_tcp_stream().await;

        println!("{}", data.value.unwrap());
    }


    pub async fn start_server(&self) {
        let listener = tokio::net::TcpListener::bind(HOST).await.unwrap();
        println!("Server on {}", listener.local_addr().unwrap());

        loop {
            let (socket, _) = listener.accept().await.unwrap();
            self.process(socket).await;
        }
    }

}
