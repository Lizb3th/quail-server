//
//
//
//
use std::net::TcpStream;
use std::sync::{ Arc, Mutex };

use super::super::serve::UserSessionPool;
//use crate::serve::uid::UserSessionPool;
// use crate::html::ResponseHandler;

pub struct ConnectionHandler {
    user_pool: Arc<Mutex<UserSessionPool>>,
}

impl ConnectionHandler {
    pub fn new(stream: TcpStream, id_pool: Arc<Mutex<UserSessionPool>>) {
        
    }
}