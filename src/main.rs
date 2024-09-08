use std::borrow::Borrow;
use std::sync::Arc;

use server::Db;
use server::HOST;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use utils::{cmd::Command, Connection};

mod client;
mod errors;
mod server;
mod utils;

#[tokio::main]
async fn main() {
    simple_logger::SimpleLogger::new().init().unwrap();

    let listener = tokio::net::TcpListener::bind(HOST).await.unwrap();
    log::info!("Server on {}", listener.local_addr().unwrap());

    let db = Arc::new(Mutex::new(Db::new()));

    loop {
        let (socket, _) = listener.accept().await.unwrap();

        let db = db.clone();

        tokio::spawn(async move {
            process(socket, db).await;
        });
    }
}

async fn process(socket: TcpStream, db: Arc<Mutex<Db>>) {
    let mut connection = Connection::new(socket);

    while let Ok(request) = connection.read_stream().await {
        if request == "" {
            break;
        }

        let response = match Command::parse_to_command(request).await {
            Ok(command) => match command {
                Command::Get { key } => {
                    let db = db.lock().await;
                    let result = db.get(&key).await;
                    // TODO: handle commands
                }
                _ => {}
            },
            Err(errors::CommandError::CommandNotImplemented(bad_command)) => {
                log::warn!("Uniplemented: {bad_command}")
            }
        };
    }

    // match connection.read_stream().await {
    //     Ok(data) => {
    //         println!("{}", data);

    //         connection.write_to_stream("Yo").await.unwrap();
    //     }
    //     Err(_) => {
    //         println!("Error")
    //     }
    // }
}
