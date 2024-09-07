use server::Db;


mod server;
mod client;

#[tokio::main]
async fn main() {
    let db = Db::new();

    db.start_server().await;

    // let count = Arc::new(Mutex::new(0));

    // for i in 0..10 {
    //     let count_clone = count.clone();
    //     tokio::spawn(async move {
    //         println!("[{i}] increment");
    //         let mut lock = count_clone.lock().await;
    //         *lock += 1;
    //     });
    // }

    // {
    //     let mut num = count.lock().unwrap();
    //     *num += 1
    // };

    // println!("{:?}", count);
}
