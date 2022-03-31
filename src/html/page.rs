
use std::io::{ Read, Result, Cursor };
use std::net::TcpStream;
use crate::http::{ Response, StatusLine, HeaderFields, StatusCode };

use super::page_template::{ PageTemplate, PageTemplateRule };

#[allow(dead_code)]
pub struct Page<'a, 'b, 'c> {
    //buffer: String
    //template: & 'a PageTemplate,
    //rules: Vec<PageTemplateRule<'b,'c>>,

    store: std::Box<str>
}

struct PageWriter {
    data: Vec<u8>
}

// impl Fn<()> for PageWriter {
//     type Output = [u8];

//     pub fn call(&self) -> [u8] {
//         *(self.data)
//     }
// }

impl<'a,'b,'c> Page<'a,'b,'c> {

    pub fn new(template: &'a PageTemplate, rules: Vec<PageTemplateRule<'b,'c>>) -> Page<'a,'b,'c> {
        Page{ template: template, rules: rules }
    }

    // fn new(data: &[u8]){
    //     Page{resource: data}
    // }

    pub fn serve(&self, stream: TcpStream) -> Result<()>{
        //stream.write_all(&self.buffer.as_bytes());
        return Ok(());
    }

    pub fn to_request(&self) -> Response<impl Read> {


        let body = 

        return Response{
            status_line: StatusLine{
                version: "Http 1.1".to_string(),
                status_code: StatusCode::Ok,
                reason_phrase: "OK".to_string(),
            },
            header_fields: HeaderFields{
                headers: Vec::new()
            },
            body: Cursor::new(vec![0; 15]) //vec::Vec::new(),
        }
    }
}


struct PageBuffer {

}