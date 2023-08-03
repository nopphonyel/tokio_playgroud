use mini_redis::Connection;
use tokio::net::{TcpListener, TcpStream};

async fn process(tcp_stream: TcpStream) {
    let mut conn = Connection::new(tcp_stream);

}

#[tokio::main]
async fn main() {
    let bind_res = TcpListener::bind("127.0.0.1:6379").await;
    match bind_res {
        Ok(listener) => {
            loop {
                // We use accept().await here because we want to wait for any incoming connections
                // we not put accept().await inside the loop since the listener will flood the memory
                let accepted_res = listener.accept().await;
                if let Ok((tcp_stream, _sock_addr)) = accepted_res{
                    // After successfully recieve a data, we will spawn a new thread for processing those data
                    // When new thread spawned, we will waiting for new connection asap.
                    tokio::spawn(async move {
                        process(tcp_stream).await;
                    });
                } else if let Err(e) = accepted_res {
                    println!("<X> TCP Listener has failed to accept the connection: {:?}", e);
                }
            }
        },
        Err(e) => {
            println!("<X> Binding failed: {:?}", e);
        },
    }
}