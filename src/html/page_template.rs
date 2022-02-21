
use crate::html::page::Page;

use std::path::Path;
//use page::Page;

pub struct PageTemplateRule<'a> {
    rule: &'a str,
    replacement: &'a str,
}

pub struct PageTemplate {
    buffer: String,
}

impl PageTemplate {

    pub fn from_file(path: &Path) -> std::io::Result<PageTemplate>{
        Ok(PageTemplate{ buffer: "".to_string() })
    }

    pub fn to_page(&self, rules: &[PageTemplateRule]) -> Page {
        return Page{ buffer: "".to_string() };
    }
}

pub enum PageTemplateKind {
    main,
    login,
}

pub trait PageTemplateProvider {
    
    fn get_template(&self, kind: PageTemplateKind) -> std::io::Result<PageTemplate>;
}

