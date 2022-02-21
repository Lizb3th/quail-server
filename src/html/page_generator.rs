
use std::option::Option;

use crate::resources::Config;
use crate::html::page_template::{ PageTemplate,
                                  PageTemplateProvider,
                                  PageTemplateKind,
                                  PageTemplateRule };

use crate::html::page::Page;

use std::path::Path;
//use resources::Config;

enum PageKind {
    Login,
    LinkProvider(String),
    Main,
    Logs,
}

// Storage for page templates
struct PageCache {
    main: PageTemplate,
    login: PageTemplate,
}

impl PageCache {
    fn new(main: &Path, login: &Path) -> std::io::Result<PageCache> {
        return Ok(PageCache{ main: PageTemplate{ buffer: "".to_string() },
                             login: PageTemplate{ buffer: "".to_string() }});
    }
}

impl PageTemplateProvider for PageCache {
    fn get_template(&self, kind: PageTemplateKind) -> std::io::Result<PageTemplate> {
        match kind {
            main => Ok(self.main),
            login => Ok(self.login)
        }
    }
}

struct PageTemplateLoader<'a> {
    main: &'a Path,
    login: &'a Path,
}

impl<'a> PageTemplateProvider for PageTemplateLoader<'a> {
    fn get_template(&self, kind: PageTemplateKind) -> std::io::Result<PageTemplate> {
        match kind {
            main => PageTemplate::from_file(self.main),
            login => PageTemplate::from_file(self.login),
        }
    }
}

struct PageGenerator {
    // templateProvider: Option<PageCache>
    templateProvider: dyn PageTemplateProvider,
}

impl PageGenerator {
    fn new(config: Config) -> std::io::Result<PageGenerator> {

        let provider: dyn PageTemplateProvider = if config.cache_page_templates {
            PageCache{ main: config.main_page_template_path,
                       login: config.login_page_template_path }
        } else {
            PageTemplateLoader{ main: config.main_page_template_path,
                                login: config.login_page_template_path }
        };

        Ok(PageGenerator{ templateProvider: provider })
    }

    const login_rules: &'static [PageTemplateRule<'static>] = [
        PageTemplateRule{ rule: "",
                          replacement: "" },           
    ];

    fn generate(&self, kind: PageKind) -> Page {
        match kind {
            PageKind::LinkProvider(address) => {
                let template = self.templateProvider.get_template(PageTemplateKind::login);
                return template.to_page([""])
            }
            PageKind::Login => todo!(),
            PageKind::Main => todo!(),
            PageKind::Logs => todo!(),
        }
    }
}