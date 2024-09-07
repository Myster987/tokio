use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufReader, BufWriter},
    net::TcpStream,
};

pub struct Frame {
    pub stream: TcpStream,
    pub value: Option<String>,
}

impl Frame {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream,
            value: None,
        }
    }

    pub async fn parse_tcp_stream(&mut self) {
        let mut request_string = String::new();
        let mut reader = BufReader::new(&mut self.stream);

        reader.read_to_string(&mut request_string).await.unwrap();

        self.value = Some(request_string);

        println!("{}", self.value.as_ref().unwrap());
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
