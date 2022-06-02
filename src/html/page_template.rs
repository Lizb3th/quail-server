
use crate::html::page::Page;
use crate::html::page_template_rule::PageTemplateRule;

use std::path::Path;
//use page::Page;

/// PageTemplate
/// A template for an HTML Page.
/// stores a buffer and can be combined with a rule to create an HTML Page.


/// Example
/// ``` HTML
/// <html>
/// <header><title>[[title]]</title></header>
/// <body>
/// [[body]]
/// </body>
/// </html>
/// ```


#[allow(dead_code)]
pub struct PageTemplate {
    pub(crate) buffer: String,
    // internal: Box<Data>
}

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
    pub fn apply_rules<'a,'b>(&self, rules: &[PageTemplateRule<'a,'b>]) -> Page {
        let page_buffer = rules.iter().fold( self.buffer.clone(),
            |acc, rule| {
                let pattern = format!("[[{}]]", rule.pattern);
                acc.replace(&pattern, rule.value)
            });

        Page::new( page_buffer )
    }

    // pub fn apply_rules<'b,'c>(&self, rules: [PageTemplateRule<'c>]) -> Page {
    //     let bytes = rules.iter().fold( self.buffer.clone(),
    //         |acc, rule|
    //             acc.replace(rule.pattern, rule.value) ).as_bytes();

    //     Page::new( bytes )
    // }
}
