// Some license stuff here

// A UID is a unique identifier that is used to link a user across multiple http sessions
// It is stored as a 128-bit number represented as an array of u8s

use std::collections::HashMap;

struct Uid {
    value: u128,
}

impl Uid {
    fn new() {

    }

    fn str(&self) -> String {
        let v:[u8] = [0; 8];

        
    }
}

struct UidDictionary<T> {
    value: HashMap<Uid, T>
}

// type UidDictionary<T> = ;

