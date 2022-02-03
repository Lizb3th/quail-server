
use crate::serve::Page;

#[allow(dead_code)]
pub struct MainPage {
    pub page: Page,
}

impl MainPage {
    #[allow(dead_code)]
    pub fn new() -> MainPage {
        return MainPage{ page: Page{ title: "Main".to_string(), 
                                     body: "Hello World".to_string() } }
    }
}