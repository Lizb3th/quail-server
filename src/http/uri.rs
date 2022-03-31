
enum RequestTarget {
    origin(String),
    absolute,
    authority,
    asterisk,
}

impl RequestTarget {
    fn new(buffer: &[u8]) -> Option<RequestTarget> {
        
    }
}
