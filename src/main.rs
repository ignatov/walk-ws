use std::{env, io::Error};
use std::fs;

use futures_util::{SinkExt, StreamExt};
use jwalk::WalkDir;
use log::info;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::{http, Message};

/*
fn main() {
    let start = std::time::Instant::now();

    let root = "/Users/ignatov/src/intellij-erlang";
    // let root = "/Users/ignatov/src/community-export";
    // let root = "/Users/ignatov/src/ultimate";
    // let root = "/Users/ignatov/src/ripgrep";
    
    for entry in WalkDir::new(root) {
        let entry = entry.unwrap();
        if entry.file_type().is_file() {
            let path = entry.path();
            let content = match fs::read_to_string(path) {
                Ok(s) => s,
                Err(e) => "".to_string(),
            };
        }
        // println!("{}", entry.path().display());
    }
    println!("Took {}", start.elapsed().as_millis());
}
*/

#[tokio::main]
async fn main() -> Result<(), Error> {
    let _ = env_logger::try_init();
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8082".to_string());

    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let mut listener = try_socket.expect("Failed to bind");
    info!("Listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(accept_connection(stream));
    }

    Ok(())
}

async fn accept_connection(stream: TcpStream) {
    let addr = stream
        .peer_addr()
        .expect("connected streams should have a peer address");
    info!("Peer address: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");

    info!("New WebSocket connection: {}", addr);

    let (mut write, read) = ws_stream.split();

    let root = "/Users/ignatov/src/intellij-erlang";
    // let root = "/Users/ignatov/src/ripgrep";


    use std::io;
    use std::io::prelude::*;
    use std::fs;

    for entry in WalkDir::new(root) {
        let entry = entry.unwrap();
        if entry.file_type().is_file() {
            // let write = write.clone();

            let path = entry.path();
            // let mut buffer = Vec::new();
            // read the whole file
            // let mut f = std::fs::File::open(path).unwrap();
            // let bin = f.read_to_end(&mut buffer).unwrap();

            let content = match fs::read_to_string(path) {
                Ok(s) => s,
                Err(e) => "".to_string(),
            };
            write.send(Message::text(content)).await;
            // write.send(Message::binary(buffer)).await;
        }
        write.send(Message::text(entry.path().to_string_lossy().to_string())).await;
    }

    write.send(Message::Close(None)).await;
    drop(write);
}


 