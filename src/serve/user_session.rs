// Some license stuff here

// A UID is a unique identifier that is used to link a user across multiple http sessions
// It is stored as a 128-bit number represented as an array of u8s
use std::string::String;

#[derive(Clone, PartialEq, Eq)]
pub struct Uuid {
    value: u128,
}

impl Uuid {
    pub fn from_str(s: &str) -> Option<Uuid> {
        match u128::from_str_radix( &s, 16) {
            Ok(val) => Some(Uuid{ value: val }),
            _ => None
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}", self.value)
    }
}

// "QSSK-XXXX-NNNN-7777-BBBB"  < goal not there yet

// impl Uuid {

//     const KEY_BUFFER_SIZE: usize = 25;

//     pub fn from_key(s: &str) -> Option<Uuid> {
//         match u128::from_str_radix( &s, 16) {
//         Ok(val) => Some(Uuid{ value: val }),
//         _ => None
//         }
//     }

//     pub fn to_key(&self) -> String {
//         format!("{}", &self.value)
//     }
// }

pub struct UserSession {
    pub value: Uuid,
}

// type UidDictionary<T> = ;

