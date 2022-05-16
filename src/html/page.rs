
use std::io::{ Read, Result, Cursor, BufWriter };
use std::net::TcpStream;
use crate::http::{ Response, StatusLine, HeaderFields, StatusCode, RequestGet };

use super::page_template::{ PageTemplate, PageTemplateRule };



#[allow(dead_code)]
pub struct Page {
    //buffer: String
    //template: & 'a PageTemplate,
    //rules: Vec<PageTemplateRule<'b,'c>>,

    // store: Box<[u8]>
    store: Box<dyn Read>,
    //reader: fn(&mut self, buf: &mut [u8]) -> Result<usize>;
}


impl Page {

    pub fn new<R: 'static>(buffer: R) -> Page where R: Read {
        Page { store: Box::new(buffer) }
    }

    // fn new(data: &[u8]){
    //     Page{resource: data}
    // }

    pub fn serve(&self, stream: TcpStream) -> Result<()>{
        //stream.write_all(&self.buffer.as_bytes());
        return Ok(());
    }

    pub fn to_request(self) -> Response<impl Read> {


        let body = self.store;

        return Response {
            status_line: StatusLine{
                version: "Http 1.1".to_string(),
                status_code: StatusCode::Ok,
                reason_phrase: "OK".to_string(),
            },
            header_fields: HeaderFields{
                headers: Vec::new()
            },
            body: body
        }
    }
}


struct PageBuffer {

}