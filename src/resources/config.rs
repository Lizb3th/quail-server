// use std::net::SocketAddr;
use std::path::PathBuf;

pub struct Config {
    
    pub static_address: String,

    pub session_path: String, // [session/]
    pub login_path: String, // [login/]
    pub log_view_path: String, // [logs/]
    pub guest_path: String, // [gueast/]
    pub icon: String, // [faveicon.ico]

    pub cache_page_templates: bool,
    pub login_page_template_path: PathBuf,
    pub main_page_template_path: PathBuf,

    pub local_port: u16,
    pub external_port: u16,
    pub login_port: u16,

    pub max_request_size: u64,
}

impl Config {
    
    pub fn new() -> Config {
        Config { 
            static_address: String::new(),
            session_path: String::from("session-id"),
            login_path: String::from("login"),
            log_view_path: String::from("logs"),
            guest_path: String::from("guest"),
            icon: String::from("faveicon.ico"),

            cache_page_templates: true,
            login_page_template_path: PathBuf::new(),
            main_page_template_path: PathBuf::new(),

            local_port: 8080,
            external_port: 8050,
            login_port:8030,

            max_request_size: 515,
        }
    }

    /// Get the config file for the current instance of the server
    pub fn get() -> Result<Config, std::io::Error> {
        Ok(Config { 
            static_address: String::new(),
            session_path: String::from("session-id"),
            login_path: String::from("login"),
            log_view_path: String::from("logs"),
            guest_path: String::from("guest"),
            icon: String::from("faveicon.ico"),

            cache_page_templates: true,
            login_page_template_path: PathBuf::new(),
            main_page_template_path: PathBuf::new(),

            local_port: 8080,
            external_port: 8050,
            login_port:8030,

            max_request_size: 515,
        })
    }
}