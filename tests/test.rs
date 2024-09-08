use tokio::{io::{self, AsyncBufReadExt, AsyncWriteExt}, net::TcpStream};


const HOST: &str = "127.0.0.1:6173";

#[tokio::test]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let mut connection = tokio::net::TcpStream::connect(HOST).await.unwrap();

    connection.write_all(b"Maciek").await?;
    connection.flush().await?;


    tokio::spawn(async move {
        loop {   
            while let Ok(response) = read_stream(&mut connection).await {
                if response != "" {
                    println!("{}", response);
                }
            }
            // let response = read_stream(&mut connection).await.unwrap();
        }
    });

    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    Ok(())
}

async fn read_stream(socket: &mut TcpStream) -> io::Result<String> {
    let mut reader = io::BufReader::new(socket);
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