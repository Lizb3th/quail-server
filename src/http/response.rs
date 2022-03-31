

const fn str_to_code_rep(buf: &str) -> u32 {
    return u32::from_ne_bytes([ buf.as_bytes()[0], buf.as_bytes()[1],
                                buf.as_bytes()[2], 32 ]);
}

const fn code_to_str(code: &StatusCode) -> &str {
    unsafe {
        let code =  std::mem::transmute::<&StatusCode, &u32>(code);
        let arr = std::mem::transmute::<&u32, &[u8; 4]>(code);
        let arr = std::mem::transmute::<&[u8; 4], &[u8; 3]>(arr);

        std::str::from_utf8_unchecked(arr)
    }
}

const fn code_to_str_ws(code: &StatusCode) -> &str {

    unsafe {
        let code =  std::mem::transmute::<&StatusCode, &u32>(code);
        let arr = std::mem::transmute::<&u32, &[u8; 4]>(code);

        std::str::from_utf8_unchecked(arr)
    }
}

#[repr(u32)]
pub enum StatusCode {
    // informational response
    Continue=str_to_code_rep("100"),// = ['1','0','0'],
    EarlyHint=str_to_code_rep("103"),// = ['1','0','3'],

    // success
    Ok=str_to_code_rep("200"),// = ['2','0','0'],
    PartialContent=str_to_code_rep("206"),// = ['2','0','6'],
    
    // redirection
    SeeOther=str_to_code_rep("303"),// = ['3','0','3'],
    NotModified=str_to_code_rep("304"),// = ['3','0','4'],
    TemporaryRedirect=str_to_code_rep("307"),// = ['3','0','7'],
    PermanentRedirect=str_to_code_rep("308"),// = ['3','0','8'],

    // client errors
    BadRequest=str_to_code_rep("400"),//  = ['4','0','0'],
    Forbidden=str_to_code_rep("403"),// = ['4','0','3'],
    NotFound=str_to_code_rep("404"),// = ['4','0','4'],
    MethodNotAllowed=str_to_code_rep("405"),// = ['4','0','5'],
    RequestTimeout=str_to_code_rep("408"),// = ['4','0','8'],
    Gone=str_to_code_rep("410"),// = ['4','1','0'],
    LengthRequired=str_to_code_rep("411"),// = ['4','1','1'],
    PreconditionFailed=str_to_code_rep("412"),// = ['4','1','2'],
    PayloadTooLarge=str_to_code_rep("413"),// = ['4','1','3'],
    URITooLong=str_to_code_rep("414"),// = ['4','1','4'],
    UnsupportedMediaType=str_to_code_rep("415"),// = ['4','1','5'],
    RangeNotSatisfiable=str_to_code_rep("416"),// = ['4','1','6'],
    UpgradeRequired=str_to_code_rep("426"),// = ['4','2','6'],
    TooManyRequests=str_to_code_rep("429"),// = ['4','2','9'],
    RequestHeaderFieldsTooLarge=str_to_code_rep("431"),// = ['4','3','1'],

    // server errors
    InternalServerError=str_to_code_rep("500"),// = ['5','0','0'],
    NotImplemented=str_to_code_rep("501"),// = ['5','0','1'],
    BadGateway=str_to_code_rep("502"),// = ['5','0','2'],
    ServiceUnavailable=str_to_code_rep("503"),// = ['5','0','3'],
    GatewayTimeout=str_to_code_rep("504"),// = ['5','0','4'],
    HTTPVersionNotSupported=str_to_code_rep("505"),// = ['5','0','5'],
}

impl StatusCode {
    pub fn as_str(&self) -> &str {
        code_to_str(&self)
    }

    pub fn as_str_ws(&self) -> &str{
        code_to_str_ws(&self)
    }
}

pub struct StatusLine {
    pub version: String,
    pub status_code: StatusCode, //[u8; 3],
    pub reason_phrase: String,
}

pub struct HeaderFields {
    pub headers: Vec<String>,
}

pub struct Response<F> where F: std::io::Read {
    pub status_line: StatusLine,
    pub header_fields: HeaderFields,
    pub body: F,
}

impl<F> Response<F> where F: std::io::Read {
    pub fn get_body(& mut self) -> &mut F {
        return &mut (self.body)
    }
}