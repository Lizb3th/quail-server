
use std::net::TcpStream;

use super::page_template::{ PageTemplate, PageTemplateRule };

#[allow(dead_code)]
pub struct Page<'a, 'b, 'c> {
    //buffer: String
    template: & 'a PageTemplate,
    rules: Vec<PageTemplateRule<'b,'c>>,
}

impl<'a,'b, 'c> Page<'a,'b, 'c> {

    pub fn new(template: &'a PageTemplate, rules: Vec<PageTemplateRule<'b,'c>>) -> Page<'a,'b,'c> {
        Page{ template: template, rules: rules }
    }

    // fn new(data: &[u8]){
    //     Page{resource: data}
    // }

    pub fn serve(&self, stream: TcpStream) -> std::io::Result<()>{
        //stream.write_all(&self.buffer.as_bytes());
        return Ok(());
    }
}
