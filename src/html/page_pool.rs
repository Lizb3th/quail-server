
use crate::resources::Config;
use crate::html::page_template::PageTemplate;
use crate::html::page_template_rule::PageTemplateRule;

use crate::html::page::Page;

use super::page_template_rule::PageTemplateRuleProvider;

#[derive(PartialEq)]
pub enum PageRequest<'a> {
    Login,
    LinkProvider(&'a str),
    Main,
    Logs,
}

// impl<'a> PageTemplateRuleProvider<'static, 'a> for PageRequest<'a> {
    
// }


pub struct PagePool {
    main_template: PageTemplate,
    login_template: PageTemplate,
}

impl PagePool {
    pub fn new(config: Config) -> std::io::Result<PagePool>  {

        let main_template = PageTemplate::from_file(config.main_page_template_path.as_path());
        let login_template = PageTemplate::from_file(config.login_page_template_path.as_path());

        return main_template.and_then(|main| -> std::io::Result<PagePool>{
            let login_template = PageTemplate::from_file(config.login_page_template_path.as_path());

            login_template.and_then(|login| -> std::io::Result<PagePool>{
                Ok(PagePool{ main_template: main,
                    login_template: login })
            })
        })
    }

    fn page<'a, 'b>(&'a self, kind: &'b PageRequest) -> Page {
        match kind {
            PageRequest::LinkProvider(address) => {
                return self.main_template.apply_rules(&[
                    PageTemplateRule{ pattern: "LINK_ADDRESS", value: &address },
                ]);
            }
            PageRequest::Login => todo!(),
            PageRequest::Main => todo!(),
            PageRequest::Logs => todo!(),
        }
    }
}
