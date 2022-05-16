use std::{ net::TcpStream, io::Write };

use crate::serve::ConnectionHandler;

use super::response::Response;


// Handle response by streaming it to a stream
pub struct ResponseHandler {
    // connection_handler: ConnectionHandler
}

impl ResponseHandler {
    pub fn new() -> ResponseHandler {
        ResponseHandler{}
        // ResponseHandler{ connection_handler: ConnectionHandler:: }
    }

    pub fn handle<R: std::io::Read, 
              W: std::io::Write>(mut response: Response<R>,
                                stream: &mut W) -> std::io::Result<()> {
        //Status Line
        let status_line = response.status_line;
        write!(stream, "HTTP/{} {}{}\r\n", status_line.version,
               status_line.status_code.as_str_ws(), status_line.reason_phrase).and_then(|_|{
            for header in response.header_fields.headers {
                match write!(stream, "{}\r\n", header) {
                    Err(e) => return Err(e),
                    _=>{}
                }
            }
            write!(stream, "\r\n");
            Ok(())
        }).and_then(|_|{
            std::io::copy(&mut response.body, stream).map(|_|{()})
        })
        
    }
}