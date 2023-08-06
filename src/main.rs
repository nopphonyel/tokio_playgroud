use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

async fn process(tcp_stream: TcpStream) {
    use mini_redis::Command::{self, Get, Set};
    use std::collections::HashMap;

    // Store data in hash map
    let mut db = HashMap::new();

    // Create a connection from TcpStream
    let mut conn = Connection::new(tcp_stream);

    //let a = conn.read_frame().await;

    while let Some(frame) = conn.read_frame().await.unwrap() {
        let resp = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                let key = cmd.key().to_string();
                let value = cmd.value().to_vec();
                print!("<I> Inserting {:?} into key:\'{key}\'... ", value);
                db.insert(key, value);
                println!("DONE!");
                Frame::Simple("OK".to_string())
            },
            Get(cmd) => {
                if let Some(val) = db.get(cmd.key()) {
                    println!("<I> Reading key:{}", cmd.key());
                    Frame::Bulk(val.clone().into())
                } else {
                    Frame::Null
                }
            },
            cmd => panic!("<X> \'{:?}\' is not yet implemented", cmd),
        };

        // response back to the client
        conn.write_frame(&resp).await.unwrap();
    }
}

#[tokio::main]
async fn main() {
    let bind_res = TcpListener::bind("127.0.0.1:6379").await;
    match bind_res {
        Ok(listener) => {
            loop {
                // We use accept().await here because we want to wait for any incoming connections
                // using .await will be automatically block until the connection has been accepted
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