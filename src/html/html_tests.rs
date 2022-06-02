
#[cfg(test)]
mod html_tests {
    use std::io::Read;

    use crate::html::page;
    use crate::html::page_template::PageTemplate;
    use crate::html::page_template_rule::PageTemplateRule;

    #[test]
    fn page_pool_basic() {

        let rules: &[PageTemplateRule] = &[
            PageTemplateRule { 
                pattern: "TEST_HEADER",
                value: "SET VALUE" 
            }
        ];

        let test_template = r#"
            <!DOCTYPE html>
            <html>
            <body>
                <h2 align=middle>[[TEST_HEADER]]</h2>
            </body>
            </html>
            "#;

        let page_template = PageTemplate{
            buffer: test_template.to_string()
        };

        let page = page_template.apply_rules(rules);

        let mut page_string = String::new();

        page.to_request().body.read_to_string(&mut page_string);

        assert_eq!(page_string.as_str(), r#"
            <!DOCTYPE html>
            <html>
            <body>
                <h2 align=middle>SET VALUE</h2>
            </body>
            </html>
            "#)

    }
}