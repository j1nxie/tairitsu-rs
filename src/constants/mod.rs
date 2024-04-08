use lazy_static::lazy_static;

pub mod version;

pub const API_URL: &str = "https://webapi.lowiro.com/webapi/user/me";
pub const POISE_VERSION: &str = "0.6.1";
lazy_static! {
    pub static ref STARTUP_TIME: std::time::SystemTime = std::time::SystemTime::now();
}
