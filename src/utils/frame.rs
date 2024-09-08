use tokio::{
    io::AsyncReadExt, net::TcpStream
};

pub enum Frame {
    Simple(String)
}

impl Frame {
    pub fn new() -> Self {
        Self::Simple(String::new())
    }

    pub async fn parse_stream_to_string(&mut self, stream: &mut TcpStream) -> String {
        let mut request_string = String::new();

        stream.read_to_string(&mut request_string).await;

        self::Frame::Simple(request_string.clone());

        request_string
    }

    // pub async fn parse_to_buffer(&mut self) -> Result<(), &str> {
    //     if self.value.is_none() {
    //         return Err("Stream not parsed yet.");
    //     }

    //     let mut writer = BufWriter::new(&mut self.stream);

    //     let bytes = self.value.as_ref().unwrap().as_bytes();

    //     for byte in bytes {
    //         writer.write(&[*byte]);
    //     }

    //     Ok(())
    // }
}
