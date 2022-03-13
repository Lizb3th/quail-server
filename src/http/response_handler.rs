
use std::path::{ Path, Component };
use std::option::Option;
use std::sync::{ Arc, Mutex };

use crate::html::PagePool;
use crate::html::PageKind;
use crate::http::Request;
use crate::http::request::RequestGet;
use crate::serve::UserIdPool;
use crate::serve::UserSession;
use crate::resources::Config;

/// 
enum PathRout {
    Login,
    Session,
    Guest,
    Logs,
    Icon,
    Unrecognised,
}

/// handles http request through tcp connection
pub struct ResponseHandler {
    page_pool: Arc<Mutex<PagePool>>,
    id_pool: Arc<Mutex<UserIdPool>>,
    config: Config,
}

impl ResponseHandler {

    pub fn new( page_pool: Arc<Mutex<PagePool>>, id_pool: Arc<Mutex<UserIdPool>>,
            config: Config) -> ResponseHandler {
        ResponseHandler{ page_pool: page_pool, id_pool: id_pool, config: config }
    }

    /// continue with the response
    fn resume(&self) {
        
    }

    fn parse_path_rout(&self, rout: Component) -> PathRout {

        let path = if let path = rout.as_os_str().to_str() { path } else {
            return PathRout::Unrecognised
        };

        let login_path = self.config.login_path.as_str();
        let session_path = self.config.session_path.as_str();
        let guest_path = self.config.session_path.as_str();
        let log_view_path = self.config.log_view_path.as_str();
        let icon = self.config.icon.as_str();

        match path {
            login_path => { PathRout::Login },
            session_path => { PathRout::Session },
            guest_path => { PathRout::Guest },
            log_path => { PathRout::Logs },
            icon => { PathRout::Icon },
            _ => { PathRout::Unrecognised },
        }
    }

    fn process_session(&self, id: Option<UserSession>, path: &Path) -> Option<RequestGet> {
        

        
        return Some(RequestGet::TextFile)
    }

    pub fn process_path_extern(&self, path: &Path) -> Option<RequestGet> {
        let mut path_components = path.components();
        
        let rout = path_components.next().map(|a|{
            self.parse_path_rout(a)
        });

        let out = match rout {
            Some(PathRout::Session) => { 
                let id = path_components.next().and_then (|a|{
                    self.id_pool.lock().unwrap().get(a)
                });

                let req = id.and_then(|a|{
                    self.process_session(Some(a), path_components.as_path())
                });

                req
            },

            _ => None
        };

        out
    }

    fn process_path_intern(&self, path: &Path) -> Option<RequestGet> {
        let mut path_components = path.components();
        
        let rout = path_components.next().map(|a|{
            self.parse_path_rout(a)
        });

        let out = match rout {
            Some(PathRout::Session) => { 
                let id = path_components.next().and_then (|a|{
                    self.id_pool.lock().unwrap().get(a)
                });

                let req = id.and_then(|a|{
                    self.process_session(Some(a), path_components.as_path())
                });

                req
            },
            Some(PathRout::Guest) => {
                self.process_session(None, path_components.as_path())
            },
            Some(PathRout::Logs) => { 
                Some(RequestGet::Html(PageKind::Logs))
            },
            _ => None
        };

        out
    }

    fn process_login_query<'a>(&self, path: &'a Path) -> Option<&'a str> {
        path.as_os_str().to_str()
    }

    pub fn process_path_login<'a>(&self, path: &'a Path) -> Option<RequestGet<'a>> {
        let mut path_components = path.components();
        
        let rout = path_components.next().map(|a|{
            self.parse_path_rout(a)
        });

        let out = match rout {
            Some(PathRout::Login) => { 

                let pkind = match path_components.next() {
                    Some(path) => {
                        self.process_login_query(path_components.as_path()).map(|a|{
                            PageKind::LinkProvider(a)
                        })
                    },
                    None => Some(PageKind::Login)
                };

                pkind.map(|a|{
                    RequestGet::Html(a)
                })
            },
            Some(PathRout::Logs) => { 
                Some(RequestGet::Html(PageKind::Logs))
            },
            Some(PathRout::Icon) => {
                Some(RequestGet::BinaryFile)
            }
            _ => None
        };

        out
    }

    // fn process_login_path(Components)

    // fn get_user_id(&self, request: &Request) -> Option<UserId>{

    //     let path:&Path = request.path.as_path();

    //     let path_components = path.components();

    //     match path.next();

        

    //     if path.starts_with(self.config.session_path) {
    //         if path
    //     }
    // }
}