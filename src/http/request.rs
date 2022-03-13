
use std::collections::HashMap;
use std::option::Option;
use std::net::TcpStream;
use std::path::PathBuf;

use crate::resources::Config;
use crate::html::PageKind;

struct Headers {
    storage: HashMap<String, String>,
}

#[derive(PartialEq, Debug)]
pub enum RequestGet<'a> {
    Html(PageKind<'a>),
    BinaryFile,
    TextFile,
}

pub struct Request {
    path: PathBuf,
    version: String,
    headers: Headers,
}

impl Request {

    fn from_stream(config: &Config, stream: &TcpStream) -> Option<Request> {
        let max_size = config.max_request_size;

        Some(Request{ path: PathBuf::new(), version: String::from("1.1"), 
                      headers: Headers { storage: HashMap::new() } })

    }
}

