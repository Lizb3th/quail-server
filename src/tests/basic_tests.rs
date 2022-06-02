
use std::net::{ TcpListener, TcpStream };

#[cfg(test)]
mod basic_tests {
    use std::io::Read;

    use crate::html::page;
    use crate::html::page_template::PageTemplate;
    use crate::html::page_template_rule::PageTemplateRule;
    use crate::serve::socket_handler::SocketHandler;

    /// simple_test:
    /// 
    /// A simple test that checks the server can serve a basic page
    ///
    #[test]
    fn simple_request() {

        let socket_handler = SocketHandler::new();

        let listener = TcpStream::connect("127.0.0.1:34254");
    }
}