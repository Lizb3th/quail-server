// use std::fmt::format;
use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown, ToSocketAddrs};
use std::io::{Read, Write};
use std::time::{self, UNIX_EPOCH, Duration};
use std::sync::{Arc, Mutex};
use std::fs::File;

mod serve;
use serve::MainPage;


struct Temperature {
    degrees: u64,
}

struct TestData{
    // data: String,
    mtx: Mutex<String>,
}

// static TEST_DATA: test_data = test_data{ data: "Time,Temp (Degrees C),Pressure (hPa)\n\
//                                                 1642975995,25.4,1015\n\
//                                                 ".to_string(),
//                                           mtx: Mutex::new(0) };

struct DataReader {
    #[allow(dead_code)]
    ptr: Arc<TestData>,
}

struct DataWriter {
    ptr: Arc<TestData>,
}

impl DataWriter {

    fn new(temp: Temperature) -> DataWriter {
        let time = time::SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        let s = format!("{}\n{},{}\n", "time,temp(C)", time, temp.degrees);

        return DataWriter{ptr: Arc::new(TestData{ mtx: Mutex::new(s) })};
    }

    fn get_reader(&self) -> DataReader {
        return DataReader{ptr: self.ptr.clone()};
    }

    fn write(&self, temp: Temperature) {
        let time = time::SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        
        let mut guard = self.ptr.mtx.lock().unwrap();
        guard.push_str(format!("{},{}\n", time, temp.degrees ).as_str());
    }

    fn clone(&mut self) -> DataWriter {
        return DataWriter{ ptr: self.ptr.clone() };
    }
}

// struct Page {
//     title: String,
//     // header: String,
//     body: String,
// }

// impl Page {
//     fn new(title: &str) -> Page {
//         return Page{ title: title.to_string(),
//                      // header: String::new(),
//                      body: String::new() };
//     }

//     fn print(&self) -> String {
//         return format!("<html>\n\
//                         <header>\n\
//                         <title>{}</title>\n\
//                         </header>\n\
//                         <body>\n
//                         {}\n
//                         </body>\n\
//                         </html>\n", self.title, self.body)
//     }
// }



// fn get_headers(stream: TcpStream) -> HashMap<String,String> {
//     let mut data = [0 as u8; 50]; // using 50 byte buffer
//     while match stream.read(&mut data) {
//         Ok(_size) => {

//         },
//         Err(_) => {
//             println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
//             stream.shutdown(Shutdown::Both).unwrap();
//             false
//         }
//     } {}
// }

struct ConnectionRouter{
    #[allow(dead_code)]
    listener: TcpListener,
}

impl ConnectionRouter {
    fn bind<A: ToSocketAddrs>(addr: A, data_writer: DataWriter) -> ConnectionRouter {
        let listener = TcpListener::bind(addr).unwrap();

        println!("Server listening on port 3333");
        for server in listener.incoming() {
            match server {
                Ok(stream) => {
                    println!("New connection: {}", stream.peer_addr().unwrap());
                    let linkdata = data_writer.get_reader();
                    thread::spawn(move|| { 
                        // connection succeeded
                        handle_client(stream, linkdata)
                    });
                }
                Err(e) => {
                    println!("Error: {}", e);
                    /* connection failed */
                }
            }
        }

        ConnectionRouter{ listener: listener }
    }
}
trait HttpConsumer{
    fn add_headers(& mut self);

    fn add_body(& mut self);

    fn signal_error(& mut self);

    fn signal_end(& mut self);
}

struct ConnectionHandler{
    // consumer: dyn http_consumer,
}


impl ConnectionHandler{
    
    #[allow(dead_code)]
    fn new(stream: TcpStream, consumer: &dyn HttpConsumer) -> Option<ConnectionHandler>{
        
        let mut check_byte = [0 as u8; 1];

        stream.peek(&mut check_byte).ok().and_then(|_| {
            if check_byte.first() == Some(&0x16u8) {
                Self::handle_https(stream, consumer)
            } else {
                Self::handle_http(stream, consumer)
            }
        })
    }

    #[allow(dead_code)]
    fn handle_https(_: TcpStream, _: &dyn HttpConsumer) -> Option<ConnectionHandler> {
        
        // let mut recordHeader = [0 as u8; 5];

        // let size = stream.read(&mut recordHeader);

        // let handshake_size = size.ok().and_then(|s|{
        //     println!("0x16 Handshake Record");
        //     if s == 5 {
        //         print!("did not get rocord header");
        //         return None
        //     }
        //     if recordHeader[1] != 3 || recordHeader[1] != 1 {
        //         println!("protocol version is 3.1");
        //     }

        //     let mut size_buffer = [0 as u8; 8];
        //     size_buffer[6] = recordHeader[3];
        //     size_buffer[7] = recordHeader[4];

        //     Some(usize::from_be_bytes(size_buffer))
        // });
        
        // handshake_size.and_then(|s|{
        //     println!("handshake_size {}", s);

        //     stream.read(&mut recordHeader).ok()
        // })

        Some(ConnectionHandler{})
    }

    #[allow(dead_code)]
    fn handle_http(_: TcpStream, _: &dyn HttpConsumer) -> Option<ConnectionHandler> {
        Some(ConnectionHandler{})
    }
}

fn get_headers(mut stream: &TcpStream) -> Option<String> {
    let mut data = [0 as u8; 50]; // using 50 byte buffer
    
    let mut output = String::new();
    
    while match stream.read(&mut data) {
        Ok(_size) => {
            // println!("Size: {}", _size);
            let as_str = String::from_utf8(data[.._size].to_vec()).unwrap();
            // println!("StrSize: {}", asStr.len());
            // println!("asStr {}", asStr);
            output += as_str.as_str();
            if as_str.is_empty() || as_str.contains("\r\n\r\n") {
                // println!("False");
                false
            } else {
                true
            }
        }
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}

    // println!("END");

    return Some(output);
}

fn handle_client(mut stream: TcpStream, _: DataReader) {

    let main_page = MainPage::new();

    let page_text = main_page.page.print();
    let data2 = page_text.as_bytes();

    let header_text = format!("HTTP/1.1 200 OK\n\
    Content-Type: text/html; charset=UTF-8\n\
    Content-Length: {}\n\n", data2.len());

    let header_data = header_text.as_bytes();

    let headers = get_headers(&stream).unwrap();

    print!("{}", headers);

    if headers.contains("favicon.ico") {
        print!("icon!");
        let mut f = File::open("C:\\Users\\Beth\\Documents\\playground\\rusting\\quail_server\\target\\debug\\quol.ico").unwrap();
        let mut ico_file = Vec::<u8>::new();
        let _ = f.read_to_end(& mut ico_file).unwrap();
    
        let header_text = format!("HTTP/1.1 200 OK\n\
        Content-Type: image/x-ico
        Content-Length: {}\n\n", ico_file.len());

        let header_data = header_text.as_bytes();

        //let mut buffer = [0; 10];  
        stream.write(&header_data[0..header_data.len()]).unwrap();
        let _ = stream.write(&ico_file);
    } else {
        stream.write(&header_data[0..header_data.len()]).unwrap();
        stream.write(&data2[0..data2.len()]).unwrap();
        //stream.flush().unwrap();
    }

    return;

    #[allow(unreachable_code)]
    {
    let mut data = [0 as u8; 50]; // using 50 byte buffer
    while match stream.read(&mut data) {
        Ok(_size) => {

            let message = String::from_utf8(data.to_vec());

            if message.unwrap().starts_with("GET /favicon.ico") {
                stream.write(&header_data[0..header_data.len()]).unwrap();
                print!("icon!");
                
                //stream.shutdown(Shutdown::Both);
            } else {

                // print!("PATH:{{{}}}\n", get_path(message.as_str()).unwrap());
                print!("Respond: {}\n", String::from_utf8(data.to_vec()).unwrap());
                //print!("Respond: {:02x?}\n", data);
                // echo everything!
                stream.write(&header_data[0..header_data.len()]).unwrap();
                stream.write(&data2[0..data2.len()]).unwrap();
                stream.flush().unwrap();
                //stream.shutdown(Shutdown::Both);
                // strem.write([0]);
                // let mut eof = [5 as u8; 1];
                // stream.write(&eof);
            }
            true
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
    print!("Thread End")
}
}

fn main() {

    let mut writer = DataWriter::new(Temperature{ degrees: 25 });

    let off_thread_writer = writer.clone();
    thread::spawn(move||{
        for _ in 0..100 {
            thread::sleep(Duration::from_secs(1));
            off_thread_writer.write(Temperature{ degrees: 25})
        }
    });

    let router = ConnectionRouter::bind("127.0.0.1:3333", writer);

    // let listener = TcpListener::bind("127.0.0.1:3333").unwrap();
//    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    // accept connections and process them, spawning a new thread for each one
//    println!("Server listening on port 3333");
    // for server in listener.incoming() {
    //     match server {
    //         Ok(stream) => {
    //             println!("New connection: {}", stream.peer_addr().unwrap());
    //             let linkdata = writer.get_reader();
    //             thread::spawn(move|| { 
    //                 // connection succeeded
    //                 handle_client(stream, linkdata)
    //             });
    //         }
    //         Err(e) => {
    //             println!("Error: {}", e);
    //             /* connection failed */
    //         }
    //     }
    // }

    // close the socket server
    drop(router);
}