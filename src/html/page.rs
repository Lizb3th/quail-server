
use std::net::TcpStream;
use std::result::Result;

pub struct Page {
    buffer: String
}

impl Page {
    // fn new(data: &[u8]){
    //     Page{resource: data}
    // }

    fn serve(&self, stream: TcpStream) -> std::io::Result<()>{
        stream.write_all(&self.buffer.as_bytes());
        return Ok(());
    }
}
