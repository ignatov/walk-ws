# walk-ws

Yo may use a simple client for testing that server.

```rust
use std::env;

use futures_util::StreamExt;
use tokio_tungstenite::connect_async;

#[tokio::main]
async fn main() {
    let connect_addr = env::args()
        .nth(1)
        .unwrap_or_else(|| panic!("this program requires at least one argument"));

    let url = url::Url::parse(&connect_addr).unwrap();

    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("WebSocket handshake has been successfully completed");

    let (write, read) = ws_stream.split();

    read.for_each(|message| async {
                let data = message.unwrap().into_data();
                // println!("{:?}", data.len());
                // println!("{:?}", data.len());
            }).await;
}
```
