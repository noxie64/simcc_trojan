use crate::websockets::payloads::{client::CommandOutputPayload, general::StringPayload, server::CommandPayload };

pub fn handle_goodbye(payload: StringPayload) {
    println!("Got: {}", payload.content);
}

pub fn handle_command(payload: CommandPayload) -> CommandOutputPayload {
    CommandOutputPayload::builder()
        .stdout(format!("Test: {}", payload.command))
        .id(payload.id)
        .build()
}
