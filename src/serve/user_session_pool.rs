
// #[path = "user_session.rs"]
// mod user_session;

use super::UserSession;
use super::Uuid;


pub trait UuidConv {
    fn to_uuidkey(&self) -> Option<Uuid>;
}

impl UuidConv for std::path::Component<'_> {
    fn to_uuidkey(&self) -> Option<Uuid> {
        self.as_os_str().to_str().and_then(|s| {
            Uuid::from_str(s)
        })
    }
}

// A library of all the user sessions
pub struct UserSessionPool {
    pool: Vec<UserSession>,
}

impl UserSessionPool {

    pub fn new() -> UserSessionPool {
        UserSessionPool { pool: Vec::new() }
    }

    pub fn get(&self, component: std::path::Component) -> Option<UserSession> {
        
        let key = if let Some(id) = component.to_uuidkey() { id } else {
            return None;
        };

        self.pool.iter().find(|id|{
            key == id.value
        }).map( |r|{
            UserSession{ value: key }
        })
    } 

    pub fn add(&self, nick_name: &str) -> Option<UserSession> {
        return None
    }

    pub fn all(&self) -> &Vec<UserSession> {
        return &self.pool
    }
}
