use std::{error::Error, fs::File, io::Write, sync::{Mutex, OnceLock}};

use crate::constants::paths;
use serde::{Deserialize, Serialize};
use rmp_serde::{Deserializer, Serializer};


#[derive(Debug, PartialEq, Deserialize, Serialize, Default)]
pub struct StorageController {
    pub iid: Option<String>
}

impl StorageController {
    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let mut buf = Vec::new();
        self.serialize(&mut Serializer::new(&mut buf))?;

        let mut data_f = File::create(paths::DATA)?;
        data_f.write_all(
            &buf
        )?;

        Ok(())
    }
}

static STORAGE: OnceLock<Mutex<StorageController>> = OnceLock::new();

pub fn get_storage() -> &'static Mutex<StorageController> {
    STORAGE.get_or_init(|| Mutex::new(StorageController::default()))
}
