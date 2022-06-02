
/// PageTemplateRule
/// A simple holder for a page template pattern and a value to replace it
/// 
// pub trait PageTemplateRule {
//     fn pattern(&self) -> &str;
//     fn value(&self) -> &str;
// }

pub struct PageTemplateRule<'a,'b> {
    pub pattern: &'a str,
    pub value: &'b str, 
}

pub trait PageTemplateRuleProvider<'a,'b> {
    fn to_rules() -> [PageTemplateRule<'a,'b>];
}

pub trait PageTemplateRuleSlice {
    
}