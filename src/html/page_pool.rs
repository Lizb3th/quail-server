
use crate::resources::Config;
use crate::html::page_template::{ PageTemplate,
                                  PageTemplateRule };

use crate::html::page::Page;

type Rule<'a, 'b> = PageTemplateRule<'a, 'b>;

#[derive(PartialEq, Debug)]
pub enum PageKind<'a> {
    Login,
    LinkProvider(&'a str),
    Main,
    Logs,
}

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

    fn page<'a, 'b>(&'a self, kind: &'b PageKind) -> Page<'a, 'static, 'b> {
        match kind {
            PageKind::LinkProvider(address) => {
                return self.main_template.apply_rule(vec![
                    Rule{ switch: "LINK_ADDRESS", replacement: &address },
                ]);
            }
            PageKind::Login => todo!(),
            PageKind::Main => todo!(),
            PageKind::Logs => todo!(),
        }
    }


}
