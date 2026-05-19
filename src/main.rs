mod constants;
mod storage;
mod ws;
mod web;

use crate::constants::{compiled, paths};
use crate::storage::{STORAGE, locked_store};
use anyhow::{Context, Result};

#[tokio::main]
async fn main() -> Result<()> {
    locked_store()?.load()?;
    if locked_store()?.iid.is_none() {
        {
            let mut storage = STORAGE.lock().unwrap();
            storage.iid = Some(
                web::request_iid()?
            );
        }
    }

    locked_store()?.save()?;
    println!("Wrote data to {}!", paths::DATA.to_str().unwrap());

    Ok(())
}
