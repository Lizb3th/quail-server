
use std::collections::HashMap;
use std::io::Read;
use std::option::Option;
use std::net::TcpStream;
use std::path::PathBuf;
use std::ptr::NonNull;
use std::string::String;

use crate::resources::Config;
use crate::html::PageKind;

const WS: u8 = 32;

struct Headers {
    storage: HashMap<String, String>,
}

impl Headers {
    fn from_stream<R: std::io::BufRead>(reader: R) -> Option<Headers> {
        
        let mut foo = HashMap::new();

        reader.lines().try_fold(foo, |mut acc, line|{
            let read_result = line.and_then(|line|{
                let f = line.split_once(':').map(|(name, val)|{
                    (name.trim().to_string(), val.trim().to_string())
                });

                match f {
                    Some(hashMap) => Ok(hashMap),
                    _ => Err(std::io::Error::new(std::io::ErrorKind::InvalidData, ""))
                }
            });
            
            // let f = read_result.map(|f|{
            //     let (name, val) = f;
            //     acc.insert(name, val);
            //     acc
            // });

            // return f;

            match read_result {
                Ok(f) => {
                    let (name, val) = f;
                    acc.insert(name, val);
                    Some(acc)
                }
                _ => None
            }
        }).map(|hashMap|{
            Headers{ storage: hashMap }
        })

        // None
        // Headers{ storage: HashMap::new() }
    }

    fn get_host(&self) -> Option<&str> {
        return None
    }

}

#[derive(PartialEq)]
pub enum RequestGet<'a> {
    Html(PageKind<'a>),
    BinaryFile,
    TextFile,
}

#[derive(PartialEq, Eq)]
pub enum RequestMethod {
    Get,
    Head,
    Post,
    Put,
    Delete,
    Connect,
    Options,
    Trace,
}

const GET_T:[u8; 7] =     [71, 69, 84,  0,  0,  0,  0];  // GET
const HEAD_T:[u8; 7] =    [72, 69, 65, 68,  0,  0,  0]; // HEAD
const POST_T:[u8; 7] =    [80, 79, 83, 84,  0,  0,  0]; // POST
const PUT_T:[u8; 7] =     [80, 85, 84,  0,  0,  0,  0]; // PUT
const DELETE_T:[u8; 7] =  [68, 69, 76, 69, 84, 69,  0]; // DELETE
const CONNECT_T:[u8; 7] = [67, 79, 78, 78, 69, 67, 84]; // CONNECT
const OPTIONS_T:[u8; 7] = [79, 80, 84, 73, 79, 78, 83]; // OPTIONS
const TRACE_T:[u8; 7] =   [84, 82, 65, 67, 69,  0,  0]; // TRACE

impl RequestMethod {



    // reads the RequestMethod from a tcp stream and if successfull 
    // leaves the tcp stream at the end of the Request otherwise
    // leave the TcpStream where it was passed in.
    // if an io error occures then the tcp stream may not be where 
    // it is suposed to be
    // pub fn from_stream(stream: &mut TcpStream) -> Option<RequestMethod> {

    //     let mut buffer: [u8; 8] = [0; 8];

    //     let streamPeek = stream.peek(&mut buffer);

    //     let mut check = |txt: &[u8]| -> bool {
    //         if !buffer.starts_with(txt) {
    //             return false;
    //         }
    
    //         if buffer[txt.len()] != 10 {
    //             return false;
    //         }
            
    //         for i in [0..txt.len()] {
    //             Self::adv_stream::<1>(stream);
    //         }

    //         return true;
    //     };

    //     match streamPeek {
    //         Ok(_) => {
    //             if check(&GET_T) {
    //                 Some(Self::Get)
    //             } else if check(&POST_T) {
    //                 Some(Self::Post)
    //             } else if buffer.starts_with(&HEAD_T) {
    //                 Some(Self::Head)
    //             } else if buffer.starts_with(&PUT_T) {
    //                 Some(Self::Put)
    //             } else if buffer.starts_with(&DELETE_T) {
    //                 Some(Self::Delete)
    //             } else if buffer.starts_with(&CONNECT_T) {
    //                 Some(Self::Connect)
    //             } else if buffer.starts_with(&OPTIONS_T) {
    //                 Some(Self::Options)
    //             } else if buffer.starts_with(&TRACE_T) {
    //                 Some(Self::Trace)
    //             } else {
    //                 None
    //             }
    //         }
    //         _ => None
    //     }
    // }

    pub fn from_buffer<R: std::io::Read>(reader: &mut R) -> Option<RequestMethod> {

        let mut bytes = reader.bytes();

        let mut buffer = [0; 7];
        
        //let f = bytes.next();

        for u in buffer.iter_mut() {
            if let Some(Ok(byte)) = bytes.next() {
                if byte == WS {
                    break
                }
                
                *u = byte
            } else {
                return None
            }
        }

        // get the leftover bit
        if buffer[6] != 0 {
            if let Some(Ok(byte)) = bytes.next() {
                if byte != WS {
                    return None
                }
            }
        }

        if buffer.starts_with(&GET_T) {
            Some(Self::Get)
        } else if buffer.starts_with(&POST_T) {
            Some(Self::Post)
        } else if buffer.starts_with(&HEAD_T) {
            Some(Self::Head)
        } else if buffer.starts_with(&PUT_T) {
            Some(Self::Put)
        } else if buffer.starts_with(&DELETE_T) {
            Some(Self::Delete)
        } else if buffer.starts_with(&CONNECT_T) {
            Some(Self::Connect)
        } else if buffer.starts_with(&OPTIONS_T) {
            Some(Self::Options)
        } else if buffer.starts_with(&TRACE_T) {
            Some(Self::Trace)
        } else {
            None
        }
    }

    fn adv_stream<const count: usize>(stream: &mut TcpStream) {
        let mut buf: [u8; count] = [0; count];

        match stream.read(&mut buf) {
            Ok(_) => {} ,
            _ => {
                // if there is an erro here ther isn't much that 
                // can be done hopefully the next read will fail 
                // and then the connection will be terminated
            } 
        }
    }
}

pub struct Request {
    // path: PathBuf,
    method: RequestMethod,
    uri: Option<String>,
    version: String,
    headers: Headers,
}

impl Request {

    pub fn from_buffer<R: std::io::Read>(buffer: &mut R) -> Option<Request> {

        RequestMethod::from_buffer(buffer).and_then(|method| {
            Request::uri_from_stream(buffer).and_then(|uri|{
                Request::vesrion_from_stream(buffer).and_then(|version|{
                    // let mut buff = &mut 

                    let mut buff2 = std::io::BufReader::new(buffer);

                    Headers::from_stream(&mut buff2).map(|headers|{
                        Request{ 
                            method: method,
                            uri: Some(uri),
                            version: version,
                            headers: headers,
                        }
                    })
                })
            })
        })
    }

    fn uri_from_stream<R: std::io::Read>(buffer: &mut R) -> Option<String> {
        Some("".to_string())
    }

    fn vesrion_from_stream<R: std::io::Read>(buffer: &mut R) -> Option<String> {
        Some("".to_string())
    }

    pub fn effective_request_uri(&self) -> String {
        return "".to_string()
    }
}

