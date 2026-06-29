use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    println!("Listening on {}", listener.local_addr().unwrap());

    loop {
        let (socket, _) = listener.accept().await.unwrap();

        println!("Accepted connection on {}", socket.peer_addr().unwrap());
        tokio::spawn(async move {
            //    process(socket, db).await;
        });
    }
}

