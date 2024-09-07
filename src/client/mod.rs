use std::net::TcpStream;

use super::server::HOST;

pub struct Client {
    connection: TcpStream,
}

impl Client {
    pub fn new() -> Self {
        let connection = TcpStream::connect(HOST).unwrap();

        Self { connection }
    }

    
}
