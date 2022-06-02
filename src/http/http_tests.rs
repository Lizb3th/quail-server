
use std::{net::UdpSocket, io::Read};

struct BodyBuffer {
    store: String,
    consumed: usize,
}

impl BodyBuffer {
    pub fn new(store: String) -> BodyBuffer {
        return BodyBuffer{ store: store, consumed: 0 };
    }
}

impl Read for BodyBuffer {

    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let mut subspan =  &self.store.as_bytes()[self.consumed..];
        subspan.read(buf).map(|count| {
            self.consumed += count;
            count
        })
    }
}

#[cfg(test)]
mod http_tests {

    // use crate::http::Request;

    use std::hash::BuildHasher;
    use std::io::{Write, Read};
    use std::net::{TcpStream, TcpListener, IpAddr, SocketAddr};
    use std::ops::DerefMut;
    use std::thread::{Builder};

    use crate::http::{Request, StatusCode, Response, StatusLine, HeaderFields, response_handler, ResponseHandler};
    use crate::http::request::RequestMethod;
    use crate::{serve::SocketHandler, resources::Config};

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::super::RequestGet;
    use super::BodyBuffer;

    // #[test]
    // fn some_test() {
    //     let sut = RequestGet::TextFile;
    //     assert_eq!(RequestGet::TextFile, sut);
    // }

    #[test]
    fn parse_header() {

        let addr = IpAddr::V4(std::net::Ipv4Addr::LOCALHOST);

        let socket = SocketAddr::new(addr, 55501);

        let request: Option<Request> = None;

        let mut request = std::sync::Arc::new(std::sync::Mutex::new(request));

        let other = request.clone();

        let mut test_listener = TcpListener::bind(socket).expect("couldn't Bind");

        test_listener.set_nonblocking(true).expect("couldn't non-block");;

        let thread = Builder::new().spawn(move||{
        
            std::thread::sleep(std::time::Duration::new(1,0));

            let (mut stream,_) = test_listener.accept().expect("couldn't accept");
        
            let mut lock = other.lock().expect("couldn't lock");
            
            let ref_p = lock.deref_mut();
            *ref_p = Request::from_buffer(&mut stream);
        
        }).expect("couldn't spawn");

        // std::thread::sleep(std::time::Duration::new(1,0));

        let mut test_connection = TcpStream::connect_timeout(
            &socket, std::time::Duration::new(1,0)
        ).expect("couldn't connect");

        let buf = "GET foo HTTP 1.1\r\nHost : 10.1.1.1";

        test_connection.write(buf.as_bytes());

        let request = thread.join().expect("couldn't join");

    }

    #[test]
    fn parse_request_method() {
        let mut buf = "GET ".as_bytes();

        let parse_get = RequestMethod::from_buffer(&mut buf).expect("Parse Failed");

        assert!(parse_get == RequestMethod::Get);

        let mut buf = "GET".as_bytes();

        assert!(RequestMethod::from_buffer(&mut buf).is_none());

        assert!(parse_get == RequestMethod::Get);

        let mut buf = "GET something".as_bytes();

        let parse_get = RequestMethod::from_buffer(&mut buf).expect("Parse Failed");

        assert!(parse_get == RequestMethod::Get);

        let mut buf = "GETS something".as_bytes();

        assert!(RequestMethod::from_buffer(&mut buf).is_none());
    }

    #[test]
    fn parse_request_version() {
        let mut buf = "GET ".as_bytes();

        let parse_get = RequestMethod::from_buffer(&mut buf).expect("Parse Failed");

        assert!(parse_get == RequestMethod::Get);

        let mut buf = "GET".as_bytes();

        assert!(RequestMethod::from_buffer(&mut buf).is_none());

        assert!(parse_get == RequestMethod::Get);

        let mut buf = "GET something".as_bytes();

        let parse_get = RequestMethod::from_buffer(&mut buf).expect("Parse Failed");

        assert!(parse_get == RequestMethod::Get);

        let mut buf = "GETS something".as_bytes();

        assert!(RequestMethod::from_buffer(&mut buf).is_none());
    }

    #[test]
    fn status_code() {
        assert!(StatusCode::Continue.as_str() == "100");
        assert!(StatusCode::Continue.as_str_ws() == "100 ");
    }

    #[test]
    fn test_response() {
        let test_status_line = StatusLine{
            version: "1.1".to_string(),
            status_code: StatusCode::Ok,
            reason_phrase: "Ok".to_string(),
        };

        let test_headers = HeaderFields{
            headers: Vec::new()
        };

        let test_response = Response{
            status_line: test_status_line,
            header_fields: test_headers,
            //body: BodyBuffer::new( "BODY".to_string()),
            body: Box::new( "BODY".as_bytes() ),
        };

        let mut test_buff = Vec::new();

        ResponseHandler::handle(test_response, &mut test_buff);

        let test_data = String::from_utf8(test_buff).expect("not utf8 convertable srting");

        let expected_data = "HTTP/1.1 200 Ok\r\n\
                                 \r\n\
                                 BODY";

        assert!(test_data == expected_data);

    }
}