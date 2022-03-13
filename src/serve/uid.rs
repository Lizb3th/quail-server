// Some license stuff here

// A UID is a unique identifier that is used to link a user across multiple http sessions
// It is stored as a 128-bit number represented as an array of u8s

use std::collections::HashMap;
use std::string::String;

#[derive(Clone, PartialEq, Eq)]
pub struct Uuid {
    value: u128,
}

// "QSSK-XXXX-NNNN-7777-BBBB"

impl Uuid {

    const KEY_BUFFER_SIZE: usize = 25;

    // fn from_key(s: &str) -> Option<Uuid> {
    //     if s.len() >= 24 && !s.starts_with("QSSK-") {
    //         return None
    //     }

    //     let s = s.as_bytes();

    //     let mut buff: [u8; 16] = [0; 16];

    //     buff[0..3].copy_from_slice(&s[5..9]);
    //     buff[4..7].copy_from_slice(&s[5..9]);
    //     buff[8..11].copy_from_slice(&s[5..9]);
    //     buff[12..15].copy_from_slice(&s[5..9]);

    //     match std::str::from_utf8(&buff) {
    //         Ok(buff) => match u128::from_str_radix( &buff, 24) {
    //             Ok(v1) => match u128::from_str_radix( &buff, 24) {
    //                 Ok(v2) => match u128::from_str_radix( &buff, 24) {
    //                     Ok(v3) => {
    //                         Some([v1, v2, v3])
    //                     },
    //                     _ => None
    //                 },
    //                 _ => None
    //             },
    //             _ => None
    //         },
    //         _ => None
    //     }.map(|val|{
    //         Uuid{ value: val }
    //     })
    // }

    pub fn from_key(s: &str) -> Option<Uuid> {
        match u128::from_str_radix( &s, 16) {
        Ok(val) => Some(Uuid{ value: val }),
        _ => None
        }
    }



    // fn format_radix(mut x: u32, radix: u32) -> String {
    //     let mut result = vec![];
    
    //     loop {
    //         let m = x % radix;
    //         x = x / radix;
    
    //         // will panic if you use a bad radix (< 2 or > 36).
    //         result.push(std::char::from_digit(m, radix).unwrap());
    //         if x == 0 {
    //             break;
    //         }
    //     }
    //     result.into_iter().rev().collect()
    // }

    pub fn to_key(&self) -> String {
        format!("{}", &self.value)
    }

    // // keys start with QSSK-  0-9, A-P. 
    // fn to_key(&self, buffer: & mut [char; Self::KEY_BUFFER_SIZE]) {

    //     let intBuffer:[u8; 16] = [0; 16];

    //     self.value[0].to_be_bytes()
        
    //     // std::fmt::write(intBuffer, "{}")

    //     std::fmt::write(buffer, "{QSSK-{}-{}-{}-{}}", )
        
    //     //self.value.to_be_bytes()
    //     //buffer
    //     //buffer //= ['Q','S','S','K','-'];
    // }
}

pub struct UserSession {
    pub value: Uuid,
}

#[derive(Clone)]
pub struct UserId {
    pub value: u128,
    pub expires: std::time::Instant,
    pub nickName: String,
}

trait UidConv {
    fn to_uidkey(&self) -> Option<u128>;
}

impl UidConv for std::path::Component<'_> {
    fn to_uidkey(&self) -> Option<u128> {
        return Some( 7 )
    }
}


impl UserId {
    fn str(&self) -> String {
        let v:[u8; 8] = [0; 8];

        return "".to_string();
    }

    fn new(component: std::path::Component) -> Option<UserId> {
        let idStr = component.as_os_str().to_str();

        let id = idStr.and_then( |s|{
            u128::from_str_radix( s, 16).ok()
        });

        return id.map(|id|{
            UserId{ value: id, expires: std::time::Instant::now(), nickName: "name".to_string() }
        })
    }

}


pub struct UserIdPool {
    pool: Vec<UserSession>,
}

impl UserIdPool {

    pub fn new() -> UserIdPool {
        UserIdPool { pool: Vec::new() }
    }

    pub fn get(&self, component: std::path::Component) -> Option<UserSession> {
        
        let key = if let Some(id) = component.to_uidkey() { id } else {
            return None;
        };

        self.pool.iter().find(|id|{
            key == id.value.value
        }).map( |r|{
            UserSession{ value: Uuid{ value: r.value.value } }
        })
    } 

    pub fn add(&self, nick_name: &str) -> Option<UserSession> {
        return None
    }

    pub fn all(&self) -> &Vec<UserSession> {
        return &self.pool
    }
}

// type UidDictionary<T> = ;

