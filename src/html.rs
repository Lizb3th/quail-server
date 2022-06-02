
pub mod page_pool;

#[cfg(test)]
mod html_tests;


pub mod page_template;
pub mod page;
pub mod page_template_rule;


pub type PagePool = page_pool::PagePool;
pub type PageKind<'a> = page_pool::PageRequest<'a>;
