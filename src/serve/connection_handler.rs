//
//
//
//
use std::net::TcpStream;
use std::sync::{ Arc, Mutex };

use crate::serve::uid::UserIDPool;
use crate::html::ResponseHandler;

struct ConnectionHandler {
    user_pool: Arc<Mutex<UserIDPool>>,
}

impl ConnectionHandler {
    fn new(stream: TcpStream, id_pool: Arc<Mutex<UserIDPool>>) {
        
    }
}