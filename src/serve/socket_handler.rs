
use std::sync::{Arc, Mutex};
use std::net::{TcpListener, TcpStream, Ipv4Addr, SocketAddr, IpAddr};
// use std::thread;

// use crate::serve::UserSessionPool;
use super::UserSessionPool;
use super::super::resources::Config;

// use crate::serve::ConnectionHandler;

/// handles http request through tcp connection
pub struct SocketHandler {
    // thread_pool: &ThreadPool,
    listener: TcpListener,
    user_pool: Arc<Mutex<UserSessionPool>>, // connection_handler: [ConnectionHandler]
}

trait PortConvertable {
    fn with_port(&self, port: u16) -> SocketAddr;
}

impl PortConvertable for std::net::IpAddr {
    fn with_port(&self, port: u16) -> SocketAddr {
        SocketAddr::new(*self, port)
    }
}

impl SocketHandler {
    pub fn new(config: &Config) -> std::io::Result<SocketHandler> {
        const LOCAL_HOST: IpAddr = IpAddr::V4(Ipv4Addr::LOCALHOST);

        let local_addr = LOCAL_HOST.with_port( config.local_port);
        let external_addr = LOCAL_HOST.with_port(config.external_port);
        let login_addr = LOCAL_HOST.with_port(config.local_port);

        let addrs = [
            local_addr,
            external_addr,
            login_addr,
        ];

        let listener = std::net::TcpListener::bind(&addrs[..]);

        listener.map(|listener| -> SocketHandler {
            SocketHandler {
                listener: listener,
                user_pool: Arc::new(Mutex::new(UserSessionPool::new())),
            }
        })
    }

    pub fn resume(&self) {
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    // self.handle_connection(stream);
                }
                Err(e) => { /* connection failed */ }
            }
        }
    }

    // fn stop(&self) {}

    fn handl_connection(&self, connection: TcpStream) {
        //thread::spawn(|| ConnectionHandler::new(connection, UserSessionPool::new()));
    }
}
