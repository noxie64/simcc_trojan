use std::{
    fs::File,
    io::Write,
    sync::{LazyLock, Mutex, MutexGuard},
};

use anyhow::{Error, Result};
use crate::constants::paths;
use rmp_serde::Serializer;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize, Default)]
pub struct StorageController {
    pub iid: Option<String>,
}

impl StorageController {
    pub fn save(&self) -> Result<()> {
        let mut buf = Vec::new();
        self.serialize(&mut Serializer::new(&mut buf))?;

        {
            let mut data_f = File::create(paths::DATA.to_str().unwrap())?;
            data_f.write_all(&buf)?;
        }

        Ok(())
    }

    pub fn load(&mut self) -> Result<bool> {
        if !paths::DATA.exists() {
            return Ok(false)
        }

        *self = rmp_serde::from_read(File::open(paths::DATA.to_str().unwrap())?)?;
        Ok(true)
    }
}

pub fn locked_store() -> Result<MutexGuard<'static, StorageController>> {
    STORAGE.lock().map_err(|e| Error::msg(e.to_string()))
}

pub static STORAGE: LazyLock<Mutex<StorageController>> = LazyLock::new(||Mutex::new(StorageController::default()));


pub fn load() -> Result<bool> {
    STORAGE.lock().map_err(|e| Error::msg(e.to_string()))?.load()
}

pub fn save() -> Result<()> {
    STORAGE.lock().map_err(|e| Error::msg(e.to_string()))?.save()
}
