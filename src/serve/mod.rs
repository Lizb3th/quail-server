pub struct Page {
    pub title: String,
    // header: String,
    pub body: String,
}

impl Page {
    #[allow(dead_code)]
    pub fn new(title: &str) -> Page {
        return Page{ title: title.to_string(),
                     // header: String::new(),
                     body: String::new() };
    }

    pub fn print(&self) -> String {
        return format!("<html>\n\
                        <header>\n\
                        <title>{}</title>\n\
                        </header>\n\
                        <body>\n
                        {}\n
                        </body>\n\
                        </html>\n", self.title, self.body)
    }
}

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