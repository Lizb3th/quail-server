
use crate::serve::Page;

pub struct ForwardingPage {
    pub page: Page,
}

impl ForwardingPage {

    pub fn new(generated_url: String) -> ForwardingPage {

        let body = "";

        return ForwardingPage{ page: Page{ title: "Main".to_string(), 
                                     body: body.to_string() } }
    }
}