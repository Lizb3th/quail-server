
use crate::html::page::Page;

use std::path::Path;
//use page::Page;

pub trait PageTemplateRule {

    fn pattern(&self) -> &str;
    fn value(&self) -> &str;
}

#[allow(dead_code)]
pub struct PageTemplate {
    buffer: String,
    // internal: Box<Data>
}

/// 
/// 
/// ``` HTML
/// <html>
/// <header><title>[[title]]</title></header>
/// <body>
/// [[body]]
/// </body>
/// </html>
/// ```


impl PageTemplate {

    #[allow(dead_code)]
    pub fn from_file(path: &Path) -> std::io::Result<PageTemplate>{
        Ok(PageTemplate{ buffer: "".to_string() })
    }

    // #[allow(dead_code)]
    // pub fn from_slice(s: &str) -> std::io::Result<PageTemplate>{
    //     Ok(PageTemplate{ buffer: s.to_string() })
    // }

    #[allow(dead_code)]
    pub fn from_String(buffer: String) -> std::io::Result<PageTemplate>{
        Ok(PageTemplate{ buffer: buffer })
    }

    #[allow(dead_code)]
    pub fn apply_rules<'b,'c>(&self, rules: dyn &[PageTemplateRule]) -> Page {
        let bytes = rules.iter().fold( self.buffer.clone(),
            |acc, rule|
                acc.replace(rule.pattern, rule.value) ).as_bytes();

        Page::new( bytes )
    }

    pub fn apply_rules<'b,'c>(&self, rules: [dyn PageTemplateRule]) -> Page {
        let bytes = rules.iter().fold( self.buffer.clone(),
            |acc, rule|
                acc.replace(rule.pattern, rule.value) ).as_bytes();

        Page::new( bytes )
    }
}
