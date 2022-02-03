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
