
/// A quick macro to either load a compiled env or replace it with a default value 
#[macro_export]
macro_rules! env_or {
    ($key:expr, $default:expr, $ty:ty) => {
        match option_env!($key) {
            Some(val) => match <$ty>::from_str_radix(val, 10) {
                Ok(parsed) => parsed,
                Err(_) => $default,
            },
            None => $default,
        }
    };
}
