use std::net::TcpStream;

pub struct Client {
    connection: TcpStream,
}

impl Client {
    pub fn connect(host: &str) -> Self {
        let connection = TcpStream::connect(host).unwrap();

        Self { connection }
    }

    pub fn get(&self, key: &str) {
        // self.connection.
    }    
}
