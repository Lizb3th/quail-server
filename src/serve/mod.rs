pub mod page;
pub mod login_page;

use page::Page;

struct EmbededGraph {
    #[allow(dead_code)]
    pub page: Page,
}

impl EmbededGraph {
    #[allow(dead_code)]
    pub fn new() -> EmbededGraph {
        return EmbededGraph{ page: Page{ title: "Embeded".to_string(), 
                                         body: "".to_string() } }
    }
}

#[allow(dead_code)]
pub fn get_path(headers: &str) -> Option<&str> {
    return headers.split_once("GET ").and_then(|(_a,b)|{
        return b.split_once(" ").map(|(path, _s)|{
            return path;
        });
    });
}