
use std::sync::{Arc, Mutex};
use std::thread;
use std::net::{TcpListener, TcpStream, Ipv4Addr, SocketAddrV4, SocketAddr};

use crate::serve::UserIdPool;
use crate::resources::Config;

// use crate::serve::ConnectionHandler;

/// handles http request through tcp connection
pub struct SocketHandler {
    // thread_pool: &ThreadPool,
    listener: TcpListener,
    user_pool: Arc<Mutex<UserIdPool>>, // connection_handler: [ConnectionHandler]
}

trait PortConvertable {
    fn with_port(&self, port: u16) -> SocketAddrV4;
}

impl PortConvertable for std::net::Ipv4Addr {
    fn with_port(&self, port: u16) -> SocketAddrV4 {
        SocketAddrV4::new(*self, port)
    }
}

impl SocketHandler {
    pub fn new(config: &Config) -> std::io::Result<SocketHandler> {
        const LOCAL_HOST: Ipv4Addr = Ipv4Addr::LOCALHOST;

        let local_addr = SocketAddr::V4(LOCAL_HOST.with_port(config.local_port));
        let external_addr = SocketAddr::V4(LOCAL_HOST.with_port(config.external_port));
        let login_addr = SocketAddr::V4(LOCAL_HOST.with_port(config.local_port));

        let addrs = [
            local_addr,
            external_addr,
            login_addr,
        ];

        let listener = std::net::TcpListener::bind(&addrs[..]);

        listener.map(|listener| -> SocketHandler {
            SocketHandler {
                listener: listener,
                user_pool: Arc::new(Mutex::new(UserIdPool::new())),
            }
        })
    }

    // fn resume(&self) {
    //     for stream in listener.incoming() {
    //         match stream {
    //             Ok(stream) => {
    //                 self.handle_connection(stream);
    //             }
    //             Err(e) => { /* connection failed */ }
    //         }
    //     }
    // }

    // fn stop(&self) {}

    // fn handl_connection(&self, connection: TcpStream) {
    //     thread::spawn(|| ConnectionHandler::new(connection, UserIDPool));
    // }
}
