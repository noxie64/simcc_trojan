mod constants;
mod storage;
mod web;
mod ws;

use std::time::Duration;

use crate::constants::{compiled, paths};
use crate::storage::{STORAGE, locked_store};
use anyhow::{Context, Result};
use tokio::time::interval;

#[tokio::main]
async fn main() -> Result<()> {
    if storage::load()? {
        println!("Loaded storage from {}!", paths::DATA.to_str().unwrap());
    } else {
        println!("Storage not loaded from disk!");
    }

    if locked_store()?.iid.is_none() {
        let mut interval = interval(Duration::from_millis(compiled::RETRY_MILLIS));

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
                    }
                    println!("Retrieved iid from commander!");
                    break;
                }
            }
        }
    }

    locked_store()?.save()?;
    println!("Wrote data to {}!", paths::DATA.to_str().unwrap());

    Ok(())
}
