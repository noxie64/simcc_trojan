pub mod paths {
    #[cfg(not(debug_assertions))]
    pub const DATA: &'static str = "/var/boot_watcher"; // we discuise our self as somthing normal looking

    #[cfg(debug_assertions)]
    pub const DATA: &'static str = concat!(env!("CARGO_MANIFEST_DIR"), "/test_data/simcc_data");
}

