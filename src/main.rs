mod constants;
mod storage;
mod web;
mod ws;

use std::time::Duration;

use crate::constants::{compiled, paths};
use crate::storage::{STORAGE, locked_store};
use anyhow::{Context, Result};
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<()> {
    locked_store()?.load()?;
    if locked_store()?.iid.is_none() {
        {
            loop {
                if let Ok(iid) = web::request_iid().await {
                    let mut storage = STORAGE.lock().unwrap();
                    storage.iid = Some(iid);
                    println!("Retrieved iid from commander!");
                    break;
                }
                sleep(Duration::from_millis(100));
                println!("Failed to retrieve iid!")
            }
        }
    }

    locked_store()?.save()?;
    println!("Wrote data to {}!", paths::DATA.to_str().unwrap());

    Ok(())
}
