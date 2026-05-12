mod constants;
mod storage;
mod ws;

use crate::constants::paths;
use crate::storage::get_storage;

fn main() {
    {
        let mut storage = get_storage().lock().unwrap();
        storage.iid = Some(String::from("IID"));
        if let Err(e) = storage.save() {
            println!("Failed: {}", e);
        } else {
            println!("Wrote data to {}!", paths::DATA);
        };
    }
}
