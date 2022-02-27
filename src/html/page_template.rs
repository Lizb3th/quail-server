
use crate::html::page::Page;

use std::path::Path;
//use page::Page;

pub struct PageTemplateRule<'a, 'b> {
    pub switch: &'a str,
    pub replacement: &'b str,
}

#[allow(dead_code)]
pub struct PageTemplate {
    buffer: String,
}

impl PageTemplate {

    #[allow(dead_code)]
    pub fn from_file(path: &Path) -> std::io::Result<PageTemplate>{
        Ok(PageTemplate{ buffer: "".to_string() })
    }

    #[allow(dead_code)]
    pub fn apply_rule<'a, 'b, 'c>(&'a self, rules: Vec<PageTemplateRule<'b, 'c>>) -> Page<'a, 'b, 'c> {
        return Page::new(&self, rules) //Page{ buffer: "".to_string() };
    }
}
