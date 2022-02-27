
use std::net::SocketAddr;
use std::path::PathBuf;

pub const FAVICON_BYTES: &[u8] = std::include_bytes!("quol.ico");

pub const STATIC_ADDRESS: &'static str = "https://a925b11adf9301d29ecc2d6ebafc2fc32.asuscomm.com:53423";

// pub const LOGIN_SOCKET: &SocketAddr = &SocketAddr::from(([127, 0, 0, 1], 33333));

pub const MAIN_PORT: u16 = 56365;

/// Config is a holder for app configuration settings at start up
pub struct Config {
    pub static_address: String,
    pub cache_page_templates: bool,
    pub login_page_template_path: PathBuf,
    pub main_page_template_path: PathBuf,
}

impl Config {
    
    /// Get the config file for the current instance of the server
    pub fn get() -> Result<Config, std::io::Error> {
        Ok(Config { 
            static_address: String::new(),
            cache_page_templates: true,
            login_page_template_path: PathBuf::new(),
            main_page_template_path: PathBuf::new(),
        })
    }
}