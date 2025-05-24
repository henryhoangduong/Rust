use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::Message};

#[tokio::main]
async fn main() {
    let url = "wss://echo.websocket.events";

    println!("Connecting to - {}", url);
    let (_ws_stream, _) = connect_async(url).await.expect("Fail to connect");
    println!("Connected to agent network");

    let (mut write, mut read) = _ws_stream.split();

    let msg = Message::Text("aloha echo server".into());

    if let Some(message) = read.next().await {
        let message = message.expect("Fail to read message");
        print!("Recieved a message: {}", message)
    };
    println!("Sending message {}", msg);
    write.send(msg).await.expect("Fail to send message");
    if let Some(message) = read.next().await {
        let message = message.expect("Fail to read message");
        print!("Recieved a message: {}", message)
    };
}
