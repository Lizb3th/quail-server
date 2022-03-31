// use std::sync::{ Arc, Mutex };


// use crate::http::ResponseHandler;
// use crate::http::RequestGet;
// use crate::html::PagePool;
// use crate::html::PageKind;
// use crate::serve::UserIdPool;
// use crate::resources::Config;


// // impl<'a> PartialEq<PageKind<'a>> for PageKind<'a> {
// //     fn eq<'b>(&self, other: &PageKind<'b>) -> bool {
// //         match self {
// //             crate::html::PageKind::Login => match other { PageKind::Login => true, _=> false }
// //             crate::html::PageKind::LinkProvider(val) => {
// //                 match other {
// //                     PageKind::LinkProvider(valOther) => val == valOther,
// //                     _ => false,
// //                 }
// //             }
// //             crate::html::PageKind::Main => match other { PageKind::Main => true, _=> false }
// //             crate::html::PageKind::Logs => match other { PageKind::Logs => true, _=> false }
// //         }
// //     }
// // }

// #[cfg(test)]
// mod tests {
//     use std::{path::PathBuf, str::FromStr};

//     use crate::serve::UserIdPool;

//     // Note this useful idiom: importing names from outer (for mod tests) scope.
//     use super::*;

//     #[test]
//     fn test_basic() {

//         let mut config = Config::new();

//         let page_pool = Arc::new(Mutex::new(PagePool::new(Config::new()).unwrap()));
//         let id_pool = Arc::new(Mutex::new(UserIdPool::new()));


//         let handler = ResponseHandler::new(page_pool, id_pool, config);

//         let path = PathBuf::from_str("login").unwrap();

//         let rout = handler.process_path_login(&path).unwrap();

//         assert!(rout == RequestGet::Html(PageKind::Login))
//     }

//     #[test]
//     fn test_basic_2() {

//         let mut config = Config::new();

//         let page_pool = Arc::new(Mutex::new(PagePool::new(Config::new()).unwrap()));
//         let id_pool = Arc::new(Mutex::new(UserIdPool::new()));

//         let handler = ResponseHandler::new(page_pool, id_pool.clone(), config);

//         let path = PathBuf::from_str("login?user=go4wood").unwrap();

//         let rout = handler.process_path_login(&path).unwrap();

//         let ids = id_pool.as_ref().lock().unwrap().all();

//         // assert!(rout == RequestGet::Html(PageKind::LinkProvider("")))
//     }
// }


#[cfg(test)]
mod tests {

    use crate::serve::SocketHandler;
    use crate::resources::Config;
    use std::path::PathBuf;

    #[cfg(test)]
    fn get_login_page() {

        let mut testConfig = Config::new();

        testConfig.login_page_template_path = PathBuf::from("login.html");

        let socket_handler = SocketHandler::new(&testConfig).unwrap();

        // socket_handler.

    }
}