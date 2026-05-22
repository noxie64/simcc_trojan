use std::{
    fs::File,
    io::Write,
    sync::{LazyLock, Mutex},
};

use crate::constants::paths;
use anyhow::{Error, Result};
use rmp_serde::Serializer;
use serde::{Deserialize, Serialize};

/// Struct containing things saved into the storage:
/// - `iid`: retrieved from the server by using the `ccid`
#[derive(Debug, PartialEq, Deserialize, Serialize, Default)]
pub struct StorageController {
    pub iid: Option<String>,
}

impl StorageController {
    /// Save the current state of [STORAGE]
    pub fn save(&self) -> Result<()> {
        let mut buf = Vec::new();
        self.serialize(&mut Serializer::new(&mut buf))?;

        {
            let mut data_f = File::create(paths::DATA.to_str().unwrap())?;
            data_f.write_all(&buf)?;
        }

        Ok(())
    }

    /// Load the current saved state of [STORAGE]. If none exists, return false, true otherwise.
    pub fn load(&mut self) -> Result<bool> {
        if !paths::DATA.exists() {
            return Ok(false);
        }

        *self = rmp_serde::from_read(File::open(paths::DATA.to_str().unwrap())?)?;
        Ok(true)
    }
}

pub static STORAGE: LazyLock<Mutex<StorageController>> =
    LazyLock::new(|| Mutex::new(StorageController::default()));

