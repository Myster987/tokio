use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

mod frame;
pub mod cmd;

pub struct Connection {
    socket: TcpStream,
}

impl Connection {
    pub fn new(socket: TcpStream) -> Self {
        Self { socket }
    }

    pub async fn read_stream(&mut self) -> io::Result<String> {
        let mut reader = io::BufReader::new(&mut self.socket);
        let received: Vec<u8> = reader.fill_buf().await?.to_vec();

        reader.consume(received.len());

        match String::from_utf8(received) {
            Ok(content) => Ok(content),
            Err(_) => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid utf8 string",
            )),
        }
    }

    pub async fn write_to_stream(&mut self, data: &str) -> io::Result<()> {
        let bytes = data.as_bytes();

        self.socket.write(bytes).await?;
        self.socket.flush().await?;

        Ok(())
    }
}
