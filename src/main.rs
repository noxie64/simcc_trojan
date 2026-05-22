mod constants;
mod storage;
mod web;
mod websockets;
#[macro_use]
mod macros;

use std::time::Duration;

use crate::constants::{compiled, paths};
use crate::storage::STORAGE;
use crate::websockets::start_ws_loop;
use anyhow::{Context, Result};
use tokio::time::interval;

#[tokio::main]
async fn main() -> Result<()> {
    if STORAGE.lock().unwrap().load()? {
        println!("Loaded storage from {}!", paths::DATA.to_str().unwrap());
    } else {
        println!("Storage not loaded from disk!");
    }

    if STORAGE.lock().unwrap().iid.is_some() {
        let mut interval = interval(Duration::from_millis(compiled::HTTP_COMMANDER_RECONNECT));

        loop {
            interval.tick().await;
            match web::request_iid().await {
                Err(e) => {
                    println!("Failed to retrieve iid: {}", e)
                }
                Ok(iid) => {
                    {
                        let mut storage = STORAGE.lock().unwrap();
                        storage.iid = Some(iid);
                        println!("Retrieved iid from commander!");
                        storage.save();
                        println!("Wrote data to {}!", paths::DATA.to_str().unwrap());
                    }
                    break;
                }
            }
        }
    }

    println!("Starting websocket-loop!");
    start_ws_loop().await?;

    Ok(())
}
