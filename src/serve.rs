#[cfg(test)]
mod user_session_tests;

mod user_session;
mod user_session_pool;
mod socket_handler;
mod connection_handler;

pub type UserSessionPool = user_session_pool::UserSessionPool;
pub type SocketHandler = socket_handler::SocketHandler;

// pub type UserSession = user_session::UserSession;
pub use user_session::UserSession;
pub use user_session::Uuid;
pub use connection_handler::ConnectionHandler;